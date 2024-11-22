pub fn calculate_readable_amount(amount: u64, decimals: u8) -> u64 {
    amount.checked_div(10u64.pow(decimals as u32)).unwrap()
}

pub fn calculate_internal_amount(amount: u64, decimals: u8) -> u64 {
    amount.checked_mul(10u64.pow(decimals as u32)).unwrap()
}
