use std::collections::{BTreeMap, BTreeSet};

pub struct Suggest<'a> {
    patterns: BTreeMap<&'a str, Vec<String>>,
    words: BTreeSet<&'a str>,
    common_suffixes: Vec<String>,
}

impl<'a> Suggest<'a> {
    pub fn new() -> Self {
        let patterns_data = include_bytes!("../data/preprocessed-patterns.json");
        let words_data = include_str!("../data/source-words.txt");
        let common_data = include_bytes!("../data/source-common-patterns.json");

        let patterns = serde_json::from_slice(patterns_data).unwrap();
        let words = BTreeSet::from_iter(words_data.lines());
        let common_suffixes = serde_json::from_slice(common_data).unwrap();

        Suggest {
            patterns,
            words,
            common_suffixes,
        }
    }

    fn find_pattern(&self, input: &'a str) -> Option<(&&str, &Vec<String>)> {
        self.patterns
            .range(..=input)
            .rfind(|(k, _)| input.starts_with(*k))
    }

    fn prefix_exists(&self, input: &str) -> bool {
        self.words
            .range(input..format!("{}{}", input, char::MAX).as_str())
            .next()
            .is_some()
    }

    pub fn suggest(&self, input: &str) -> Vec<String> {
        let input = fix_string(input);

        let mut remaining = &input[..];
        let mut matched_strings = vec![String::new()];

        while !remaining.is_empty() {
            let Some((key, patterns)) = self.find_pattern(remaining) else {
                break;
            };
            remaining = &remaining[key.len()..];

            matched_strings = matched_strings
                .iter()
                .flat_map(|m| patterns.iter().map(|p| m.to_owned() + p))
                .filter(|m| self.prefix_exists(m))
                .collect::<Vec<_>>();

            let additional_matches = matched_strings
                .iter()
                .flat_map(|m| self.common_suffixes.iter().map(|p| m.to_owned() + p))
                .filter(|m| self.prefix_exists(m))
                .collect::<Vec<_>>();
            matched_strings.extend(additional_matches);
        }

        let suggestions: BTreeSet<_> = matched_strings.iter().map(|s| s.as_str()).collect();
        suggestions
            .intersection(&self.words)
            .map(|s| s.to_string())
            .collect()
    }
}

fn fix_string(s: &str) -> String {
    let s = s.trim();

    let mut result = String::new();
    let mut prev = ' '; // prev is non-alphabetic at first
    for c in s.chars() {
        // Fix string for o. In the beginning, after punctuations etc it should be capital O
        if (c == 'o' || c == 'O') && !prev.is_ascii_alphabetic() {
            result.push('O');
        } else if c.is_ascii_alphabetic() {
            result.push(c.to_ascii_lowercase());
        }

        prev = c;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_string() {
        assert_eq!(fix_string("o"), "O");
        assert_eq!(fix_string("o!"), "O");
        assert_eq!(fix_string("o!o"), "OO");
        assert_eq!(fix_string("osomapto"), "Osomapto");
    }

    #[test]
    fn test_suggestions() {
        let suggest = Suggest::new();

        fn sort(mut vec: Vec<String>) -> Vec<String> {
            vec.sort();
            vec
        }

        assert_eq!(
            sort(suggest.suggest("sari")),
            vec![
                "শারি",
                "শারী",
                "শাড়ি",
                "শাড়ী",
                "সারি",
                "সারী",
                "সাড়ি",
                "সাড়ী",
                "স্মঅরী"
            ]
        );
        assert_eq!(sort(suggest.suggest("sar")), vec!["ষাঁড়", "সার", "সার্ব", "সাড়"]);
        assert_eq!(sort(suggest.suggest("amra")), vec!["অমরা", "আমরা", "আমড়া"]);
        assert_eq!(sort(suggest.suggest("lalshak")), vec!["লালশাক"]);
        assert_eq!(sort(suggest.suggest("lalrong")), vec!["লালরং", "লালরঙ"]);
        assert_eq!(sort(suggest.suggest("ongshochched")), vec!["অংশচ্ছেদ"]);
        assert_eq!(sort(suggest.suggest("ongshocched")), vec!["অংশচ্ছেদ"]);
        assert_eq!(sort(suggest.suggest("shadhinota")), vec!["স্বাধীনতা"]);
        assert_eq!(sort(suggest.suggest("dukkho")), vec!["দুঃখ", "দুখ"]);
        assert_eq!(
            sort(suggest.suggest("cool")),
            vec!["চুল", "চূল", "চোল", "ছুঁল", "ছুল", "ছোল"]
        );
        assert_eq!(
            sort(suggest.suggest("shokti")),
            vec!["শকতি", "শক্তি", "সক্তি"]
        );
        assert_eq!(sort(suggest.suggest("chup")), vec!["চুপ", "ছুপ"]);
        assert_eq!(
            sort(suggest.suggest("as")),
            vec!["অশ্ব", "অশ্ম", "আঁশ", "আশ", "আস", "এস"]
        );
        assert_eq!(sort(suggest.suggest("apni")), vec!["আপনি"]);
        assert_eq!(
            sort(suggest.suggest("kkhet")),
            vec!["ক্ষেত", "খেঁট", "খেট", "খেত", "খ্যাঁট", "খ্যাঁত", "খ্যাত"]
        );
        assert_eq!(sort(suggest.suggest("ebong")), vec!["এবং"]);
        assert_eq!(sort(suggest.suggest("shesh")), vec!["শেষ", "সেস"]);
    }
}
