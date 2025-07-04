/// Sends an encoded message to a smart contract (program) and waits for a reply.
/// 
/// # Parameters
///
/// - `$program_id`: Address of the destination actor (typically `ActorId`).
/// - `$service_name`: A `&'static str` literal indicating the service name.
/// - `$method_name`: A `&'static str` literal indicating the method to call.
/// - `$return_type`: The expected type to return (must match the target smart contract response type).
/// - `$payload`: *(optional)* An encodable value to send as payload (defaults to `()`).
/// - `$value`: *(optional)* Number of tokens to attach (defaults to `0`).
///
/// # Usage
///
/// This macro supports three forms:
///
/// 1. **Full usage**:
/// ```rust
/// let result = send_msg_for_reply!(
///     program_id,
///     "UserService",
///     "GetUser",
///     UserDetails,
///     ("juan".to_string(), 123u32),
///     utils::ONE_TOKEN * 10
/// );
/// ```
///
/// 2. **Without value (defaults to 0)**:
/// ```rust
/// let result = send_msg_for_reply!(
///     program_id,
///     "UserService",
///     "GetUser",
///     UserDetails,
///     ("juan", 123u32)
/// );
/// ```
///
/// 3. **Without payload or value (payload is `()`, value is `0`)**:
/// ```rust
/// let result = send_msg_for_reply!(
///     program_id,
///     "UserService",
///     "GetUser",
///     UserDetails
/// );
/// ```
///
/// # Return
///
/// `Result<$return_type, gstd::errors::Error>` - Result of the remote call.
///
/// # Example with full signature:
///
/// ```rust
/// let result: Result<MyOutput, Error> = send_msg_for_reply!(
///     program_id,
///     "TokenService",
///     "get_balance",
///     MyOutput,
///     user_id,
///     0
/// );
/// ```
///
#[macro_export]
macro_rules! send_msg_for_reply {
    (
        $program_id:expr,
        $service_name:literal,
        $method_name:literal,
        $return_type:ty,
        $payload:expr,
        $value:expr
    ) => {{
        use gstd::{msg, errors::Error};

        let request = [
            $service_name.encode(),
            $method_name.encode(),
            $payload.encode()
        ].concat();

        match msg::send_bytes_for_reply_as::<_, (String, String, $return_type)>($program_id, request, $value, 0) {
            Ok(call) => {
                let result = call.await;
                result.map(|res| res.2)
            },
            Err(e) => Err(Error::Core(e))
        }
    }};

    (
        $program_id:expr,
        $service_name:literal,
        $method_name:literal,
        $return_type:ty,
        $payload:expr
    ) => {
        $crate::send_msg_for_reply!($program_id, $service_name, $method_name, $return_type, $payload, 0)
    };

    (
        $program_id:expr,
        $service_name:literal,
        $method_name:literal,
        $return_type:ty
    ) => {
        $crate::send_msg_for_reply!($program_id, $service_name, $method_name, $return_type, (), 0)
    };
}