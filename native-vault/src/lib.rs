use instructions::{deposit, withdraw, VaultInstructions};
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

    let (discriminator, data) = data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    // The incomming data is just the `amount` value so no need of complex deserialization

    assert_eq!(data.len(), 8); // u64
    let amount = u64::from_le_bytes([
        data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
    ]);

    match VaultInstructions::try_from(discriminator)? {
        VaultInstructions::Deposit => deposit::process(accounts, amount)?,
        VaultInstructions::Withdraw => withdraw::process(accounts, amount)?,
    };

    Ok(())
}
