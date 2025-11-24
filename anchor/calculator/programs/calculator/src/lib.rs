use anchor_lang::prelude::*;

declare_id!("Cc77fUiSGynWr1bCfUugP9PVYN5ysShksNAsFjRpR22E");

#[program]
pub mod anchor_calculator {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, init_value: u32) -> Result<()> {
        ctx.accounts.account.num = init_value;
        Ok(())
    }

    pub fn double(ctx: Context<Double>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num * 2;
        Ok(())
    }

    pub fn halve(ctx: Context<Halve>) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num / 2;
        Ok(())
    }

    pub fn add(ctx: Context<Add>, amount: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num + amount;
        Ok(())
    }

    pub fn sub(ctx: Context<Sub>, amount: u32) -> Result<()> {
        ctx.accounts.account.num = ctx.accounts.account.num - amount;
        Ok(())
    }
}

#[account]
pub struct DataShape {
    num: u32,
}
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 8 + 4)]
    pub account: Account<'info, DataShape>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Double<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Halve<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    pub signer: Signer<'info>,
}

#[derive(Accounts)]
pub struct Sub<'info> {
    #[account(mut)]
    pub account: Account<'info, DataShape>,
    pub signer: Signer<'info>,
}
