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
vara-contract-utils = { git = "https://github.com/Vara-Lab/Vara-Contract-Utils" }
```

Then in your crate:

```rust
use vara_contract_utils::*;
```

Or:

```rust
use vara_contract_utils::{
    send_delayed_msg, // macro
    send_msg,  // macro
    send_msg_for_reply, // macro 
    utils, // utils
    funcs // util functions
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

## 🏗️ Builder

You can use a builder that is in the crate, that is the `MessageBuilder`, it will help you to send a 
message to a user or contract.

To get this builder you have to "import" the function `new_message` from the `funcs` module:

```rust
use vara_contract_utils::funcs::new_message; // func to get the MessageBuilder
```

The builder contains the next methods:

- `send_to`: Set the address to send the message
- `blocks_to_send_delayed_message`: Set the blocks to wait until send the delayed message, it only works if
  you call the `send_delayed` and `send_delayed_with_reservation`, It's optional, by default is 0.
- `service_name`: Service to call from a sails contract, this method is optional, but if you set the service name, you have to 
  set the method name.
- `method_name`: Method to call from a service sails contract, this method is optional, but if you set this method, you have
  to set the service name to call.
- `add_arg`: Set an argument to send in the message, you can call this method as many times as you want. This method is optional,
  if you dont call this method, it will send () by default.
- `with_value`: Set the value that you'll send in the message, by defaul is 0. This method is optional.
- `send`: This method will send the message with all the values that you set previosly.
- `send_delayed`: This method will send the message with the feature that it will wait until the blocks that you set pass.
- `send_delayed_with_reservation`: same as send_delayed, but it will take a gas_reservation_id to send the delayed message.

Then, in your contract, you can use this builder to send a message to a user or a contract, examples:

- Send a message to a user:

```rust
// Send a message to the user who calls the contract
let source = Syscall::message_source();

// If you no specify the payload, it will send () instead
let result = new_message() // get Ok(MessageId) or Err(gstd::errors::Error)
    .send_to(source) // Set the address to send the message
    .with_value(3 * utils::ONE_TOKEN) // Send 3 VARAS to the user
    .send(); // Send the message
```

- Send a message to a sails contract:

```rust
let contract_address = "0xab43...";

let result = new_message() // get Ok(MessageId) or Err(gstd::errors::Error)
    .send_to(contract_address) // Set the contract to send the message
    .service_name("ServiceName") // Set the service name to call 
    .method_name("MethodName") // Set the method name to call
    .add_arg(102u64) // Set an argument
    .add_arg(String::from("Hello!")) // You can set one or more arguments
    .with_value(5 * utils::ONE_TOKEN) // You can set the value to send
    .send(); // Send the message
```

- Send a message and wait for response, you can use this method (`send_recv`) 
  to read state (query message) from contracts

```rust
let contract_address = "0xab43...";

let result = new_message() // get Ok(ContractResponseType) or Err(gstd::errors::Error)
    .send_to(contract_address) // Set the contract to send the message
    .service_name("ServiceName") // Set the service name to call 
    .method_name("MethodName") // Set the method name to call
    .add_arg(String::from("Hello!")) // You can set one or more arguments
    .send_recv::<String>() // Set the value to receive with turbofish
    .await; // await for response
```

- Send a delayed message

```rust
let contract_address = "0xab43...";

let result = new_message() // get Ok(MessageId) or Err(gstd::errors::Error)
    .send_to(contract_address) // Set the contract to send the message
    .service_name("ServiceName") // Set the service name to call 
    .method_name("MethodName") // Set the method name to call
    .send_delayed(); // Send the delayed message
```

- Send a delayed message with reservation id

```rust
let contract_address = "0xab43...";
let reservation_id = ReservationId::zero(); // example

let result = new_message() // get Ok(MessageId) or Err(gstd::errors::Error)
    .send_to(contract_address) // Set the contract to send the message
    .service_name("ServiceName") // Set the service name to call 
    .method_name("MethodName") // Set the method name to call
    .add_arg(String::from("Message")) // Set the arg to send in the message
    .with_value(5 * utils::ONE_TOKEN) // You can set the value to send
    .send_delayed_with_reservation(reservation_id); // Send the delayed message with reservation_id
```

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

### `testnet_active_era`

Returns the current testnet active era.

```rust
let current_era = testnet_active_era();
```

### `mainnet_active_era`

Returns the current mainnet active era.

```rust
let current_era = mainnet_active_era();
```

### `blocks_left_for_next_testnet_era`

Returns the time remaining until the current testnet active era ends, in blocks.

```rust
let blocks_left = blocks_left_for_next_testnet_era();
```

### `blocks_left_for_next_mainnet_era`

Returns the time remaining until the current mainnet active era ends, in blocks.

```rust
let blocks_left = blocks_left_for_next_mainnet_era();
```

### `eras_passed_since_init_block`

Returns how many eras have passed since the initial block set for the first calculated given era.

```rust
let eras_passed = eras_passed_since_init_block();
```

### `blocks_passed_since_init_block`

Returns how many blocks have passed since the last estimated block with the era change active

```rust
let blocks_passed = blocks_passed_since_init_block();
```
