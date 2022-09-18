use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::associated_token;
use anchor_spl::token;
use std::convert::TryFrom;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[derive(Debug)]
#[account]
pub struct OptionMarket {
    base_mint: Pubkey,
    collateral_mint: Pubkey,
    option_note_mint: Pubkey,
    vault: Pubkey,
    vault_bump: u8,
    strike_price: u64,
    premium_per_lot: u64,
    expiry_price: u64,
    expiry_timestamp: i64,
    is_put: bool,
}

pub fn init_option_handler(
    mut ctx: Context<InitOption>,
    mut strike_price: u64,
    mut expiry_timestamp: i64,
    mut is_put: bool,
    mut lot_size: u64,
    mut premium_per_lot: u64,
) -> Result<()> {
    let mut payer = &mut ctx.accounts.payer;
    let mut market = &mut ctx.accounts.market;
    let mut base_mint = &mut ctx.accounts.base_mint;
    let mut collateral_mint = &mut ctx.accounts.collateral_mint;
    let mut option_note_mint = &mut ctx.accounts.option_note_mint;
    let mut vault = &mut ctx.accounts.vault;

    market.base_mint = base_mint.key();

    market.collateral_mint = collateral_mint.key();

    market.option_note_mint = option_note_mint.key();

    market.vault = vault.key();

    market.vault_bump = *ctx.bumps.get("vault").unwrap();

    market.strike_price = strike_price;

    market.expiry_timestamp = (expiry_timestamp);

    market.is_put = is_put;

    market.expiry_price = 0;

    market.lot_size = lot_size;

    market.premium_per_lot = premium_per_lot;

    Ok(())
}

pub fn buy_option_handler(mut ctx: Context<BuyOption>, mut lot_size: u64) -> Result<()> {
    let mut depositor = &mut ctx.accounts.depositor;
    let mut market = &mut ctx.accounts.market;
    let mut base_mint = &mut ctx.accounts.base_mint;
    let mut collateral_mint = &mut ctx.accounts.collateral_mint;
    let mut option_note_mint = &mut ctx.accounts.option_note_mint;
    let mut vault = &mut ctx.accounts.vault;
    let mut option_note_account = &mut ctx.accounts.option_note_account;
    let mut collateral_account = &mut ctx.accounts.collateral_account;
    let mut total_amount: u64 =
        <u64 as TryFrom<_>>::try_from(lot_size * market.premium_per_lot).unwrap();

    require!(collateral_account.amount < total_amount, ProgramError::E000);

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: collateral_account.to_account_info(),
                authority: depositor.to_account_info(),
                to: vault.to_account_info(),
            },
        ),
        total_amount,
    )?;

    token::mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::MintTo {
                mint: option_note_mint.to_account_info(),
                authority: market.to_account_info(),
                to: option_note_account.to_account_info(),
            },
            &[&[
                "market".as_bytes().as_ref(),
                base_mint.key().as_ref(),
                collateral_mint.key().as_ref(),
            ]],
        ),
        total_amount,
    )?;

    Ok(())
}

pub fn settle_expiry_handler(mut ctx: Context<SettleExpiry>, mut expiry_price: u64) -> Result<()> {
    let mut market = &mut ctx.accounts.market;

    require!(expiry_price < (0 as u64), ProgramError::E001);

    if market.expiry_price == (0 as u64) {
        market.expiry_price = expiry_price;
    }

    Ok(())
}

