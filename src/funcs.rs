use core::fmt::Debug;
use gstd::{exec, ext, format};
use crate::{
    builders::message::MessageBuilder,
    consts::*
};

/// # Create a new MessageBuilder
pub fn new_message() -> MessageBuilder {
    MessageBuilder::new()
}

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
/// The estimated active era number on testnet (relative to the reference era).
///
pub fn testnet_active_era_since_last_block_registered() -> u64 {
    let eras_passed = eras_passed_since_init_block(LAST_TESTNET_ACTIVE_ERA_INIT_BLOCK);

    LAST_TESTNET_ACTIVE_ERA_REGISTERED.saturating_add(eras_passed)
}

/// Estimates the current active era on the **mainnet** based on the local block height.
/// 
/// # Returns
/// The estimated active era index on mainnet (relative to the reference era)
///
pub fn mainnet_active_era_since_last_timestamp_registered() -> u64 {
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
/// ### Returns
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

/// Calculates how may milliseconds remain until the next era begins on the **mainnet**
/// 
/// ### Returns
/// The number of milliseconds left before the next mainnet era starts
pub fn ms_left_for_next_mainnet_era() -> u64 {
    let eras_passed = eras_passed_since_init_timestamp(LAST_MAINNET_ACTIVE_ERA_INIT_TIMESTAMP);
    let ms_passed = ms_passed_since_init_timestamp(LAST_MAINNET_ACTIVE_ERA_INIT_TIMESTAMP);

    if eras_passed >= 1 {
        let temp = ms_passed % ONE_ERA_IN_MILLISECONDS;
        let ms_left = ONE_ERA_IN_MILLISECONDS.saturating_sub(temp);

        ms_left
    } else {
        ms_passed
    }
}

/// Calculates how many full eras have passed since a given initial block.
///
/// ### Parameters
/// - `era_init_block`: The reference block at which a known era started.
///
/// ### Returns
/// Number of complete eras that have passed since the given block.
pub fn eras_passed_since_init_block(era_init_block: u64) -> u64 {
    let blocks_passed = blocks_passed_since_init_block(era_init_block);
    let eras_passed = blocks_passed.saturating_div(ONE_ERA_IN_BLOCKS);
    eras_passed
}

/// Calculate how many full eras have passed since a given initial timestamp
/// 
/// ### Parameters
/// `era_init_timestamp`: The referece timestamp at which a known era started
pub fn eras_passed_since_init_timestamp(era_init_timestamp: u64) -> u64 {
    let ms_passed = ms_passed_since_init_timestamp(era_init_timestamp);
    let eras_passed = ms_passed.saturating_div(ONE_ERA_IN_MILLISECONDS);

    eras_passed
}

/// Calculates how many blocks have passed since a given initial block.
///
/// ### Parameters
/// - `era_init_block`: The block number to use as reference.
///
/// ### Returns
/// Number of blocks that have passed since the reference block.
pub fn blocks_passed_since_init_block(era_init_block: u64) -> u64 {
    let block_height = exec::block_height() as u64;
    let blocks_passed = block_height
        .checked_sub(era_init_block)
        .unwrap_or(0);

    blocks_passed
}

/// Calculate how many milliseconds have passed since a given initial timestamp
/// 
/// ### Parameters
/// - `era_init_timestamp`: The timestamp to use as a referene
/// 
/// ### Returns
/// Milliseconds that have passed since the reference timestamp
pub fn ms_passed_since_init_timestamp(era_init_timestamp: u64) -> u64 {
    let timestamp = exec::block_timestamp();
    let ms_passed = timestamp
        .checked_sub(era_init_timestamp)
        .unwrap_or(0);

    ms_passed
}