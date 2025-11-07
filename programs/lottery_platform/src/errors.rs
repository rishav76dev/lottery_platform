use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Not authorized to perform this action.")]
    NotAuthorized,

    #[msg("No players have joined the lottery.")]
    NoPlayers,

    #[msg("Winner has already been selected.")]
    WinnerAlreadyChosen,

    #[msg("Winner has not been selected yet.")]
    WinnerNotChosen,

    #[msg("The provided randomness account does not match the lottery's randomness account.")]
    IncorrectRandomnessAccount,

    #[msg("Randomness value has not been revealed yet.")]
    RandomnessNotResolved,

    #[msg("Randomness was already revealed or committed too late.")]
    RandomnessAlreadyRevealed,

    #[msg("Could not find the entrant index account in remaining_accounts.")]
    MissingEntrant,

    #[msg("Entrant index PDA does not match expected PDA derived by the program.")]
    IncorrectEntrantPda,

    #[msg("The caller is not the selected winner.")]
    WrongWinner,

    #[msg("Winner's token account is invalid or mismatched.")]
    InvalidWinnerTokenAccount,
}
