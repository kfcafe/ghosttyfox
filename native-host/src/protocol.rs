use std::io::{self, Read, Write};

use serde_json::Value;

pub fn read_message(reader: &mut impl Read) -> io::Result<Value> {
    let mut length_bytes = [0_u8; 4];
    reader.read_exact(&mut length_bytes)?;

    let length = u32::from_le_bytes(length_bytes) as usize;
    let mut payload = vec![0_u8; length];
    reader.read_exact(&mut payload)?;

    serde_json::from_slice(&payload)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

pub fn write_message(writer: &mut impl Write, msg: &Value) -> io::Result<()> {
    let payload = serde_json::to_vec(msg)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let length = u32::try_from(payload.len())
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "message too large"))?;

    writer.write_all(&length.to_le_bytes())?;
    writer.write_all(&payload)?;
    writer.flush()
}
