use crate::routes::russian::*;
use chrono::Utc;
use models::prelude::*;
use models::unrecognized_words::{Column as UnrecognizedWordsColumn, ActiveModel as UnrecognizedWord};
use rocket::State;
use sea_orm::{EntityTrait, DatabaseConnection, ColumnTrait, QueryFilter, DbErr, Set};

enum Presence {
    AlreadyPresent,
    SuccessfullyAdded
} use Presence::*;


pub async fn insert_unrecognized_if_absent(word: &String, db: &State<DatabaseConnection>) -> Result<Presence, DbErr> {

    match UnrecognizedWords::find()
        .filter(UnrecognizedWordsColumn::Form.eq(word))
        .one(db.inner())
        .await {
            Err(e) => { Err(e) },
            Ok(m) => {
                match m {
                    Some(_) => Ok(AlreadyPresent),
                    None => {
                        let unrecognized_word = UnrecognizedWord {
                            form: Set(word.to_string()),
                            encountered: Set(Some(Utc::now().date_naive())),
                            ..Default::default()
                        };
                        match UnrecognizedWords::insert(unrecognized_word)
                            .exec(db.inner())
                            .await {
                                Ok(m) => Ok(SuccessfullyAdded),
                                Err(e) => Err(e)
                            }
                    },
                } 
            }
        }
}