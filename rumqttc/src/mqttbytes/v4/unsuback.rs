use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::mqttbytes::{FixedHeader, MqttError, read_u16};

/// Acknowledgement to unsubscribe
#[derive(Debug, Clone, PartialEq)]
pub struct UnsubAck {
    pub pkid: u16,
}

impl UnsubAck {
    pub fn new(pkid: u16) -> UnsubAck {
        UnsubAck { pkid }
    }

    pub fn read(fixed_header: FixedHeader, mut bytes: Bytes) -> Result<Self, MqttError> {
        if fixed_header.remaining_len != 2 {
            return Err(MqttError::PayloadSizeIncorrect);
        }

        let variable_header_index = fixed_header.fixed_header_len;
        bytes.advance(variable_header_index);
        let pkid = read_u16(&mut bytes)?;
        let unsuback = UnsubAck { pkid };

        Ok(unsuback)
    }

    pub fn write(&self, payload: &mut BytesMut) -> Result<usize, MqttError> {
        payload.put_slice(&[0xB0, 0x02]);
        payload.put_u16(self.pkid);
        Ok(4)
    }
}