use crate::{Base58PublicKey, Base58Sha256Hash, Base58Signature, Lamports, TransactionError};

#[derive(Debug)]
pub struct AccountInfo {
    account_type: AccountType,
    slot: u64,
    data: PdaData,
    executable: bool,
    lamports: Lamports,
    onwer: Base58PublicKey,
    rent_epoch: u32,
}

pub enum AccountType {
    Wallet,
    Pda,
    Program,
    Vote,
    Stake,
    Config,
    Bpf,
    Ed25519,
    Secp256k1,
}
