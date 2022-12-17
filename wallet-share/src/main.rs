use once_cell::sync::Lazy;

#[cfg(feature = "wallet")]
use rgb_lib::wallet::Wallet;

use rgb_lib::wallet::WalletData;
use std::sync::{Arc, RwLock};

pub struct WalletState {
    wallet_data: Option<WalletData>,
    #[cfg(feature = "wallet")]
    wallet: Option<Wallet>,
}

impl WalletState {
    fn new() -> WalletState {
        WalletState {
            wallet_data: None,
            #[cfg(feature = "wallet")]
            wallet: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use test_context::{test_context, AsyncTestContext};

    static WALLET_STATE: Lazy<Arc<RwLock<WalletState>>> = Lazy::new(|| Arc::new(RwLock::new(WalletState::new())));
    
    pub struct WalletTestContext {
        pub wallet_state: Arc<RwLock<WalletState>>,
    }

    #[async_trait::async_trait]
    impl AsyncTestContext for WalletTestContext {
        async fn setup() -> WalletTestContext {
            WalletTestContext {
                wallet_state: Arc::clone(&WALLET_STATE),
            }
        }

        async fn teardown(self) {
            /* nothing to do */
        }
    }

    #[test_context(WalletTestContext)]
    #[actix::test]
    async fn test1(ctx: &mut WalletTestContext) {
       if let Ok(mut wallet_state) = ctx.wallet_state.write() {
           wallet_state.wallet_data = Some(WalletData {
               data_dir: "/tmp".to_string(),
               bitcoin_network: rgb_lib::BitcoinNetwork::Regtest,
               database_type: rgb_lib::wallet::DatabaseType::Sqlite,
               pubkey: "tpubD6NzVbkrYhZ4YT9CY6kBTU8xYWq2GQPq4NYzaJer1CRrffVLwzYt5Rs3WhjZJGKaNaiN42JfgtnyGwHXc5n5oPbAUSbxwuwDqZci5kdAZHb".to_string(),
               mnemonic: Some("save call film frog usual market noodle hope stomach chat word worry".to_string()),
           });
       }
    }

    #[cfg(feature = "wallet")]
    #[test_context(WalletTestContext)]
    #[actix::test]
    async fn test2(ctx: &mut WalletTestContext) {
       if let Ok(mut wallet_state) = ctx.wallet_state.write() {
           wallet_state.wallet = Some(Wallet::new(wallet_state.wallet.clone()));
       }
    }
}
