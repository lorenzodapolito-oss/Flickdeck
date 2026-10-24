//! Review › Thesaurus: synonyms from a small built-in English word list (`data/thesaurus.txt`),
//! looked up offline so it works in the browser build too. For the default language (Italian in
//! the desktop app) a MyThes thesaurus (`th_it_IT_v2.dat`) is read from the dictionary folders when
//! one is installed; the built-in list is the fallback.

const DATA: &str = include_str!("../data/thesaurus.txt");

/// One sense of a word: its headword and synonyms.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
pub struct Sense {
    pub word: String,
    pub synonyms: Vec<String>,
}

fn senses_of(head: &str) -> Vec<Sense> {
    DATA.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| l.split_once('|'))
        .filter(|(h, _)| h.trim() == head)
        .map(|(h, s)| Sense { word: h.trim().to_string(), synonyms: s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect() })
        .collect()
}

/// A MyThes thesaurus (`th_xx_YY_v2.dat`): headword lines `word|N` followed by `N` sense lines
/// `(part of speech)|synonym|synonym…`.
type MyThes = std::collections::HashMap<String, Vec<Sense>>;

/// The most headwords read from one thesaurus file, so a damaged or huge file can't use all memory.
const MAX_HEADWORDS: usize = 600_000;

fn parse_mythes(bytes: &[u8]) -> MyThes {
    let head_end = bytes.iter().position(|b| *b == b'\n').unwrap_or(bytes.len());
    let latin1 = String::from_utf8_lossy(bytes.get(..head_end).unwrap_or(&[])).to_ascii_uppercase().contains("8859-1");
    let text = if latin1 { bytes.iter().map(|b| char::from(*b)).collect::<String>() } else { String::from_utf8_lossy(bytes).into_owned() };
    let mut map = MyThes::new();
    let mut lines = text.lines().skip(1);
    while let Some(line) = lines.next() {
        let Some((head, n)) = line.rsplit_once('|') else { continue };
        let Ok(n) = n.trim().parse::<usize>() else { continue };
        let head = head.trim().to_lowercase();
        let mut senses = Vec::new();
        for _ in 0..n.min(64) {
            let Some(sense) = lines.next() else { break };
            let synonyms: Vec<String> = sense.split('|').skip(1).map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect();
            if !synonyms.is_empty() {
                senses.push(Sense { word: head.clone(), synonyms });
            }
        }
        if !senses.is_empty() && !head.is_empty() && map.len() < MAX_HEADWORDS {
            map.entry(head).or_default().extend(senses);
        }
    }
    map
}

/// The `th_*.dat` file for `lang` in `dirs`: `th_it_IT_v2.dat`, `th_it_IT.dat`, then any of the language.
fn find_mythes(dirs: &[std::path::PathBuf], lang: &str) -> Option<std::path::PathBuf> {
    let full = lang.replace('-', "_");
    if full.is_empty() || !full.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
        return None;
    }
    let base = full.split('_').next().unwrap_or("").to_string();
    for d in dirs {
        for stem in [&full, &base] {
            for name in [format!("th_{stem}_v2.dat"), format!("th_{stem}.dat")] {
                let p = d.join(name);
                if p.is_file() {
                    return Some(p);
                }
            }
        }
        if let Ok(rd) = std::fs::read_dir(d) {
            let mut names: Vec<String> = rd
                .filter_map(|e| e.ok()?.file_name().into_string().ok())
                .filter(|n| n.starts_with(&format!("th_{base}_")) && n.ends_with(".dat"))
                .collect();
            names.sort();
            if let Some(n) = names.first() {
                return Some(d.join(n));
            }
        }
    }
    None
}

