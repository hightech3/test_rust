use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("CFSwwcAPi9K5E7R3JAi5uv1VkToHFFFqerxyxXvz4jR2");

#[program]
pub mod sol_to_token_exchange {
    use super::*;

    pub fn exchange(ctx: Context<Exchange>, amount_sol: u64) -> Result<()> {
        let price_rate_sol_usdt: f64 = 0.0065;
        let price_rate_token1_usdt: f64 = 12.17;

        let amount_usdt = (amount_sol as f64) / price_rate_sol_usdt;

        // Convert USDT to TOKEN1
        let amount_token1 = amount_usdt * price_rate_token1_usdt;
        let amount_token1_u64 = amount_token1 as u64; // Convert to integer

        // Transfer TOKEN1 to the user
        let cpi_accounts = Transfer {
            from: ctx.accounts.token1_vault.to_account_info(),
            to: ctx.accounts.user_token1_account.to_account_info(),
            authority: ctx.accounts.exchange_pda.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount_token1_u64)?;

        Ok(())
    }

    pub fn buy_ticket_spl(ctx: Context<BuyTicketSPL>) -> Result<()> {
        let ticket_price_token1: u64 = 2_000_000; // Adjust based on TOKEN1 equivalent of 0.002 SOL
        let operations_fee = ticket_price_token1.checked_div(10).unwrap();
        let remaining_after_operations = ticket_price_token1.checked_sub(operations_fee).unwrap();

        let jackpot = remaining_after_operations
            .checked_mul(60)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let referral = remaining_after_operations
            .checked_mul(20)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let remaining_after_jr = remaining_after_operations
            .checked_sub(jackpot)
            .unwrap()
            .checked_sub(referral)
            .unwrap();

        // Distribute rewards
        let pool_5_amount = remaining_after_jr
            .checked_mul(20)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let pool_4_ball_amount = remaining_after_jr
            .checked_mul(18)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let pool_4_amount = remaining_after_jr
            .checked_mul(16)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let pool_3_ball_amount = remaining_after_jr
            .checked_mul(14)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let pool_3_amount = remaining_after_jr
            .checked_mul(12)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let pool_2_ball_amount = remaining_after_jr
            .checked_mul(10)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let pool_1_ball_amount = remaining_after_jr
            .checked_mul(7)
            .unwrap()
            .checked_div(100)
            .unwrap();
        let pool_ball_amount = remaining_after_jr
            .checked_mul(3)
            .unwrap()
            .checked_div(100)
            .unwrap();

        // Update prize pools
        ctx.accounts.round.pool_list.pool_5_ball = ctx
            .accounts
            .round
            .pool_list
            .pool_5_ball
            .checked_add(jackpot)
            .unwrap();
        ctx.accounts.round.pool_list.pool_5 = ctx
            .accounts
            .round
            .pool_list
            .pool_5
            .checked_add(pool_5_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_4_ball = ctx
            .accounts
            .round
            .pool_list
            .pool_4_ball
            .checked_add(pool_4_ball_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_4 = ctx
            .accounts
            .round
            .pool_list
            .pool_4
            .checked_add(pool_4_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_3_ball = ctx
            .accounts
            .round
            .pool_list
            .pool_3_ball
            .checked_add(pool_3_ball_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_3 = ctx
            .accounts
            .round
            .pool_list
            .pool_3
            .checked_add(pool_3_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_2_ball = ctx
            .accounts
            .round
            .pool_list
            .pool_2_ball
            .checked_add(pool_2_ball_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_1_ball = ctx
            .accounts
            .round
            .pool_list
            .pool_1_ball
            .checked_add(pool_1_ball_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_ball = ctx
            .accounts
            .round
            .pool_list
            .pool_ball
            .checked_add(pool_ball_amount)
            .unwrap();
        ctx.accounts.round.pool_list.pool_referral = ctx
            .accounts
            .round
            .pool_list
            .pool_referral
            .checked_add(referral)
            .unwrap();
        ctx.accounts.round.pool_list.pool_operations = ctx
            .accounts
            .round
            .pool_list
            .pool_operations
            .checked_add(operations_fee)
            .unwrap();

        // Verify correct distribution
        let total_distributed = operations_fee
            + jackpot
            + referral
            + pool_5_amount
            + pool_4_ball_amount
            + pool_4_amount
            + pool_3_ball_amount
            + pool_3_amount
            + pool_2_ball_amount
            + pool_1_ball_amount
            + pool_ball_amount;

        require!(
            total_distributed == ticket_price_token1,
            ErrorCode::DistributionError
        );

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub exchange_pda: AccountInfo<'info>,

    #[account(
        mut,
        token::mint = token1_mint,
        token::authority = exchange_pda
    )]
    pub token1_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub user_token1_account: Account<'info, TokenAccount>,

    pub token1_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct BuyTicketSPL<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub round: Account<'info, Round>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Round {
    pub pool_list: PoolList,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct PoolList {
    pub pool_5_ball: u64,
    pub pool_5: u64,
    pub pool_4_ball: u64,
    pub pool_4: u64,
    pub pool_3_ball: u64,
    pub pool_3: u64,
    pub pool_2_ball: u64,
    pub pool_1_ball: u64,
    pub pool_ball: u64,
    pub pool_referral: u64,
    pub pool_operations: u64,
}

/// Custom error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Distribution error: Total does not match the ticket price")]
    DistributionError,
}
