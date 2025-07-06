# 🎯 Contract Utils

- Utils: provides a constant that is equal to a Vara within the smart contract, and two functions to throw an error when calculating the gas, thus saving on gas fees
- Macros: A collection of reusable, ergonomic Rust macros for sending messages in [`Vara Network`](https://wiki.vara.network/docs/welcome) smart contracts using the [`gstd`](https://docs.rs/gstd/latest/gstd/index.html) crate.

This crate provides flexible helpers to:
- Send simple messages.
- Send messages and await typed responses.
- Schedule delayed messages using reservations or gas-based delay.
- Provides functions to mention users who failed internally (when panicking)

## 📦 Installation

In your `Cargo.toml`, add:

```toml
[dependencies]
gear-contract-utils = { git = "https://github.com/Vara-Lab/Gear-Contract-Utils" }
```

Then in your crate:

```rust
use gear_contract_utils::*;
```

Or:

```rust
use gear_contract_utils::{
    send_delayed_msg, 
    send_msg, 
    send_msg_for_reply, 
    utils
};
```

---

## 🔧 Macros Overview

### `send_msg!`

Sends an encoded message without expecting a reply.

```rust
let result = send_msg!(
    actor_id,                   // Contract address
    "MyService",                // Contract service name
    "do_something",             // Contract service method name
    ("param1".to_string(), 42), // payload (optional)
    utils::ONE_TOKEN * 10       // Tokens to send (optional)
);
```

Supports:
- Minimal form: `send_msg!(actor, "Service", "Method")`
- With payload and default value: `send_msg!(actor, "Service", "Method", String::from("Hello"))`
- With payload and value: `send_msg!(actor, "Service", "Method", String::from("Hello"), utils::ONE_TOKEN * 2)`
- Defaults: payload as `()` and value as `0`

---

### `send_msg_for_reply!`

Sends an encoded message and awaits a typed reply from the destination program.

```rust
let result: Result<MyResponse, _> = send_msg_for_reply!(
    actor_id,            // Contract address
    "MyService",         // Contract service name 
    "compute_result",    // Contract service method name
    MyResponseType,      // Return type
    ("data".to_sitring(), 99),        // Payload (optional)
    utils::ONE_TOKEN * 5 // Tokens to send (optional)
);
```

Supports:
- Minimal form: `send_msg_for_reply!(actor, "Service", "Method", ReturnType)`
- With payload and default value: `send_msg_for_reply!(actor, "Service", "Method", ReturnType, 128u128)`
- With payload and value: `send_msg_for_reply!(actor, "Service", "Method", ReturnType, 128u128, utils::ONE_TOKEN * 5)`
- Defaults: payload as `()` and value as `0`

---

### `send_delayed_msg!`

Schedules a message to be sent in the future, optionally using a `ReservationId`.

```rust
let result = send_delayed_msg!(
    actor_id,                   // Contract address
    "MyService",                // Contract service name
    "do_later",                 // Contract service method name
    200_000,                    // explicit gas limit (if reservation id is provided, this will be ignored, set to 0)
    10,                         // amount of blocks to wait
    (),                         // Payload (optional)
    0,                          // Tokens to send (Optional)
    Some(reservation_id)        // Sending from reservation (needs to be wrapped in `Some`)
);
```

Supports:
- Full reservation form
- Without reservation
- Defaults: payload as `()` and value as `0`

---

## 🧩 Utility Functions

The crate also includes general-purpose helpers used across Gear smart contracts:

### `panicking`

Executes a closure that returns a `Result`, and panics if it returns `Err`.

```rust
let value = panicking(|| Ok("hello")); // returns "hello"
let value = panicking(|| Err("fail")); // panics
```

---

### `panic`

Wraps any `Debug`-able error into a formatted string and triggers `ext::panic`.

```rust
panic("Something went terribly wrong");
panic(my_error_struct);
```

Used to cleanly and safely terminate contract execution with an error message.
