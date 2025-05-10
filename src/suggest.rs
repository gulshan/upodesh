use std::collections::{BTreeMap, HashSet};

use crate::{trie::Trie, utils::fix_string};

pub struct Suggest<'a> {
    patterns: BTreeMap<&'a str, Vec<String>>,
    words: Trie,
    common_suffixes: Vec<String>,
}

impl<'a> Suggest<'a> {
    pub fn new() -> Self {
        let patterns_data = include_bytes!("../data/preprocessed-patterns.json");
        let words_data = include_str!("../data/source-words.txt");
        let common_data = include_bytes!("../data/source-common-patterns.json");

        let patterns = serde_json::from_slice(patterns_data).unwrap();
        let words = Trie::from_strings(words_data.lines());
        let common_suffixes = serde_json::from_slice(common_data).unwrap();

        Suggest {
            patterns,
            words,
            common_suffixes,
        }
    }

    fn find_pattern(&self, input: &'a str) -> Option<(&&str, &Vec<String>)> {
        let start = &input[..1];
        self.patterns
            .range(start..=input)
            .rfind(|(k, _)| input.starts_with(*k))
    }

    pub fn suggest(&self, input: &str) -> Vec<String> {
        let input = fix_string(input);

        let mut remaining = &input[..];
        let mut matched_nodes = vec![self.words.matching_node("").unwrap()];

        while !remaining.is_empty() {
            let Some((key, block)) = self.find_pattern(remaining) else {
                break;
            };
            remaining = &remaining[key.len()..];

            matched_nodes = block
                .iter()
                .flat_map(|p| {
                    matched_nodes
                        .iter()
                        .filter_map(|node| node.get_matching_node(p))
                })
                .flat_map(|node| {
                    self.common_suffixes
                        .iter()
                        .filter_map(|suffix| node.get_matching_node(suffix))
                })
                .collect::<Vec<_>>();
        }

        let suggestions: HashSet<_> = matched_nodes.iter().filter_map(|n| n.get_word()).collect();
        suggestions.into_iter().map(|s| s.to_string()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
