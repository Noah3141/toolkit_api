// use crate::routes::russian::*;
// use chrono::Utc;
// use models::prelude::*;
// use models::unrecognized_words::{Column as UnrecognizedWordsColumn, ActiveModel as UnrecognizedWord};
// use rocket::State;
// use sea_orm::{EntityTrait, DatabaseConnection, ColumnTrait, QueryFilter, DbErr, Set};

// pub enum Presence {
//     AlreadyPresent,
//     SuccessfullyAdded
// } use Presence::*;


// pub async fn insert_unrecognized_if_absent(word: &String, db: &DatabaseConnection) -> Result<Presence, DbErr> {

//     match UnrecognizedWords::find()// Try to find in the table `unrecognized_words`
//         .filter(UnrecognizedWordsColumn::Form.eq(word)) // at least one entry that looks like the input word
//         .one(db) // in the database
//         .await { // when you get a resonse
//             Err(e) => { Err(e) }, // an error means we couldn't connect, return error result (generally we won't care much)
//             Ok(m) => { // an okay means we connected, and got back a result
//                 match m { // the result can be either:
//                     Some(_) => Ok(AlreadyPresent), // That we already have the word in the database
//                     None => { // Or that the word is NOT in the database, and we want to add it so
//                         let unrecognized_word = UnrecognizedWord { // Create a model out of the word
//                             form: Set(word.to_string()),
//                             encountered: Set(Some(Utc::now().date_naive())),
//                             ..Default::default()
//                         };
//                         match UnrecognizedWords::insert(unrecognized_word) // Inserting the model into `unrecognized_words`
//                             .exec(db) // into the database
//                             .await { // which will later respond with either:
//                                 Ok(m) => Ok(SuccessfullyAdded), // Successfully added
//                                 Err(e) => Err(e) // Couldn't add, pass up the error
//                             }
//                     },
//                 } 
//             }
//         }
// }