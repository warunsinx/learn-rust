pub fn log_operation(operation: &str) {
    println!("Log Operation {} !", operation);
}

mod swap {
    pub mod swap_general {
        pub fn swap_token() {
            println!("Swap Token !");
            crate::log_operation("Swap");
        }
    }
}

pub mod strategy {
    pub mod token_strat {
        pub fn dca_token() {
            crate::swap::swap_general::swap_token();
            println!("DCA Token !");
            super::super::log_operation("DCA");
        }
    }
}
