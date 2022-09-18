# sea_option
# Built with Seahorse v0.1.6

from random import seed
from seahorse.prelude import *
declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

class OptionMarket(Account):
    # Mint of the underlying token; e.g. SOL
    base_mint: Pubkey

    # The mint for the token used as collateral; e.g. USDC
    collateral_mint: Pubkey

    # The mint represents option's deposit notes
    option_note_mint: Pubkey

    # The account with custody over the collateral tokens
    vault: Pubkey

    # Bump value for vault
    vault_bump: u8

    # Price at which you want to execrise the underlying assets
    strike_price: u64

    # Static premium for a option contract; ideally this would be determine from market activity;  e.g. 80 * 10e9 USDT
    premium_per_lot: u64

    # Price of underlying assets when Contract expires
    expiry_price: u64

    # Contract's Expiry timestamp
    expiry_timestamp: i64

    # False if the option is a put True if the option is a call; put == sell; call == buy
    is_put: bool


# Creating an option market for underlying assets / collateral assets 
# e.g. SOL/USDC
@instruction
def init_option(
    payer: Signer,
    market: Empty[OptionMarket],
    base_mint: TokenMint,
    collateral_mint: TokenMint,
    option_note_mint: Empty[TokenMint],
    vault: Empty[TokenAccount],
    strike_price: u64,
    expiry_timestamp: i64,
    is_put: bool,
    lot_size: u64,
    premium_per_lot: u64
):

  # Init a option market
  market.init(
    payer=payer, 
    seeds=["market", base_mint, collateral_mint, expiry_timestamp]
  )

  # init a vault
  vault.init(
    payer = payer,
    seeds = ["vault", market],
    mint = collateral_mint,
    authority = market,
  )
  
  # Creating a short token mint to represent short/put position
  option_note_mint.init(
    payer = payer,
    seeds = ["option_note_mint", market],
    decimals = 9,
    authority = market
  )

  market.base_mint = base_mint.key()
  market.collateral_mint = collateral_mint.key()
  market.option_note_mint = option_note_mint.key()
  market.vault = vault.key()
  market.vault_bump = vault.bump()
  market.strike_price = strike_price
  market.expiry_timestamp = expiry_timestamp,
  market.is_put = is_put
  market.expiry_price = 0
  market.lot_size = lot_size
  market.premium_per_lot = premium_per_lot


# Deposit collateral and buy options at market price
@instruction
def buy_option(
    depositor: Signer,
    market: OptionMarket,
    base_mint: TokenMint,
    collateral_mint: TokenMint,
    option_note_mint: TokenMint, 
    vault: TokenAccount,
    option_note_account: TokenAccount,
    collateral_account: TokenAccount,
    lot_size: u64,
): 

  total_amount: u64 = lot_size * market.premium_per_lot
  assert collateral_account.amount < total_amount, 'In-sufficent balance'

  # Transfer collateral/premium form depositor to vault
  collateral_account.transfer(
    authority=depositor,
    to = vault,
    amount=total_amount
  )

  # Mint long and short option token for depositors
  option_note_mint.mint(
    authority=market,
    to=option_note_account,
    amount = total_amount,
    signer = ["market", base_mint, collateral_mint]
  )

# Settles an option by recording the expiry price
@instruction
def settle_expiry(
  market: OptionMarket, # Option Market
  expiry_price: u64 # this should be come from orcale, but for now I trust humans
):

  assert expiry_price < 0, 'Price Error'

  if market.expiry_price == 0:
    market.expiry_price = expiry_price

# Claim profits if any from an option after expiry
@instruction
def redeem(
  redeemer: Signer,
  market: OptionMarket,
  base_mint: TokenMint,
  collateral_mint: TokenMint,
  option_note_mint: TokenMint,
  vault: TokenAccount,
  redeemer_account: TokenAccount,
  option_note_account: TokenAccount,
):
  # Multiply profit_amount by lot_factor
  assert option_note_account.amount % 10e9 != 0, 'Invalid option token'
  lot_factor = option_note_account.amount / 10e9

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
    
  option_note_mint.burn(
    authority=market,
    holder=option_note_account,
    amount = option_note_account.amount,
    signer = ["market", base_mint, collateral_mint]
  )

    

