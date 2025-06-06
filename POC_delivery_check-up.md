# POC Delivery :mailbox:

**Deliverables**

| Number | Deliverable | Link | Notes |
|--|--|--|--|
| 1. :heavy_check_mark: | License | [CLI license](https://github.com/bsn-si/figaro-cli/blob/main/LICENSE), [contract license](https://github.com/bsn-si/figaro-contract/blob/main/LICENSE) | Apache 2.0 |
| 2. :heavy_check_mark: | Smart contract | Figaro [CosmWasm smart contract](https://github.com/bsn-si/figaro-contract) | This is a "proof of concept" for testing the ZK-evidence feature - an algorythm to proove a successful delivery by a courrier using a smart contract and user's wallets (private keys) |
| 3. :heavy_check_mark: | Test CLI | Figaro [CLI test tool](https://github.com/bsn-si/figaro-cli) | A simple CLI tool to provide testing of declared functions of the smart contract | 

### Features test example:

Here's the video of the installation and testing process:

[<img width="406" alt="Untitled" src="https://user-images.githubusercontent.com/98888366/188181026-5299addc-cb9b-412f-8ec9-c15d210f6115.png">](https://media.bsn.si/ipehr/figaro_install_and_using_tests.mp4)

## Installing & testing instructions

To give access to `figaro-cli` command in your environment:
```
git clone git@github.com:bsn-si/figaro-cli.git
cd figaro-cli/ && npm run install:global
```

`Tested on MacOS & Linux, with wasmd-0.27+ and nodejs 16.0`

Also you can run CLI from the docker:

``` bash
git clone git@github.com:bsn-si/figaro-cli.git
cd figaro-cli/

docker build -t figaro-cli:latest .
docker run --restart always --network host -v ~/.figaro:/root/.figaro figaro-cli:latest --help

# Also you can add command alias
alias figaro-cli="docker run --restart always --network host -v ~/.figaro:/root/.figaro figaro-cli:latest"
figaro-cli --help
```

Or generate single binary bundle of the CLI tool via `node-pkg`. (You can modify targets in package.json, or run a custom command manually)

``` bash
npm run build:bundle
```

## Before interaction
For some operations a node RPC is needed, by default `http://127.0.0.1:26657` is used.
To install a local node please follow installation instructions provided [here](https://docs.cosmwasm.com/docs/1.0/getting-started/installation). Also we have a simple helper script to configure `wasmd` [here](https://github.com/bsn-si/figaro-contract/blob/main/common/setup.sh).

### Config
By default you can find a config for the CLI tool in `~/.figaro/config.json` and have these options:

``` js
{
  // Enable log results
  "logging": true,
  // Enable full error tracing
  "trace": true,
  // RPC endpoint
  "apiUrl": "http://127.0.0.1:26657",
  // bech32 prefix for target node
  "addressPrefix": "wasm",
  // output options
  "display": {
    "bech32": true
  },
  // name of node native coins
  "units": {
    "stake": "ustake",
    "fee": "ufee"
  }
}
```

Also you can set data directory with environment variable `DATA_DIR`, this can be used for different networks or databases.

*Example config for Malaga testnet*

``` js
{
  "logging": true,
  "trace": true,
  "apiUrl": "https://rpc.malaga-420.cosmwasm.com/",
  "addressPrefix": "wasm",
  "display": {
    "bech32": true
  },
  "units": {
    "stake": "uand",
    "fee": "umlg"
  }
}
```

## Tests
In the `test` folder you'll find a simple e2e test script for running all features of the contract: from instantiating a new cw20 token to confirmation of a delivery by a courier.

``` bash
cd test/
chmod +x test.sh
./test.sh
```

## Usage
Please use `--help` to get info about all commands & options.

``` bash
➜  ~ figaro-cli 
Usage: figaro-cli [options] [command]

A tool to interact with Figaro: manage requests & delivery

Options:
  --json          output all results as json
  -V, --version   output the version number
  -h, --help      display help for command

Commands:
  sender          Interact as a sender
  courier         Interact as a courier
  common          Shared command & contract query for everyone
  help [command]  display help for command
```

### Helpers

#### Convert mnemonic to `secp256k1` hex
After typing `figaro-cli common mnemonic_to_hex` you need enter mnemonic to prompt, after that you give hex codes.

``` bash
figaro-cli common mnemonic_to_hex
Please enter mnemonic to convert
**********************

Private Key         0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 
Public Key          0x03a01544b4f2523aaf9ac889d89a49d11fcd3ad09a4d962dc6f4fa6519277b9620 
```

#### Get account balance
With this command you can get balance of the signer's account (by `--secret` option), or by address with option `--address` in bech32

``` bash
figaro-cli common balance \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --address wasm1xyuvrj4wrqr40es6pxpxh67fz69uuhp4musjnp

Address             wasm1xyuvrj4wrqr40es6pxpxh67fz69uuhp4musjnp 
Balance Stake       1000000000 ustake 
Balance Fee         999030252 ufee 
```

#### Upload contract code
Before instantiating a contract you need to have a `code_id` of the contract in Cosmos. You can get `code_id` when you upload contract to a node. To upload the Figaro contract to a node:

``` bash
figaro-cli common upload_contract \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82

Contract uploaded
Code Id             9 
Transaction Hash    4201E9FD5349ECB81C185DF89BE5AC4C048D50A3E958ECAAB3B99500C4DFE3F5 
Gas Used            1930812 
```

#### Base information about contract
Also you can request information from the contract about current status, locations, applied courier, etc.

_status_

``` bash
figaro-cli common info \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \
  --query status

Contract "wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs" status info
Contract            wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Query               status 
Result              "WaitPaymentBySender"
```

_locations_

``` bash
figaro-cli common info \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \
  --query locations

Contract "wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs" locations info
Contract            wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Query               locations 
Result              {
  "exact": {
    "from": "",
    "to": ""
  },
  "rough": {
    "from": "[[0,0],[0,0]]",
    "to": "[[2,2],[2,2]]"
  }
} 
```

_token info_

``` bash
figaro-cli common info \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \
  --query token_info

Contract "wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs" token_info info
Contract            wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Query               token_info 
Result              [
  "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d",
  {
    "name": "Token",
    "symbol": "TOK",
    "decimals": 6,
    "total_supply": "2000000"
  }
] 
```

_funds_

``` bash
figaro-cli common info \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \
  --query funds     

Contract "wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs" funds info
Contract            wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Query               funds 
Result              {
  "deposit": "200",
  "payment": "200"
}
```

#### Instantiate new CW20 contract
Also you can instantiate a new cw20 token from the CLI

``` bash
figaro-cli common cw20_instantiate \
  --secret 0x6bead0e84230da9ee73ec5b151776a871ef50a3da1660a32c79b4735d6219103 \
  --initial-balances wasm1xyuvrj4wrqr40es6pxpxh67fz69uuhp4musjnp_1000000,wasm1m7n94k95yt3ha26eqme5f55pclyypmhrpljpuq_1000000 \
  --minter wasm1xyuvrj4wrqr40es6pxpxh67fz69uuhp4musjnp_99900000000 \
  --decimals 6 \
  --symbol TOK \
  --name Token
```

### Sender

#### Instantiate/Make new request for delivery
This command creates a new instance of the Figaro delivery contract. You have to specify starting options that cannot be changed later - an amount of payment for the delivery, a required deposit from the courier, an address of the cw20 token or tokens that inherit it (for example, Tgrade), as well as approximate locations of the zones "from" and "to" where the delivery will be made.

The delivery zone format is a rectangle created from two geo coordinates. `lng,lat|lng,lat` or `0.0,0.0|0.0,0.0`.

Also here you specify a `secp256k1` public key from the secret verification key that you will give to the recipient. Or you can pass that option and command will generate secret & public key for this contract.

``` bash
figaro-cli sender instantiate \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --location-from "0.0,0.0|0.0,0.0" \
  --location-to "2.0,2.0|2.0,2.0" \
  --token wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d \
  --confirm-public 03678b9fe66df3403176ce96b041a004d5d8d8996783083b2a2c813ff17f90aaee \
  --deposit 200 \
  --payment 200 \
  --contract-code-id 9

Contract Instantiated
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    AB1CB9453B541711B381D9E425C20A27F436252D14CC99F3758F19959ECFCF33 
Gas Used            273741
```

#### Make payment
Before searching for couriers you need to pay for this request (This is a part of the guarantee mechanism).

_Disclaimer: this command automatically requests permission to withdraw funds from your wallet._  

``` bash
figaro-cli sender make_payment \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs

Delivery payment
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    E8BD43CD7F7DB2BD894E6993A707E0CAD6F17EA9BB187A11838D5C53AA6755C8 
Gas Used            288144 
Logs                [
  # ...detailed transaction logs
]
```

#### Set details
After a courier has made his deposit, you need to enter the exact locations and comment on the delivery where the courier should come to pick up the parcel.


``` bash
figaro-cli sender set_details \
--secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
--contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \
--location-from 1.0,1.0 \
--location-to 2.0,2.0 \
--comment Comment!   
   
Set destination details
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    0C927B477BD5A6721C8BD9C4C759FB8CF68791780B93543DDB35B57DBA34C65D 
Gas Used            149073 
Logs                [
  # ...detailed transaction logs
]
```

#### Parcel issued to the courier
Сommand to confirm that the package has been passed to the courier.


``` bash
figaro-cli sender approve_parcel_issued \
  --secret 0xd1326af99088846451f1eb5eab2892ff5c325c962d76d0e1def0866027ab1a82 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs
  
Parcel passed to the courier
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    B6E7B2F57365BEE4708871E59A7636411A82067626273F0612A2F837D882C72F 
Gas Used            138461 
Logs                [
  # ...detailed transaction logs
]
```

#### Cancel delivery
Command for cancelling delivery

_Disclaimer: After cancelling request on your side depending on the status and the deadlines set you may lose funds._

``` bash
figaro-cli courier cancel_delivery \
  --secret 0x6bead0e84230da9ee73ec5b151776a871ef50a3da1660a32c79b4735d6219103 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \

Delivery was cancelled
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    68E4EE490113FB976B57209DC728E205004319371038165A2ED6D9AC7A60F13F 
Gas Used            216527 
Logs                [
  # ...detailed transaction logs
]
```

### Courier

#### Accept Application
To accept request you need to have a target contract address.

``` bash
figaro-cli courier accept_request \
  --secret 0x6bead0e84230da9ee73ec5b151776a871ef50a3da1660a32c79b4735d6219103 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs

Accept delivery request
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    68A6030D684602141506F9AE4E7493C9FE3D0E53EAC547B7F8C122DCEA72F1F7 
Gas Used            142041 
Logs                [
  # ...detailed transaction logs
]
```

#### Make deposit
After acceptance the courier needs to make a deposit to the contract (This is part of the guarantee mechanism).

_Disclaimer: this command automatically requests permission to withdraw funds from your wallet._  

``` bash
figaro-cli courier make_deposit \       
  --secret 0x6bead0e84230da9ee73ec5b151776a871ef50a3da1660a32c79b4735d6219103 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs

Delivery payment
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    FFADDDF4AAEE35DAAB68E4EE49016DBE0E6CD0C8666A887139DCE59662303D31 
Gas Used            299778 
Logs                [
  # ...detailed transaction logs
]
```

#### Confirm delivery
Confirmation of the delivery and request for the payment

``` bash
figaro-cli courier confirm_delivery \
  --secret 0x6bead0e84230da9ee73ec5b151776a871ef50a3da1660a32c79b4735d6219103 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \
  --confirm-private 0x6bead0e84230da9ee73ec5b151776a871ef50a3da1660a32c79b4735d6219103

Delivery confirmed, payout successful
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    6D9AC7A60F13FB976B57209DC728E205004319371038165A2ED63B16F30B818A 
Gas Used            220343 
Logs                [
  # ...detailed transaction logs
]
```

#### Cancel delivery
Command for cancelling a delivery

_Disclaimer: Please note that depending on the status of the delivery you may lose the deposit if the package has already been passed to you, but has not been delivered._

``` bash
figaro-cli courier cancel_delivery \
  --secret 0x6bead0e84230da9ee73ec5b151776a871ef50a3da1660a32c79b4735d6219103 \
  --contract wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs \

Delivery was cancelled
Contract Address    wasm1vhndln95yd7rngslzvf6sax6axcshkxqpmpr886ntelh28p9ghuq0rxlxs 
Transaction Hash    68E4EE490113FB976B57209DC728E205004319371038165A2ED6D9AC7A60F13F 
Gas Used            216527 
Logs                [
  # ...detailed transaction logs
]
```
