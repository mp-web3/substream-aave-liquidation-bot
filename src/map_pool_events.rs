use crate::pb::events::v1::Events;
use hex_literal::hex;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::Event;
use crate::abi;
use crate::pb::events::v1 as events;
use crate::util;
use substreams_ethereum::pb::eth::v2::Block;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;


const L2_POOL_TRACKED_CONTRACT: [u8; 20] = hex!("794a61358d6845594f94dc1db02a252b5b4814ad");


#[substreams::handlers::map]
fn map_pool_events
(pool_address: String, blk: Block) -> Result<Events, Error> {
    verify_parameter(&pool_address)?;

    let mut events = Events {
        l2_pool_borrows: vec![],
        l2_pool_flash_loans: vec![],
        l2_pool_liquidation_calls: vec![],
        l2_pool_repays: vec![],
        l2_pool_supplies: vec![],
        l2_pool_withdraws: vec![],
    };

    events.l2_pool_borrows.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::Borrow::match_and_decode(log) {
                        return Some(events::L2PoolBorrow {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            borrow_rate: event.borrow_rate.to_string(),
                            interest_rate_mode: event.interest_rate_mode.to_u64(),
                            on_behalf_of: event.on_behalf_of,
                            referral_code: event.referral_code.to_u64(),
                            reserve: event.reserve,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_liquidation_calls.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::LiquidationCall::match_and_decode(log) {
                        return Some(events::L2PoolLiquidationCall {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            collateral_asset: event.collateral_asset,
                            debt_asset: event.debt_asset,
                            debt_to_cover: event.debt_to_cover.to_string(),
                            liquidated_collateral_amount: event.liquidated_collateral_amount.to_string(),
                            liquidator: event.liquidator,
                            receive_a_token: event.receive_a_token,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_repays.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::Repay::match_and_decode(log) {
                        return Some(events::L2PoolRepay {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            repayer: event.repayer,
                            reserve: event.reserve,
                            use_a_tokens: event.use_a_tokens,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_supplies.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::Supply::match_and_decode(log) {
                        return Some(events::L2PoolSupply {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            on_behalf_of: event.on_behalf_of,
                            referral_code: event.referral_code.to_u64(),
                            reserve: event.reserve,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_withdraws.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::Withdraw::match_and_decode(log) {
                        return Some(events::L2PoolWithdraw {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            reserve: event.reserve,
                            to: event.to,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    Ok(events)
}

// Verify that the provided pool address is valid
fn verify_parameter(address: &String) -> Result<(), Error> {
    if !util::is_address_valid(address) {
        return Err(Error::msg(format!("Contract address ({}) is not valid", address)));
    }
    Ok(())
}