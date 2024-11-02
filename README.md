# aave_liquidation_bot Substreams modules

This package was initialized via `substreams init`, using the `evm-events-calls` template.

## Usage

```bash
substreams build
substreams auth
substreams gui
```

## Modules

All of these modules produce data filtered by these contracts:
- _l2_pool_ at **0x794a61358d6845594f94dc1db02a252b5b4814ad**
### `map_events_calls`

This module gets you events _and_ calls


### `map_events`

This module gets you only events that matched.



### `map_calls`

This module gets you only calls that matched.


---

```
substreams run -e arb-one.streamingfast.io:443 substreams.yaml map_events --start-block 270295702 --stop-block +1
```