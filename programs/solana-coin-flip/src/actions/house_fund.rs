use crate::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(params: HouseFundParams)]
pub struct HouseFund<'info> {
    #[account(
        mut,
        seeds = [HOUSE_SEED],
        bump = house.load()?.bump,
    )]
    pub house: AccountLoader<'info, HouseState>,

    pub game_token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = game_token_mint,
        associated_token::authority = house,
    )]
    pub house_vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = game_token_mint,
        associated_token::authority = payer,
    )]
    pub payer_ata: Account<'info, TokenAccount>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK:
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct HouseFundParams { amount: u64}

impl HouseFund<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &HouseFundParams,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, params: &HouseFundParams) -> anchor_lang::Result<()> {
        msg!("house_fund");

        let house: &mut std::cell::RefMut<'_, HouseState> = &mut ctx.accounts.house.load_init()?;

        transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.payer_ata.to_account_info(),
                    to: ctx.accounts.house_vault.to_account_info(),
                    authority: ctx.accounts.payer.to_account_info(),
                },
            ),
            params.amount,
        )?;

        drop(house);

        msg!("current total house amount: {}", ctx.accounts.house_vault.amount);
        Ok(())
    }
}
