mod schemas;
mod storage;

pub use schemas::*;
pub use storage::*;

use poseidon_common::AccountOptions;

fn main() {
    smol::block_on(async {
        let mut storage =
            PoseidonRepo::new("1debf1c96419ede62342df4550e4e4ed3824e0eb6102a8168ae977a84b01f2d6")
                .unwrap();
        dbg!(&storage);

        dbg!(&storage.init_repo().await);
        dbg!(&storage.init_databases().await);

        dbg!(&storage);
        dbg!(&storage.add_tx(AccountOptions::CreateIfNone ,"31oVduNkaRxEP7PMzL8dgFTR8B3hHQcffcLfWnP131gq", "2r55TJ3r8FycjtMZuHNUEheKJkHPGMGWXKEqEz8ct77WiRjrZRZuDXmzn4fdLPEut4bpjxrhCUqecqLnK5kRwsF6").await);
        dbg!(&storage.list_txs().await);
    })
}
