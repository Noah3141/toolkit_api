pub mod raw_vocabulary;
pub mod verb_pairs;
pub mod verb_trees;




/* 

    
    let (input_text, breadth, style) = match list_req.into_inner() {
        GenerateListRequest {input: input_text, breadth, style} => (input_text, breadth, style)
    };

    let mut input_text: String = input_text
        .replace("\n", " ");
    input_text.retain(|ch: char| -> bool {ch != ','});
    let words: Vec<&str> = input_text.split(" ").collect();

    let mut input_stats = InputLemmaStats {
        form_lemma: HashMap::new(),
        lemma_frequency: HashMap::new(),
    };
    'input_loop: for input_word in words.iter() {
        let find_result = RussianWords::find()
            .filter(RussianWordsColumn::Form.eq(input_word.to_owned()))
            .one(db.inner())
            .await;

        let model = match find_result {
            Ok(m) => {  // Successfully connected:
                match m {
                    // Give me the model we found in the database
                    Some(m) => {m},

                    // Handle not having found the word in our database
                    None => {
                        let unrecognized_word = unrecognized_words::ActiveModel {
                            form: Set(input_word.to_string()),
                            encountered: Set(Some(Utc::now().date_naive())),
                            ..Default::default()
                        };

                        in_check()

                        continue 'input_loop
                    },
                }
            },
            // Error state occurs when some kind of database connection fail occurs
            Err(err) => { 
                println!("{err}"); 
                continue 'input_loop
            } 
        };

        let lemma = model.lemma.unwrap_or("".to_string());
        input_stats.form_lemma.insert( 
            model.form , 
            lemma
        );
        
        input_stats.lemma_frequency.entry(lemma).and_modify(|freq| *freq += 1).or_insert("1);



    }

    
    match style.as_str() {
        "Raw Vocabulary" => {},
        "Verb Pairs" => {},
        "Verb Trees" => {},
        _ => {},
    }

    // Initialize the response we will send
    let list_response = GenerateListResponse {
        list: vec![],
        style,
        metadata: String::from("")
    }; 

    Ok(Json(list_response))
}



*/