pub fn redeem_handler(mut ctx: Context<Redeem>) -> Result<()> {
    let mut redeemer = &mut ctx.accounts.redeemer;
    let mut market = &mut ctx.accounts.market;
    let mut base_mint = &mut ctx.accounts.base_mint;
    let mut collateral_mint = &mut ctx.accounts.collateral_mint;
    let mut option_note_mint = &mut ctx.accounts.option_note_mint;
    let mut vault = &mut ctx.accounts.vault;
    let mut redeemer_account = &mut ctx.accounts.redeemer_account;
    let mut option_note_account = &mut ctx.accounts.option_note_account;

    require!(
        ((option_note_account.amount as f64) % 10000000000f64) != (0 as f64),
        ProgramError::E002
    );

    let mut lot_factor = (option_note_account.amount as f64) / 10000000000f64;

    require!(market.expiry_price <= (0 as u64), ProgramError::E003);

    if market.is_put == true {
        if market.expiry_price > market.strike_price {
            let mut profit_amount =
                (market.expiry_price - market.strike_price) * (lot_factor as u64);

            msg!("{} {}", "profit :: ", profit_amount);

            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    token::Transfer {
                        from: vault.to_account_info(),
                        authority: market.to_account_info(),
                        to: redeemer_account.to_account_info(),
                    },
                    &[&[
                        "market".as_bytes().as_ref(),
                        base_mint.key().as_ref(),
                        collateral_mint.key().as_ref(),
                    ]],
                ),
                profit_amount,
            )?;
        }
    } else {
        if market.expiry_price < market.strike_price {
            let mut profit_amount =
                (market.strike_price - market.expiry_price) * (lot_factor as u64);

            msg!("{} {}", "profit :: ", profit_amount);

            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    token::Transfer {
                        from: vault.to_account_info(),
                        authority: market.to_account_info(),
                        to: redeemer_account.to_account_info(),
                    },
                    &[&[
                        "market".as_bytes().as_ref(),
                        base_mint.key().as_ref(),
                        collateral_mint.key().as_ref(),
                    ]],
                ),
                profit_amount,
            )?;
        }
    }

    token::burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Burn {
                mint: option_note_mint.to_account_info(),
                authority: market.to_account_info(),
                from: option_note_account.to_account_info(),
            },
            &[&[
                "market".as_bytes().as_ref(),
                base_mint.key().as_ref(),
                collateral_mint.key().as_ref(),
            ]],
        ),
        option_note_account.amount,
    )?;

    Ok(())
}

#[derive(Accounts)]
# [instruction (lot_size : u64 , expiry_timestamp : i64 , strike_price : u64 , premium_per_lot : u64 , is_put : bool)]
pub struct InitOption<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        seeds = [
            "market".as_bytes().as_ref(),
            base_mint.key().as_ref(),
            collateral_mint.key().as_ref(),
            expiry_timestamp.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + std::mem::size_of::<OptionMarket>()
    )]
    pub market: Box<Account<'info, OptionMarket>>,
    #[account(mut)]
    pub base_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub collateral_mint: Box<Account<'info, token::Mint>>,
    #[account(
        init,
        payer = payer,
        seeds = [
            "option_note_mint".as_bytes().as_ref(),
            market.key().as_ref()
        ],
        bump,
        mint::decimals = 9,
        mint::authority = market
    )]
    pub option_note_mint: Box<Account<'info, token::Mint>>,
    #[account(
        init,
        payer = payer,
        seeds = ["vault".as_bytes().as_ref(), market.key().as_ref()],
        bump,
        token::mint = collateral_mint,
        token::authority = market
    )]
    pub vault: Box<Account<'info, token::TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
# [instruction (lot_size : u64)]
pub struct BuyOption<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(mut)]
    pub market: Box<Account<'info, OptionMarket>>,
    #[account(mut)]
    pub base_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub collateral_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub option_note_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub vault: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub option_note_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub collateral_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[derive(Accounts)]
pub struct SettleExpiry<'info> {
    #[account(mut)]
    pub market: Box<Account<'info, OptionMarket>>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub redeemer: Signer<'info>,
    #[account(mut)]
    pub market: Box<Account<'info, OptionMarket>>,
    #[account(mut)]
    pub base_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub collateral_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub option_note_mint: Box<Account<'info, token::Mint>>,
    #[account(mut)]
    pub vault: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub redeemer_account: Box<Account<'info, token::TokenAccount>>,
    #[account(mut)]
    pub option_note_account: Box<Account<'info, token::TokenAccount>>,
    pub token_program: Program<'info, token::Token>,
}

#[program]
pub mod sea_option {
    use super::*;

    pub fn init_option(
        ctx: Context<InitOption>,
        strike_price: u64,
        expiry_timestamp: i64,
        is_put: bool,
        lot_size: u64,
        premium_per_lot: u64,
    ) -> Result<()> {
        init_option_handler(
            ctx,
            strike_price,
            expiry_timestamp,
            is_put,
            lot_size,
            premium_per_lot,
        )
    }

    pub fn buy_option(ctx: Context<BuyOption>, lot_size: u64) -> Result<()> {
        buy_option_handler(ctx, lot_size)
    }

    pub fn settle_expiry(ctx: Context<SettleExpiry>, expiry_price: u64) -> Result<()> {
        settle_expiry_handler(ctx, expiry_price)
    }

    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        redeem_handler(ctx)
    }
}

#[error_code]
pub enum ProgramError {
    #[msg("In-sufficent balance")]
    E000,
    #[msg("Price Error")]
    E001,
    #[msg("Invalid option token")]
    E002,
    #[msg("Expiry price not found")]
    E003,
}
