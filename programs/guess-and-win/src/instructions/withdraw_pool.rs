use crate::program_error;
use crate::states::Pool;
use anchor_lang::prelude::*;

// 取出 Sol
#[derive(Accounts)]
pub struct WithdrawPool<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_pool(ctx: Context<WithdrawPool>) -> Result<()> {
    require!(
        ctx.accounts.pool.owner == ctx.accounts.signer.key(),
        program_error::OperationError::NotOwner
    );
    ctx.accounts.signer.add_lamports(ctx.accounts.pool.bonus)?;
    ctx.accounts.pool.sub_lamports(ctx.accounts.pool.bonus)?;
    ctx.accounts.pool.bonus = 0;
    msg!(
        "title: {} of pool is withdraw by {}",
        ctx.accounts.pool.title,
        ctx.accounts.pool.owner
    );
    Ok(())
}
