
pub fn clean(mut input_text: String) -> String {

        input_text = input_text
            .replace("\n", "")
            .replace("-", " ");


        input_text.retain(|ch: char| -> bool {match ch {
            '.' |
            ',' |
            '!' |
            ':' |
            '?' |
            '…' |
            '«' |
            '»' |
            '?' |
            '"' |
            '—' |
            ';' |
            '(' |
            ')' => false,
            _ => true
        }});


        input_text
}
