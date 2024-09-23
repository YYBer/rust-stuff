his project is a part of teamwork: Lottory.
Build a Simple Oracle:

Objective: The oracle's role is to provide random numbers for determining lottery winners. Since Solana programs are deterministic, I'll need off-chain randomness via an oracle.

Approach to Building the Oracle:

Oracle as a Program (Smart Contract) on Solana:
The oracle program fetches randomness from an off-chain source (e.g., using Chainlink VRF).
Design the oracle so that it takes requests from I lottery contract and returns the random numbers securely.
Steps for the Oracle Implementation:
Create the Oracle Program:
In Rust, create a new Solana program (a separate one from the lottery contract). This program will serve as the oracle.
Use the Solana Program Library (SPL) framework to build the oracle logic.
Request Random Numbers: Implement the lottery contract to send requests to the oracle contract.
Integrate Off-Chain Data: Use Chainlink VRF (Verifiable Random Function) oracles to fetch true randomness.
Secure Randomness: When the lottery ends, the oracle provides the random numbers for determining the winning tickets. Ensure the randomness is cryptographically secure.

Set Up the Solana Environment:

Install the Solana CLI and Rust toolchain. Set up your environment to work with Solana.
```
solana config set --url https://api.devnet.solana.com
rustup component add rustfmt
```
Install anchor
```
cargo install --git https://github.com/coral-xyz/anchor --tag v0.30.1 anchor-cli
avm --version
anchor --version
```

###TODO:
Write the Oracle Logic:

Define the Solana program entry points (initialize, request_random_numbers, fulfill_random_numbers).
The oracle receives a request for a random number and sends the result back to the lottery contract.
Integrate Chainlink VRF:

Implement interaction with an off-chain oracle (e.g., Chainlink VRF) to get random numbers.
In your smart contract, provide a way to securely store and retrieve these numbers.
Communication Between Contracts:

Ensure your lottery contract can communicate with the oracle contract via cross-program invocations (CPI).
When the lottery is ready for drawing, it requests random numbers from the oracle contract.
The oracle contract sends back random numbers, which are used to determine the winner.
Testing the Oracle:

Write unit and integration tests for the oracle to ensure it handles random number generation and can respond to requests from the lottery contract.
Test on Solana Devnet before moving to Mainnet.
