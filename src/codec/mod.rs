mod command;
mod error;
mod framed;
mod greeting;
mod mechanism;
mod zmq_codec;

pub(crate) use command::{ZmqCommand, ZmqCommandName};
pub(crate) use error::{CodecError, CodecResult};
pub(crate) use framed::FramedIo;
pub(crate) use greeting::ZmqGreeting;
pub(crate) use zmq_codec::ZmqCodec;

use crate::message::ZmqMessage;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Greeting(ZmqGreeting),
    Command(ZmqCommand),
    Message(ZmqMessage),
    Multipart(Vec<ZmqMessage>),
}
