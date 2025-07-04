/// Sends a delayed message to a remote program, with optional reservation and gas customization.
///
/// # Parameters
///
/// - `$program_id`: Address of the destination program (actor).
/// - `$service_name`: A string expression (e.g., `"MyService"`).
/// - `$method_name`: A string expression (e.g., `"do_task"`).
/// - `$with_gas`: A `u64` value to specify the gas limit.
/// - `$blocks`: A `u32` indicating the number of blocks after which the message will be executed.
/// - `$payload`: *(optional)* Payload data (must implement `Encode`). Defaults to `()`.
/// - `$value`: *(optional)* Number of tokens to send with the message. Defaults to `0`.
/// - `$id_opt`: *(optional)* `Option<ReservationId>`, used for message reservation. Defaults to `None`.
///
/// # Usage
///
/// This macro supports four usage forms:
///
/// 1. **Explicit `ReservationId` usage:**
/// ```rust
/// let result = send_delayed_msg!(
///     program_id,
///     "TaskService",
///     "schedule",
///     500_000,              // with_gas
///     10,                   // blocks
///     my_payload,
///     0,                    // value
///     Some(reservation_id) // Option<ReservationId>
/// );
/// ```
///
/// 2. **With payload and value, no reservation:**
/// ```rust
/// let result = send_delayed_msg!(
///     program_id,
///     "TaskService",
///     "schedule",
///     500_000,
///     10,
///     my_payload,
///     0
/// );
/// ```
///
/// 3. **With payload, value is default (`0`):**
/// ```rust
/// let result = send_delayed_msg!(
///     program_id,
///     "TaskService",
///     "schedule",
///     500_000,
///     10,
///     my_payload
/// );
/// ```
///
/// 4. **Minimal usage (payload is `()`, value is `0`):**
/// ```rust
/// let result = send_delayed_msg!(
///     program_id,
///     "TaskService",
///     "schedule",
///     500_000,
///     10
/// );
/// ```
///
/// # Return
///
/// Returns `Result<(), gstd::errors::Error>`, propagating any error during message scheduling.
///
/// # Notes
///
/// - If you pass a gas reservation ID, you should always use Some()
///
/// # Example
///
/// ```rust
/// use gstd::ReservationId;
///
/// let reservation_id = ReservationId::reserve(RESERVATION_AMOUNT, TIME)
///     .expect("Reservation across executions");
/// let send_result = send_delayed_msg!(
///     actor_id,
///     "WorkerService",
///     "RunDelayed",
///     10_000_000_000,
///     5,
///     ("payload".to_string(), 42u32),
///     utils::ONE_TOKEN * 10,
///     Some(reservation_id)
/// );
/// ```

#[macro_export]
macro_rules! send_delayed_msg {
    (
        $program_id:expr,
        $service_name:expr,
        $method_name:expr,
        $with_gas:expr,
        $blocks:expr,
        $payload:expr,
        $value:expr,
        $id_opt:expr
    ) => {{
        use gstd::{msg, errors::Error, ReservationId};

        let request = [
            $service_name.encode(),
            $method_name.encode(),
            $payload.encode()
        ].concat();

        let result = if let Some(reservation_id) = $id_opt {
            msg::send_bytes_delayed_from_reservation(reservation_id, $program_id, request, $value, $blocks)
        } else {
            msg::send_bytes_with_gas_delayed($program_id, request, $with_gas, $value, $blocks)
        };

        result.map_err(|e| Error::Core(e))
    }};

    (
        $program_id:expr,
        $service_name:expr,
        $method_name:expr,
        $with_gas:expr,
        $blocks:expr,
        $payload:expr,
        $value:expr
    ) => {
        $crate::send_delayed_msg!($program_id, $service_name, $method_name, $with_gas, $blocks, $payload, $value, None::<ReservationId>)
    };

    (
        $program_id:expr,
        $service_name:expr,
        $method_name:expr,
        $with_gas:expr,
        $blocks:expr,
        $payload:expr
    ) => {
        $crate::send_delayed_msg!($program_id, $service_name, $method_name, $with_gas, $blocks, $payload, 0, None::<ReservationId>)
    };

    (
        $program_id:expr,
        $service_name:expr,
        $method_name:expr,
        $with_gas:expr,
        $blocks:expr
    ) => {
        $crate::send_delayed_msg!($program_id, $service_name, $method_name, $with_gas, $blocks, (), 0, None::<ReservationId>)
    };
}
