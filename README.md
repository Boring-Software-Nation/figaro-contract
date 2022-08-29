## Disclaimer
> 💀 This is a **Work in Progress**.  
> Current status: Common PoC data storage and methods available. Partially tested.   
> **Use at your own risk**.

<h1 align="center">
  🔑 ✨ Figaro 📦 👛
</h1>

<p align="center">
This is a  <a href="https://github.com/CosmWasm/cosmwasm">CosmWasm!</a> smartcontract implementing a p2p delivery logic. <br>
With this contract you can create a delivery request worth an amount of tokens based on cw20 for Cosmos blockchain users.
</p>

## Design and features
* The contract works on the principle of mutual deposit.
* Verification is based on a signature with a secp256k1 secret key, which a sender gives to the recipient who receives the package.
* None of the parties can withdraw the deposit before the contract terms execution, or an expiration of obligations.

## How to
### Install Prerequisites
Please follow installation instructions provided [here](https://docs.cosmwasm.com/docs/1.0/getting-started/installation). Also we have a simple helper script to configure `wasmd` [here](https://github.com/bsn-si/figaro-cli/blob/main/common/setup.sh).

### Clone this repo
```
git clone https://github.com/bsn-si/figaro-contract
```

### Build Contract + metadata
```
cd figaro-contract/
RUSTFLAGS='-C link-arg=-s' cargo wasm
```

### Example usage
Please use our [CLI](https://github.com/bsn-si/figaro-cli) to interact with the contract, or use original `wasmd query` command.  

## Related repos
- [Management tools with CLI](https://github.com/bsn-si/figaro-cli)

## License
[Apache License 2.0](https://github.com/bsn-si/figaro-contract/blob/main/LICENSE) © Bela Supernova ([bsn.si](https://bsn.si))
