import { web3Accounts, web3Enable } from '@polkadot/extension-dapp';

(async () => {

  const injectedPromise = web3Enable('forum-app');

  await injectedPromise
    .catch(console.error);

    console.log("injected promise:", injectedPromise);

    let accounts = await web3Accounts();
    console.log("accounts: ", accounts);

})()
