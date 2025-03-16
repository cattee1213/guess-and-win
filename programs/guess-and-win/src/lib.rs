use anchor_lang::prelude::*;

declare_id!("HM7nL9wpgvMCrmyaAiYfyeJ3R11rCouuQHgV88NJjnt");

mod instructions;
mod program_error;
mod states;
use instructions::*;

#[program]
mod guess_and_win {

    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, title: String, bonus: u64) -> Result<()> {
        instructions::initialize_pool(ctx, title, bonus)
    }

    pub fn delete_pool(ctx: Context<DeletePool>) -> Result<()> {
        instructions::delete_pool(ctx)
    }

    pub fn claim_bonus_pool(ctx: Context<ClaimBonusPool>) -> Result<()> {
        instructions::claim_bonus_pool(ctx)
    }

    pub fn withdraw_pool(ctx: Context<WithdrawPool>) -> Result<()> {
        instructions::withdraw_pool(ctx)
    }
}
