use crate::error::ErrorCode;
use crate::state::Bounty;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawFromBounty<'info> {
    #[account(mut)]
    pub participant: Signer<'info>,
    #[account(
        mut,
        seeds = [b"bounty", bounty.authority.key().as_ref(), bounty.mint.as_ref()],
        bump = bounty.bump[0],
        constraint = bounty.bounty_type == 1 @ ErrorCode::NotOpenBounty,
    )]
    pub bounty: Account<'info, Bounty>,
    #[account(mut, associated_token::mint = bounty.payment_mint, associated_token::authority = participant)]
    pub participant_ata: Account<'info, TokenAccount>,
    #[account(mut, associated_token::mint = bounty.payment_mint, associated_token::authority = bounty)]
    pub bounty_ata: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawFromBounty<'info> {
    pub fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = Transfer {
            authority: self.bounty.to_account_info(),
            from: self.bounty_ata.to_account_info(),
            to: self.participant_ata.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

pub fn withdraw_from_bounty(ctx: Context<WithdrawFromBounty>, amount: u64) -> Result<()> {
    if amount == 0 {
        return Err(ProgramError::InvalidInstructionData.into());
    }

    let bounty = &mut ctx.accounts.bounty;
    let participant = ctx.accounts.participant.key();

    bounty.decrease_participant_shares(participant, amount)?;

    token::transfer(ctx.accounts.transfer_ctx(), amount)?;

    Ok(())
}