use substreams::prelude::*;
use substreams::{store::StoreAddInt64, store::StoreSetInt64, store::StoreGetInt64};
use substreams_ethereum::pb::eth::v2::Block;
use crate::pb::eth::event::v1::Events;

// The module will store user balances across blocks
#[substreams::handlers::store]
fn store_user_balances(events: Events, output: StoreAddInt64) {
    // Loop through each `Supply` event in the block
    for event in events.events {
        // Decode `onBehalfOf` and `amount` from topics
        let on_behalf_of = &event.topics[1];
        let amount_str = &event.topics[2];
        
        // Convert amount from hex to i64 (assuming it fits within this range)
        let amount = i64::from_str_radix(amount_str.trim_start_matches("0x"), 16)
            .expect("Amount conversion error");
        
        // Update the cumulative balance for each user (onBehalfOf)
        output.add(on_behalf_of.as_str(), amount);
    }
}
