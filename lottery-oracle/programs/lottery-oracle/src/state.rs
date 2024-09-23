use anchor_lang::prelude::*;

pub struct LotteryState {
    pub authority: Pubkey,
    pub total_tickets: u64,
    pub max_tickets: u64,
    pub ticket_price: u64,
    pub is_active: bool,
    pub winner: Option<Pubkey>,
    pub random_number: Option<u64>,
    pub end_time: i64,
}

impl LotteryState{
    pub fn is_lottery_active(&self) -> bool {
        self.is_active && self.end_time > Clock::get().unwrap.unix_timestamp
    }
}

pub fn initialize(ctx:Context<Initialize>, max_tickets:u64, ticket_price: u64, end_time: i64) -> Result<()> {
    let 
}