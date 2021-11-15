vote-coffe-near
=================================

## Description

This contract implements simple vote for the best coffe in indonesia using near protocol.
Contract in `contract/src/lib.rs` provides methods to add_coffe, vote, get_coffes and get it's current value.


## Building this contract
Run the following, and we'll build our rust project up via cargo. This will generate our WASM binaries into our res/ directory. This is the smart contract we'll be deploying onto the NEAR blockchain later.
```
./build.sh
```

## Quickest deploy
You can build and deploy this smart contract to a development account. Dev Accounts are auto-generated accounts to assist in developing and testing smart contracts. Please see the Standard deploy section for creating a more personalized account to deploy to.
```
near dev-deploy 
```

## To Test

```
cd contract
cargo test -- --nocapture
```

# Donate
If this repository has been useful to you, please consider saying "thanks" by donating money, even if it's just one cents. This shows your appreciation and keeps me motivated to work on further software improvements.

## Bitcoin
Please send [bitcoin](https://bitcoin.org) to the following address:

[1BBqhnWbr9TBE1d1QXWpD23BiGKQXRQMPb](https://blockstream.info/address/1BBqhnWbr9TBE1d1QXWpD23BiGKQXRQMPb)

![bitcoin QR code](https://raw.githubusercontent.com/giowck/symphytum/master/stuff/donation-resources/bitcoin_qr.png)

## Ethereum
Please send ETH to the following address:

[0x9Be4c638CDC4b7d89b8Ea3720cd1f39E32276E53](https://etherscan.io/address/0x9Be4c638CDC4b7d89b8Ea3720cd1f39E32276E53)

![ETH QR code](https://raw.githubusercontent.com/giowck/symphytum/master/stuff/donation-resources/ethereum_qr.png)