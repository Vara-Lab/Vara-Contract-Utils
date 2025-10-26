/// Total number of blocks required to complete the unbonding process.
///
/// In Vara Network, each block takes ≈3 seconds to produce.
/// Therefore:
///
/// 201_600 blocks × 3 = 604_800 seconds = 168 hours = 7 days
/// 
/// You can add more blocks (100 or more) to avoid a margin of error in the unstaking time.
pub const TOTAL_BLOCKS_TO_UNBOND: u64 = 201_600;

/// Unbonding duration in milliseconds
///  
/// Calculating the time in milliseconds within the contract is more accurate than in 
/// using block "time", since this way you can know if in the current block
pub const TOTAL_MILISECONDS_TO_UBOND: u64 = 604_800_000;

/// Number of blocks that make up a single era.
///
/// Since each block takes ~3 seconds, this corresponds to:
///
/// 14_400 blocks × 3 = 43_200 seconds = 12 hours per era
pub const ONE_ERA_IN_BLOCKS: u64 = 14_400;

/// Milliseconds of one era
pub const ONE_ERA_IN_MILLISECONDS: u64 = 43_200_000;

/// Last active era registered on the testnet used as a reference point.
///
/// Useful for estimating the current era based on block height,
/// especially in offline environments or during network analysis.
pub const LAST_TESTNET_ACTIVE_ERA_REGISTERED: u64 = 1526;

/// Block number at which the `LAST_TESTNET_ACTIVE_ERA_REGISTERED` started on the testnet.
///
/// Note: this value may be approximately ~10 seconds later than the actual start of the era
/// due to timing offsets or recording delay.
pub const LAST_TESTNET_ACTIVE_ERA_INIT_BLOCK: u64 = 21_804_388;

/// Timestamp at which the `LAST_TESTNET_ACTIVE_ERA_REGISTERED` started on the testnet.
/// 
/// Note: This value may be approximately ~10 seconds later then the actual start of the era
/// due to timing offsets or recording delay (handle it in milliseconds is mor accurate than in blocks)
pub const LAST_TESTNET_ACTIVE_ERA_INIT_TIMESTAMP: u64 = 1_761_246_986_000; 

/// Last mainnet active era registered on the mainnet used as a reference point.
///
/// This value currently matches the mainnet, but may diverge as networks evolve.
pub const LAST_MAINNET_ACTIVE_ERA_REGISTERED: u64 = 1669;

/// Block number at which the `LAST_MAINNET_ACTIVE_ERA_REGISTERED` started on the mainnet.
///
/// Note: this may be offset by approximately ~10 seconds from the true start of the era.
pub const LAST_MAINNET_ACTIVE_ERA_INIT_BLOCK: u64 = 27_161_113;

/// Timestamp at which the `LAST_MAINNET_ACTIVE_ERA_REGISTERED` started on the mainnet.
/// 
/// Note: This value may be approximately ~10 seconds later then the actual start of the era
/// due to timing offsets or recording delay (handle it in milliseconds is mor accurate than in blocks)
pub const LAST_MAINNET_ACTIVE_ERA_INIT_TIMESTAMP: u64 = 1_761_264_579_001;

/// A constant representing one whole token in Vara Network (1 token = 10¹² units).
///
/// This is a convenience constant for specifying token values in smart contracts
///
/// ## Example
/// ```
/// let reward = 5 * ONE_TOKEN; // equivalent to 5_000_000_000_000 or 5 Varas
/// ```
pub const ONE_TOKEN: u128 = 1e12 as u128;

