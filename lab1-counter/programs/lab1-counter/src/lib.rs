use anchor_lang::prelude::*;

declare_id!("9tRDqYisRoNbPorAjC6pCbTEFX7jZzFKSJf1uq1d8uFn");

#[program]
pub mod lab1_counter {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        msg!("Setting counter to {}", counter.count);
        Ok(())
    }
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter value {}", counter.count);
        counter.count =counter.count.checked_add(1).unwrap();
        msg!("Incremented counter value {}", counter.count);
        Ok(())
    }
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter value {}", counter.count);
        counter.count =counter.count.checked_sub(1).unwrap();
        msg!("Decremented counter value {}", counter.count);
        Ok(())
    }

}
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + Counter::INIT_SPACE)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    pub user: Signer<'info>,
}

