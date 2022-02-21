use solana_program::{account_info::{AccountInfo,next_account_info},entrypoint,entrypoint::ProgramResult,pubkey::Pubkey,msg};
use mokshyafeed::collection_price;
use std::{
    str::FromStr
  };

// Returns the current price of the collection as per the collection account info

entrypoint!(read_price);
 pub fn read_price(
    _program_id: &Pubkey, // Ignored
    account_info: &[AccountInfo], // Public key of the account to read price data from
    _instruction_data: &[u8],
)-> ProgramResult
{
    let program_id="GK4UuXCYhFYjUbf9514GrWuCDtxd75xkqpg9bZ1K9EL7"; //always fixed
    let program_id = Pubkey::from_str(program_id).unwrap();
    let accounts_iter = &mut account_info.iter();
    // This is the account of our our account
    let data_account = next_account_info(accounts_iter)?;
        
    msg!("The price of the nft collection is ");
    let price=collection_price(&program_id, data_account)?;
    msg!("{}",price);
    Ok(())

}