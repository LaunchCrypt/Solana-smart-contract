use anchor_lang::prelude::*;

declare_id!("CGUmftUy7fZZhrHJzqkBbAH8AuKsp4PHPJcEcbFXxBbp");

#[program]
pub mod launch_crypt {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
