<a href="https://github.com/w3f/Open-Grants-Program/pull/268"><img src="https://user-images.githubusercontent.com/2616844/113636716-e3857f80-9627-11eb-842a-dcb1e1a96689.png" alt="oak-web3-open-grant" /></a>
OAK(Onchain Autonomous Framework) is a unique blockchain built on Substrate framework with event-driven smart contract VM, autonomous transactions, and on-chain scheduler.

Documentation
----------

* [Website](https://oak.tech/)
* [Documentation](https://docs.oak.tech/)

Community
--------- 

* General discussion: [Telegram (Coming Soon)]()
* Technical discussion: [Discord](https://discord.gg/7W9UDvsbwh)
* Subscribe on [OAK Twitter](https://twitter.com/oak_network)
* Subscribe on [Founder's Twitter](https://twitter.com/chrisli2046)

Table of Contents
-----------------

* [Introduction](https://github.com/OAK-Foundation/OAK-blockchain#introduction)
* [Install OAK](https://github.com/OAK-Foundation/OAK-blockchain#install-oak-blockchain)
* [OAK Validator Program](https://github.com/OAK-Foundation/OAK-blockchain/blob/master/docs/validator-setup.md)

Introduction
============

**OAK, or Onchain Automation Framework, is equipped with a novel smart contract virtual machine that supports an event-driven execution model, enabling developers to build fully autonomous decentralized application.** By extending the current set of atomic operations, namely, opcodes of EVM, OAK introduces an innovative way for contracts to interact with each other. Contracts can emit signal events, on which other contracts can listen. Once an event is triggered, corresponding handler functions are automatically executed as a new type of transaction, signal transaction. Applications implemented with the new approach will eliminate the dependency of unreliable mechanisms like off-chain relay servers, and in return, to significantly simplify the execution flow of the application and can avoid security risks such as relay manipulation attacks.

Based on the above, OAK has some features.
- **OAK Virtual Machine**
- **Autonomous Transactions**
- **On-chain Relayer**
- **Validator Staking**

Live Networks
============

- `oak-testnet`: standalone testnet (built off of substrate)
- `neumann`: testnet parachain (coming soon)
- `turing`: kusama parachain (coming soon)
- `oak`: polkadot parachain (coming soon)

Install OAK Blockchain 
=============

* OAK releases [releases](https://github.com/OAK-Foundation/OAK-blockchain/releases).
* Node [custom types](). 

> Latest version you can try to build from source.

Building from source
--------------------

Ensure you have Rust and the support software (see shell.nix for the latest functional toolchain):

    curl https://sh.rustup.rs -sSf | sh
    # on Windows download and run rustup-init.exe
    # from https://rustup.rs instead

    rustup update nightly
    rustup target add wasm32-unknown-unknown --toolchain nightly

You will also need to install the following dependencies:

* Linux: `sudo apt install cmake git clang libclang-dev build-essential`
* Mac: `brew install cmake git llvm`
* Windows: Download and install the Pre Build Windows binaries of LLVM from http://releases.llvm.org/download.html

Install additional build tools:

    cargo +nightly install --git https://github.com/alexcrichton/wasm-gc

Install the OAK node from git source:

    git clone git@github.com:OAK-Foundation/OAK-blockchain.git    

Build your executable:

    cargo build --release

Run your Local Network
-----------

Launch a local setup including a Relay Chain and a Parachain.
Note: local PARA_ID is defaulted to 2000

### Launch the Relay Chain

```bash
# Compile Polkadot with the real overseer feature
git clone https://github.com/paritytech/polkadot
cargo build --release

# Alice
./target/release/polkadot --chain ../OAK-blockchain/resources/rococo-local.json --alice --tmp

# Bob (In a separate terminal)
./target/release/polkadot --chain ../OAK-blockchain/resources/rococo-local.json --bob --tmp --port 30334
```

### Launch the Parachain

```bash
# Compile
git clone https://github.com/OAK-Foundation/OAK-blockchain
cargo +nightly build --release

# Export genesis state
./target/release/neumann-collator export-genesis-state > genesis-state

# Export genesis wasm
./target/release/neumann-collator export-genesis-wasm > genesis-wasm

# Collator1
./target/release/neumann-collator --collator --alice --force-authoring --tmp --port 40335 --ws-port 9946 -- --execution wasm --chain resources/rococo-local.json --port 30335
```

### Register the parachain

![image](https://user-images.githubusercontent.com/2915325/99548884-1be13580-2987-11eb-9a8b-20be658d34f9.png)



Run the neumann network
-----------

More info to come...
Detailed OnFinality docs [here](https://support.onfinality.io/hc/en-us/articles/4407873248025-Create-a-custom-relay-chain-and-parachain-in-OnFinality)

### onf

```bash
# Get the onf command
sudo curl -s https://raw.githubusercontent.com/OnFinality-io/onf-cli/master/scripts/install/install.sh | sudo bash

# Setup with API keys (under Account tab)
onf setup

# Setup relay chain
cd resources
onf network bootstrap -f bootstrap-relaychain-config.yaml

# Setup parachain
onf network bootstrap -f bootstrap-parachain-config.yaml

```

Future Work
------------
Here are the key milestones.

1. Start the crowdloan with Kusama network
2. Become a Kusama Parachain (TBA)
3. Become a Polkadot Parachain. (TBA)

If you have any questions, please ask us on [Discord](https://discord.gg/7W9UDvsbwh)

Contacts
--------

**Maintainers**

* [Charles Chen](https://github.com/imstar15)
* [Ryan Huttman](https://github.com/rhuttman)
* [Chris Li](https://github.com/chrisli30)
* [Irsal McGinnis](https://github.com/irsal)
* [Laura Reesby](https://github.com/lreesby)

* * *

OAK blockchain is licensed under the GPLv3.0 by OAK Foundation.
