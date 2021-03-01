## Project description
Solaris is a Lending/Borrowing protocol that brings Flashloans to Solana blockchain (inspired by Aave and Compound)
The project is a modification of the (solana lending program)[https://github.com/solana-labs/solana-program-library]

## Motivation. Why Flashloans?
Flashloans add a possibility to add many useful DeFi apps on top of it: 
- arbitrage bots(for example between Serum limit orders and AMMs)
- liquidation/liquidation protection bots, 
- margin trading with stop-loss/take profit bots, 
- DeFi portfolio rebalancing and so on.

## How to use it
1. Init and setup lending program, move initial funds to the borrow reserve
2. Use flashloan-program-template to create your program that will will use flashloans. Add your logic(arbitrage, for example) in ExecuteOperation instruction and call MyFlashLoanCall instruction