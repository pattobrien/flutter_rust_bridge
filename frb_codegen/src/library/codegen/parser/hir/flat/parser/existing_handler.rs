use crate::codegen::parser::hir::flat::parser::flattener::SynItemWithMeta;
use crate::library::misc::consts::HANDLER_NAME;
use crate::utils::namespace::NamespacedName;
use anyhow::ensure;
use itertools::Itertools;

pub(super) fn parse_existing_handler(
    items: &[SynItemWithMeta],
) -> anyhow::Result<Option<NamespacedName>> {
    let existing_handlers = (items.iter())
        .filter(|item| parse_has_executor(&item.item))
        .map(|item| NamespacedName::new(item.meta.namespace.to_owned(), HANDLER_NAME.to_owned()))
        .collect_vec();
    ensure!(
        existing_handlers.len() <= 1,
        // frb-coverage:ignore-start
        // This will stop the whole generator and tell the users, so we do not care about testing it
        "Should have at most one custom handler (currently: {})",
        existing_handlers.iter().map(|x| x.rust_style()).join(", ")
    );
    // frb-coverage:ignore-end
    Ok(existing_handlers.first().cloned())
}

fn parse_has_executor(item: &syn::Item) -> bool {
    let code = quote::quote!(#item).to_string();
    code.contains(&format!("static {HANDLER_NAME}"))
}
