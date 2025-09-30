use sails_rs::{
    prelude::*
};
use gstd::{
    msg,
    MessageId,
    errors::Error,
    ReservationId
};
use crate::utils;

pub struct MessageBuilder {
    to: Option<ActorId>,
    service_name: Option<&'static str>,
    method_name: Option<&'static str>,
    payload: Option<Vec<u8>>,
    delayed_duration: u32,
    value: u128,
}

impl MessageBuilder {
    pub fn new() -> Self {
        Self {
            to: None,
            service_name: None,
            method_name: None,
            payload: None,
            delayed_duration: 0,
            value: 0
        }
    }

    /// ## Set the address to send the message
    pub fn send_to(mut self, to: ActorId) -> Self {
        self.to = Some(to);

        self
    }

    /// ## Set the time in blocks to send the message
    pub fn blocks_to_send_delayed_message(mut self, blocks: u32) -> Self {
        self.delayed_duration = blocks;

        self
    }

    /// ## Set the service name to call
    /// If you set the service name, you have to set the method name too, because it will send the 
    /// message to a sails contract
    pub fn service_name(mut self, service_name: &'static str) -> Self {
        self.service_name = Some(service_name);

        self
    }

    /// ## Set the method name to call
    /// If you set the method name, you have to set the service name too, because it will send the
    /// message to a sails contract
    pub fn method_name(mut self, method_name: &'static str) -> Self {
        self.method_name = Some(method_name);

        self
    }

    /// ## Add arguments to the message
    /// You can call this method all the times you want, it will store and sent all the arguments in one unified
    /// payload (for example, to a contract, etc)
    /// The order you call this function, will be the same order that the message will be send: 
    /// 
    /// ```rust
    ///     // other method calls
    ///     .add_arg(68u64)
    ///     .add_arg(String::from("Hello"))
    ///     .add_arg(EnumType::Variant)
    ///     // other method calls
    /// ```
    /// 
    /// The example will send (68u64, String::from("Hello"), EnumType::Variant), all arguments that you'll send need
    /// to derive the trait `Encode`
    pub fn add_arg(mut self, arg: impl Encode) -> Self {
        let payload_vec = self
            .payload
            .get_or_insert(vec![]);

        arg.encode_to(payload_vec);

        self
    }

    /// ## Value to send in the message
    /// By default is zero, but you can set the number of Tokens to send in the message
    pub fn with_value(mut self, value: u128) -> Self {
        self.value = value;

        self
    }

    /// ## Method to send the message
    pub fn send(mut self) -> Result<MessageId, Error> {
        self.check_data();

        let request = self.get_request();

        msg::send_bytes(self.to.unwrap(), request, self.value)
            .map_err(|error| Error::Core(error))
    }

    /// ## Send a delayed message
    pub fn send_delayed(mut self) -> Result<MessageId, Error> {
        self.check_data();

        let request = self.get_request();

        let message_id = msg::send_bytes_delayed(
            self.to.unwrap(), 
            request, 
            self.value, 
            self.delayed_duration
        )?;

        Ok(message_id)
    }

    /// ## Send a delayed message with reservation
    pub fn send_delayed_with_reservation(mut self, reservation_id: ReservationId) -> Result<MessageId, Error> {
        self.check_data();
        let request = self.get_request();

        let result = msg::send_bytes_delayed_from_reservation(
            reservation_id, 
            self.to.unwrap(), 
            request, 
            self.value, 
            self.delayed_duration
        )?;

        Ok(result)
    }

    /// ## Send a message and waits for the response
    /// This method will send the message to the destination, you have to set the type of the response, because
    /// it will decode the response and return it tou you.
    pub async fn send_recv<R: Decode>(mut self) -> Result<R, Error> {
        self.check_data();

        let request = self.get_request();

        let call = msg::send_bytes_for_reply_as::<_, (String, String, R)>(
            self.to.unwrap(), 
            request, 
            self.value, 
            0
        )?;

        let call_result = call
            .await?;

        Ok(call_result.2)
    }

    fn check_data(&self) {
        if self.to.is_none() {
            utils::panic("Address to send message cant be empty");
        }

        let sails_check_1 = self.service_name.is_some() && self.method_name.is_none();
        let sails_check_2 = self.service_name.is_none() && self.method_name.is_some();

        if sails_check_1 || sails_check_2 {
            utils::panic("To send a message to a contract, set the service and method name");
        }
    }

    fn get_request(&mut self) -> Vec<u8> {

        
        let payload = if self.payload.is_none() {
            self.payload.take().unwrap()
        } else {
            ().encode()
        };

        let contract_payload = self.service_name.is_some() && self.method_name.is_some();

        let request = if contract_payload {
            [
                self.service_name.unwrap().encode(),
                self.method_name.unwrap().encode(),
                payload
            ].concat()
        } else {
            payload
        };

        request
    }
}
