use borsh::BorshDeserialize;
use solana_program::pubkey::Pubkey;

solana_program::declare_id!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

#[derive(BorshDeserialize, Debug)]
pub enum AccountKey {
    Uninitialized,
    EditionV1,
    MasterEditionV1,
    ReservationListV1,
    MetadataV1,
    ReservationListV2,
    MasterEditionV2,
    EditionMarker,
    UseAuthorityRecord,
    CollectionAuthorityRecord,
}

#[derive(BorshDeserialize, Debug)]
pub struct Metadata {
    pub key: AccountKey,
    pub update_authority: Pubkey,
    pub mint: Pubkey,
    pub data: Data,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub edition_nonce: Option<u8>,
    pub token_standard: Option<TokenStandard>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
    pub collection_details: Option<CollectionDetails>,
    pub programmable_config: Option<ProgrammableConfig>,
}

#[derive(BorshDeserialize, Debug)]
pub enum TokenStandard {
    NonFungible,             // This is a master edition
    FungibleAsset,           // A token with metadata that can also have attrributes
    Fungible,                // A token with simple metadata
    NonFungibleEdition,      // This is a limited edition
    ProgrammableNonFungible, // NonFungible with programmable configuration
}

#[derive(BorshDeserialize, Debug)]
pub enum ProgrammableConfig {
    V1 {
        rule_set: Option<Pubkey>,
    },
}

#[derive(BorshDeserialize)]
pub struct MetadataExt {
    pub edition_nonce: Option<u8>,
}

#[derive(BorshDeserialize)]
pub struct MetadataExtV1_2 {
    pub token_standard: Option<u8>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
}

#[derive(BorshDeserialize, Debug)]
pub struct Data {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
}

#[derive(BorshDeserialize)]
pub struct DataV2 {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub creators: Option<Vec<Creator>>,
    pub collection: Option<Collection>,
    pub uses: Option<Uses>,
}

#[derive(BorshDeserialize, Debug)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[derive(BorshDeserialize, Debug)]
pub struct Collection {
    pub verified: bool,
    pub key: Pubkey,
}

#[derive(BorshDeserialize, Debug)]
pub struct Uses {
    pub use_method: u8,
    pub remaining: u64,
    pub total: u64,
}

#[derive(BorshDeserialize, Debug)]
pub enum CollectionDetails {
    V1 { size: u64 },
}
