use hello_mod::{log_operation, strategy::token_strat};

pub mod pool;

fn main() {
    token_strat::dca_token();
    let target = Some(pool::pool::PoolDest::UniSwap);
    pool::pool::add_liquidity(target);
    log_operation("Add Liquidity");
}
