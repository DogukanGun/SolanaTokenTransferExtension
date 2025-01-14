use anchor_lang::prelude::*;
use std::cell::RefMut;
use anchor_spl::{
    associated_token::AssociatedToken, token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount,
            BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    }, token_interface::{ Mint, TokenAccount, TokenInterface }
};
use anchor_spl::token_interface::{transfer_checked,TransferChecked};
use crate::{error::TransferError, Config, WhiteList};
// Order of accounts matters for this struct.
// The first 4 accounts are the accounts required for token transfer (source, mint, destination, owner)
// Remaining accounts are the extra accounts required from the ExtraAccountMetaList account
// These accounts are provided via CPI to this program from the token2022 program
#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(token::mint = mint, token::authority = owner)]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(seeds = [b"extra-account-metas", mint.key().as_ref()], bump)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(seeds = [b"white_list"], bump)]
    pub white_list: Account<'info, WhiteList>,
    #[account(seeds = [b"config"], bump)]
    pub config: Account<'info, Config>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint_b,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub taker: Box<InterfaceAccount<'info, TokenAccount>>,
}

impl<'info> TransferHook<'info> {
    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(&mut self, _amount: u64) -> Result<()> {
        // Fail this instruction if it is not called from within a transfer hook
        self.check_is_transferring()?;

        if !self.white_list.white_list.contains(&self.destination_token.key()) {
            let transfer_accounts = TransferChecked {
                from: self.taker.to_account_info(),
                mint: self.mint.to_account_info(),
                to: self.maker_ata_b.to_account_info(),
                authority: self.taker.to_account_info(),
            };
    
            let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
    
            transfer_checked(cpi_ctx, self.config.fee, self.mint_b.decimals)?
        }
        msg!("Account in white list, all good!");
        Ok(())
    }

    fn check_is_transferring(&mut self) -> Result<()> {
        let source_token_info = self.source_token.to_account_info();
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;
    
        if !bool::from(account_extension.transferring) {
            return Err(TransferError::IsNotCurrentlyTransferring.into());
        }
    
        Ok(())
    }
    
}