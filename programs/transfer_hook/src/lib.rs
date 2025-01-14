use anchor_lang::prelude::*;


declare_id!("FXDy9LxZZmnLnNynGDNhxGHx2iudiqUx278bji1SiMcX");

pub mod context;
pub use context::*;

pub mod state;
pub use state::*;
pub mod error;

#[program]
pub mod transfer_hook {
    use super::*;

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>
    ) -> Result<()> {
        ctx.accounts.initialize_extra_account_meta_list()
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(_amount)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhiteList>) -> Result<()> {
        ctx.accounts.add_to_whitelist()
    }
}





