use crate::program_error;
use crate::states::Pool;
use anchor_lang::prelude::*;

// 领取奖金
#[derive(Accounts)]
pub struct ClaimBonusPool<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn claim_bonus_pool(ctx: Context<ClaimBonusPool>) -> Result<()> {
    require!(
        ctx.accounts.pool.winer == ctx.accounts.signer.key(),
        program_error::OperationError::NotOwner
    );
    ctx.accounts.signer.add_lamports(ctx.accounts.pool.bonus)?;
    ctx.accounts.pool.sub_lamports(ctx.accounts.pool.bonus)?;
    ctx.accounts.pool.bonus = 0;
    msg!(
        "title: {} of pool is claimed by {}",
        ctx.accounts.pool.title,
        ctx.accounts.pool.winer
    );
    Ok(())
}
