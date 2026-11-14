//! Numbering documents, one counter document per numbered collection.

use polodb_core::bson::doc;
use polodb_core::{CollectionT, Transaction};
use serde::{Deserialize, Serialize};

use crate::error::AppError;

const COLLECTION: &str = "sequences";

/// A document of `sequences`: the last number given out in a collection.
#[derive(Debug, Serialize, Deserialize)]
struct Sequence {
    /// The numbered collection.
    #[serde(rename = "_id")]
    name: String,
    last: i64,
}

/// The next number of the `name` collection, from 1.
pub fn next_number(transaction: &Transaction, name: &str) -> Result<i64, AppError> {
    let sequences = transaction.collection::<Sequence>(COLLECTION);
    let Some(sequence) = sequences.find_one(doc! { "_id": name })? else {
        sequences.insert_one(Sequence {
            name: name.to_string(),
            last: 1,
        })?;
        return Ok(1);
    };
    let next = sequence.last + 1;
    sequences.update_one(doc! { "_id": name }, doc! { "$set": { "last": next } })?;
    Ok(next)
}
