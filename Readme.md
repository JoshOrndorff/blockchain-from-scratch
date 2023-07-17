# Blockchain From Scratch

Learn the fundamentals of blockchain by building it from scratch. In Rust.

Solutions are available on the `solutions` branch.

## Table of Contents

Some sections are less important than others and may be skipped if you are in a hurry. Less important sections are marked with a `*`.

### Chapter 1: State Machines

We formalize the notion of a state machine and implement several examples. We do not yet discuss the blockchain data structure or consensus. Examples range from simple toys for learning purposes, to realistic multi-user state machines common in real-world blockchain systems.

- Part 1 - Switch-based state machines - Two dead simple state machines to learn the basics.
- Part 2 - Laundry Machine - A toy state machine modeling the lifecycle of clean and dirty laundry.
- Part 3\* - Automated Teller Machine - A semi-realistic, but significantly simplified state machine modelling a common ATM.
- Part 4\* - Accounted Currency - A realistic state machine used as the foundation for many cryptocurrencies such as Ethereum and Polkadot.
- Part 5 - Digital Cash - A realistic state machine used as the foundation for many cryptocurrencies such as Monero, Dogecoin, and Litecoin.

### Chapter 2: Blockchain

We introduce the blockchain data structure and scaffold it from a simple hash-linked list to a proper blockchain with the Body distinct from the Header, and a consensus digest included. This is the most important chapter of the book.

- Part 1 - Header Chain - A minimal hash-linked list with no real state or execution logic.
- Part 2 - Extrinsics and State - We extend our chain to track state and introduce a simple notion of extrinsics.
- Part 3 - Consensus - We introduce a basic notion of consensus using proof of work as our first example.
- Part 4 - Batched Extrinsics - We separate the block body out of our header, and show that there are multiple extrinsics in a single block
- Part 5 - Fork Choice - We introduce the notion of a fork choice rule and the idea that consumers of the blockchain data structure must decide which of multiple chains is real _for them_.
- Part 6 - Rich State - We show that in real-world blockchains the state is not stored directly in the blocks and must be tracked separately. We also introduce the concept of genesis state.

### Chapter 3: Consensus

We formalize the notion of consensus see how our previous look at Proof of Work fits into this framework and explore several other consensus schemes including Proof of Authority, a brief look at Proof of Stake, and some higher-order consensus concepts.

- Part 1 - Proof of Work - We re-implement Proof of Work in our new consensus framework
- Part 2\* - Dictator - A toy identity-based consensus system where a single authority, the dictator, says what blocks are valid
- Part 3 - Proof of Authority - We implement several identity-based consensus systems, some of them realistic, others just toys. We briefly discuss Proof of Stake
- Part 4\* - Even Only - We explore the notion of "arbitrary" consensus rules more formally.
- Part 5\* - Interleave - This section is still under development. - We will explore how to interleave different consensus rules on a block-by-block basis.
- Part 6 - Forking - We explore how to coordinate consensus handoffs so that consensus rules can change as the result of a fork part way through a blockchain's history.

### Chapter 4: Blockchain Framework and Client

This chapter is still under development. We begin by extending our blockchain data structure from chapter 2 to be fully generic over both the state machine (using the framework from Chapter 1) and the consensus engine (using the framework from chapter 3). We then continue on to develop a proper blockchain client which is able to import and export blocks, create blocks, manage a transaction pool, and decide on which fork is best. We may even introduce a notion of finality eventually.

## License

Licensed under the terms of the [GPL-3](https://www.gnu.org/licenses/gpl-3.0.en.html) or later.
