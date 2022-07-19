use crate::constants::DID_ACCOUNT_SEED;
use crate::errors::DidSolError;
use crate::state::{DidAccount, Secp256k1RawSignature};
use crate::utils::check_other_controllers;
use anchor_lang::prelude::*;

pub fn set_controllers(
    ctx: Context<SetControllers>,
    set_controllers_arg: SetControllersArg,
    eth_signature: Option<Secp256k1RawSignature>,
) -> Result<()> {
    let data = &mut ctx.accounts.did_data;
    if eth_signature.is_some() {
        data.nonce += 1;
    }

    data.native_controllers = set_controllers_arg.native_controllers;
    // Make sure that vector does not contain duplicates.
    data.native_controllers.sort_unstable();
    data.native_controllers.dedup(); // requires sorted vector

    let own_authority = Pubkey::new(&data.initial_verification_method.key_data);

    require!(
        !data.native_controllers.contains(&own_authority),
        DidSolError::InvalidNativeControllers,
    );

    require!(
        check_other_controllers(&set_controllers_arg.other_controllers),
        DidSolError::InvalidOtherControllers
    );

    data.other_controllers = set_controllers_arg.other_controllers;
    // Make sure that vector does not contain duplicates.
    data.other_controllers.sort_unstable();
    data.other_controllers.dedup(); // requires sorted vector

    Ok(())
}

#[derive(Accounts)]
#[instruction(set_controllers_arg: SetControllersArg, eth_signature: Option<Secp256k1RawSignature>)]
pub struct SetControllers<'info> {
    #[account(
        mut,
        seeds = [DID_ACCOUNT_SEED.as_bytes(), did_data.initial_verification_method.key_data.as_ref()],
        bump = did_data.bump,
        constraint = did_data.find_authority(&authority.key(), &set_controllers_arg.try_to_vec().unwrap(), eth_signature.as_ref(), None).is_some(),
    )]
    pub did_data: Account<'info, DidAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

/// Argument
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct SetControllersArg {
    pub native_controllers: Vec<Pubkey>,
    pub other_controllers: Vec<String>,
}
