pub mod uniswap;

pub mod pool {
    pub enum PoolDest {
        UniSwap,
    }

    pub fn add_liquidity(pool_dest: Option<PoolDest>) {
        match pool_dest {
            Some(pool_dest_name) => match pool_dest_name {
                PoolDest::UniSwap => crate::pool::uniswap::add_liquidity_uniswap(),
            },
            None => return,
        }
    }
}
