#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StorageOutcome {
    Inserted,
    Updated,
    Removed,
    OperationFailure(String),
    DatabaseNotFound,
    KeyAlreadyExists,
    TxAlreadyExists,
}
