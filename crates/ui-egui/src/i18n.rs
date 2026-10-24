//! Interface language. Strings in the code stay English and are the lookup keys; the Italian
//! catalog (`i18n/it.tsv`: `English<TAB>Italiano`, escapes `\n`, `\t`, `\\`) maps them to display
//! text when the UI is drawn. Command ids, document text, file names, the control channel, the CLI
//! and MCP never see translated text. A string without a translation shows in English.
//!
//! The library starts in English (so tests drive the UI by its English labels). The desktop app
//! calls [`set_language`] at start-up: Italian by default, English with `FLICKDECK_LANG=en`.

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

static ITALIAN: AtomicBool = AtomicBool::new(false);

const IT_SOURCE: &str = include_str!("i18n/it.tsv");

/// Choose the interface language: `"it"` for Italian, anything else for English.
pub fn set_language(code: &str) {
    ITALIAN.store(code.trim().eq_ignore_ascii_case("it"), Ordering::Relaxed);
}

/// Whether the interface is currently Italian.
pub fn is_italian() -> bool {
    ITALIAN.load(Ordering::Relaxed)
}

/// Parse a catalog into English → translation. Lines starting with `#`, blank lines and lines
/// without a TAB are skipped; a translation that needs an escape is unescaped (and kept alive for
/// the life of the process, like the rest of the catalog).
fn parse(source: &'static str) -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();
    for line in source.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let mut cols = line.splitn(2, '\t');
        let (Some(key), Some(value)) = (cols.next(), cols.next()) else { continue };
        let value = value.trim_end_matches(['\r', '\n']);
        if key.is_empty() || value.is_empty() {
            continue;
        }
        let key: &'static str = if key.contains('\\') { Box::leak(unescape(key).into_boxed_str()) } else { key };
        let value: &'static str = if value.contains('\\') { Box::leak(unescape(value).into_boxed_str()) } else { value };
        map.insert(key, value);
    }
    map
}

fn unescape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('\\') => out.push('\\'),
            Some(other) => {
                out.push('\\');
                out.push(other);
            }
            None => out.push('\\'),
        }
    }
    out
}

fn italian() -> &'static HashMap<&'static str, &'static str> {
    static CATALOG: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    CATALOG.get_or_init(|| parse(IT_SOURCE))
}

/// The translation of an English UI string in the current language (the string itself when there
/// is none, or when the interface is English).
pub fn t(s: &str) -> &str {
    if !is_italian() {
        return s;
    }
    italian().get(s).copied().unwrap_or(s)
}

/// Translate a template and fill its `{…}` placeholders, in order, with `vals`. The English text is
/// the key (placeholders included); a translation keeps the same placeholders in the same order.
pub fn tf(template: &str, vals: &[&dyn std::fmt::Display]) -> String {
    let template = t(template);
    let mut out = String::with_capacity(template.len() + 16);
    let mut vals = vals.iter();
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open..];
        match after.find('}') {
            Some(close) => {
                if let Some(v) = vals.next() {
                    out.push_str(&v.to_string());
                }
                rest = &after[close + 1..];
            }
            None => {
                out.push_str(after);
                rest = "";
            }
        }
    }
    out.push_str(rest);
    out
}

/// Fill `{name}` placeholders in a translated template.
pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (name, value) in args {
        out = out.replace(&format!("{{{name}}}"), value);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_parses_without_losing_lines() {
        let map = parse(IT_SOURCE);
        let rows = IT_SOURCE.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).count();
        assert!(!map.is_empty());
        assert_eq!(map.len(), rows, "duplicate keys or malformed lines in it.tsv");
    }

    /// Every string the UI looks up with a literal `t("…")` has an Italian entry, so a new label
    /// can't silently stay English. Names, numbers and symbols are exempt.
    #[test]
    fn every_literal_lookup_is_in_the_italian_catalog() {
        let map = parse(IT_SOURCE);
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut missing = Vec::new();
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if !name.ends_with(".rs") || name.ends_with("_tests.rs") || name == "i18n.rs" {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let code = text.split("\n#[cfg(test)]").next().unwrap_or("");
            let needle = "crate::i18n::t(\"";
            let mut rest = code;
            while let Some(i) = rest.find(needle) {
                rest = &rest[i + needle.len()..];
                let mut end = 0;
                let bytes = rest.as_bytes();
                while end < bytes.len() && !(bytes[end] == b'"' && (end == 0 || bytes[end - 1] != b'\\')) {
                    end += 1;
                }
                let key = unescape(&rest[..end]);
                if key.chars().filter(|c| c.is_alphabetic()).count() >= 2 && !map.contains_key(key.as_str()) && !EXEMPT.contains(&key.as_str()) {
                    missing.push(format!("{name}: {key}"));
                }
            }
        }
        missing.sort();
        missing.dedup();
        assert!(missing.is_empty(), "{} strings without an Italian entry:\n{}", missing.len(), missing.join("\n"));
    }

    /// Looked-up strings that read the same in Italian (names, formats, symbols).
    const EXEMPT: &[&str] = &[
        "AaBbCcDd",
        "github.com/lorenzodapolito-oss/Flickdeck",
        "in",
        "pt",
        "command_palette",
        "RRGGBB",
        "File (Alt+F)",
        "Standard (4:3)",
        "Widescreen (16:9)",
        "Width: ",
        "Flickdeck.it",
        "PDF (*.pdf)",
        "Account",
        "Audio",
        "Video",
        "Volume",
        "Tab",
        "Zoom",
        "Trigger",
        "Layout",
        "Cameo",
        "Backspace",
        "Home",
        "File",
        "GIF",
        "PNG",
        "JPEG",
    ];

    #[test]
    fn unescape_handles_newlines_and_backslashes() {
        assert_eq!(unescape("a\\nb\\\\c"), "a\nb\\c");
    }

    #[test]
    fn tf_fills_placeholders_in_order_and_survives_odd_templates() {
        assert_eq!(tf("Slide {} of {}", &[&2, &9]), "Slide 2 of 9");
        assert_eq!(tf("No placeholders", &[&1]), "No placeholders");
        assert_eq!(tf("Open {name", &[&1]), "Open {name");
        assert_eq!(tf("{} {}", &[&"a"]), "a ");
    }

    #[test]
    fn fmt_fills_placeholders() {
        assert_eq!(fmt("Slide {n} of {m}", &[("n", "2"), ("m", "9")]), "Slide 2 of 9");
    }
}
