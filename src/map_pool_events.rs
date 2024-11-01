use crate::pb::eth::event::v1::Event;
use crate::pb::eth::event::v1::Events;
use crate::util;
use anyhow::anyhow;
use anyhow::Ok;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::EventTrait;

// Address of the Aave contract
const CONTRACT_ADDRESS: &str = "0x794a61358D6845594F94dc1DB02A252b5b4814aD";

// `Supply` event signature
const SUPPLY_EVENT_SIGNATURE: &str = "0x8e1af1fe2fefe401e1727a80e3d1fc7acdc803f625b7b84c8e8da256a46c3e3a";

#[substreams::handlers::map]
fn map_contract_events(blk: Block) -> Result<Events, Error> {
    let mut events = vec![];

    for log in blk.logs() {
        // Filter for logs from the Aave contract address
        if log.address().to_vec() != Hex::decode(CONTRACT_ADDRESS).expect("Invalid contract address") {
            continue;
        }

        // Check if the event matches the `Supply` event signature
        if log.topics.get(0).map(Hex::encode).as_deref() != Some(SUPPLY_EVENT_SIGNATURE) {
            continue;
        }

        // Parse `Supply` event details
        if let [_, reserve, on_behalf_of, amount] = &log.topics[..] {
            events.push(Event {
                address: Hex::encode(log.address()),
                topics: vec![
                    Hex::encode(reserve),
                    Hex::encode(on_behalf_of),
                    Hex::encode(amount),
                ],
                tx_hash: Hex::encode(&log.receipt.transaction.hash),
            });
        }
    }

    Ok(Events { events })
}

fn verify_parameter(address: &String) -> Result<(), Error> {
    if !util::is_address_valid(&address) {
        return Err(anyhow!("Contract address ({}) is not valid", address));
    }

    Ok(())
}
