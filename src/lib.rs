mod error;

use anchor_lang::prelude::*;
use crate::error::ErrorCode;
declare_id!("66TSa2MG2MMzYSesUAwKdf5SZ72wteTY1En1bzVNC9r1");

pub const STALE_AFTER_SLOTS_ELAPSED: u64 = 1;

#[account]
#[derive(Default, Debug)]
pub struct LpPrice {
    pub last_update_slot:u64,
    pub stale:bool,
    pub lp_price:u64,
    pub lp_price_expo:u8,
    pub total_lp_amount:u64,
}


impl LpPrice {
    pub fn slots_elapsed(&self, slot: u64) -> Result<u64, ProgramError> {
        let slots_elapsed = slot
            .checked_sub(self.last_update_slot)
            .ok_or(ErrorCode::MathOverflow)?;
        Ok(slots_elapsed)
    }
    /// Set last update slot
    pub fn update_slot(&mut self, slot: u64) {
        self.last_update_slot = slot;
        self.stale = false;
    }

    /// Set stale to true
    pub fn mark_stale(&mut self) {
        self.stale = true;
    }

    /// Check if marked stale or last update slot is too long ago
    pub fn is_stale(&self, slot: u64) -> Result<bool, ProgramError> {
        Ok(self.stale || self.slots_elapsed(slot)? >= STALE_AFTER_SLOTS_ELAPSED)
    }
}
#[account]
#[derive(Default, Debug, PartialEq, Eq)]
pub struct WithdrawLpAccount{
    pub owner:Pubkey,
    pub pool_id:Pubkey,
    pub amount:u64,
}