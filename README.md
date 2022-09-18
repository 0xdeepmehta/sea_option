A option market is a market holding all the option market state.
A option market account is PDA


- `base_mint` is basically underlying assets like ETH.
- `collateral_mint` is basically the `premium_mint` like USDT.
- `option_type` is whetehere this call is for put or call
- `srtike_price` is the price at which buyer is willing to execrise the contract
- `size` is basically the lot size.
- `period` is basically the expiration date.
- `vault` is the PDA for taking custody over collateral token


# Europen style option market
There are pre-listed options, listed in the market from brokers.
have pre-fillied state like
 - call option: call/put
 - underlying_mint or base_mint
 - collateral_mint
 - strike price
 - size #
 - period
 - strike_price
 - price_to_break-even
 - total_cost

 premium = $8
 available = 18