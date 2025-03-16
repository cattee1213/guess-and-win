use crate::program_error;
use crate::states::Pool;
use anchor_lang::prelude::*;

// 删除奖池
#[derive(Accounts)]
pub struct DeletePool<'info> {
    #[account(mut,close = signer)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn delete_pool(ctx: Context<DeletePool>) -> Result<()> {
    require!(
        ctx.accounts.pool.owner == ctx.accounts.signer.key(),
        program_error::OperationError::NotOwner
    );
    msg!(
        "title: {} of pool is deleted by {}",
        ctx.accounts.pool.title,
        ctx.accounts.pool.owner
    );
    Ok(())
}
