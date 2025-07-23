use core::fmt::Debug;
use gstd::{exec, ext, format};

/// Total number of blocks required to complete the unbonding process.
///
/// In Vara Network, each block takes 3 seconds to produce.
/// Therefore:
///
/// 230_400 blocks × 3 = 691_200 seconds = 192 hours = 8 days
pub const TOTAL_BLOCKS_TO_UNBOND: u64 = 230_400;

/// Number of blocks that make up a single era.
///
/// Since each block takes ~3 seconds, this corresponds to:
///
/// 14_400 blocks × 3 = 43_200 seconds = 12 hours per era
pub const ONE_ERA_IN_BLOCKS: u64 = 14_400;

/// Last active era registered on the testnet used as a reference point.
///
/// Useful for estimating the current era based on block height,
/// especially in offline environments or during network analysis.
pub const LAST_TESTNET_ACTIVE_ERA_REGISTERED: u64 = 1326;

/// Block number at which the `LAST_TESTNET_ACTIVE_ERA_REGISTERED` started on the testnet.
///
/// Note: this value may be approximately 40 seconds later than the actual start of the era
/// due to timing offsets or recording delay.
pub const LAST_TESTNET_ACTIVE_ERA_INIT_BLOCK: u64 = 18_925_403; // ~10s late

/// Last active era registered on the mainnet used as a reference point.
///
/// This value currently matches the testnet, but may diverge as networks evolve.
pub const LAST_MAINNET_ACTIVE_ERA_REGISTERED: u64 = 1469;

/// Block number at which the `LAST_MAINNET_ACTIVE_ERA_REGISTERED` started on the mainnet.
///
/// Note: this may be offset by approximately 40 seconds from the true start of the era.
pub const LAST_MAINNET_ACTIVE_ERA_INIT_BLOCK: u64 = 24_302_610; // ~10s late

/// A constant representing one whole token in Vara Network (1 token = 10¹² units).
///
/// This is a convenience constant for specifying token values in smart contracts
///
/// ## Example
/// ```
/// let reward = 5 * ONE_TOKEN; // equivalent to 5_000_000_000_000 or 5 Varas
/// ```
pub const ONE_TOKEN: u128 = 1e12 as u128;

/// Executes a closure, panicking if it returns an error.
///
/// This helper is useful in tests or internal logic where failure is not recoverable,
/// and you want to surface the error immediately with a clear panic.
///
/// # Type Parameters
/// - `T`: The success type returned by the closure.
/// - `E`: The error type (must implement `Debug`).
///
/// # Arguments
/// - `f`: A closure returning `Result<T, E>`.
///
/// # Panics
/// Panics with the error if `f()` returns `Err`.
///
/// # Example
/// ```
/// let result = panicking(|| Ok(42)); // returns 42
/// let result = panicking(|| Err("failure")); // panics with "failure"
/// ```
pub fn panicking<T, E: Debug, F: FnOnce() -> Result<T, E>>(f: F) -> T {
    match f() {
        Ok(v) => v,
        Err(e) => panic(e),
    }
}

/// Terminates execution with a panic, logging the debug representation of the provided error.
///
/// This is a wrapper around `gstd::ext::panic` that formats the error for clarity.
/// It is suitable for terminating execution inside smart contracts when a fatal error occurs.
/// 
/// This helps save on gas fees since the error is detected when calculating the total amount of gas used.
///
/// # Arguments
/// - `err`: Any value implementing `Debug`, used for formatting the panic message.
///
/// # Behavior
/// Calls `ext::panic` with a `format!("{err:?}")` message and does **not** return.
///
/// # Example
/// ```
/// panic("unrecoverable failure");
/// panic(my_enum_error);
/// ```
pub fn panic(err: impl Debug) -> ! {
    ext::panic(format!("{err:?}"))
}

