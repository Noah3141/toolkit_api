fn main() {

    let db: DatabaseConnection = match Database::connect(dotenvy::var("SECONDARY_TABLE").unwrap()).await {
        Ok(db) => db,
        Err(e) => panic!("Error launching: {e}")
    };

    let f = {
        std::fs::read_to_string("./full_russian_data.txt")
        .unwrap()
    };

    let mut lines: Vec<&str> = f.split("\n").collect();

    //println!("Line 5: {}", lines[5]);

    for chunk in lines.chunks(300) {

        let mut query = String::from(r#"INSERT INTO russian_words (form, lemma) VALUES "#);
        let mut idx = 0;
        for line in chunk {
            let mut words: Vec<&str> = line.split(",").collect();
            idx +=1;
            println!("Line {}", idx);
            for i in 0..words.len() {
                let segment = format!("('{}', '{}'), ", words[i], words[0]);
                query.push_str(segment.as_str());
            }
        }

        let end_index = query.rfind("), ").expect("presence of the pattern anywhere");
        query.replace_range(end_index.., ");");

        println!("Sending query...");
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