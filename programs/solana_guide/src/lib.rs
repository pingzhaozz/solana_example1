use anchor_lang::prelude::*;
use std::collections::{HashMap, BTreeMap};

declare_id!("4vKTpAtJdbbVwR7ifmALqVswKUv9SnmmvtY7UqjkUypu");

#[program]
pub mod test {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        // 简单类型
        let pubkey = Pubkey::new_from_array([0; 32]);
        let balance: u64 = 1000;
        let is_active: bool = true;
        msg!("pubkey: {:?}, balance: {}, is_active: {}", pubkey, balance, is_active);

        // 映射类型
        let mut hmap: HashMap<u64, u64> = HashMap::new();
        hmap.insert(1, 1001);
        let mut bmap: BTreeMap<u64, u64> = BTreeMap::new();
        bmap.insert(2, 1002);
        msg!("HashMap: {:?}, BTreeMap: {:?}", hmap, bmap);

        // 数组类型（无默认值）
        let mut data1 = vec![];
        data1.push(42);
        data1.push(7);
        msg!("data1: {:?}", data1);

        // 数组类型（有默认值）
        let data2: Vec<u8> = vec![0; 1024]; 
        msg!("data2: {:?}", data2); 

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
