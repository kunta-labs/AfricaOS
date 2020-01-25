# Africa Operating System (AfricaOS)
A simple, customizable, proposal-based, replicated state machine (RSM), inspired by pBFT (Practical Byzantine Fault Tolerance) written in pure Rust

## Get the updated code, and documentation on XS code
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

# Stress with input/female transaction
To create/submit a new transaction every m minutes
```
make stress
```

**under active development**

## Discord
https://discord.gg/dMvtDeW

## Telegram
http://t.me/africaos