/// The MyThes thesaurus installed for the default language, read once.
fn installed() -> Option<std::sync::Arc<MyThes>> {
    use std::sync::{Arc, Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<std::collections::HashMap<String, Option<Arc<MyThes>>>>> = OnceLock::new();
    let lang = crate::spell::default_language();
    let cache = CACHE.get_or_init(Default::default);
    if let Ok(c) = cache.lock()
        && let Some(t) = c.get(&lang)
    {
        return t.clone();
    }
    let loaded = find_mythes(&crate::spell::search_dirs(), &lang).and_then(|p| std::fs::read(p).ok()).map(|b| Arc::new(parse_mythes(&b)));
    if let Ok(mut c) = cache.lock() {
        c.insert(lang, loaded.clone());
    }
    loaded
}

/// The senses for `word`, trying simple English endings when the word itself isn't listed
/// ("goals" → "goal", "improved" → "improve", "growing" → "grow"). Synonyms take the word's
/// capitalisation ("Goal" → "Aim", "GOAL" → "AIM").
pub fn lookup(word: &str) -> Vec<Sense> {
    let w: String = word.trim().trim_matches(|c: char| !c.is_alphanumeric() && c != '-').chars().take(64).collect();
    let lower = w.to_lowercase();
    if lower.is_empty() {
        return vec![];
    }
    if let Some(th) = installed() {
        let found = th.get(&lower).cloned().unwrap_or_default();
        if !found.is_empty() {
            return found;
        }
    }
    let mut tries = vec![lower.clone()];
    for (suffix, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ied", "y"), ("ed", "e"), ("ed", ""), ("ing", "e"), ("ing", "")] {
        if let Some(stem) = lower.strip_suffix(suffix)
            && stem.chars().count() >= 2
        {
            tries.push(format!("{stem}{add}"));
        }
    }
    let Some(found) = tries.iter().map(|t| senses_of(t)).find(|v| !v.is_empty()) else { return vec![] };
    let upper = w.chars().count() > 1 && w.chars().all(|c| !c.is_lowercase());
    let title = !upper && w.chars().next().is_some_and(char::is_uppercase);
    found
        .into_iter()
        .map(|s| Sense {
            word: s.word,
            synonyms: s
                .synonyms
                .into_iter()
                .map(|x| {
                    if upper {
                        x.to_uppercase()
                    } else if title {
                        let mut c = x.chars();
                        c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
                    } else {
                        x
                    }
                })
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn senses_endings_and_case() {
        let g = lookup("goal");
        assert_eq!(g.len(), 1);
        assert!(g[0].synonyms.contains(&"objective".to_string()));
        assert_eq!(lookup("change").len(), 2, "verb and noun senses");
        assert_eq!(lookup("Goals")[0].synonyms[0], "Aim");
        assert_eq!(lookup("GOAL")[0].synonyms[0], "AIM");
        assert_eq!(lookup("improved")[0].word, "improve");
        assert_eq!(lookup("growing")[0].word, "grow");
        assert_eq!(lookup("strategies")[0].word, "strategy");
        assert_eq!(lookup("“goal,”")[0].word, "goal", "punctuation around the word");
        assert!(lookup("").is_empty() && lookup("zzxq").is_empty() && lookup("   ").is_empty());
        assert!(lookup(&"a".repeat(10_000)).is_empty());
    }

    #[test]
    fn data_is_well_formed() {
        for l in DATA.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            let (h, s) = l.split_once('|').unwrap_or_else(|| panic!("no `|`: {l}"));
            assert_eq!(h, h.trim().to_lowercase(), "{l}");
            let syn: Vec<&str> = s.split(',').map(str::trim).collect();
            assert!(syn.len() >= 2 && syn.iter().all(|x| !x.is_empty() && *x != h), "{l}");
            let mut d = syn.clone();
            d.sort();
            d.dedup();
            assert_eq!(d.len(), syn.len(), "duplicate synonym: {l}");
        }
    }

    #[test]
    fn commands_look_up_and_replace_the_word_at_the_caret() {
        use serde_json::json;
        let mut s = crate::Session::with_new();
        let id = s.execute("insert.textBox", &json!({"rect": [50, 50, 400, 40]})).unwrap()["id"].as_u64().unwrap();
        s.execute("text.edit", &json!({"id": id})).unwrap();
        s.execute("text.insert", &json!({"text": "Our Goal is growth."})).unwrap();
        s.execute("text.edit", &json!({"id": id, "at": [0, 6]})).unwrap();
        let r = s.execute("review.thesaurus", &json!({})).unwrap();
        assert_eq!(r["word"], "Goal");
        assert_eq!(r["senses"][0]["synonyms"][0], "Aim");
        s.execute("review.replaceWord", &json!({"with": "Aim"})).unwrap();
        let text = |s: &crate::Session| {
            s.doc().unwrap().shapes().iter().find(|x| x.id.0 as u64 == id).and_then(|x| x.text.as_ref()).map(|t| t.paragraphs[0].text()).unwrap()
        };
        assert_eq!(text(&s), "Our Aim is growth.");
        s.execute("edit.undo", &json!({})).unwrap();
        assert_eq!(text(&s), "Our Goal is growth.");
        assert_eq!(s.execute("review.thesaurus", &json!({"word": "growth."})).unwrap()["senses"][0]["word"], "growth");
        assert!(s.execute("review.replaceWord", &json!({"with": ""})).is_err());
    }

    #[test]
    fn a_mythes_file_is_read() {
        let th = parse_mythes(
            "UTF-8\ncasa|2\n(nome)|abitazione|dimora|focolare\n(nome)|ditta|azienda\nstrada|1\n(nome)|via|percorso\nrotto|zz\n".as_bytes(),
        );
        assert_eq!(th.get("casa").map(Vec::len), Some(2));
        assert_eq!(th["casa"][0].synonyms, ["abitazione", "dimora", "focolare"]);
        assert_eq!(th["strada"][0].synonyms[0], "via");
        assert!(!th.contains_key("rotto"));
        assert!(parse_mythes(b"").is_empty());
        let latin = parse_mythes(b"ISO8859-1\nperch\xe9|1\n(cong.)|poich\xe9\n");
        assert_eq!(latin["perché"][0].synonyms[0], "poiché");
    }

    #[test]
    fn a_thesaurus_is_found_by_language_in_the_folders() {
        let dir = std::env::temp_dir().join(format!("flickdeck-th-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        assert_eq!(find_mythes(std::slice::from_ref(&dir), "it-IT"), None);
        std::fs::write(dir.join("th_it_IT_v2.dat"), b"UTF-8\n").unwrap();
        assert_eq!(find_mythes(std::slice::from_ref(&dir), "it-IT"), Some(dir.join("th_it_IT_v2.dat")));
        assert_eq!(find_mythes(std::slice::from_ref(&dir), "it"), Some(dir.join("th_it_IT_v2.dat")));
        assert_eq!(find_mythes(std::slice::from_ref(&dir), "../etc"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
