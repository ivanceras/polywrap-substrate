import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"

const BOB = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

(async () => {
  // api
  const wsProvider = new WsProvider('ws://0.0.0.0:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // keyring
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

  console.log("alice: {}", alice);
    console.log("alice public: {}", alice.publicKey);
  console.log("alice to_json: {}", alice.toJson());
  console.log("JSON alice: {}", JSON.stringify(alice));
 console.log("alice encoded pkcs8: {}", alice.encodePkcs8());

 let totalIssuance = await api.query.balances.totalIssuance();
    console.log("total issuance: ", totalIssuance);

    let account = await api.query.balances.account(alice.address);
    console.log("account: ", account);

    let account2 = await api.query.balances.account(alice.publicKey);
    console.log("account2: ", account2);

  // Make a transfer from Alice to BOB, waiting for inclusion
  await api.tx.balances
    .transfer(BOB, 12345)
    .signAndSend(alice, (result) => {
      console.log(`Current status is ${result.status}`);

      if (result.status.isInBlock) {
        console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
      } else if (result.status.isFinalized) {
        console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
        process.exit();
      }
    });
})()
