use anchor_lang::prelude::*;
use crate::states::{State, User, get_position_index};
use crate::error::ErrorCode;
use anchor_spl::{token, token_interface::{TokenAccount, TokenInterface}};
use anchor_spl::token::Transfer;

#[derive(Accounts)]
#[instruction(market_index: u16)]
pub struct Withdraw<'info,> {
    pub state: Account<'info, State>,

    #[account(mut, has_one = authority)]
    pub user: Account<'info, User>,

    pub authority: Signer<'info>,

        #[account(
            mut, 
            seeds = [b"perp_market_vault", market_index.to_le_bytes().as_ref()],
            bump,
        )]
            pub perp_market_vault: InterfaceAccount<'info, TokenAccount>,

            #[account(constraint = state.signer.eq(&drift_signer.key()))]
        /// CHECK: validate via state.signer
        pub drift_signer: AccountInfo<'info>,

        #[account(
            mut,
            constraint = &perp_market_vault.mint.eq(&user_token_account.mint)
        )]
            pub user_token_account: InterfaceAccount<'info, TokenAccount>,

            pub token_program: Interface<'info, TokenInterface>,
            pub system_program: Program<'info, System>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>, market_index: u16, amount: u64) -> Result<()> {
    let signer_bump = ctx.accounts.state.signer_bump;
    let seeds = [b"drift_signer".as_ref(), &[signer_bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.perp_market_vault.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.drift_signer.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(cpi_ctx, amount)?;

    let user = &mut ctx.accounts.user;
    user.total_collateral = user.total_collateral.checked_sub(amount).ok_or(ErrorCode::ArithmeticOverflow)?;

    let idx = get_position_index(&user.perp_positions, market_index)?;
    user.perp_positions[idx].collateral = user.perp_positions[idx].collateral.checked_sub(amount).ok_or(ErrorCode::ArithmeticOverflow)?;

    Ok(())
}

