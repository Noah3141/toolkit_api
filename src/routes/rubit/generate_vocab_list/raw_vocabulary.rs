use std::collections::HashMap;

use sea_orm::{DatabaseConnection, ColumnTrait, EntityTrait, QueryFilter};
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

use crate::routes::rubit::{
    models::{
        prelude::RussianWords, 
        russian_words::Column as RussianWordsColumn
    }, 
    utils::{stop_words::remove_stop_words}};

// INCOMING REQUEST
#[derive(Deserialize)]
pub struct GenerateListRequest {
    input_text: String,
    style: String,
    breadth: String,
}

// OUTGOING RESPONSES
#[derive(Serialize)]
pub struct RawVocabularyList {
    entry_list: Vec<RawVocabEntry>,
    metadata: String
}

#[derive(Serialize)]
pub struct RawVocabEntry { // Verb, Noun, Adjective ...
    forms: Vec<String>,
    lemma: String,
    frequency: u16,
    perfective: Option<bool>,
}


    // todo) Verify that the request contains input text of Russian, not some injection attempt or something

    // todo) Begin processing Russian text for:
        // todo) *new lines*            -> delete
        // todo) *hyphenated* words     -> (1) stop words > (2) unique combos, can be addressed as two words >> REPLACE WITH SPACE
        // todo) *em and en dashes*     -> delete
        // todo) *periods*              -> (1) end of sentence, delete (2) acronyms, delete (nothing we can do) (3) ellipses, delete
        // todo) *exclamation marks*    -> end of sentence, delete
        // todo) *quotes* ["", «»]      -> delete
        // todo)
        // todo) Delete empty strings resultant from above

use super::super::utils::input_parsing::wordify;
use super::super::utils::input_cleaning::clean;

#[post("/generate-list/raw-vocabulary", format = "json", data = "<list_req>")]
pub async fn list_vocab(db: &State<DatabaseConnection> , list_req: Json<GenerateListRequest>) -> Result<Json<RawVocabularyList>, Status> {

    // Intake the request information
    let (mut input_text, breadth, style) = match list_req.into_inner() {
        GenerateListRequest {input_text, breadth, style} => (input_text, breadth, style)
    };

    let clean_text = clean(input_text);
    //dbg!(&clean_text);
    let input_words = wordify(clean_text);
    //dbg!(&input_words);
    let input_words = remove_stop_words(input_words);
    //dbg!(&words);

    // Dict of > Lemma : Forms[]
    let mut dictionary: HashMap<String, Vec<String>> = HashMap::new();
    dictionary.reserve(input_words.len());


    let mut models = match RussianWords::find()
        .filter(RussianWordsColumn::Form.is_in(&input_words))
        .all(db.inner())
        .await {
            Ok(v) => v,
            Err(dberr) => return Err(Status::InternalServerError)
        };

    models.sort_unstable_by_key(|m| m.form.clone());
    models.dedup_by_key(|m| m.form.clone());
    
    // Make a map of how many times each FORM occurs in the input text
    let mut form_frequency: HashMap<String, u16> = HashMap::new();
    for word in input_words {
        form_frequency.entry(word).and_modify(|x| *x += 1).or_insert(1);
    };

    let mut lemma_frequency: HashMap<String, u16> = HashMap::new();


    // Get all the input words from db, and build a dictionary of lemma:vec<forms>. 
    for model in models {

        // Dictionary is to contain each lemma, paired with all the forms it occurs with
        dictionary
            .entry( model.lemma.clone() )
            .and_modify(|list| list.push(model.form.clone())) // will not duplicate, because we're looping through unique set from db search
            .or_insert(vec![model.form.clone()]);

        // Lemma is counted by going form by form (models), and adding up all the form-counts beneath the lemma
        lemma_frequency
            .entry( model.lemma )
            .and_modify(|v| *v += form_frequency[&model.form])
            .or_insert(form_frequency[&model.form]);
    }
    


    // Build the response 
    let mut input_list: Vec<RawVocabEntry> = vec![];
    for (lemma, form_list)  in dictionary {
        
        let perf = None;
        input_list.push( RawVocabEntry { 
                frequency: *lemma_frequency.get(&lemma).expect("presence of lemma") ,
                forms: form_list, 
                lemma: lemma, 
                perfective: perf 
            } 
        ) 

    } 

    let response: RawVocabularyList = RawVocabularyList {
        entry_list: input_list, 
        metadata: String::from(""),
    };


    Ok(Json(response))
}


/* 

    #[post("/generate-list/raw-vocabulary", format = "json", data = "<list_req>")]
    pub async fn list_vocab(db: &State<DatabaseConnection> , list_req: Json<GenerateListRequest>) -> Result<Json<RawVocabularyList>, Status> {
    
        // Intake the request information
        let (mut input_text, breadth, style) = match list_req.into_inner() {
            GenerateListRequest {input_text: input_text, breadth, style} => (input_text, breadth, style)
        };
    
        let clean_text = clean(input_text);
        dbg!(&clean_text);
        let words = wordify(clean_text);
        dbg!(&words);
        let words = remove_stop_words(words);
        dbg!(&words);
    
        // Dict of > Lemma : Forms[]
        let mut dictionary: HashMap<String, Vec<String>> = HashMap::new();
        dictionary.reserve(words.len());
    
        let mut fail_list = String::from("");
    
        'scan: for input_form in words {
            let find_result = RussianWords::find()
                .filter(RussianWordsColumn::Form.eq(input_form.to_owned()))
                .one(db.inner())
                .await;
    
            let model = match find_result {
                Ok(m) => m,
                Err(e) => { println!("Error: {e}"); continue},
            };
    
            let lemma = match model {
                None => { match insert_unrecognized_if_absent(&input_form, &db.inner()).await {
                        Ok(presence) => match presence {
                                Presence::AlreadyPresent => fail_list.push_str(format!("\nUnrecognized: {}", &input_form).as_str()),
                                Presence::SuccessfullyAdded => fail_list.push_str(format!("\nAdded Unrecognized: {}", &input_form).as_str())
                            },
                        Err(e) => fail_list.push_str(format!("\nCouldn't add form: {}", &input_form).as_str()),
                    }; 
                    continue 
                },
                Some(m) => m.lemma.expect("presence of lemma form"),
            };
    
            dictionary.entry(lemma)
                .and_modify(|form_list| form_list.push(input_form.clone()))
                .or_insert(vec![input_form]);
    
        }
    
        let mut input_list: Vec<RawVocabEntry> = vec![];
        for (lemma, form_list)  in dictionary {
            let perf = None;
            input_list.push(
                RawVocabEntry { 
                    frequency: form_list.len() as u16, 
                    forms: form_list, 
                    lemma: lemma, 
                    perfective: perf 
                } // This entry
            ) // Pushing
        } // Loop through dict
    
    
    
        let response: RawVocabularyList = RawVocabularyList {
            entry_list: input_list, 
            metadata: fail_list,
        };
    
    
        Ok(Json(response))
    }
    


*/