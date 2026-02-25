use super::{Order, PerpPosition};
use crate::constants::*;
use crate::constraints::*;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    pub authority: Pubkey,
    pub orders: [Order; MAX_ORDERS_PER_USER],
    pub total_collateral: u64,
    pub perp_positions: [PerpPosition; MAX_POSITIONS_PER_USER],
    pub next_order_id: u64,
    pub open_orders: u64,
    pub account_id: u16,
}

impl User {
    pub fn get_last_order_id(&self) -> u64 {
        if self.next_order_id == 1 {
            u64::MAX
        } else {
            self.next_order_id - 1
        }
    }

    pub fn get_order(&self, order_id: u64) -> Option<&Order> {
        self.orders
            .iter()
            .find(|o| o.order_id == order_id && o.status == OrderStatus::Open)
    }
}
