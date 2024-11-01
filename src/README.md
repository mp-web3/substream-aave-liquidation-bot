# map_contract_events.rs

run the module with:

```
substreams run -e arb-one.streamingfast.io:443 substreams.yaml map_contract_events --start-block 269630912 --stop-block +1
```

## Inspecting the Code

Declaration of the module in the manifest (substreams.yaml):

```
- name: map_contract_events
    kind: map
    inputs:
      - params: string
      - source: sf.ethereum.type.v2.Block
    output:
      type: proto:eth.event.v1.Events
```

> The module expects two inputs: the parameter as a string, and a raw Ethereum block. The output is the Events object defined in the Protobuf.

The corresponding Rust function declaration, which matches the name of the module, map_contract_events:

```
fn map_contract_events(contract_address: String, blk: Block) -> Result<Events, Error> {
    verify_parameter(&contract_address)?; // Verify address

    let events: Vec<Event> = blk
        .logs() // 1.
        .filter(|log| log.address().to_vec() == Hex::decode(&contract_address).expect("already validated")) // 2.
        .map(|log| Event { // 3.
            address: Hex::encode(log.address()),
            topics: log.topics().into_iter().map(Hex::encode).collect(),
            tx_hash: Hex::encode(&log.receipt.transaction.hash),
        })
        .collect(); // 4.

    Ok(Events { events })
}
```

In this example, you do not need to parse the parameters, as contract_address is the only string passed and you can use it directly. However, it is necessary to verify that the parameter is a valid Ethereum; this verification is performed by the verify_parameter function.

Then, you iterate over the events of the contract:

The .logs() function iterates over the logs of successful transactions within the block.

For every log of a successful transaction, you verify if its address matches the smart contract address (i.e. you verify if the log was actually emitted by the smart contract). For the comparison, both log.address() and contract_address are converted to `Vec<u8>`.

Every filtered log (i.e. every log that belongs to the smart contract) is mapped to a pb::eth::event::v1::Event struct, which was specified in the Protobuf definition.

Finally, you collect all the events in a vector.

## Example 
parameter: "0x794a61358D6845594F94dc1DB02A252b5b4814aD" (Aave V3 Pool contract on Arbitrum)
start block: 269630912

run the module with:

```
substreams run -e arb-one.streamingfast.io:443 substreams.yaml map_contract_events --start-block 269630912 --stop-block +1
```


The result of running the module:


```
----------- BLOCK #269,630,912 (c216f55431da7584fcf8bc6b06bb0fb6871b18d763d15a00ad205eae1abb2df3) ---------------
{
  "@module": "map_contract_events",
  "@block": 269630912,
  "@type": "eth.event.v1.Events",
  "@data": {
    "events": [
      {
        "address": "794a61358d6845594f94dc1db02a252b5b4814ad",
        "topics": [
          "804c9b842b2748a22bb64b345453a3de7ca54a6ca45ce00d415894979e22897a",
          "000000000000000000000000fd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9"
        ],
        "txHash": "ece8990ab0514e427a4eea65e954f5f5c6077def0a686f2e9aa231331cdaac1d"
      },
      {
        "address": "794a61358d6845594f94dc1db02a252b5b4814ad",
        "topics": [
          "b3d084820fb1a9decffb176436bd02558d15fac9b0ddfed8c465bc7359d7dce0",
          "000000000000000000000000fd086bc7cd5c481dcc9c85ebe478a1c0b69fcbb9",
          "00000000000000000000000062a2aab792d506b7181bd3710f04a603b2223954",
          "0000000000000000000000000000000000000000000000000000000000000000"
        ],
        "txHash": "ece8990ab0514e427a4eea65e954f5f5c6077def0a686f2e9aa231331cdaac1d"
      }
    ]
  }
}
```

0xece8990ab0514e427a4eea65e954f5f5c6077def0a686f2e9aa231331cdaac1d