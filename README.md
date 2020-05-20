# Africa Operating System (AfricaOS)
A simple, customizable, proposal-based, replicated state machine (RSM), inspired by pBFT (Practical Byzantine Fault Tolerance) written in pure Rust

| Status Type | Status |
| --- | --- |
| `Master Build` | [![Build Status](https://travis-ci.org/kunta-labs/AfricaOS.svg?branch=master)](https://travis-ci.org/kunta-labs/AfricaOS) |
| `Development Build` | [![Build Status](https://travis-ci.org/kunta-labs/AfricaOS.svg?branch=development)](https://travis-ci.org/kunta-labs/AfricaOS) |
| `Issues` | [![Issues](https://img.shields.io/github/issues/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/issues) |
| `Last Commit` | [![Last commit](https://img.shields.io/github/last-commit/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/commits/master) |
| `Docker Stars` | [![Docker Stars](https://img.shields.io/docker/stars/kuntalabs/africaos.svg)](https://hub.docker.com/r/kuntalabs/africaos) |
| `Docker Pulls` | [![Docker Pulls](https://img.shields.io/docker/pulls/kuntalabs/africaos.svg)](https://hub.docker.com/r/kuntalabs/africaos) |
| `Docker Automated` | [![Docker Automated](https://img.shields.io/docker/cloud/automated/kuntalabs/africaos.svg)](https://hub.docker.com/r/kuntalabs/africaos) |
| `Docker Build` | [![Docker Build](https://img.shields.io/docker/cloud/build/kuntalabs/africaos.svg)](https://hub.docker.com/r/kuntalabs/africaos) |
| `License` | [![License](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/kunta-labs/AfricaOS/blob/master/LICENSE) |
| `Releases` | [![Releases](https://img.shields.io/github/downloads/kunta-labs/AfricaOS/total.svg)](https://github.com/kunta-labs/AfricaOS/releases) |
| `Latest Release` | [![Latest release](https://img.shields.io/github/v/release/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/releases) |
| `Top Language` | [![Top language](https://img.shields.io/github/languages/top/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS) |
| `Code Size` | [![Code size in bytes](https://img.shields.io/github/languages/code-size/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS) |
| `Chat` | ![Discord](https://img.shields.io/discord/430502296699404308) |

## White paper
Read here: https://arxiv.org/abs/2003.10486

## Get the updated code, and documentation on XS code
All code updates, and documentation are pushed to our sponsorship repository, and eventually pushed into this free repository. The sponsorship repository comes with extra features, detailed/updated documentation, and priority bug fixes. Subscribe to the sponsorship repository on XS Code:
https://cp.xscode.com/Kunta-Labs/AfricaOS

# To start a single node
Edit the Makefile in /core/ to input the IP addresses, and ports for your other nodes. This version comes with a 3-node setup, for Alice, Bob, and Cici (the default is 127.0.0.1:x, to run all 3 nodes on a single workstation.
```
make alice # from inside ./core/
```

To reset Alice
```
make clean_alice # from inside ./core/
```

# To start 3-node network
To create Bob, copy the folder, and create another folder one level up from the projects root, and run the same command for bob
```
make bob # from inside ./core/
```

To reset bob,
```
make clean_bob # from inside ./core/
```

Do the same for Cici
```
make cici # from inside ./core/
```

Now they should be pointed to each other from a peer perspective

# Begin proposal generation
This will submit a first proposal to alice, which will initiate proposal generation amongst each node (alice, bob, and cici)
```
make p_alice # from inside ./core/
```

# Stress with output/female transaction
To create/submit a new transaction every m seconds
```
make stress # from inside ./core/
```

This will create one output tx
```
make stress_output # from inside ./core/
```

This will create an input tx for that output
```
tx_hash=< OUTPUT-TX-HASH > make stress_input
```

Viewing state from your container
```
docker cp 8dcbe580eb6f:storage/states.db ./states.db ; cat ./states.db ; rm ./states.db
```

# Docker
To build the core docker container, run (from inside ./)
```
make dbm # stands for "docker build main"
```

this will build the container from source

### Pulling AOS core container
to pull a minimal docker image of AfricaOS, run
```
docker pull kuntalabs/africaos:latest
```

### Running 3-node network
to run the 3 containers, and set up the 3-node network, run (from inside ./core/)
```
make rac #  or make dbm, stands for "run all containers"
```

# Transactions
Combined txs will have the following default values (this is to be customized for your use case):
```
partner_sender <partner_tx_hash> <sig> <pk> <pkhash> <amount>
```

## Output
```
<pkhash> <amount>
```

Senders submit the hash of the receivers public key to the blockchain, and the amount to send to the receiver

## Input
```
partner_sender <partner_tx_hash> <sig> <pk>
```
Receivers submit the sender of the amount, the hash of the output transaction, their signature of an arbitrary string (default "TEST"), and finally the receiver's public key

# Customization
We expose common blockchain customization points
- Block Validation
- Proposal Validation
- Proposal Creator Election
- Transaction Output Logic
- Transaction Input Logic

## To Contribute
TODO:
- Submit Issue using template

**under active development**

## Discord
https://discord.gg/dMvtDeW

## Telegram
http://t.me/africaos
