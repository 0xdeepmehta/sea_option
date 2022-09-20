# Cash-settled options protocol written in seahorse lang    

## What is options market?
Options market is a derivative market where you trade contracts of a underlying assets. options contract allows ( but not obligation) trading between two parties to buy and sell underlying assets at a fixed price within a specific period of time.

### ELI5
- Alice wants to trade BTC, and the market price of BTC is $25,000.
- He can go in market and looks for Options market that allows him to trade BTC contract.
- Options market gibs him to right (but not obligation) to SHORT/LONG BTC at fixed price and in a specified timeperiod.
- Now he is bullish and wants to long on BTC.
- He buys `BTC_CALL_26000_012023` contract. this contract gives him rights to buy BTC at 26,000 within or at the day of expiry, in this case january 2023.
- He buys 1 contract at the a premium $100. Price of premium genereally depends on market demand for that specific contract.
- There can be two outcomes for this contract within or at the day of expiry.
    - Let's say market price of BTC on that day is $30,000 and he decides to settles the contract. Now he is making lumpsup profit on this trade that is $4,000 ($30,000-$26,000) but `net_profit` is $3,900 ($4000-$100(premium)).
    - or the market price of BTC on that day stays below $26,000. Let say $23,000. So he is smarts and he let the contract expire. The loss is only premium that is $100.


## Explain Contract instruction
- `init_option`
    - this instruction is used to create options market with the followung arguments.
    - `payer: Signer` is basically the initializer of the contract
    - `market: Empty[OptionMarket]` is a PDA which is used to hold market state.
    -`base_mint: TokenMint` is the underlying asset which is being traded
    - `collateral_mint: TokenMint` the asset used to trade the contracts i.e $USDC
    - `option_note_mint: Empty[TokenMint]` this mint represents option's deposit notes
    - `vault: Empty[TokenAccount]` is account with custody over the collateral tokens
    - `strike_price: u64` ia the price at which you want to execrise the underlying assets
    - `expiry_timestamp: i64` is the expiry timestamp of the contract
    - `is_put: bool` is a flag representing if the contract is put or not
    - `lot_size: u64` is the number of contracts you want to trade
    - `premium_per_lot: u64` premium decided by the market per lot

<br>

- `buy_option` intruction allow users to buy options contract. user needs to specify how many lots they wants to buy.

- `settle_expiry` is used to record the market price of the  underlying asset at the time of settlement.

- `redeem` is used by the user to settles the contract. It settles trade in cash, in our case that will be collateral asset ($USDC). The core logic for this ix is 
    - ```python
        assert market.expiry_price <= 0, 'Expiry price not found'
        
        if market.is_put == False:
            if market.expiry_price > market.strike_price:
            profit_amount = (market.expiry_price - market.strike_price) * u64(lot_factor)
            print("profit :: ", profit_amount)
            vault.transfer(
                authority= market,
                to = redeemer_account,
                amount=profit_amount,
                signer=["market", base_mint, collateral_mint]
            )
        else:
            if market.expiry_price < market.strike_price:
            profit_amount = (market.strike_price - market.expiry_price) * u64(lot_factor)
            print("profit :: ", profit_amount)
            vault.transfer(
                authority= market,
                to = redeemer_account,
                amount=profit_amount,
                signer=["market", base_mint, collateral_mint]
            )
        ```
### Prerequisites
1. [Solana](https://docs.solana.com/cli/install-solana-cli-tools)
2. [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#install-rust)
3. [NodeJs](https://nodejs.org/en/)
4. [Seahorse](https://seahorse-lang.org/docs/installation)

### Steps to run
1. Clone this repo.
2. Go into the project's directory
3. execute `seahorse build` command on terminal