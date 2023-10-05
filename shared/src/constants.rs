use near_sdk::Gas;

pub const BURN_ADDRESS: &str = "system";
pub const TRANSFER_DEPOSIT: u128 = 1u128;
pub const GAS_FOR_TX: Gas = Gas(5_000_000_000_000);
pub const SECP256K1_SIGNATURE_LENGTH: usize = 64;
pub const SECP256K1_PUBLIC_KEY_LENGTH: usize = 64;
pub const HASH_LENGTH: usize = 32;
pub const NFT_MINT_STORAGE_DEPOSIT: u128 = 2_000_000_000_000_000_000_000_00; // 0.2 NEAR
pub const FT_MINT_STORAGE_DEPOSIT: u128 = 125_000_000_000_000_000_000_0; // 0.00125 NEAR
pub const REGISTER_STORAGE_DEPOSIT: u128 = 120_000_000_000_000_000_000_0; // 0.00120 NEAR
pub const NO_ARGS: Vec<u8> = vec![];
pub const CALL_GAS: Gas = Gas(200_000_000_000_000); // 200 TGAS
