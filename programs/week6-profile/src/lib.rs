pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("EgvAE7sfipvKGE4wa238K1E5WK2HNFVJ4qcsJ4dn5KRg");

#[program]
pub mod week6_profile {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        username: String,
        bio: String,
    ) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx, username, bio)
    }
}
