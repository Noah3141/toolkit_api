
pub fn clean(mut input_text: String) -> String {

        input_text = input_text
            .replace("\n", "")
            .replace("-", " ");
    let russian = "йцукенгшщзхъфывапролджэячсмитьб ";
    //https://www.livelib.ru/book/1002916502-vojna-i-mir-kniga-1-tom-1-2-lev-tolstoj
        input_text.retain(|ch: char| -> bool {russian.contains(ch) 
        });


        input_text
}
