## TODO
- [ ] Make a new client for substrate that can be compiled to wasm module
    - Use parts of the code in substrate-api-client. The substrate-api-client can not be compiled to wasm due to reliance to `std::net` and `tokio` crate.
    - Make use of `reqwest` which has a corresponding code for `wasm32` targets
