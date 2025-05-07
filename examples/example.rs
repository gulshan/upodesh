use upodesh::suggest::Suggest;


fn main() {
    let suggest = Suggest::new();

    let Some(word) = std::env::args().nth(1) else {
        eprintln!("Please provide a word");
        std::process::exit(1);
    };

    let suggestions = suggest.suggest(&word);

    println!("Word: {}", word);
    println!("Suggestions: [{}]", suggestions.join(", "));
}
