use std::collections::{BTreeMap, HashSet};

use serde::Deserialize;

use crate::{
    trie::{Trie, TrieNode},
    utils::fix_string,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub transliterate: Vec<String>,
    pub entire_block_optional: Option<bool>,
}

pub struct Suggest<'a> {
    patterns: BTreeMap<&'a str, Block>,
    words: Trie,
    common_suffixes: Vec<String>,
}

impl<'a> Suggest<'a> {
    pub fn new() -> Self {
        let patterns_data = include_bytes!("../data/preprocessed-patterns.json");
        let words_data = include_str!("../data/source-words.txt");
        let common_data = include_bytes!("../data/source-common-patterns.json");

        let patterns = serde_json::from_slice(patterns_data).unwrap();
        let words = Trie::from_strings(words_data.lines().map(|s| s.trim()));
        let common_suffixes = serde_json::from_slice(common_data).unwrap();

        Suggest {
            patterns,
            words,
            common_suffixes,
        }
    }

    fn find_pattern(&self, input: &'a str) -> Option<(&&str, &Block)> {
        let start = &input[..1];
        self.patterns
            .range(start..=input)
            .rfind(|(k, _)| input.starts_with(*k))
    }

    pub fn suggest(&self, input: &str) -> Vec<String> {
        let input = fix_string(input);
        let matched_nodes = vec![self.words.matching_node("").unwrap()];
        let suggestions = self.suggest_recursive(&input, matched_nodes);
        suggestions.into_iter().map(|s| s.to_string()).collect()
    }

    fn suggest_recursive(
        &'a self,
        remaining: &str,
        mut matched_nodes: Vec<&'a TrieNode>,
    ) -> HashSet<&'a str> {
        if remaining.is_empty() {
            return matched_nodes.iter().filter_map(|n| n.get_word()).collect();
        }

        let Some((key, block)) = self.find_pattern(remaining) else {
            return matched_nodes.iter().filter_map(|n| n.get_word()).collect();
        };

        let remaining = &remaining[key.len()..];

        let new_matched_nodes = block
            .transliterate
            .iter()
            .flat_map(|p| {
                matched_nodes
                    .iter()
                    .filter_map(|node| node.get_matching_node(p))
            })
            .collect::<Vec<_>>();

        if block.entire_block_optional == Some(true) {
            // Entirely optional patterns like "([ওোঅ]|(অ্য)|(য়ো?))?" may not yield any result
            matched_nodes.extend(new_matched_nodes);
        } else {
            matched_nodes = new_matched_nodes;
        }

        let additional_matched_nodes = matched_nodes
            .iter()
            .flat_map(|node| {
                self.common_suffixes
                    .iter()
                    .filter_map(|suffix| node.get_matching_node(suffix))
            })
            .collect::<Vec<_>>();
        matched_nodes.extend(additional_matched_nodes);

        self.suggest_recursive(remaining, matched_nodes)
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
