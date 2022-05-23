
# Polywrap substrate

Expose a substrate chain as a graphql endpoint

```shell
cargo run -p server --release
```

Navigate to: http://localhost:8000

Interact with the graphql endpoint with this example query to get the block
```graphql
{

  block(number: 2) {
    number
    header {
      parentHash
      extrinsicsRoot
      stateRoot
    }
  }
}
```
# Show the metadata

```graphql
{
  metadata
}
```

# Links
- https://github.com/w3f/Grants-Program/blob/master/applications/substrate_core_polywrapper.md
- https://github.com/polywrap/integrations/tree/substrate-integration
- https://github.com/ChainSafe/polywrap-substrate
