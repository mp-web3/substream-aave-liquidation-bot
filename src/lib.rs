mod abi;
mod pb;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

const L2_POOL_TRACKED_CONTRACT: [u8; 20] = hex!("794a61358d6845594f94dc1db02a252b5b4814ad");

fn map_l2_pool_events(blk: &eth::Block, events: &mut contract::Events) {
    events.l2_pool_back_unbackeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::BackUnbacked::match_and_decode(log) {
                        return Some(contract::L2PoolBackUnbacked {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            backer: event.backer,
                            fee: event.fee.to_string(),
                            reserve: event.reserve,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_borrows.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::Borrow::match_and_decode(log) {
                        return Some(contract::L2PoolBorrow {
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
    events.l2_pool_flash_loans.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::FlashLoan::match_and_decode(log) {
                        return Some(contract::L2PoolFlashLoan {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount: event.amount.to_string(),
                            asset: event.asset,
                            initiator: event.initiator,
                            interest_rate_mode: event.interest_rate_mode.to_u64(),
                            premium: event.premium.to_string(),
                            referral_code: event.referral_code.to_u64(),
                            target: event.target,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_isolation_mode_total_debt_updateds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::IsolationModeTotalDebtUpdated::match_and_decode(log) {
                        return Some(contract::L2PoolIsolationModeTotalDebtUpdated {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            asset: event.asset,
                            total_debt: event.total_debt.to_string(),
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
                        return Some(contract::L2PoolLiquidationCall {
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
    events.l2_pool_mint_unbackeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::MintUnbacked::match_and_decode(log) {
                        return Some(contract::L2PoolMintUnbacked {
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
    events.l2_pool_minted_to_treasuries.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::MintedToTreasury::match_and_decode(log) {
                        return Some(contract::L2PoolMintedToTreasury {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            amount_minted: event.amount_minted.to_string(),
                            reserve: event.reserve,
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
                        return Some(contract::L2PoolRepay {
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
    events.l2_pool_reserve_data_updated_1s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::ReserveDataUpdated1::match_and_decode(log) {
                        return Some(contract::L2PoolReserveDataUpdated1 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            liquidity_index: event.liquidity_index.to_string(),
                            liquidity_rate: event.liquidity_rate.to_string(),
                            reserve: event.reserve,
                            stable_borrow_rate: event.stable_borrow_rate.to_string(),
                            variable_borrow_index: event.variable_borrow_index.to_string(),
                            variable_borrow_rate: event.variable_borrow_rate.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_reserve_data_updated_2s.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::ReserveDataUpdated2::match_and_decode(log) {
                        return Some(contract::L2PoolReserveDataUpdated2 {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            liquidity_index: event.liquidity_index.to_string(),
                            liquidity_rate: event.liquidity_rate.to_string(),
                            reserve: event.reserve,
                            stable_borrow_rate: event.stable_borrow_rate.to_string(),
                            variable_borrow_index: event.variable_borrow_index.to_string(),
                            variable_borrow_rate: event.variable_borrow_rate.to_string(),
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_reserve_used_as_collateral_disableds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::ReserveUsedAsCollateralDisabled::match_and_decode(log) {
                        return Some(contract::L2PoolReserveUsedAsCollateralDisabled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            reserve: event.reserve,
                            user: event.user,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_reserve_used_as_collateral_enableds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::ReserveUsedAsCollateralEnabled::match_and_decode(log) {
                        return Some(contract::L2PoolReserveUsedAsCollateralEnabled {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            reserve: event.reserve,
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
                        return Some(contract::L2PoolSupply {
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
    events.l2_pool_upgradeds.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::Upgraded::match_and_decode(log) {
                        return Some(contract::L2PoolUpgraded {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            implementation: event.implementation,
                        });
                    }

                    None
                })
        })
        .collect());
    events.l2_pool_user_e_mode_sets.append(&mut blk
        .receipts()
        .flat_map(|view| {
            view.receipt.logs.iter()
                .filter(|log| log.address == L2_POOL_TRACKED_CONTRACT)
                .filter_map(|log| {
                    if let Some(event) = abi::l2_pool_contract::events::UserEModeSet::match_and_decode(log) {
                        return Some(contract::L2PoolUserEModeSet {
                            evt_tx_hash: Hex(&view.transaction.hash).to_string(),
                            evt_index: log.block_index,
                            evt_block_time: Some(blk.timestamp().to_owned()),
                            evt_block_number: blk.number,
                            category_id: event.category_id.to_u64(),
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
                        return Some(contract::L2PoolWithdraw {
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
}
fn map_l2_pool_calls(blk: &eth::Block, calls: &mut contract::Calls) {
    calls.l2_pool_call_admins.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Admin::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Admin::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::Admin::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolAdminCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_back_unbackeds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::BackUnbacked::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::BackUnbacked::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::BackUnbacked::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolBackUnbackedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                fee: decoded_call.fee.to_string(),
                                output_param0: output_param0.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_borrow_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Borrow1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Borrow1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolBorrow1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                interest_rate_mode: decoded_call.interest_rate_mode.to_string(),
                                on_behalf_of: decoded_call.on_behalf_of,
                                referral_code: decoded_call.referral_code.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_borrow_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Borrow2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Borrow2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolBorrow2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_configure_e_mode_categories.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::ConfigureEModeCategory::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::ConfigureEModeCategory::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolConfigureEModeCategoryCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                id: decoded_call.id.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_configure_e_mode_category_borrowable_bitmaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::ConfigureEModeCategoryBorrowableBitmap::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::ConfigureEModeCategoryBorrowableBitmap::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolConfigureEModeCategoryBorrowableBitmapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                borrowable_bitmap: decoded_call.borrowable_bitmap.to_string(),
                                id: decoded_call.id.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_configure_e_mode_category_collateral_bitmaps.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::ConfigureEModeCategoryCollateralBitmap::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::ConfigureEModeCategoryCollateralBitmap::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolConfigureEModeCategoryCollateralBitmapCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                collateral_bitmap: decoded_call.collateral_bitmap.to_string(),
                                id: decoded_call.id.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_deposits.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Deposit::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Deposit::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolDepositCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                on_behalf_of: decoded_call.on_behalf_of,
                                referral_code: decoded_call.referral_code.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_drop_reserves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::DropReserve::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::DropReserve::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolDropReserveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_finalize_transfers.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::FinalizeTransfer::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::FinalizeTransfer::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolFinalizeTransferCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                balance_from_before: decoded_call.balance_from_before.to_string(),
                                balance_to_before: decoded_call.balance_to_before.to_string(),
                                from: decoded_call.from,
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_flash_loans.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::FlashLoan::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::FlashLoan::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolFlashLoanCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amounts: decoded_call.amounts.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                assets: decoded_call.assets.into_iter().map(|x| x).collect::<Vec<_>>(),
                                interest_rate_modes: decoded_call.interest_rate_modes.into_iter().map(|x| x.to_string()).collect::<Vec<_>>(),
                                on_behalf_of: decoded_call.on_behalf_of,
                                params: decoded_call.params,
                                receiver_address: decoded_call.receiver_address,
                                referral_code: decoded_call.referral_code.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_flash_loan_simples.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::FlashLoanSimple::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::FlashLoanSimple::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolFlashLoanSimpleCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                params: decoded_call.params,
                                receiver_address: decoded_call.receiver_address,
                                referral_code: decoded_call.referral_code.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_get_liquidation_grace_periods.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::GetLiquidationGracePeriod::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::GetLiquidationGracePeriod::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::GetLiquidationGracePeriod::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolGetLiquidationGracePeriodCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                                output_param0: output_param0.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_implementations.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Implementation::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Implementation::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::Implementation::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolImplementationCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                output_param0: output_param0,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_init_reserves.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::InitReserve::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::InitReserve::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolInitReserveCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                a_token_address: decoded_call.a_token_address,
                                asset: decoded_call.asset,
                                interest_rate_strategy_address: decoded_call.interest_rate_strategy_address,
                                variable_debt_address: decoded_call.variable_debt_address,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_initialize_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Initialize1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Initialize1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolInitialize1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                u_data: decoded_call.u_data,
                                u_logic: decoded_call.u_logic,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_initialize_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Initialize2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Initialize2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolInitialize2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                provider: decoded_call.provider,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_liquidation_call_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::LiquidationCall1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::LiquidationCall1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolLiquidationCall1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                collateral_asset: decoded_call.collateral_asset,
                                debt_asset: decoded_call.debt_asset,
                                debt_to_cover: decoded_call.debt_to_cover.to_string(),
                                receive_a_token: decoded_call.receive_a_token,
                                user: decoded_call.user,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_liquidation_call_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::LiquidationCall2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::LiquidationCall2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolLiquidationCall2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args1: Vec::from(decoded_call.args1),
                                args2: Vec::from(decoded_call.args2),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_mint_to_treasuries.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::MintToTreasury::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::MintToTreasury::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolMintToTreasuryCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                assets: decoded_call.assets.into_iter().map(|x| x).collect::<Vec<_>>(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_mint_unbackeds.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::MintUnbacked::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::MintUnbacked::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolMintUnbackedCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                on_behalf_of: decoded_call.on_behalf_of,
                                referral_code: decoded_call.referral_code.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_repay_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Repay1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Repay1::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::Repay1::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolRepay1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                                output_param0: output_param0.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_repay_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Repay2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Repay2::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::Repay2::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolRepay2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                interest_rate_mode: decoded_call.interest_rate_mode.to_string(),
                                on_behalf_of: decoded_call.on_behalf_of,
                                output_param0: output_param0.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_repay_with_a_tokens_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::RepayWithATokens1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::RepayWithATokens1::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::RepayWithATokens1::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolRepayWithATokens1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                interest_rate_mode: decoded_call.interest_rate_mode.to_string(),
                                output_param0: output_param0.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_repay_with_a_tokens_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::RepayWithATokens2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::RepayWithATokens2::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::RepayWithATokens2::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolRepayWithATokens2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                                output_param0: output_param0.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_repay_with_permit_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::RepayWithPermit1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::RepayWithPermit1::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::RepayWithPermit1::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolRepayWithPermit1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                                output_param0: output_param0.to_string(),
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_repay_with_permit_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::RepayWithPermit2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::RepayWithPermit2::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::RepayWithPermit2::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolRepayWithPermit2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                deadline: decoded_call.deadline.to_string(),
                                interest_rate_mode: decoded_call.interest_rate_mode.to_string(),
                                on_behalf_of: decoded_call.on_behalf_of,
                                output_param0: output_param0.to_string(),
                                permit_r: Vec::from(decoded_call.permit_r),
                                permit_s: Vec::from(decoded_call.permit_s),
                                permit_v: decoded_call.permit_v.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_rescue_tokens.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::RescueTokens::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::RescueTokens::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolRescueTokensCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                to: decoded_call.to,
                                token: decoded_call.token,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_reset_isolation_mode_total_debts.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::ResetIsolationModeTotalDebt::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::ResetIsolationModeTotalDebt::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolResetIsolationModeTotalDebtCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_set_configurations.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SetConfiguration::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SetConfiguration::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSetConfigurationCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_set_liquidation_grace_periods.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SetLiquidationGracePeriod::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SetLiquidationGracePeriod::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSetLiquidationGracePeriodCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                                until: decoded_call.until.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_set_reserve_interest_rate_strategy_addresses.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SetReserveInterestRateStrategyAddress::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SetReserveInterestRateStrategyAddress::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSetReserveInterestRateStrategyAddressCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                                rate_strategy_address: decoded_call.rate_strategy_address,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_set_user_e_modes.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SetUserEMode::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SetUserEMode::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSetUserEModeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                category_id: decoded_call.category_id.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_set_user_use_reserve_as_collateral_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SetUserUseReserveAsCollateral1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SetUserUseReserveAsCollateral1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSetUserUseReserveAsCollateral1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_set_user_use_reserve_as_collateral_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SetUserUseReserveAsCollateral2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SetUserUseReserveAsCollateral2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSetUserUseReserveAsCollateral2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                                use_as_collateral: decoded_call.use_as_collateral,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_supply_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Supply1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Supply1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSupply1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                on_behalf_of: decoded_call.on_behalf_of,
                                referral_code: decoded_call.referral_code.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_supply_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Supply2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Supply2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSupply2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_supply_with_permit_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SupplyWithPermit1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SupplyWithPermit1::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSupplyWithPermit1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                deadline: decoded_call.deadline.to_string(),
                                on_behalf_of: decoded_call.on_behalf_of,
                                permit_r: Vec::from(decoded_call.permit_r),
                                permit_s: Vec::from(decoded_call.permit_s),
                                permit_v: decoded_call.permit_v.to_u64(),
                                referral_code: decoded_call.referral_code.to_u64(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_supply_with_permit_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SupplyWithPermit2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SupplyWithPermit2::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSupplyWithPermit2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                                r: Vec::from(decoded_call.r),
                                s: Vec::from(decoded_call.s),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_sync_indexes_states.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SyncIndexesState::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SyncIndexesState::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSyncIndexesStateCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_sync_rates_states.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::SyncRatesState::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::SyncRatesState::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolSyncRatesStateCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                asset: decoded_call.asset,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_update_bridge_protocol_fees.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::UpdateBridgeProtocolFee::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::UpdateBridgeProtocolFee::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolUpdateBridgeProtocolFeeCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                protocol_fee: decoded_call.protocol_fee.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_update_flashloan_premiums.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::UpdateFlashloanPremiums::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::UpdateFlashloanPremiums::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolUpdateFlashloanPremiumsCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                flash_loan_premium_to_protocol: decoded_call.flash_loan_premium_to_protocol.to_string(),
                                flash_loan_premium_total: decoded_call.flash_loan_premium_total.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_upgrade_tos.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::UpgradeTo::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::UpgradeTo::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolUpgradeToCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_upgrade_to_and_calls.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::UpgradeToAndCall::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::UpgradeToAndCall::decode(call) {
                        Ok(decoded_call) => {
                            Some(contract::L2PoolUpgradeToAndCallCall {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                data: decoded_call.data,
                                new_implementation: decoded_call.new_implementation,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_withdraw_1s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Withdraw1::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Withdraw1::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::Withdraw1::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolWithdraw1call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                amount: decoded_call.amount.to_string(),
                                asset: decoded_call.asset,
                                output_param0: output_param0.to_string(),
                                to: decoded_call.to,
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
    calls.l2_pool_call_withdraw_2s.append(&mut blk
        .transactions()
        .flat_map(|tx| {
            tx.calls.iter()
                .filter(|call| call.address == L2_POOL_TRACKED_CONTRACT && abi::l2_pool_contract::functions::Withdraw2::match_call(call))
                .filter_map(|call| {
                    match abi::l2_pool_contract::functions::Withdraw2::decode(call) {
                        Ok(decoded_call) => {
                            let output_param0 = match abi::l2_pool_contract::functions::Withdraw2::output(&call.return_data) {
                                Ok(output_param0) => {output_param0}
                                Err(_) => Default::default(),
                            };
                            
                            Some(contract::L2PoolWithdraw2call {
                                call_tx_hash: Hex(&tx.hash).to_string(),
                                call_block_time: Some(blk.timestamp().to_owned()),
                                call_block_number: blk.number,
                                call_ordinal: call.begin_ordinal,
                                call_success: !call.state_reverted,
                                args: Vec::from(decoded_call.args),
                                output_param0: output_param0.to_string(),
                            })
                        },
                        Err(_) => None,
                    }
                })
        })
        .collect());
}

#[substreams::handlers::map]
fn map_events_calls(
    events: contract::Events,
    calls: contract::Calls,
) -> Result<contract::EventsCalls, substreams::errors::Error> {
    Ok(contract::EventsCalls {
        events: Some(events),
        calls: Some(calls),
    })
}
#[substreams::handlers::map]
fn map_events(blk: eth::Block) -> Result<contract::Events, substreams::errors::Error> {
    let mut events = contract::Events::default();
    map_l2_pool_events(&blk, &mut events);
    Ok(events)
}
#[substreams::handlers::map]
fn map_calls(blk: eth::Block) -> Result<contract::Calls, substreams::errors::Error> {
let mut calls = contract::Calls::default();
    map_l2_pool_calls(&blk, &mut calls);
    Ok(calls)
}

