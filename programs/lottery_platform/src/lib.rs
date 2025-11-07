#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

declare_id!("AHAshr6jtXZyvUDsXmzUdH6eMqg7UAWhKNwNHaL39Nvc");

pub mod instructions;
pub use instructions::*;


pub mod states;
pub mod errors;

#[program]
pub mod lottery_platform {
    use super::*;

     pub fn initialize_platform(
        ctx: Context<InitializePlatform>,
        treasury: Pubkey,
        fee_bps: u16,
    ) -> Result<()> {
        instructions::initialize_platform(ctx, treasury, fee_bps)
    }

    pub fn create_lottery(ctx: Context<CreateLottery>,
        prize_amount: u64,
        ticket_price: u64,
    ) -> Result<()> {
        instructions::create_lottery(ctx,prize_amount,ticket_price)
    }

    pub fn join_lottery(ctx: Context<JoinLottery>)-> Result<()> {
        instructions::join_lottery(ctx)
    }
}
