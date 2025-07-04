/// Sends an encoded message to a smart contract (program) without expecting a reply.
///
/// # Parameters
///
/// - `$program_id`: Address of the destination actor (`ActorId` or similar).
/// - `$service_name`: A string literal (`&'static str`) representing the service namespace.
/// - `$method_name`: A string literal (`&'static str`) representing the method to be called.
/// - `$payload`: *(optional)* The input value to encode and send (must implement `Encode`). Defaults to `()`.
/// - `$value`: *(optional)* Number of tokens to attach with the message. Defaults to `0`.
///
/// # Usage
///
/// This macro supports three usage forms:
///
/// 1. **Full usage**:
/// ```rust
/// let result = send_msg!(
///     program_id,
///     "ServiceName",
///     "MethodName",
///     ("argument_1".to_string, 10),
///     utils::ONE_TOKEN * 10
/// );
/// ```
///
/// 2. **Without value (defaults to 0)**:
/// ```rust
/// let result = send_msg!(
///     program_id,
///     "ServiceName",
///     "MethodName",
///     ("argument_1".to_string, 10),
/// );
/// ```
///
/// 3. **Without payload or value**:
/// ```rust
/// let result = send_msg!(
///     program_id,
///     "ServiceName",
///     "MethodName"
/// );
/// ```
///
/// # Return
///
/// Returns `Result<MessageId, gstd::errors::Error>`, where `MessageId` identifies the sent message.
///
/// # Example
///
/// ```rust
/// let send_result = send_msg!(
///     address,
///     "Logger",
///     "LogData",
///     "Adrian".to_string()
/// );
///
/// if let Err(e) = send_result {
///     gstd::msg::reply("Failed to send log", 0).ok();
/// }
/// ```
#[macro_export]
macro_rules! send_msg {
    (
        $program_id:expr,
        $service_name:literal,
        $method_name:literal,
        $payload:expr,
        $value:expr
    ) => {{
        use gstd::{msg, errors::Error};

        let request = [
            $service_name.encode(),
            $method_name.encode(),
            $payload.encode()
        ].concat();

        msg::send_bytes($program_id, request, $value)
            .map_err(|e| Error::Core(e))
    }};

    (
        $program_id:expr,
        $service_name:literal,
        $method_name:literal,
        $payload:expr
    ) => {
        $crate::send_msg!($program_id, $service_name, $method_name, $payload, 0)
    };

    (
        $program_id:expr,
        $service_name:literal,
        $method_name:literal
    ) => {
        $crate::send_msg!($program_id, $service_name, $method_name, (), 0)
    };
}