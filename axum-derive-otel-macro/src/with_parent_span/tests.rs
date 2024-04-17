#![cfg(test)]

use proc_macro2::TokenStream;
use quote::quote;

use super::macros;

#[test]
fn with_parent_span() {
    let before = quote! {
        pub async fn email(ExtractLocale(locale): ExtractLocale) -> Html<String> {
            let ctx = EmailTemplate { locale };

            Html(ctx.render_once().await.unwrap_or(String::new()))
        }
    };
    let expected = quote! {
        pub async fn email(
            axum_derive_otel::ExtractOtelContext(axum_derive_otel_context): axum_derive_otel::ExtractOtelContext,
            ExtractLocale(locale): ExtractLocale
        ) -> Html<String> {
            axum_derive_otel::OpenTelemetrySpanExt::set_parent(
                &axum_derive_otel::Span::current(),
                axum_derive_otel_context,
            );

            let ctx = EmailTemplate { locale };

            Html(ctx.render_once().await.unwrap_or(String::new()))
        }
    };

    let after = macros(quote! {}, before);

    assert_tokens_eq(&expected, &after);
}

fn assert_tokens_eq(expected: &TokenStream, actual: &TokenStream) {
    let expected = expected.to_string();
    let actual = actual.to_string();

    if expected != actual {
        println!(
            "{}",
            colored_diff::PrettyDifference {
                expected: &expected,
                actual: &actual,
            }
        );

        panic!("expected != actual");
    }
}
