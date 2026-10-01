use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Profile {
    pub authority: Pubkey,
    #[max_len(32)]
    pub username: String,
    #[max_len(160)]
    pub bio: String,
}
