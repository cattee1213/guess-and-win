use anchor_lang::prelude::*;
// 奖池Account
#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub owner: Pubkey,
    pub winer: Pubkey,
    #[max_len(20)]
    pub title: String,
    pub bonus: u64,
    // 随机整数
    pub num: u8,
    // pool状态 0:未开始 1:进行中 2:已结束
    pub status: u8,
}
