
# Polywrap substrate

Expose a substrate chain as a graphql endpoint

## Prerequisite
Clone and run `substrate-node-template`

```shell
git clone --depth=1 https://github.com/substrate-developer-hub/substrate-node-template
cd substrate-node-template
cargo run --release -- --dev
```

## Building and running

```shell
git clone -b develop https://github.com/ChainSafe/polywrap-substrate.git
cd polywrap-substrate
cargo run -p server --release
```

Navigate to: http://localhost:8000

Interact with the graphql endpoint with this example query to get the block
```graphql
{
  block(url: "http://localhost:9933", number: 0) {
    block
  }
}
```
# Show the metadata

```graphql
{
  metadata(url: "http://localhost:9933") {
    metadata
    pallets
    events
    errors
  }
}
```

# Show the rpc Methods

```graphql
{
  rpcMethods(url: "http://localhost:9933")
}
```

# You can even call multiple methods in the same graphql
```graphql
{
  block(url: "http://localhost:9933", number: 0) {
    block
  }
  rpcMethods(url: "http://localhost:9933")
  metadata(url: "http://localhost:9933") {
    metadata
    pallets
    events
    errors
  }
}
```

## Testing
Run all the unit tests
```shell
cargo test --all
```

# Links
- https://github.com/w3f/Grants-Program/blob/master/applications/substrate_core_polywrapper.md
- https://github.com/polywrap/integrations/tree/substrate-integration
- https://github.com/ChainSafe/polywrap-substrate
