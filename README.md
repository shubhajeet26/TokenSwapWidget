# 🔄 Token Swap Widget (Soroban Smart Contract)



<img width="1920" height="1080" alt="image" src="https://github.com/user-attachments/assets/8b012888-d25b-49d0-82b0-13db152033f7" />

https://lab.stellar.org/r/testnet/contract/CAI4CEZBMCDIIPADZDAYZ342MYKCJ7MMO26J7VIRLHKCLBQQTCLXRHUK


## 📌 Project Description
The Token Swap Widget is a decentralized smart contract built on the Stellar Soroban platform. It allows users to seamlessly swap one token for another in a simple and efficient way.

This project is designed as a beginner-friendly decentralized finance (DeFi) application and demonstrates how token transfers and swaps can be implemented using Soroban smart contracts.

---

## ⚙️ What it does
- Enables users to swap Token A for Token B
- Uses a simple fixed-rate swapping mechanism (1:1 for demonstration)
- Handles secure token transfers using Soroban's token interface
- Allows liquidity deposits into the contract

---

## ✨ Features
- 🔐 Secure transactions using authentication (`require_auth`)
- 🔁 Token swapping functionality
- 💧 Liquidity deposit support
- ⚡ Built on Stellar Soroban (fast and low-cost execution)
- 🧩 Easy to integrate into a frontend widget (React / Web UI)

---

## 🚀 How it works
1. User calls the `swap` function
2. Contract transfers Token A from user to itself
3. Contract sends Token B back to the user
4. Swap completes instantly

---

## 📦 Tech Stack
- Rust (Soroban SDK)
- Stellar Soroban Smart Contracts
- Token Interface (soroban_sdk::token)

---

## 🔗 Deployed Smart Contract Link
https://stellar.expert/explorer/testnet/contract/CAI4CEZBMCDIIPADZDAYZ342MYKCJ7MMO26J7VIRLHKCLBQQTCLXRHUK

---

## 🛠️ Future Improvements
- Add dynamic pricing (AMM like Uniswap)
- Add liquidity pools
- Add swap fees
- Slippage protection
- Frontend UI integration

---

## 👨‍💻 Author
Shubhajeet Saha
