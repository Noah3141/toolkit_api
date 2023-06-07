
pub fn wordify(input_text: String) -> Vec<String> {


    let input_text: String = input_text
        .replace("\n", "");

    let mut words = vec![];
    for word in input_text.split_whitespace() {
        words.push(word.to_string().to_lowercase());
    };

    words
}
