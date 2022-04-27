pub mod my_mod {

    #[cfg(feature = "solana_1_9")]
    pub fn hello() {
        println!("Hello from 1.9")
    }

    #[cfg(feature = "solana_1_10")]
    pub fn hello() {
        println!("Hello from 1.10")
    }
}
