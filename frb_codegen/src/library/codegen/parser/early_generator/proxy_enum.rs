use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::api_dart::spec_generator::class::proxy_variant;
use crate::codegen::ir::hir::flat::pack::HirFlatPack;
use crate::codegen::ir::mir::pack::MirPack;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegateProxyVariant};
use crate::codegen::ir::mir::ty::MirType;
use crate::if_then_some;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

pub(crate) fn generate(pack: &HirFlatPack, tentative_mir_pack: &MirPack) -> anyhow::Result<String> {
    let distinct_types = tentative_mir_pack.distinct_types(None);
    let proxy_variants = (distinct_types.iter())
        .filter_map(|ty| if_then_some!(let MirType::Delegate(MirTypeDelegate::ProxyVariant(inner)) = ty, inner.clone()))
        .collect_vec();

    let proxy_variants_of_enum =
        (proxy_variants.into_iter()).into_group_map_by(|ty| ty.proxy_enum.safe_ident());

    Ok((proxy_variants_of_enum.into_iter())
        .map(|(_, proxy_variants)| generate_proxy_enum(&proxy_variants))
        .join(""))
}

fn generate_proxy_enum(proxy_variants: &[MirTypeDelegateProxyVariant]) -> String {
    let proxy_enum_ty = *proxy_variants[0].proxy_enum.clone();
    let proxy_enum_dart_api_type =
        ApiDartGenerator::new(proxy_enum_ty.clone(), context).dart_api_type();

    let enum_name = format!("{proxy_enum_dart_api_type}ProxyEnum");

    let variants = ""; // TODO

    format!(
        "
    enum {enum_name} {{
        {variants}
    }}
    "
    )
}
