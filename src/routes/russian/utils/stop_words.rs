pub fn remove_stop_words(word_list: Vec<String>) -> Vec<String> {

    let stop_words = std::fs::read_to_string("./src/routes/russian/utils/stop_words.txt").expect("presence of stop_words.txt");

    let mut checked_list: Vec<String> = vec![];
    for word in word_list {
        if !stop_words.contains(&word) {
            checked_list.push(word)
        }
    }
    checked_list
}