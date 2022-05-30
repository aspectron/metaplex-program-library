use mpl_token_metadata::{
    instruction::{self},
    state::{MasterEditionV2, EDITION, PREFIX},
};
use solana_program::borsh::try_from_slice_unchecked;
use solana_sdk::{pubkey::Pubkey, signature::Signer, transaction::Transaction, transport};
use spl_associated_token_account::get_associated_token_address;

use crate::{
    core::{
        helpers::{clone_keypair, clone_pubkey, get_account},
        MetadataManager,
    },
    *,
};

#[derive(Debug)]
pub struct MasterEditionManager {
    pub authority: Keypair,
    pub edition_pubkey: Pubkey,
    pub metadata_pubkey: Pubkey,
    pub mint_pubkey: Pubkey,
    pub token_account: Pubkey,
    pub owner: Keypair,
}

impl Clone for MasterEditionManager {
    fn clone(&self) -> Self {
        Self {
            authority: clone_keypair(&self.authority),
            edition_pubkey: clone_pubkey(&self.edition_pubkey),
            metadata_pubkey: clone_pubkey(&self.metadata_pubkey),
            mint_pubkey: clone_pubkey(&self.mint_pubkey),
            token_account: clone_pubkey(&self.token_account),
            owner: clone_keypair(&self.owner),
        }
    }
}

impl MasterEditionManager {
    pub fn new(metadata: &MetadataManager) -> Self {
        let program_id = mpl_token_metadata::id();
        let mint_pubkey = metadata.mint.pubkey();

        let master_edition_seeds = &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            mint_pubkey.as_ref(),
            EDITION.as_bytes(),
        ];
        let edition_pubkey =
            Pubkey::find_program_address(master_edition_seeds, &mpl_token_metadata::id()).0;

        Self {
            authority: clone_keypair(&metadata.authority),
            edition_pubkey,
            metadata_pubkey: metadata.pubkey,
            mint_pubkey,
            token_account: get_associated_token_address(
                &metadata.owner.pubkey(),
                &metadata.mint.pubkey(),
            ),
            owner: clone_keypair(&metadata.owner),
        }
    }

    pub async fn get_data(&self, context: &mut ProgramTestContext) -> MasterEditionV2 {
        let account = get_account(context, &self.edition_pubkey).await;
        try_from_slice_unchecked(&account.data).unwrap()
    }

    pub async fn get_data_from_account(
        context: &mut ProgramTestContext,
        pubkey: &Pubkey,
    ) -> MasterEditionV2 {
        let account = get_account(context, pubkey).await;
        try_from_slice_unchecked(&account.data).unwrap()
    }

    pub async fn create_v3(
        &self,
        context: &mut ProgramTestContext,
        max_supply: Option<u64>,
    ) -> transport::Result<()> {
        let tx = Transaction::new_signed_with_payer(
            &[instruction::create_master_edition_v3(
                mpl_token_metadata::id(),
                self.edition_pubkey,
                self.mint_pubkey,
                self.authority.pubkey(),
                self.authority.pubkey(),
                self.metadata_pubkey,
                self.authority.pubkey(),
                max_supply,
            )],
            Some(&self.authority.pubkey()),
            &[&self.authority],
            context
                .banks_client
                .clone()
                .get_new_latest_blockhash(&context.banks_client.get_latest_blockhash().await?)
                .await?,
        );

        context.banks_client.process_transaction(tx).await
    }
}
