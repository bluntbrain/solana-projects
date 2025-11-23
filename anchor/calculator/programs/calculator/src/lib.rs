use anchor_lang::prelude::*;

declare_id!("Cc77fUiSGynWr1bCfUugP9PVYN5ysShksNAsFjRpR22E");

#[program]
pub mod calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
