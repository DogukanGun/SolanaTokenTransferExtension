use anchor_lang::prelude::*;

use crate::Config;

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    #[account(mut, has_one = authority)]
    pub config: Account<'info, Config>,
    pub authority: Signer<'info>,
}

impl<'info> UpdateFee<'info> {
    pub fn update_fee(&mut self, fee: u64) -> Result<()> {
        if self.authority.key() == self.config.authority {
            self.config.set_inner(Config{
                authority: self.authority.key(),
                fee
            });
        }
        Ok(())
    }
    
}
