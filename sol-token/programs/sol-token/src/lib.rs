use anchor_lang::prelude::*;

declare_id!("JCX3jF4us89VaEUmimya9kSmmD2magNqh7Mr7z4WCQxh");

#[program]
pub mod sol_token {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
