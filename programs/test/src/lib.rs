use anchor_lang::prelude::*;

declare_id!("4vKTpAtJdbbVwR7ifmALqVswKUv9SnmmvtY7UqjkUypu");

#[program]
pub mod test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
