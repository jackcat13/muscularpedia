use std::collections::HashMap;
use axum::extract::Request;
use axum::http::Uri;
use axum::middleware::Next;
use axum::response::Response;
use crate::CURRENT_LANG;
use crate::i18n::Lang;

pub async fn lang_middleware(req: Request, next: Next) -> Response {
    let uri: Uri = req.uri().clone();
    if let Some(query) = uri.query() {
        let params: HashMap<String, String> = to_query_map(query);

        if let Some(lang) = params.get("lang") {
            return CURRENT_LANG.scope(to_lang(lang), next.run(req)).await;
        }
    }

    CURRENT_LANG.scope(Lang::EN, next.run(req)).await
}

fn to_lang(lang: &str) -> Lang {
    match lang {
        "en" => Lang::EN,
        "fr" => Lang::FR,
        "de" => Lang::DE,
        "it" => Lang::IT,
        _ => Lang::EN
    }
}

fn to_query_map(query: &str) -> HashMap<String, String> {
    query.split('&')
        .filter_map(|pair| {
            pair.split_once('=')
                .map(|(k, v)| (k.to_string(), v.to_string()))
        })
        .collect()
}