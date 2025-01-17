use anchor_lang::prelude::*;


declare_id!("FXDy9LxZZmnLnNynGDNhxGHx2iudiqUx278bji1SiMcX");

pub mod context;
pub use context::*;

pub mod state;
pub use state::*;
pub mod error;

#[program]
pub mod transfer_hook {
    use spl_tlv_account_resolution::state::ExtraAccountMetaList;
    use spl_transfer_hook_interface::instruction::ExecuteInstruction;

    use super::*;

    pub fn init_config(ctx: Context<InitConfig>,fee:u64) -> Result<()> {
        ctx.accounts.init_config(fee)
    }

    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>,
    ) -> Result<()> {
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;

        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas
        )?;

        Ok(())
    }

    #[interface(spl_transfer_hook_interface::execute)]
    pub fn transfer_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        ctx.accounts.transfer_hook(_amount,&ctx.bumps)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhiteList>,new_account:Pubkey) -> Result<()> {
        ctx.accounts.add_to_whitelist(new_account)
    }

    pub fn update_fee(ctx: Context<UpdateFee>, fee: u64) -> Result<()> {
        ctx.accounts.update_fee(fee)
    }
}





