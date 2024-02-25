pub(crate) mod attribute_parser;
pub(crate) mod function_extractor;
pub(crate) mod function_parser;
pub(crate) mod internal_config;
pub(crate) mod misc;
pub(crate) mod reader;
pub(crate) mod source_graph;
pub(crate) mod type_alias_resolver;
pub(crate) mod type_parser;

use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::dumper::Dumper;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;
use crate::codegen::misc::GeneratorProgressBarPack;
use crate::codegen::parser::function_extractor::extract_generalized_functions_from_file;
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::parser::misc::parse_has_executor;
use crate::codegen::parser::reader::CachedRustReader;
use crate::codegen::parser::source_graph::modules::{Enum, Struct, StructOrEnumWrapper};
use crate::codegen::parser::type_alias_resolver::resolve_type_aliases;
use crate::codegen::parser::type_parser::TypeParser;
use crate::codegen::ConfigDumpContent;
use itertools::Itertools;
use log::{trace, warn};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use syn::File;
use ConfigDumpContent::SourceGraph;

pub(crate) fn parse(
    config: &ParserInternalConfig,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<IrPack> {
    let rust_input_paths = &config.rust_input_path_pack.rust_input_paths;
    trace!("rust_input_paths={:?}", &rust_input_paths);

    let file_data_arr = read_files(
        rust_input_paths,
        &config.rust_crate_dir,
        cached_rust_reader,
        dumper,
        progress_bar_pack,
    )?;

    let pb = progress_bar_pack.parse_source_graph.start();
    let crate_map = source_graph::crates::Crate::parse(
        &config.rust_crate_dir.join("Cargo.toml"),
        cached_rust_reader,
        dumper,
    )?;
    dumper.dump(SourceGraph, "source_graph.json", &crate_map)?;
    drop(pb);

    let src_fns = file_data_arr
        .iter()
        .map(|file| extract_generalized_functions_from_file(&file.ast, &file.path))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect_vec();

    let src_structs = crate_map.root_module().collect_structs();
    let src_enums = crate_map.root_module().collect_enums();
    let src_types = resolve_type_aliases(crate_map.root_module().collect_types());

    let mut type_parser = TypeParser::new(src_structs.clone(), src_enums.clone(), src_types);
    let mut function_parser = FunctionParser::new(&mut type_parser);

    let ir_funcs = src_fns
        .iter()
        .enumerate()
        .map(|(index, f)| {
            function_parser.parse_function(
                &f.generalized_item_fn,
                &f.path,
                &config.rust_crate_dir,
                &config.force_codec_mode_pack,
                (index + 1) as i32,
                config.default_rust_opaque_codec,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        // to give downstream a stable output
        .sorted_by_cached_key(|func| func.name.clone())
        .collect_vec();

    let has_executor = (file_data_arr.iter()).any(|file| parse_has_executor(&file.content));

    let (struct_pool, enum_pool) = type_parser.consume();

    let ans = IrPack {
        funcs: ir_funcs,
        struct_pool,
        enum_pool,
        has_executor,
    };

    sanity_check_unused_struct_enum(
        &ans,
        &src_structs,
        &src_enums,
        &config.rust_input_path_pack,
        &config.rust_crate_dir,
    )?;

    Ok(ans)
}

struct FileData {
    path: PathBuf,
    content: String,
    ast: File,
}

fn read_files(
    rust_input_paths: &[PathBuf],
    rust_crate_dir: &Path,
    cached_rust_reader: &mut CachedRustReader,
    dumper: &Dumper,
    progress_bar_pack: &GeneratorProgressBarPack,
) -> anyhow::Result<Vec<FileData>> {
    let _pb = progress_bar_pack.parse_cargo_expand.start();
    let contents = rust_input_paths
        .iter()
        .map(|rust_input_path| {
            let content =
                cached_rust_reader.read_rust_file(rust_input_path, rust_crate_dir, dumper)?;
            Ok((rust_input_path.to_owned(), content))
        })
        .collect::<anyhow::Result<Vec<(PathBuf, String)>>>()?;

    contents
        .into_iter()
        .map(|(rust_input_path, content)| {
            let ast = syn::parse_file(&content)?;
            Ok(FileData {
                path: rust_input_path,
                content,
                ast,
            })
        })
        .collect()
}

fn sanity_check_unused_struct_enum(
    pack: &IrPack,
    src_structs: &HashMap<String, &Struct>,
    src_enums: &HashMap<String, &Enum>,
    rust_input_path_pack: &RustInputPathPack,
    rust_crate_dir: &PathBuf,
) -> anyhow::Result<()> {
    fn extract_interest_src_types<T: StructOrEnumWrapper<I>, I>(
        src_items: &HashMap<String, &T>,
        interest_input_paths: &[Namespace],
    ) -> Vec<String> {
        src_items
            .iter()
            .filter(|(_, v)| interest_input_paths.contains(&v.inner().namespace()))
            .map(|(k, _)| k.to_owned())
            .collect_vec()
    }

    let interest_input_paths = rust_input_path_pack
        .rust_input_paths
        .iter()
        .map(|p| Namespace::new_from_rust_crate_path(p, rust_crate_dir))
        .collect::<anyhow::Result<Vec<_>>>()?;

    let all_types: HashSet<String> = [
        extract_interest_src_types(src_structs, &interest_input_paths),
        extract_interest_src_types(src_enums, &interest_input_paths),
    ]
    .concat()
    .into_iter()
    .collect();

    let used_types: HashSet<String> = pack
        .distinct_types(None)
        .into_iter()
        .filter_map(|ty| match ty {
            IrType::StructRef(ty) => Some(ty.ident.0.name.clone()),
            IrType::EnumRef(ty) => Some(ty.ident.0.name.clone()),
            IrType::Delegate(IrTypeDelegate::PrimitiveEnum(ty)) => Some(ty.ir.ident.0.name.clone()),
            _ => None,
        })
        .collect();

    let unused_types = all_types.difference(&used_types).collect_vec();

    if !unused_types.is_empty() {
        warn!(
            "Some structs/enums are exported as `pub`, but are never used in any `pub` functions, thus they are ignored: {:?}",
            unused_types,
        )
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::codegen::config::internal_config::RustInputPathPack;
    use crate::codegen::config::internal_config_parser::compute_force_codec_mode_pack;
    use crate::codegen::dumper::Dumper;
    use crate::codegen::ir::ty::rust_opaque::RustOpaqueCodecMode;
    use crate::codegen::misc::GeneratorProgressBarPack;
    use crate::codegen::parser::internal_config::ParserInternalConfig;
    use crate::codegen::parser::parse;
    use crate::codegen::parser::reader::CachedRustReader;
    use crate::codegen::parser::source_graph::crates::Crate;
    use crate::codegen::parser::IrPack;
    use crate::utils::logs::configure_opinionated_test_logging;
    use crate::utils::test_utils::{
        create_path_sanitizers, get_test_fixture_dir, json_golden_test,
    };
    use log::info;
    use serial_test::serial;
    use std::path::{Path, PathBuf};

    #[test]
    #[serial]
    fn test_simple() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/simple", None)
    }

    #[test]
    #[serial]
    fn test_methods() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/methods", None)
    }

    #[test]
    #[serial]
    fn test_multi_input_file() -> anyhow::Result<()> {
        body(
            "library/codegen/parser/mod/multi_input_file",
            Some(Box::new(|rust_crate_dir| RustInputPathPack {
                rust_input_paths: [
                    rust_crate_dir.join("src/api_one.rs"),
                    rust_crate_dir.join("src/api_two.rs"),
                ]
                .into(),
            })),
        )
    }

    #[test]
    #[serial]
    fn test_use_type_in_another_file() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/use_type_in_another_file", None)
    }

    #[test]
    #[serial]
    fn test_qualified_names() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/qualified_names", None)
    }

    #[test]
    #[serial]
    fn test_non_qualified_names() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/non_qualified_names", None)
    }

    #[test]
    #[serial]
    fn test_generics() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/generics", None)
    }

    #[test]
    #[serial]
    fn test_unused_struct_enum() -> anyhow::Result<()> {
        body("library/codegen/parser/mod/unused_struct_enum", None)
    }

    #[test]
    #[serial]
    fn test_error_non_opaque_mut() -> anyhow::Result<()> {
        let result = execute_parse("library/codegen/parser/mod/error_non_opaque_mut", None);
        assert!(format!("{:#?}", result.err().unwrap())
            .contains("If you want to use `self`/`&mut self`"));
        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn body(
        fixture_name: &str,
        rust_input_path_pack: Option<Box<dyn Fn(&Path) -> RustInputPathPack>>,
    ) -> anyhow::Result<()> {
        let (actual_ir, rust_crate_dir) = execute_parse(fixture_name, rust_input_path_pack)?;
        json_golden_test(
            &serde_json::to_value(actual_ir)?,
            &rust_crate_dir.join("expect_ir.json"),
            &[],
        )?;

        Ok(())
    }

    #[allow(clippy::type_complexity)]
    fn execute_parse(
        fixture_name: &str,
        rust_input_path_pack: Option<Box<dyn Fn(&Path) -> RustInputPathPack>>,
    ) -> anyhow::Result<(IrPack, PathBuf)> {
        configure_opinionated_test_logging();
        let test_fixture_dir = get_test_fixture_dir(fixture_name);
        let rust_crate_dir = test_fixture_dir.clone();
        info!("test_fixture_dir={test_fixture_dir:?}");

        let crate_map = Crate::parse(
            &rust_crate_dir.join("Cargo.toml"),
            &mut CachedRustReader::default(),
            &Dumper(&Default::default()),
        )?;
        json_golden_test(
            &serde_json::to_value(crate_map)?,
            &rust_crate_dir.join("expect_source_graph.json"),
            &create_path_sanitizers(&test_fixture_dir),
        )
        .unwrap();

        let pack = parse(
            &ParserInternalConfig {
                rust_input_path_pack: rust_input_path_pack.map(|f| f(&rust_crate_dir)).unwrap_or(
                    RustInputPathPack {
                        rust_input_paths: vec![rust_crate_dir.join("src/api.rs")],
                    },
                ),
                rust_crate_dir: rust_crate_dir.clone(),
                force_codec_mode_pack: compute_force_codec_mode_pack(true),
                default_rust_opaque_codec: RustOpaqueCodecMode::Nom,
            },
            &mut CachedRustReader::default(),
            &Dumper(&Default::default()),
            &GeneratorProgressBarPack::new(),
        )?;

        Ok((pack, rust_crate_dir))
    }
}
