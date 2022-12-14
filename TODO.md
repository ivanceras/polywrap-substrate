## TODO
- [X] Make a new client for substrate that can be compiled to wasm module
    - [X] Use parts of the code in substrate-api-client. The substrate-api-client can not be compiled to wasm due to reliance to `std::net` and `tokio` crate.
    - [X] Make use of `reqwest` which has a corresponding code for `wasm32` targets
- [ ] Make a graphql schema for the rust structs
- [X] Display the substrate struct conforms to graphql schema
    - Done with the graphql playground Schema menu
    - [ ] Maybe also have to write them by hand
- [ ] Make a graphql Object for Block
- [ ] Make the rpc endpoint as an argument to the graphql query
- [ ] Revisit the skipped fields of substrate structs. Solve the issue for those fields
    - #[graphql(skip)]
    - #[serde(skip)]
    - Convert the existing fields
     ```rust
     events: HashMap<(u8, u8), EventMetadata>
     ```
     to hashmap of hashmap so, it can be translated into Json or GraphQL objects
     ```rust
     events: HashMap<u8, HashMap<u8, EventMetadata>>,
     ```
- [ ] Expose some other functionality of substrate
    - [x] show storage items
        - [x] show storage value
        - [ ] show storage maps
        - [ ] show storage double maps
    - [ ] show accounts
    - [ ] Balance of an account
    - [X] Interact with pallets by calling the pallets functionality
    - [X] Execute balance transfers
    - [ ] Show storage maps and values
