use std::str::FromStr;
use near_sdk::json_types::U128;
use num_bigint::BigInt;

pub fn vector_from_32_bytes(val: [u8; 32]) -> Vec<u8> {
    Vec::from(val.as_slice())
}

pub fn u128_to_bytes(amount: U128) -> [u8; 32] {
    big_int_to_bytes(BigInt::from_str(&*amount.0.to_string()).unwrap())
}

pub fn usize_to_bytes(value: usize) -> [u8; 32] {
    big_int_to_bytes(BigInt::from(value))
}

pub fn big_int_to_bytes(val: BigInt) -> [u8; 32] {
    let mut result: [u8; 32] = Default::default();
    let bytes = val.to_signed_bytes_be().to_vec();
    for (pos, _e) in bytes.iter().enumerate() {
        result[31 - pos] = bytes[bytes.len() - 1 - pos];
    }
    result
}

pub fn to_32_bytes(slice: &[u8]) -> Vec<u8> {
    if slice.len() > 32 || slice.len() == 0 {
        return slice.to_vec();
    }

    let mut resized = vec![0; 32];
    resized[(32 - slice.len())..].copy_from_slice(&slice);
    return Vec::from(resized.clone());
}
