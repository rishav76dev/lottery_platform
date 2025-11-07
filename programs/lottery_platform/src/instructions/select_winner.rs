use anchor_lang::prelude::*;
use switchboard_on_demand::accounts::RandomnessAccountData;
use crate::states::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct SelectWinner<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"lottery", lottery.creator.as_ref()],
        bump,
        has_one = creator,
    )]
    pub lottery: Account<'info, Lottery>,

    /// CHECK: validated manually
    pub randomness_account_data: UncheckedAccount<'info>,
}

pub fn select_winner(ctx: Context<SelectWinner>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;

    require!(lottery.player_count > 0, ErrorCode::NoPlayers);
    require!(!lottery.winner_chosen, ErrorCode::WinnerAlreadyChosen);
    require!(
        ctx.accounts.randomness_account_data.key() == lottery.randomness_account,
        ErrorCode::IncorrectRandomnessAccount
    );

    let clock = Clock::get()?;

    // Parse randomness account
    let randomness_data = RandomnessAccountData::parse(
        ctx.accounts.randomness_account_data.data.borrow()
    ).map_err(|_| error!(ErrorCode::RandomnessNotResolved))?;

    // Reveal randomness
    let random_values = randomness_data
        .get_value(clock.slot)
        .map_err(|_| error!(ErrorCode::RandomnessNotResolved))?;

    // Switchboard OD gives Vec<u8>; use byte 0
    let random_byte = random_values[0] as u64;
    let winner_index = (random_byte % lottery.player_count as u64) as u32;

    msg!("🎲 Random byte: {}", random_byte);
    msg!("🎯 Winner index: {}", winner_index);

    // Retrieve entrant index account from remaining_accounts
    require!(
        (winner_index as usize) < ctx.remaining_accounts.len(),
        ErrorCode::MissingEntrant
    );
    let entrant_info = &ctx.remaining_accounts[winner_index as usize];

    // Verify PDA derivation (anti-tampering)
    let (expected_pda, _bump) = Pubkey::find_program_address(
        &[
            b"entrant",
            lottery.key().as_ref(),
            &winner_index.to_le_bytes(),
        ],
        ctx.program_id,
    );
    require_keys_eq!(expected_pda, entrant_info.key(), ErrorCode::IncorrectEntrantPda);

    // Manually deserialize the account data to avoid lifetime issues
    let data = entrant_info.try_borrow_data()?;
    let mut data_slice: &[u8] = &data[8..]; // Skip 8-byte discriminator
    let entrant_data = EntrantIndex::try_deserialize(&mut data_slice)?;

    // Store winner
    lottery.winner = Some(entrant_data.user);
    lottery.winner_index = Some(winner_index);
    lottery.winner_chosen = true;

    msg!("🏆 Winner selected: {}", entrant_data.user);

    Ok(())
}
