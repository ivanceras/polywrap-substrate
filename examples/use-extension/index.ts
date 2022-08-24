import { web3Accounts, web3Enable, web3FromSource } from '@polkadot/extension-dapp';

function stringToHex(str) {
    var hex = '';
    for(var i=0;i<str.length;i++) {
        hex += ''+str.charCodeAt(i).toString(16);
    }
    return hex;
}

export default function showAccounts(){
    console.log("in show accounts..");

    (async () => {

      const injectedPromise = await web3Enable('forum-app');
        console.log("injected promise:", injectedPromise);
        let accounts = await web3Accounts();
        console.log("accounts: ", accounts);


        // `account` is of type InjectedAccountWithMeta 
        // We arbitrarily select the first account returned from the above snippet
        const account = accounts[0];
        console.log("account:", account)

        // to be able to retrieve the signer interface from this account
        // we can use web3FromSource which will return an InjectedExtension type
        const injector = await web3FromSource(account.meta.source);


        // this injector object has a signer and a signRaw method
        // to be able to sign raw bytes
        const signRaw = injector?.signer?.signRaw;

        if (!!signRaw) {
            // after making sure that signRaw is defined
            // we can use it to sign our message
            let payload = stringToHex('message to sign');
            console.log("payload:", payload);
            const { signature } = await signRaw({
                address: account.address,
                data: payload,
                type: 'bytes'
            });

            console.log("signature: ", signature);
        }

    })();

}

