use crate::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(params: HouseInitParams)]
pub struct HouseInit<'info> {
    #[account(
        init,
        space = 8 + std::mem::size_of::<HouseState>(),
        payer = payer,
        seeds = [HOUSE_SEED],
        bump
    )]
    pub house: AccountLoader<'info, HouseState>,
    /// CHECK:
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,

    pub switchboard_mint: Box<Account<'info, Mint>>,
    #[account(
        mut,
        constraint = 
            switchboard_queue.load()?.unpermissioned_vrf_enabled == true @ VrfFlipError::OracleQueueRequiresPermissions
    )]
    pub switchboard_queue: AccountLoader<'info, OracleQueueAccountData>,

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

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    /// CHECK:
    #[account(address = solana_program::sysvar::rent::ID)]
    pub rent: AccountInfo<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct HouseInitParams {}

impl HouseInit<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &HouseInitParams,
    ) -> anchor_lang::Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: &Context<Self>, _params: &HouseInitParams) -> anchor_lang::Result<()> {
        msg!("house_init");

        let house = &mut ctx.accounts.house.load_init()?;
        let house_bump = ctx.bumps.get("house").unwrap().clone();

        house.bump = house_bump;
        house.authority = ctx.accounts.authority.key().clone();
        house.switchboard_mint = ctx.accounts.switchboard_mint.key().clone();
        house.mint = ctx.accounts.game_token_mint.key().clone();
        house.switchboard_queue = ctx.accounts.switchboard_queue.key().clone();
        house.house_vault = ctx.accounts.house_vault.key().clone();
        drop(house);

        Ok(())
    }
}
