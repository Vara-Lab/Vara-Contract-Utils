use core::fmt::Debug;
use gstd::{ext, format};

/// Total number of blocks required to complete the unbonding process.
///
/// In Vara Network, each block takes 3 seconds to produce.
/// Therefore:
///
/// 230_400 blocks × 3 = 691_200 seconds = 192 hours = 8 days
pub const TOTAL_BLOCKS_TO_UNBOND: u32 = 230_400;

/// Number of blocks that make up a single era.
///
/// Since each block takes ~3 seconds, this corresponds to:
///
/// 14_400 blocks × 3 = 43_200 seconds = 12 hours per era
pub const ONE_ERA_IN_BLOCKS: u32 = 14_400;

/// Last active era registered on the testnet used as a reference point.
///
/// Useful for estimating the current era based on block height,
/// especially in offline environments or during network analysis.
pub const LAST_TESTNET_ACTIVE_ERA_REGISTERED: u64 = 1326;

/// Block number at which the `LAST_TESTNET_ACTIVE_ERA_REGISTERED` started on the testnet.
///
/// Note: this value may be approximately 40 seconds later than the actual start of the era
/// due to timing offsets or recording delay.
pub const LAST_TESTNET_ACTIVE_ERA_INIT_BLOCK: u64 = 18_925_513; // ~40s late

/// Last active era registered on the mainnet used as a reference point.
///
/// This value currently matches the testnet, but may diverge as networks evolve.
pub const LAST_MAINNET_ACTIVE_ERA_REGISTERED: u64 = 1326;

/// Block number at which the `LAST_MAINNET_ACTIVE_ERA_REGISTERED` started on the mainnet.
///
/// Note: this may be offset by approximately 40 seconds from the true start of the era.
pub const LAST_MAINNET_ACTIVE_ERA_INIT_BLOCK: u64 = 18_925_513; // ~40s late

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