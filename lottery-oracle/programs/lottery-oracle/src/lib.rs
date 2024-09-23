use anchor_lang::prelude::*;
use state::*;

declare_id!("HzYLCEWSfBWPZVUSktehaazbp21A4FBApeY5DkonceRz");

#[program]
pub mod lottery_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub user: Signer<'info>,
    pub lottery_state: Account<'info, LotteryState>,
    pub system_program: Program<'info, System>,
}
