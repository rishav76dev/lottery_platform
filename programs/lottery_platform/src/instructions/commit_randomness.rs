use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;
use crate::states::*;
use crate::errors::ErrorCode;


#[derive(Accounts)]
pub struct CommitRandomness<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lottery", lottery.creator.as_ref()],
        bump,
        has_one = creator
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK: validated manually
    pub randomness_account_data: UncheckedAccount<'info>,
}

pub fn commit_randomness(ctx: Context<CommitRandomness>) -> Result<()> {
    let clock = Clock::get()?;
    let lottery = &mut ctx.accounts.lottery;

    // Only lottery creator may commit
    require_keys_eq!(ctx.accounts.creator.key(), lottery.creator, ErrorCode::NotAuthorized);

    // Parse randomness account
    let randomness_data = RandomnessAccountData::parse(
        ctx.accounts.randomness_account_data.data.borrow()
    ).map_err(|_| error!(ErrorCode::RandomnessNotResolved))?;

    // ✅ Enforce *commit-before-reveal*
    // This prevents someone from committing randomness *after* they see the results.
    require!(
        randomness_data.seed_slot == clock.slot - 1,
        ErrorCode::RandomnessAlreadyRevealed
    );

    // Store randomness account to use later during reveal / winner selection
    lottery.randomness_account = ctx.accounts.randomness_account_data.key();

    msg!("🔐 Randomness committed successfully at slot {}", randomness_data.seed_slot);

    Ok(())
}
