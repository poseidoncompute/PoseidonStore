use crate::{StorageOutcome, WalletAccount};
use borsh::{BorshDeserialize, BorshSerialize};
use camino::Utf8PathBuf;
use core::fmt;
use poseidon_common::{AccountOptions, Base58PublicKey, PoseidonError, PoseidonResult, StoreErr};
use solana_client::OnChainTransaction;

const BASE_PATH: &str = "PoseidonStore";
const TX_STORE: &str = "transactions";
const LOGS_STORE: &str = "logs";

pub type DatabasePath = Utf8PathBuf;
pub type LogsPath = Utf8PathBuf;
pub type BasePath = Utf8PathBuf;

pub struct PoseidonRepo {
    base_path: BasePath,
    identifier: Base58PublicKey,
    store_uri: DatabasePath,
    store: Option<sled::Db>,
    logs_uri: LogsPath,
    logs_store: Option<sled::Db>,
}

impl PoseidonRepo {
    pub fn new(identifier: &str) -> PoseidonResult<Self> {
        let identifier_is_valid = match hex::decode(&identifier) {
            Ok(valid_hex_bytes) => valid_hex_bytes,
            Err(error) => return Err(error.into()),
        };

        if identifier_is_valid.len() != 32 {
            return Err(PoseidonError::InvalidBase58Ed25519PublicKey);
        }

        let home_dir = match directories::UserDirs::new() {
            Some(dir_exists) => dir_exists.home_dir().to_owned(),
            None => return Err(PoseidonError::HomeDirectoryNotFound),
        };

        let home_dir = match home_dir.to_str() {
            Some(valid_utf8_path) => valid_utf8_path,
            None => return Err(PoseidonError::PathIsNotValidUtf8),
        };

        let mut base_path = Utf8PathBuf::new();
        base_path.push(home_dir);
        base_path.push(BASE_PATH);
        base_path.push(identifier);

        let mut storage = base_path.clone();
        storage.push(TX_STORE);

        let mut logs = Utf8PathBuf::new();
        logs.push(home_dir);
        logs.push(BASE_PATH);
        logs.push(LOGS_STORE);
        logs.push(identifier);

        Ok(PoseidonRepo {
            identifier: identifier.to_owned(),
            base_path,
            store_uri: storage,
            store: Option::default(),
            logs_uri: logs,
            logs_store: Option::default(),
        })
    }

    pub async fn init_repo(&self) -> PoseidonResult<&Self> {
        use async_fs::DirBuilder;

        DirBuilder::new().create(&self.base_path).await?;

        Ok(self)
    }

    pub async fn init_databases(&mut self) -> PoseidonResult<&Self> {
        self.init_store().await?.init_logs().await?;

        Ok(self)
    }

    pub async fn init_store(&mut self) -> PoseidonResult<&mut Self> {
        let db = sled::Config::default().path(&self.store_uri).open()?;

        self.store = Some(db);

        Ok(self)
    }

    pub async fn init_logs(&mut self) -> PoseidonResult<&mut Self> {
        let db = sled::Config::default().path(&self.logs_uri).open()?;
        self.logs_store = Some(db);

        Ok(self)
    }

    pub async fn add_tx(
        &self,
        account_options: AccountOptions,
        public_key: &str,
        signature: &str,
    ) -> PoseidonResult<StorageOutcome> {
        let db = match &self.store {
            Some(db) => db,
            None => {
                return Err(PoseidonError::SledCollectionNotFound(
                    "TX_STORE_NOT_INITIALIZED".to_owned(),
                ))
            }
        };

        let account_info_op = db.get(public_key)?;

        if let Some(account_info_bytes) = account_info_op {
            let mut account_info: WalletAccount =
                match WalletAccount::try_from_slice(&account_info_bytes) {
                    Ok(account_info) => account_info,
                    Err(_) => return Err(PoseidonError::UnableToDeserializeAccountInfo),
                };

            if account_info.transactions().contains_key(signature) {
                return Ok(StorageOutcome::TxAlreadyExists);
            } else {
                let tx_data = solana_client::SolClient::new()
                    .get_transaction(signature)
                    .await?;
                account_info.add_transaction(signature.to_owned(), tx_data);

                Ok(StorageOutcome::Inserted)
            }
        } else {
            if account_options == AccountOptions::ErrIfNone {
                Err(PoseidonError::AccountNotFound)
            } else {
                let mut account_info = WalletAccount::new(public_key);

                let tx_data = solana_client::SolClient::new()
                    .get_transaction(signature)
                    .await?;
                account_info.add_transaction(signature.to_owned(), tx_data);

                let ser_account_info = account_info.try_to_vec()?;

                db.insert(public_key, ser_account_info)?;

                Ok(StorageOutcome::Inserted)
            }
        }
    }

    pub async fn list_txs(&self) -> PoseidonResult<Vec<OnChainTransaction>> {
        if let Some(store) = &self.store {
            let mut txs: Vec<OnChainTransaction> = Vec::new();
            for value in store.iter().values() {
                let wallet = WalletAccount::try_from_slice(&value?)?;

                wallet.transactions().iter().for_each(|tx| {
                    txs.push(tx.1.clone());
                });
            }

            Ok(txs)
        } else {
            Err(PoseidonError::Store(StoreErr::StoreNotFound(
                self.store_uri.to_string(),
            )))
        }
    }
}

impl fmt::Debug for PoseidonRepo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PoseidonRepo")
            .field("base_path", &self.base_path)
            .field("identifier", &self.identifier)
            .field("store_uri", &self.store_uri)
            .field("store", &"sled::Db")
            .field("logs_uri", &self.logs_uri)
            .field("logs_store", &"sled::Db")
            .finish()
    }
}
