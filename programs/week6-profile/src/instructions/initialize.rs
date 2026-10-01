use anchor_lang::prelude::*;

use crate::{constants::PROFILE_SEED, state::Profile};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = 8 + Profile::INIT_SPACE,
        seeds = [PROFILE_SEED, payer.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,

    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(
    ctx: Context<Initialize>,
    username: String,
    bio: String,
) -> Result<()> {
    let profile = &mut ctx.accounts.profile;

    profile.authority = ctx.accounts.payer.key();
    profile.username = username;
    profile.bio = bio;

    msg!("Profile initialized");
    Ok(())
}
