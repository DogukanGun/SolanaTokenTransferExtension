use anchor_lang::prelude::*;

use crate::WhiteList;

#[derive(Accounts)]
pub struct AddToWhiteList<'info> {
    /// CHECK: New account to add to white list
    #[account()]
    pub new_account: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"white_list"],
        bump
    )]
    pub white_list: Account<'info, WhiteList>,
    #[account(mut)]
    pub signer: Signer<'info>,
}


impl<'info> AddToWhiteList<'info> {
    pub fn add_to_whitelist(&mut self) -> Result<()> {
        if self.white_list.authority != self.signer.key() {
            panic!("Only the authority can add to the white list!");
        }

        self.white_list.white_list.push(self.new_account.key());
        msg!("New account white listed! {0}", self.new_account.key().to_string());
        msg!("White list length! {0}", self.white_list.white_list.len());

        Ok(())
    }
}