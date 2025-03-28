use anchor_lang::prelude::*;
use std::collections::{HashMap, BTreeMap};

declare_id!("4vKTpAtJdbbVwR7ifmALqVswKUv9SnmmvtY7UqjkUypu");

#[program]
pub mod test {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        let my_account = &mut ctx.accounts.my_account;
        my_account.data.push(42);
        my_account.data.push(7);
        msg!("data: {:?}", my_account.data);
    
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1024)] 
    pub my_account: Account<'info, MyAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyAccount {
    pub data: Vec<u8>, // 存储动态数组的数据字段
}
