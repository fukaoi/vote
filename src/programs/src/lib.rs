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
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("this account is not program_id account");
        return Err(ProgramError::IncorrectProgramId);
    }

    // The data must be large enough to hold two u32 vote counts
    // in the next (slightly more complicated) version of the
    // program we will use solana_sdk::program_pack::Pack
    // to retrieve and deserialise the account data
    // and to check it is the correct length
    // for now, realise it's literally just 8 bytes of data.

    if account.try_data_len()? < 2 * mem::size_of::<u32>() {
        msg!("Vote account data length tool small for u32");
        return Err(ProgramError::InvalidAccountData);
    }

    msg!("program_id: {}", program_id);
    msg!("account.owner: {:#?}", account.owner);
    msg!("account.key: {:#?}", account.key);

    // let data = account.try_borrow_data()?;
    let mut data = account.try_borrow_mut_data()?;

    if 1 == instruction_data[0] {
        let vc = LittleEndian::read_u32(&data[0..4]);
        // increment by 1. voted
        LittleEndian::write_u32(&mut data[0..4], vc + 1);
        msg!("Voted for candidate1");
    }

    if 2 == instruction_data[0] {
        let vc = LittleEndian::read_u32(&data[4..8]);
        // increment by 2. voted
        LittleEndian::write_u32(&mut data[4..8], vc + 1);
        msg!("Voted for candidate2");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]

    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;

        let mut data = vec![0; 2 * mem::size_of::<u32>()];
        LittleEndian::write_u32(&mut data[0..4], 0);
        LittleEndian::write_u32(&mut data[4..8], 0);

        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        
        let mut instruction_data: Vec<u8> = vec![0];
        let accounts = vec![account];
        assert_eq!(LittleEndian::read_u32(&accounts[0].data.borrow()[0..4]), 0);
    }
}
