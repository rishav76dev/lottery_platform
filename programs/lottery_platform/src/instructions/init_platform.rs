
use anchor_lang::prelude::*;
use crate::states::*;

#[derive(Accounts)]
pub struct InitializePlatform<'info> {
    #[account(
        init,
        payer = admin,
        seeds = [b"market"],
        bump,
        space = 8 + Lottery::INIT_SPACE
    )]
    pub market: Account<'info, Market>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_platform(
    ctx: Context<InitializePlatform>,
    treasury: Pubkey,
    fee_bps: u16,
) -> Result<()> {
    let market = &mut ctx.accounts.market;

    market.admin = ctx.accounts.admin.key();
    market.treasury = treasury;
    market.fee_bps = fee_bps;
    market.bump = ctx.bumps.market;

    Ok(())
}
