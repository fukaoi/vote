use byteorder::{ByteOrder, LittleEndian};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use std::mem;

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("[lib.rs]Rust program entrypoint");

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    msg!("program_id: {}", program_id);
    Ok(())
}
