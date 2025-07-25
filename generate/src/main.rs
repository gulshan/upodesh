use std::{
    collections::HashMap,
    fs::{File, read, read_to_string},
    io::BufWriter,
    path::PathBuf,
    env::var_os,
};

use fst::raw::Builder;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub transliterate: Vec<String>,
    pub entire_block_optional: Option<bool>,
}

fn generate_words_fst() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("words.fst");

    let file = File::create(dest).expect("Failed to create words.fst");
    let writer = BufWriter::new(file);

    let mut fst = Builder::new(writer).unwrap();
    let words = read_to_string(parent.join("data/source-words.txt")).expect("Failed to read source words file");

    let mut words = words.lines().map(str::trim).collect::<Vec<_>>();

    words.sort();

    for word in words {
        let numbered_word = word.chars().map(|c| c as u8).collect::<Vec<u8>>();
        fst.add(&numbered_word).expect("Failed to add word to FST");
    }

    fst.finish().expect("Failed to finish words FST generation");
}

fn generate_patterns_fst() {
    let root = PathBuf::from(var_os("CARGO_MANIFEST_DIR").unwrap());
    let parent = root.parent().unwrap();
    let dest = parent.join("src").join("patterns.fst");

    let file = File::create(dest).expect("Failed to create patterns.fst");
    let writer = BufWriter::new(file);

    let mut fst = Builder::new(writer).unwrap();
    let patterns: HashMap<String, Block> = serde_json::from_slice(
        &read(parent.join("data/preprocessed-patterns.json")).expect("Failed to read source patterns file"),
    )
    .unwrap();

    let mut patterns = patterns.keys().map(|s| s.as_str()).collect::<Vec<_>>();

    patterns.sort();

    for pattern in patterns {
        let numbered_word = pattern.chars().map(|c| c as u8).collect::<Vec<u8>>();
        fst.add(&numbered_word)
            .expect("Failed to add pattern to FST");
    }

    fst.finish()
        .expect("Failed to finish patterns FST generation");
}

fn main() {
    generate_words_fst();
    generate_patterns_fst();
}
