use solana_program::{
    account_info::next_account_info,
    config::program,
    entrypoint::{self, ProgramResult},
    example_mocks::solana_sdk::address_lookup_table::instruction,
    msg,
    pubkey::{Pubkey, pubkey},
};

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountTnfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter());
    Ok(())
}
