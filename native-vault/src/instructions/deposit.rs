use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke,
    program_error::ProgramError, system_instruction::transfer,
};

pub fn process(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    let [signer, vault, _system_program] = accounts else {
        return Err(ProgramError::InvalidAccountData)
    };

    invoke(&transfer(signer.key, vault.key, amount), accounts);

    Ok(())
}
