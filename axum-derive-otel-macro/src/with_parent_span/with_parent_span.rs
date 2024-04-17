use syn::{parse_quote, ItemFn};

pub fn transform(source_fn: ItemFn) -> ItemFn {
    let template_fn: ItemFn = parse_quote!(
        fn index(
            axum_derive_otel::ExtractOtelContext(axum_derive_otel_context): axum_derive_otel::ExtractOtelContext,
        ) {
            tracing::Span::current().set_parent(axum_derive_otel_context);
        }
    );

    transform_using_template(source_fn, template_fn)
}

fn transform_using_template(source_fn: ItemFn, template_fn: ItemFn) -> ItemFn {
    let mut source_fn = source_fn.clone();

    // add ExtractOtelContext input to source_fn
    let extract_otel_input = template_fn.sig.inputs.first().unwrap().clone();
    source_fn.sig.inputs.insert(0, extract_otel_input);

    source_fn
        .block
        .stmts
        .insert(0, template_fn.block.stmts.first().unwrap().clone());

    source_fn.to_owned()
}
