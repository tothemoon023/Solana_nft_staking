# Solana NFT Staking

Welcome to the Solana NFT Staking repository! This project is designed to enable the staking of NFTs on the Solana blockchain, rewarding holders with tokens or other benefits for locking their NFTs.

## NFT Staking Concept
NFT staking on Solana lets users lock up their NFTs in a smart contract to earn rewards-such as tokens or special privileges-without selling or transferring ownership. When an NFT is staked, it’s held securely by the contract, and the owner receives rewards over time, which encourages people to keep their NFTs longer and stay active in the project’s community. When the owner wants their NFT back, they can unstake it, and the smart contract returns it to their wallet. This process is managed by smart contracts that track staking duration and ensure only the rightful owner can retrieve the NFT, making it a popular way for projects to reward holders and for users to earn passive income while keeping their NFTs

## Flow Chart For better Understanding

<img width="449" alt="nft_staking" src="https://github.com/user-attachments/assets/1c3455a7-6410-4bbd-97c5-235f628742d6" />

Step-by-Step Breakdown

	1.	User Connects Wallet
	•	The user connects their Solana wallet (e.g., Phantom) to the staking platform.
 
	2.	Initialize Program Accounts
	•	The staking program initializes on-chain accounts to track the user’s staking activity.
 
	3.	Check NFT Collection Eligibility
	•	The program verifies if the user’s NFT belongs to an allowed collection (e.g., via metadata checks).
 
	4.	Valid Collection?
	•	Yes: Proceed to stake the NFT.
	•	No: Display an error (“Invalid NFT”) and halt the process.
 
	5.	Stake NFT
	•	The user locks their NFT into the staking contract.
 
	6.	Delegate NFT Authority
	•	The NFT’s authority is delegated to the staking program (enables the program to manage the NFT during staking).
 
	7.	Start Reward Accumulation
	•	Rewards (e.g., tokens) begin accumulating based on the staking duration.
 
	8.	Calculate Time-Staked
	•	The program tracks how long the NFT has been staked to determine rewards.
 
	9.	Unstake Request?
	•	Yes: Check if the required staking period (freeze period) has passed.
	•	No: Continue earning rewards indefinitely.
 
	10.	Verify Freeze Period
	•	Ensures the user has met the minimum staking duration (prevents early withdrawals).
 
	11.	Period Elapsed?
	•	Yes: Allow the user to claim rewards and unstake.
	•	No: Show an error (“Too Early”) and block unstaking.
 
	12.	Claim Rewards
	•	The user claims their accumulated rewards (e.g., SPL tokens).
 
	13.	Unfreeze NFT
	•	The NFT is released from the staking contract’s control.
 
	14.	Revoke Delegation
	•	The NFT’s authority is returned to the user’s wallet.
 
	15.	Continue Earning
	•	If no unstake request is made, rewards keep accumulating.
 
Key Solana-Specific Concepts

	•	Delegate Authority: Solana NFTs use delegated authority to let programs manage assets without transferring ownership.
 
	•	Freeze Period: A cooldown period enforced by the staking contract to prevent abuse.
 
	•	Revoke Delegation: Critical for returning full control of the NFT to the user post-unstaking.
 
This flowchart represents a typical Solana NFT staking workflow

## Features

NFT Staking: Stake your NFTs securely on the Solana blockchain.

Rust-Powered Backend: High-performance smart contracts written in Rust.

Frontend with TypeScript: User-friendly and interactive interface to manage NFT staking.

Rewards System: Automatically distribute rewards to stakers based on staking duration.

## Prerequisites

To use or contribute to this repository, ensure you have the following installed:

* Rust programming language
* Node.js and npm
* Solana CLI tools
* A Solana Wallet (e.g., Phantom)

## Installation
To get started with this project, follow the steps below:

1.Clone the repository:

```
bash
 
git clone https://github.com/tothemoon023/Solana_nft_staking.git 
cd Solana_nft_staking
````


2.Install dependencies:

  + For Rust:
```
bash

rustup update
```
 + For TypeScript frontend (if applicable):
```
bash

cd frontend
npm install

```

3.Build the project:

  * Build Rust smart contracts:
```
bash

cargo build --release
```
  * Build the frontend:
```
bash

cd frontend
npm run build
```

## Usage
1.Deploy the smart contracts: Use the Solana CLI tools to deploy the compiled smart contracts to your Solana cluster (Devnet, Testnet, or Mainnet).

2.Run the frontend:

```
bash

cd frontend
npm start

```
3. Connect your Solana wallet, and start staking your NFTs!

## Directory Structure

/contracts: Contains Rust smart contracts for NFT staking.

/frontend: Contains the TypeScript-based user interface.

/scripts: Deployment or utility scripts for managing the project.

## Contributing

We welcome contributions! To contribute:

1.Fork the repository.

2.Create a new branch:
```
bash
git checkout -b feature-name
```
3.Commit your changes:
```
bash
git commit -m "Description of changes"
```
4.Push to your fork:
```
bash
git push origin feature-name
```
5.Submit a pull request.

## License
This project is licensed under the MIT License.

## Contact
For any questions or support, feel free to open an issue or reach out at [malikathaiyab023@mail.com].
