use anchor_lang::prelude::*;
use std::cell::RefMut;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount,
            BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{ Mint, TokenAccount },
};
use crate::{error::TransferError, WhiteList};
// Order of accounts matters for this struct.
// The first 4 accounts are the accounts required for token transfer (source, mint, destination, owner)
// Remaining accounts are the extra accounts required from the ExtraAccountMetaList account
// These accounts are provided via CPI to this program from the token2022 program
#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(token::mint = mint, token::authority = owner)]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner, can be SystemAccount or PDA owned by another program
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList Account,
    #[account(seeds = [b"extra-account-metas", mint.key().as_ref()], bump)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(seeds = [b"white_list"], bump)]
    pub white_list: Account<'info, WhiteList>,
}

impl<'info> TransferHook<'info> {
    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(&mut self, _amount: u64) -> Result<()> {
        // Fail this instruction if it is not called from within a transfer hook
        self.check_is_transferring()?;

        if !self.white_list.white_list.contains(&self.destination_token.key()) {
            panic!("Account not in white list!");
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