
pub fn wordify(input_text: String) -> Vec<String> {
    let mut words = vec![];
    for word in input_text.split_whitespace() {
        words.push(word.to_string().to_lowercase());
    };

    words
}
