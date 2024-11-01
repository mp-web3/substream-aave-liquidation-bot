use crate::pb::eth::event::v1::Event;
use crate::pb::eth::event::v1::Events;
use crate::util;
use anyhow::anyhow;
use anyhow::Ok;
use substreams::errors::Error;
use substreams::Hex;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
fn map_contract_events(contract_address: String, blk: Block) -> Result<Events, Error> {
    verify_parameter(&contract_address)?;

    let events: Vec<Event> = blk
        // Iterate over the logs of successful transactions within the block
        .logs()
        // verify if its address matches the smart contract address
        .filter(|log| log.address().to_vec() == Hex::decode(&contract_address).expect("already validated")) 
        .map(|log| Event {
            address: Hex::encode(log.address()),
            topics: log.topics().into_iter().map(Hex::encode).collect(),
            tx_hash: Hex::encode(&log.receipt.transaction.hash),
        })
        .collect();

    Ok(Events { events })
}

fn verify_parameter(address: &String) -> Result<(), Error> {
    if !util::is_address_valid(&address) {
        return Err(anyhow!("Contract address ({}) is not valid", address));
    }

    Ok(())
}


// ----------- BLOCK #269,630,912 (c216f55431da7584fcf8bc6b06bb0fb6871b18d763d15a00ad205eae1abb2df3) ---------------
// {
//   "@module": "map_contract_events",
//   "@block": 269630912,
//   "@type": "eth.event.v1.Events",
//   "@data": {
//     "events": [
//       {
//         "address": "794a61358d6845594f94dc1db02a252b5b4814ad",
//         "topics": [
//           "804c9b842b2748a22bb64b345453a3de7ca54a6ca45ce00d415894979e22897a",
//           "000000000000000000000000fd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9"
//         ],
//         "txHash": "ece8990ab0514e427a4eea65e954f5f5c6077def0a686f2e9aa231331cdaac1d"
//       },
//       {
//         "address": "794a61358d6845594f94dc1db02a252b5b4814ad",
//         "topics": [
//           "b3d084820fb1a9decffb176436bd02558d15fac9b0ddfed8c465bc7359d7dce0",
//           "000000000000000000000000fd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9",
//           "00000000000000000000000062a2aab792d506b7181bd3710f04a603b2223954",
//           "0000000000000000000000000000000000000000000000000000000000000000"
//         ],
//         "txHash": "ece8990ab0514e427a4eea65e954f5f5c6077def0a686f2e9aa231331cdaac1d"
//       }
//     ]
//   }
// }
