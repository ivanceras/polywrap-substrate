import { ApiPromise, Keyring, WsProvider } from "@polkadot/api"

const BOB = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty";

(async () => {
  // api
  const wsProvider = new WsProvider('ws://0.0.0.0:9944');
  const api = await ApiPromise.create({ provider: wsProvider });

  // keyring
  const keyring = new Keyring({ type: "sr25519" });
  const alice = keyring.addFromUri("//Alice");

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
