#[cfg(test)]
mod tests;

use std::io;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::{FieldType, Header, PduType, HEADER_SIZE, UNUSED_U8};
use crate::{rdp::vc::ChannelError, PduParsing};

pub const DVC_CREATION_STATUS_OK: u32 = 0x0000_0000;
const DVC_CREATION_STATUS_SIZE: usize = 4;

#[derive(Debug, Clone, PartialEq)]
pub struct CreateRequestPdu {
    pub channel_id_type: FieldType,
    pub channel_id: u32,
    pub channel_name: String,
}

impl CreateRequestPdu {
    pub fn from_buffer(
        mut stream: impl io::Read,
        channel_id_type: FieldType,
    ) -> Result<Self, ChannelError> {
        let channel_id = channel_id_type.read_buffer_according_to_type(&mut stream)?;

        let mut buffer = String::new();
        stream.read_to_string(&mut buffer)?;

        let channel_name = buffer.trim_end_matches('\0').into();

        Ok(Self {
            channel_id_type,
            channel_id,
            channel_name,
        })
    }

    pub fn to_buffer(&self, mut stream: impl io::Write) -> Result<(), ChannelError> {
        let dvc_header = Header {
            channel_id_type: self.channel_id_type as u8,
            pdu_dependent: UNUSED_U8, // because DYNVC_CAPS_VERSION1
            pdu_type: PduType::Create,
        };
        dvc_header.to_buffer(&mut stream)?;
        self.channel_id_type
            .to_buffer_according_to_type(&mut stream, self.channel_id)?;
        stream.write_all(self.channel_name.as_ref())?;
        stream.write_all(b"\0")?;

        Ok(())
    }

    pub fn buffer_length(&self) -> usize {
        HEADER_SIZE + self.channel_id_type.get_type_size() + self.channel_name.len() + "\0".len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateResponsePdu {
    pub channel_id_type: FieldType,
    pub channel_id: u32,
    pub creation_status: u32,
}

impl CreateResponsePdu {
    pub fn from_buffer(
        mut stream: impl io::Read,
        channel_id_type: FieldType,
    ) -> Result<Self, ChannelError> {
        let channel_id = channel_id_type.read_buffer_according_to_type(&mut stream)?;
        let creation_status = stream.read_u32::<LittleEndian>()?;

        Ok(Self {
            channel_id_type,
            channel_id,
            creation_status,
        })
    }

    pub fn to_buffer(&self, mut stream: impl io::Write) -> Result<(), ChannelError> {
        let dvc_header = Header {
            channel_id_type: self.channel_id_type as u8,
            pdu_dependent: UNUSED_U8,
            pdu_type: PduType::Create,
        };
        dvc_header.to_buffer(&mut stream)?;
        self.channel_id_type
            .to_buffer_according_to_type(&mut stream, self.channel_id)?;
        stream.write_u32::<LittleEndian>(self.creation_status)?;

        Ok(())
    }

    pub fn buffer_length(&self) -> usize {
        HEADER_SIZE + self.channel_id_type.get_type_size() + DVC_CREATION_STATUS_SIZE
    }
}
