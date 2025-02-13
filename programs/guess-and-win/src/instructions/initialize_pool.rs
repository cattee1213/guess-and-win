use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::program_error;
use crate::states::Pool;
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

pub fn initialize_pool(ctx: Context<InitializePool>, _title: String, _bonus: u64) -> Result<()> {
    // let clock = Clock::get().unwrap();
    // let random_num: u8 = (clock.unix_timestamp % 100 + 1) as u8;
    *ctx.accounts.pool = Pool {
        owner: ctx.accounts.signer.key(),
        winer: ctx.accounts.pool.key(),
        title: _title,
        bonus: _bonus,
        num: 0,
        status: 0,
    };
    let cpi_context = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.pool.to_account_info(),
        },
    );
    require!(
        ctx.accounts.signer.get_lamports() > _bonus,
        program_error::OperationError::NotEnoughSOL
    );
    transfer(cpi_context, _bonus)?;
    msg!(
        "title: {} of easy pool is created by {}",
        ctx.accounts.pool.title,
        ctx.accounts.pool.owner
    ); // Message will show up in the tx logs
    Ok(())
}
