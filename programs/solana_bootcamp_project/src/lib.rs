use anchor_lang::prelude::*;
// use anchor_spl::token::{self, Mint, Token};

declare_id!("DUV1ET47i14AqZnJD1N4CRiQ2X8fJNcPvJ3iXDisfDAz");
use instructions::*;

mod instructions;
mod state;

#[program]
pub mod solana_bootcamp_project {

    use super::*;

    pub fn create_event(
        ctx: Context<CreateEventContract>,
        organizer: Pubkey,
        name: String,
        description: String,
        location: String,
        date: i64,
        ticket_quantity: u32,
        ticket_price: u64,
        tickets_minted: u32,
    ) -> Result<()> {
        proccess_create_event(
            ctx,
            organizer,
            name,
            description,
            location,
            date,
            ticket_quantity,
            ticket_price,
            tickets_minted,
        )
    }
}
