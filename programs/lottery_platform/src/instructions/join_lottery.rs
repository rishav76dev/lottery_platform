
use anchor_lang::prelude::*;
use crate::states::*;

use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

#[derive(Accounts)]
pub struct JoinLottery<'info> {

  #[account(mut)]
  pub user: Signer<'info>,

  #[account(
        mut,
        seeds = [b"lottery", lottery.creator.as_ref()],
        bump
  )]
  pub lottery: Account<'info, Lottery>,

  #[account(
    mut,
    seeds = [b"vault", lottery.key().as_ref()],
    bump
  )]
  pub vault: InterfaceAccount<'info, TokenAccount>,

  #[account(
      mint::token_program = token_program
  )]
  pub mint: InterfaceAccount<'info, Mint>,

  #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    /// New participant PDA (represents a user's entry)
    #[account(
        init,
        payer = user,
        space = 8 + Participant::INIT_SPACE,
        seeds = [b"participant", lottery.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub participant: Account<'info, Participant>,

    #[account(
    init,
    payer = user,
    seeds = [b"entrant", lottery.key().as_ref(), lottery.player_count.to_le_bytes().as_ref()],
    bump,
    space = 8 + EntrantIndex::INIT_SPACE
    )]
    pub entrant_index: Account<'info, EntrantIndex>,


    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn join_lottery(
  ctx: Context<JoinLottery>
)->Result<()>{

  let lottery = &mut ctx.accounts.lottery;
  let participant = &mut ctx.accounts.participant;
  let user = &ctx.accounts.user;

  let ticket_price = lottery.ticket_price;

  let cpi_accounts = TransferChecked {
    from: ctx.accounts.user_token_account.to_account_info(),
    to: ctx.accounts.vault.to_account_info(),
    mint: ctx.accounts.mint.to_account_info(),
    authority: user.to_account_info()
  };
  let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

  token_interface::transfer_checked(cpi_ctx, ticket_price, ctx.accounts.mint.decimals)?;

  participant.lottery = lottery.key();
  participant.user = user.key();
  participant.amount_staked = ticket_price;
  participant.joined_at = Clock::get()?.unix_timestamp;


  let idx = lottery.player_count; // current index before increment
  let entrant_index = &mut ctx.accounts.entrant_index;
  entrant_index.lottery = lottery.key();
  entrant_index.index = idx;
  entrant_index.user = user.key();

  lottery.player_count += 1;


   msg!(
      "{} joined lottery {} with {} tokens",
      user.key(),
      lottery.key(),
      ticket_price
    );
  Ok(())
}
