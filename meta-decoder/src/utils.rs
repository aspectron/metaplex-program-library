use crate::{
    error::MetadataError,
    state::{
        Key, Metadata, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH, MAX_NAME_LENGTH
    },
};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo,
    borsh::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_option::COption,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(MetadataError::IncorrectOwner.into())
    } else {
        Ok(())
    }
}


pub fn is_correct_account_type(data: &[u8], data_type: Key, data_size: usize) -> bool {
    (data[0] == data_type as u8 || data[0] == Key::Uninitialized as u8) && (data.len() == data_size)
}

pub fn try_from_slice_checked<T: BorshDeserialize>(
    data: &[u8],
    data_type: Key,
    data_size: usize,
) -> Result<T, ProgramError> {
    if !is_correct_account_type(data, data_type, data_size) {
        return Err(MetadataError::DataTypeMismatch.into());
    }

    let result: T = try_from_slice_unchecked(data)?;

    Ok(result)
}

/// Strings need to be appended with `\0`s in order to have a deterministic length.
/// This supports the `memcmp` filter  on get program account calls.
/// NOTE: it is assumed that the metadata fields are never larger than the respective MAX_LENGTH
pub fn puff_out_data_fields(metadata: &mut Metadata) {
    metadata.data.name = puffed_out_string(&metadata.data.name, MAX_NAME_LENGTH);
    metadata.data.symbol = puffed_out_string(&metadata.data.symbol, MAX_SYMBOL_LENGTH);
    metadata.data.uri = puffed_out_string(&metadata.data.uri, MAX_URI_LENGTH);
}

/// Pads the string to the desired size with `0u8`s.
/// NOTE: it is assumed that the string's size is never larger than the given size.
pub fn puffed_out_string(s: &str, size: usize) -> String {
    let mut array_of_zeroes = vec![];
    let puff_amount = size - s.len();
    while array_of_zeroes.len() < puff_amount {
        array_of_zeroes.push(0u8);
    }
    s.to_owned() + std::str::from_utf8(&array_of_zeroes).unwrap()
}

/// Pads the string to the desired size with `0u8`s.
/// NOTE: it is assumed that the string's size is never larger than the given size.
pub fn zero_account(s: &str, size: usize) -> String {
    let mut array_of_zeroes = vec![];
    let puff_amount = size - s.len();
    while array_of_zeroes.len() < puff_amount {
        array_of_zeroes.push(0u8);
    }
    s.to_owned() + std::str::from_utf8(&array_of_zeroes).unwrap()
}
