use anchor_lang::prelude::*;

use crate::{Config, WhiteList};

#[derive(Accounts)]
pub struct InitConfig<'info>{
    #[account(mut)]
    payer: Signer<'info>,

    #[account(init_if_needed, seeds = [b"white_list"], bump, payer = payer, space = 400)]
    pub white_list: Account<'info, WhiteList>,
    #[account(init_if_needed, seeds = [b"config"], bump, payer = payer, space = 400)]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>
}

impl<'info> InitConfig<'info> {
    pub fn init_config(&mut self,fee:u64) -> Result<()> {
        self.white_list.set_inner(WhiteList{
            white_list: vec![],
            authority: self.payer.key()
        });
        self.config.set_inner(Config{
            authority: self.payer.key(),
            fee: 0
        });
        Ok(())
    }
    
}