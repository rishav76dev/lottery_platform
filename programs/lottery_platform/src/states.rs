use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub fee_bps: u16,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Lottery {
    pub market: Pubkey,       // <- reference to platform
    pub creator: Pubkey,
    pub vault: Pubkey,
    pub prize_amount: u64,
    pub ticket_price: u64,
    pub player_count: u32,
    pub winner: Option<Pubkey>,
    pub bump: u8,
    pub randomness_account: Pubkey,     // Switchboard On-Demand randomness acct
    pub winner_index: Option<u32>,      // picked index
    pub winner_chosen: bool,
}

#[account]
#[derive(InitSpace)]
pub struct Participant {
    pub lottery: Pubkey,
    pub user: Pubkey,
    pub amount_staked: u64,
    pub joined_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct EntrantIndex {
    pub lottery: Pubkey,
    pub index: u32,
    pub user: Pubkey,
}