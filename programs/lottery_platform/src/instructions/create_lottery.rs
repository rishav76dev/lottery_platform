use anchor_lang::prelude::*;
use crate::states::*;

use anchor_spl::{
    token_interface::{self, Mint, TokenAccount,TransferChecked, TokenInterface}
};

#[derive(Accounts)]
pub struct CreateLottery<'info> {

  #[account(mut)]
  pub creator: Signer<'info>,

  #[account(mut)]
  pub market: Account<'info, Market>,

  #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,

  #[account(
    init,
    payer = creator,
    seeds = [b"lottery", creator.key().as_ref()],
    bump,
    space = 8 + Lottery::INIT_SPACE
  )]
  pub lottery: Account<'info, Lottery>,

  #[account(
    init,
    payer = creator,
    token::mint = mint,
    token::authority = lottery,
    seeds = [b"vault", lottery.key().as_ref()],
    bump,
  )]
  pub vault: InterfaceAccount<'info, TokenAccount>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = creator,
  )]
  pub creator_token_account: InterfaceAccount<'info, TokenAccount>,
  pub token_program: Interface<'info, TokenInterface>,
  pub system_program: Program<'info, System>,

}

pub fn create_lottery(
  ctx: Context<CreateLottery>,
  prize_amount: u64,
  ticket_price: u64,
) -> Result<()>{
  let lottery = &mut ctx.accounts.lottery;

  lottery.market = ctx.accounts.market.key();
  lottery.creator = ctx.accounts.creator.key();
  lottery.vault = ctx.accounts.vault.key();
  lottery.prize_amount = prize_amount;
  lottery.ticket_price = ticket_price;
  lottery.player_count = 0;
  lottery.winner = None;
  lottery.winner_index = None;
  lottery.winner_chosen = false;
  lottery.randomness_account = Pubkey::default();
  lottery.bump = ctx.bumps.lottery;

      let cpi_accounts = TransferChecked {
        from: ctx.accounts.creator_token_account.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.creator.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);

    token_interface::transfer_checked(cpi_ctx, prize_amount, ctx.accounts.mint.decimals)?;

    msg!("✅ Lottery created and prize of {} tokens transferred!", prize_amount);
    Ok(())

}