/// Estimates the current active era on the **testnet** based on the local block height.
///
/// # Returns
/// The estimated active era number (relative to the reference era).
///
pub fn testnet_active_era() -> u64 {
    let eras_passed = eras_passed_since_init_block(LAST_TESTNET_ACTIVE_ERA_INIT_BLOCK);

    LAST_TESTNET_ACTIVE_ERA_REGISTERED.saturating_add(eras_passed)
}

/// Estimates the current active era on the **mainnet** based on the local block height.
///
/// This function calculates how many eras have passed since a known reference point:
/// `LAST_MAINNET_ACTIVE_ERA_INIT_BLOCK`, which corresponds to the start block of 
/// `LAST_MAINNET_ACTIVE_ERA_REGISTERED`.
///
/// # How it works:
/// - Fetches the current block height via `exec::block_height()`.
/// - Computes how many blocks have passed since the known reference era.
/// - Divides the number of blocks passed by `ONE_ERA_IN_BLOCKS` (blocks per era).
///
/// # Returns
/// The estimated active era index on mainnet.
///
pub fn mainnet_active_era() -> u64 {
    let eras_passed = eras_passed_since_init_block(LAST_MAINNET_ACTIVE_ERA_INIT_BLOCK);

    LAST_MAINNET_ACTIVE_ERA_REGISTERED.saturating_add(eras_passed)
}


/// Calculates how many **blocks remain** until the next era begins on the **testnet**.
///
/// # Logic
/// - If at least one era has passed:
///     - Return the number of blocks remaining in the **current era**.
/// - Otherwise:
///     - Return the number of blocks passed since the initial block.
///
/// # Returns
/// The number of blocks left before the next testnet era starts.
pub fn blocks_left_for_next_testnet_era() -> u64 {
    let eras_passed = eras_passed_since_init_block(LAST_TESTNET_ACTIVE_ERA_INIT_BLOCK);
    let blocks_passed = blocks_passed_since_init_block(LAST_TESTNET_ACTIVE_ERA_INIT_BLOCK);

    if eras_passed >= 1 {
        let temp = blocks_passed % ONE_ERA_IN_BLOCKS;
        let blocks_left = ONE_ERA_IN_BLOCKS.saturating_sub(temp);
        blocks_left
    } else {
        blocks_passed
    }
}

/// Calculates how many **blocks remain** until the next era begins on the **mainnet**.
///
/// # Logic
/// - If at least one full era has passed:
///     - Return the number of blocks remaining in the **current era**.
/// - Otherwise:
///     - Returns the number of blocks passed since the reference block.
///
/// # Returns
/// The number of blocks left before the next mainnet era starts.
pub fn blocks_left_for_next_mainnet_era() -> u64 {
    let eras_passed = eras_passed_since_init_block(LAST_MAINNET_ACTIVE_ERA_INIT_BLOCK);
    let blocks_passed = blocks_passed_since_init_block(LAST_MAINNET_ACTIVE_ERA_INIT_BLOCK);

    if eras_passed >= 1 {
        let temp = blocks_passed % ONE_ERA_IN_BLOCKS;
        let blocks_left = ONE_ERA_IN_BLOCKS.saturating_sub(temp);
        blocks_left
    } else {
        blocks_passed
    }
}

/// Calculates how many full eras have passed since a given initial block.
///
/// # Parameters
/// - `era_init_block`: The reference block at which a known era started.
///
/// # Returns
/// Number of complete eras that have passed since the given block.
pub fn eras_passed_since_init_block(era_init_block: u64) -> u64 {
    let blocks_passed = blocks_passed_since_init_block(era_init_block);
    let eras_passed = blocks_passed.saturating_div(ONE_ERA_IN_BLOCKS);
    eras_passed
}


/// Calculates how many blocks have passed since a given initial block.
///
/// # Parameters
/// - `era_init_block`: The block number to use as reference.
///
/// # Returns
/// Number of blocks that have passed since the reference block.
pub fn blocks_passed_since_init_block(era_init_block: u64) -> u64 {
    let block_height = exec::block_height() as u64;
    let blocks_passed = block_height.saturating_sub(era_init_block);
    blocks_passed
}