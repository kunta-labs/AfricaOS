# Africa Operating System (AfricaOS)
[![Build Status](https://travis-ci.org/kunta-labs/AfricaOS.svg?branch=master)](https://travis-ci.org/kunta-labs/AfricaOS)
[![Issues](https://img.shields.io/github/issues/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/issues)
[![Last commit](https://img.shields.io/github/last-commit/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/commits/master)
![Docker Stars](https://img.shields.io/docker/stars/kuntalabs/africaos.svg)
![Docker Pulls](https://img.shields.io/docker/pulls/kuntalabs/africaos.svg)
![Docker Automated](https://img.shields.io/docker/automated/kuntalabs/africaos.svg)
![Docker Build](https://img.shields.io/docker/build/kuntalabs/africaos.svg)
[![License](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/kunta-labs/AfricaOS/blob/master/LICENSE)
[![Downloads](https://img.shields.io/github/downloads/kunta-labs/AfricaOS/total.svg)](https://github.com/kunta-labs/AfricaOS/releases)
[![Latest release](https://img.shields.io/github/v/release/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/releases)
[![Top language](https://img.shields.io/github/languages/top/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS)
[![Code size in bytes](https://img.shields.io/github/languages/code-size/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS)
![Discord](https://img.shields.io/discord/430502296699404308)

| Command | Description |
| --- | --- |
| `git diff` | Show file differences that **haven't been** staged |
| `Build Status` | [![Build Status](https://travis-ci.org/kunta-labs/AfricaOS.svg?branch=master)](https://travis-ci.org/kunta-labs/AfricaOS) |
| `Issues` | [![Issues](https://img.shields.io/github/issues/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/issues) |
| `Last Commit` | [![Last commit](https://img.shields.io/github/last-commit/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/commits/master) |
| `Docker Stars` | ![Docker Stars](https://img.shields.io/docker/stars/kuntalabs/africaos.svg) |
| `Docker Pulls` | ![Docker Pulls](https://img.shields.io/docker/pulls/kuntalabs/africaos.svg) |
| `Docker Automated` | ![Docker Automated](https://img.shields.io/docker/automated/kuntalabs/africaos.svg) |
| `Docker Build` | ![Docker Build](https://img.shields.io/docker/build/kuntalabs/africaos.svg) |
| `License` | [![License](https://img.shields.io/badge/license-GPL-blue.svg)](https://github.com/kunta-labs/AfricaOS/blob/master/LICENSE) |
| `Downloads` | [![Downloads](https://img.shields.io/github/downloads/kunta-labs/AfricaOS/total.svg)](https://github.com/kunta-labs/AfricaOS/releases) |
| `Lastest Release` | [![Latest release](https://img.shields.io/github/v/release/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS/releases) |
| `Top Language` | [![Top language](https://img.shields.io/github/languages/top/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS) |
| `Code Size` | [![Code size in bytes](https://img.shields.io/github/languages/code-size/kunta-labs/AfricaOS.svg)](https://github.com/kunta-labs/AfricaOS) |
| `Discord` | ![Discord](https://img.shields.io/discord/430502296699404308) |

A simple, customizable, proposal-based, replicated state machine (RSM), inspired by pBFT (Practical Byzantine Fault Tolerance) written in pure Rust

## Get the updated code, and documentation on XS code
All code updates, and documentation are pushed to our sponsorship repository, and eventually pushed into this free repository. The sponsorship repository comes with extra features, detailed/updated documentation, and priority bug fixes. Subscribe to the sponsorship repository on XS Code:
https://cp.xscode.com/Kunta-Labs/AfricaOS

# To start a single node
Edit the Makefile in /core/ to input the IP addresses, and ports for your other nodes. This version comes with a 3-node setup, for Alice, Bob, and Cici (the default is 127.0.0.1:x, to run all 3 nodes on a single workstation.
```
make alice
```

To reset Alice
```
make clean_alice
```

# To start 3-node network
To create Bob, copy the folder, and create another folder one level up from the projects root, and run the same command for bob
```
make bob
```

To reset bob,
```
make clean_bob
```

Do the same for Cici
```
make cici
```

Now they should be pointed to each other from a peer perspective

# Begin proposal generation
This will submit a first proposal to alice, which will initiate proposal generation amongst each node (alice, bob, and cici)
```
make p_alice
```

# Stress with output/female transaction
To create/submit a new transaction every m seconds
```
make stress
```

# Docker
To build the core docker container, run
```
make dbm # stands for docker build main
```

to pull a minimal docker image of AfricaOS, run
```
docker pull kuntalabs/africaos:latest
```

# To Contribute

**under active development**

## Discord
https://discord.gg/dMvtDeW

## Telegram
http://t.me/africaos
