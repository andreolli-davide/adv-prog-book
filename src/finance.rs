mod finance {
    pub mod wallet_1 {

        #[derive(Debug)]
        pub struct Wallet {
            pub euro: f32,
        }
    }

    pub mod wallet_2 {

        #[derive(Debug)]
        pub struct Wallet {
            pub euro: u32,
            pub cents: u8,
        }
    }

    pub type Wallet1 = wallet_1::Wallet;
    pub type Wallet2 = wallet_2::Wallet;

    pub fn compare_wallet(first: &Wallet1, second: &Wallet2) -> bool {
        first.euro > (second.euro as f32) + (second.cents as f32) / 100.0
    }
}

#[cfg(test)]
mod wallet_tests {
    use super::finance::{compare_wallet, Wallet1, Wallet2};

    #[test]
    fn test_compare_wallet() {
        let wallet1 = Wallet1 { euro: 10.0 };
        let wallet2 = Wallet2 {
            euro: 10,
            cents: 50,
        };
        assert_eq!(compare_wallet(&wallet1, &wallet2), false);
    }
}
