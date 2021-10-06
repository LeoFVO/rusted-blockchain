# rusted-blockchain

Wanted a better understanding of blockchain, and practice more rust. So I'll connect both.

# Architecture

https://cryptoast.fr/bloc-blockchain-crypto-explication/

## Core concepts

### Blocks

Entity containing data, need to be validated and unfalcifiable.
Need at least this fields:

- timestamp of the creation
- previous block hash
- datas
- versionning number
- racine de merkle

### Block chain

A chained list of blocks, needing to be updated by a consensus, can't be modified except for blocks appending

## Tools

### Hash functions

**sha-256**
