# JeepPay
Digital jeepney fare payment using Stellar.

## Problem
Commuters struggle with exact fare and delays due to cash handling.

## Solution
QR-based payment using Stellar for instant fare transfers.

## Timeline
Week 1: Contract  
Week 2: Mobile UI  
Week 3: Integration  

## Stellar Features
- USDC transfers
- Soroban contracts

## Vision
Cashless public transport in the Philippines.

## Prerequisites
- Rust
- Soroban CLI

## Build
soroban contract build

## Test
cargo test

## Deploy
soroban contract deploy

## Example
soroban contract invoke --id <id> --fn pay_fare --arg <passenger> --arg <driver> --arg 13

## License
MIT 
https://stellar.expert/explorer/testnet/tx/da08f2b0777c52e0ab6b29c3f91b43ceb2d5b698517521fa6b66daf269150bc1
