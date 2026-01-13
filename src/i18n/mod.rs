use fluent_templates::{LanguageIdentifier, Loader, static_loader};
use crate::CURRENT_LANG;

static_loader! {
    static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US",
    };
}

#[derive(Clone)]
pub enum Lang {
    EN,
    FR,
    DE,
    IT,
}

pub fn i18n(key: &str) -> String {
    let lang = match CURRENT_LANG.get() {
        Lang::EN => "en-US",
        Lang::FR => "fr-FR",
        Lang::DE => "de-DE",
        Lang::IT => "it-IT",
    };
    let lang: LanguageIdentifier = lang.parse().unwrap();
    LOCALES
        .try_lookup(&lang, key)
        .unwrap_or_else(|| key.to_string())
}