use borsh::{BorshDeserialize, BorshSerialize};
use poseidon_common::{Base58Signature, DataID, UserData};
use serde::{Deserialize, Serialize};
use solana_client::OnChainTransaction;
use std::collections::{HashMap, VecDeque};
use tai64::Tai64N;

#[derive(Debug, BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
pub struct WalletAccount {
    created_on: [u8; 12],
    updated_on: [u8; 12],
    username: Option<String>,
    wallet_id: String,
    notifications: VecDeque<String>, //TODO
    notify_time: Option<[u8; 12]>,
    user_data: HashMap<DataID, UserData>,
    transactions: HashMap<Base58Signature, OnChainTransaction>,
    //subscriptions: HashMap<SubscriptionID, String>, //TODO
}

impl WalletAccount {
    pub fn new(wallet_id: &str) -> Self {
        let time_now = Tai64N::now().to_bytes();

        WalletAccount {
            created_on: time_now,
            updated_on: time_now,
            username: Option::default(),
            wallet_id: wallet_id.to_owned(),
            notifications: VecDeque::default(),
            notify_time: Option::default(),
            user_data: HashMap::default(),
            transactions: HashMap::default(),
            //subscriptions: Vec::default(),
        }
    }

    pub fn created_time(&self) -> [u8; 12] {
        self.created_on
    }

    pub fn updated_time(&self) -> [u8; 12] {
        self.updated_on
    }

    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }

    pub fn wallet_id(&self) -> String {
        self.wallet_id.clone()
    }

    pub fn notifications(&self) -> &VecDeque<String> {
        &self.notifications
    }

    pub fn user_data(&self) -> &HashMap<DataID, UserData> {
        &self.user_data
    }

    pub fn transactions(&self) -> &HashMap<Base58Signature, OnChainTransaction> {
        &self.transactions
    }

    pub fn add_username(&mut self, username: &str) -> &mut Self {
        self.username = Some(username.to_owned());

        self
    }

    pub fn add_notification(&mut self, notification: String) -> &mut Self {
        self.notifications.push_back(notification);

        self
    }

    pub fn notify_time(&mut self) -> &mut Self {
        self
    }

    pub fn add_user_data(&mut self, data_id: DataID, data: UserData) -> &mut Self {
        self.user_data.insert(data_id, data);

        self
    }

    pub fn add_transaction(
        &mut self,
        id: Base58Signature,
        transaction: OnChainTransaction,
    ) -> &mut Self {
        self.transactions.insert(id, transaction);

        self
    }
}
