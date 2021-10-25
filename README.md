*moved to https://github.com/octopus-network/wrapped-appchain-token*
## Building

To build run:

```bash
./build.sh
```

## Testing

To test run:

```bash
cargo test --package appchain-native-token -- --nocapture
```

## Deploy

To deploy run:

```bash
near dev-deploy
```

Init contract:

```
near call $APPCHAIN_NATIVE_TOKEN new '{"owner_id": "'$RELAY_CONTRACT_ID'", "total_supply": "100000000000000000000000000", "metadata": {"spec": "ft-1.0.0", "name": "TestToken", "symbol": "TEST", "decimals": 18}}' --accountId $SIGNER
```

Set owner:

```bash
near call $APPCHAIN_NATIVE_TOKEN set_owner '{"owner_id": "'$RELAY_CONTRACT_ID'"}' --accountId $SIGNER
```

Get owner:

```bash
near view $APPCHAIN_NATIVE_TOKEN get_owner
```
