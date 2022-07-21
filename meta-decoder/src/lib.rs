
pub mod deser;
pub mod state;
pub mod error;
pub mod utils;

pub use borsh;
pub use state::{PREFIX, Metadata};
pub use solana_program;
use solana_program::pubkey::Pubkey;

solana_program::declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub fn metaplex_program_id()->Pubkey{
    ID
}

pub fn get_metadata_pda(pubkey: Pubkey) -> Pubkey {
    let metaplex_pubkey = ID;

    let seeds = &[
        PREFIX.as_bytes(),
        metaplex_pubkey.as_ref(),
        pubkey.as_ref(),
    ];

    let (pda, _) = Pubkey::find_program_address(seeds, &metaplex_pubkey);
    pda
}
