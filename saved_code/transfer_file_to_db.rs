fn main() {

    dotenv().expect(".env not found!");

    let db: DatabaseConnection = match Database::connect(dotenvy::var("DATABASE_URL")?).await {
        Ok(db) => db,
        Err(e) => panic!("Error launching: {e}")
    };

    let word_blob = std::fs::read_to_string("./word_database_key.txt").unwrap();
        let words = word_blob.split_whitespace();
        let mut word_list: Vec<String> = vec![];

        for word in words {
            word_list.push(word.to_owned())
        }

        for chunk in word_list.chunks(5_000) {
            
            println!("Starting on next chunk");
            let mut query = String::from(r#"INSERT INTO russian_words (form, lemma) VALUES "#);
        
            for pair in chunk {
                // дашь,дать
                //  дашь        дать
                let (form, lemma) = match pair.split_once(",") {
                    Some((f, l)) => (f, l),
                    None => continue
                };

                if pair == chunk.last().expect("A final element") {
                    let segment = format!("('{form}', '{lemma}');");
                    query.push_str(segment.as_str());
                } else {
                    let segment = format!("('{form}', '{lemma}'), ");
                    query.push_str(segment.as_str());
                }
                

                
            }
            let res = db.execute(Statement::from_string(DatabaseBackend::MySql, query)).await;
            dbg!(res);

            use std::{thread, time};

            let second = time::Duration::from_secs(1);
            
            print!("\nWaiting a second:");
            thread::sleep(second);
            print!(" Done!")
        }

        

        db.close().await;

}