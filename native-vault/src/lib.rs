use instructions::VaultInstructions;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::{entrypoint, ProgramResult},
    program_error::ProgramError,
    pubkey::{pubkey, Pubkey},
};

mod instructions;

const ID: Pubkey = pubkey!("22222222222222222222222222222222");

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::ID) {
        return Err(ProgramError::IncorrectProgramId);
    }

    // Figure out which instruction we're calling
    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => todo!(),
        VaultInstructions::Withdraw => todo!(),
    };

    Ok(())
}
