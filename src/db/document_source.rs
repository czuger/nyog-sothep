use polodb_core::{CollectionT, Database, Transaction};
use serde::Serialize;

/// Where documents are read: the database itself, or a transaction that also sees its own writes.
pub trait DocumentSource {
    /// A collection of documents of type `T`.
    fn documents<T: Serialize>(&self, collection: &str) -> impl CollectionT<T>;
}

impl DocumentSource for Database {
    fn documents<T: Serialize>(&self, collection: &str) -> impl CollectionT<T> {
        self.collection(collection)
    }
}

impl DocumentSource for Transaction {
    fn documents<T: Serialize>(&self, collection: &str) -> impl CollectionT<T> {
        self.collection(collection)
    }
}
