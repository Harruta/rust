use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize)]

enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,      // Public key of the deployed program (not used here)
    accounts: &[AccountInfo], // An array of input addresses. This array should have all the addresses the user is going to interact with when they are calling this function.
    instruction_data: &[u8], // The actual `thing` the user wants to do. Its an array of bytes, but it can be deserialized into a struct. This contains information like what to do (add to a counter, remove from a counter, the value to add/remove)
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?; // Gets the first account in the list. This is expected to be the account holding the Counter

    let instruction_type = InstructionType::try_from_slice(instruction_data)?; // Deserialize the incoming instruction bytes into an InstructionType.

    let mut counter_data = Counter::try_from_slice(&acc.data.borrow())?; // Reads and deserializes the Counter struct from the account data.

    match instruction_type {
        InstructionType::Increment(value) => {
            msg!("Executing incress");
            counter_data.count += value;
        }

        InstructionType::Decrement(value) => {
            msg!("Executing decress");
            counter_data.count -= value;
        }
    }

    counter_data.serialize(&mut *acc.data.borrow_mut()); // Writes the updated Counter back into the account data, Serialize the data back into the account.

    msg!("Contract succeded");

    Ok(())
}
