//declare_id!("AmZpWqbmFUDvW12fDzZgtSQAXkXhYviJkv9uJ2g1KzPu");

use anchor_lang::prelude::*;

mod errors;
mod state;
use state::*;
mod instructions;
use instructions::*;

declare_id!("AmZpWqbmFUDvW12fDzZgtSQAXkXhYviJkv9uJ2g1KzPu");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_user_account(&ctx.bumps)?;
        Ok(())
    }

    pub fn initialize_config(ctx: Context<InitializeConfig>,points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)?;
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()?;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()?;
        Ok(())
    }

}