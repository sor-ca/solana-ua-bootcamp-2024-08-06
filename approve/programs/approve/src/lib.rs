pub mod constants;
pub mod error;
pub mod instructions;
//pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
use instructions::*;
//use state::*;

declare_id!("6KYacbmp1wVsy9FsCuqxXsAVyQEtAnoGAofDHZ1yhELH");

#[program]
pub mod approve {
    use super::*;
    pub fn make_offer(
        context: Context<MakeOffer>,
        maker_atk_amount: u64,
        taker_btk_amount: u64,
        id: u64,
    ) -> Result<()> {
        instructions::make_offer::make_offer(context, maker_atk_amount, taker_btk_amount, id)
        //instructions::make_offer::save_offer(context, id, token_b_wanted_amount)
    }

    pub fn take_offer(context: Context<TakeOffer>) -> Result<()> {
        instructions::take_offer::take_offer(context)
    }
}
