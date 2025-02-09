use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("HpR7fZikLNnCrQUcW6nyXpBtzeVLc3jF5s2fus5tQtGZ");

#[program]
mod anchor_vt {
    use super::*;
    pub fn initialize_pool(ctx: Context<InitializePool>, _title: String) -> Result<()> {
        *ctx.accounts.pool = Pool {
            owner: ctx.accounts.signer.key(),
            winer: ctx.accounts.pool.key(),
            title: _title,
        };

        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.signer.to_account_info(),
                to: ctx.accounts.pool.to_account_info(),
            },
        );
        require!(
            ctx.accounts.signer.get_lamports() > 5000000000,
            OperationError::NotEnoughSOL
        );
        transfer(cpi_context, 5000000000)?;

        msg!(
            "title: {} of easy pool is created by {}",
            ctx.accounts.pool.title,
            ctx.accounts.pool.owner
        ); // Message will show up in the tx logs
        Ok(())
    }

    pub fn withdraw_pool(ctx: Context<WithdrawPool>) -> Result<()> {
        require!(
            ctx.accounts.pool.owner == ctx.accounts.signer.key(),
            OperationError::NotOwner
        );
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.pool.to_account_info(),
                to: ctx.accounts.signer.to_account_info(),
            },
        );
        transfer(cpi_context, 5000000000)?;
        msg!(
            "title: {} of pool is withdraw by {}",
            ctx.accounts.pool.title,
            ctx.accounts.pool.owner
        );
        Ok(())
    }

    pub fn delete_pool(ctx: Context<DeletePool>) -> Result<()> {
        require!(
            ctx.accounts.pool.owner == ctx.accounts.signer.key(),
            OperationError::NotOwner
        );
        msg!(
            "title: {} of pool is deleted by {}",
            ctx.accounts.pool.title,
            ctx.accounts.pool.owner
        );
        Ok(())
    }
}

// 新建奖池
#[derive(Accounts)]
#[instruction(title:String)]
pub struct InitializePool<'info> {
    #[account(init, payer = signer,seeds=[signer.key().as_ref(),title.as_bytes()],bump, space = 8 + Pool::INIT_SPACE)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 取出 Sol
#[derive(Accounts)]
pub struct WithdrawPool<'info> {
    #[account(mut)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 删除奖池
#[derive(Accounts)]
pub struct DeletePool<'info> {
    #[account(mut,close = signer)]
    pub pool: Account<'info, Pool>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// 奖池Account
#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub owner: Pubkey,
    pub winer: Pubkey,
    #[max_len(20)]
    pub title: String,
}

#[error_code]
pub enum OperationError {
    #[msg("Can't access this account, no right!")]
    NotOwner,
    #[msg("Not enough SOL to pay!")]
    NotEnoughSOL,
}
