use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::Token, token_interface::Mint};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta,
    seeds::Seed,
    state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,

    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        )?,
        payer = payer
    )]
    pub extra_account_meta_list: AccountInfo<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>
}

// Define extra account metas to store on extra_account_meta_list account
impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>>  {
        Ok(
            vec![
                // index 5, wrapped SOL mint
                ExtraAccountMeta::new_with_pubkey(
                    &Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
                    false,
                    false
                )?,
                // index 6, token program (for wsol token transfer)
                ExtraAccountMeta::new_with_pubkey(&Token::id(), false, false)?,
                // index 7, associated token program
                ExtraAccountMeta::new_with_pubkey(&AssociatedToken::id(), false, false)?,
                // index 8, delegate PDA
                ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::Literal {
                            bytes: b"delegate".to_vec(),
                        },
                    ],
                    false, // is_signer
                    true // is_writable
                )?,
                // index 9, delegate wrapped SOL token account
                ExtraAccountMeta::new_external_pda_with_seeds(
                    7, // associated token program index
                    &[
                        Seed::AccountKey { index: 8 }, // owner index (delegate PDA)
                        Seed::AccountKey { index: 6 }, // token program index
                        Seed::AccountKey { index: 5 }, // wsol mint index
                    ],
                    false, // is_signer
                    true // is_writable
                )?,
                // index 10, sender wrapped SOL token account
                ExtraAccountMeta::new_external_pda_with_seeds(
                    7, // associated token program index
                    &[
                        Seed::AccountKey { index: 3 }, // owner index
                        Seed::AccountKey { index: 6 }, // token program index
                        Seed::AccountKey { index: 5 }, // wsol mint index
                    ],
                    false, // is_signer
                    true // is_writable
                )?,
                ExtraAccountMeta::new_with_seeds(
                    &[
                        Seed::Literal {
                            bytes: b"counter".to_vec(),
                        },
                    ],
                    false, // is_signer
                    true // is_writable
                )?
            ]
        )
    }
}
