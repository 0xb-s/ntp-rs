use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::error::NtpError;

pub type LeapIndicator = u8; // 2 bits
pub type VersionNumber = u8; // 3 bits
pub type Mode = u8; // 3 bits
pub type Stratum = u8;
pub type Poll = i8;
pub type Precision = i8;
pub type RootDelay = u32;
pub type RootDispersion = u32;
pub type ReferenceID = u32;
pub type Timestamp = u64;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct LiVnMode(u8);

impl LiVnMode {
    pub fn new(li: LeapIndicator, vn: VersionNumber, mode: Mode) -> Self {
        let value = ((li & 0x3) << 6) | ((vn & 0x7) << 3) | (mode & 0x7);
        LiVnMode(value)
    }

    pub fn to_byte(&self) -> u8 {
        self.0
    }

    pub fn components(&self) -> (LeapIndicator, VersionNumber, Mode) {
        let li = (self.0 >> 6) & 0x3;
        let vn = (self.0 >> 3) & 0x7;
        let mode = self.0 & 0x7;
        (li, vn, mode)
    }
}

impl TryFrom<u8> for LiVnMode {
    type Error = NtpError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(LiVnMode(value))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct NtpPacket {
    pub li_vn_mode: LiVnMode,
    pub stratum: Stratum,
    pub poll: Poll,
    pub precision: Precision,
    pub root_delay: RootDelay,
    pub root_dispersion: RootDispersion,
    pub ref_id: ReferenceID,
    pub ref_timestamp: Timestamp,
    pub orig_timestamp: Timestamp,
    pub recv_timestamp: Timestamp,
    pub tx_timestamp: Timestamp,
}

impl NtpPacket {
    pub fn new() -> Self {
        NtpPacket {
            li_vn_mode: LiVnMode::new(0, 4, 3),
            stratum: 0,
            poll: 4,
            precision: -6,
            root_delay: 0,
            root_dispersion: 0,
            ref_id: 0,
            ref_timestamp: 0,
            orig_timestamp: 0,
            recv_timestamp: 0,
            tx_timestamp: current_ntp_time(),
        }
    }

    pub fn to_bytes(&self) -> [u8; 48] {
        let mut buf = [0u8; 48];
        buf[0] = self.li_vn_mode.to_byte();
        buf[1] = self.stratum;
        buf[2] = self.poll as u8;
        buf[3] = self.precision as u8;
        BigEndian::write_u32(&mut buf[4..8], self.root_delay);
        BigEndian::write_u32(&mut buf[8..12], self.root_dispersion);
        BigEndian::write_u32(&mut buf[12..16], self.ref_id);
        BigEndian::write_u64(&mut buf[16..24], self.ref_timestamp);
        BigEndian::write_u64(&mut buf[24..32], self.orig_timestamp);
        BigEndian::write_u64(&mut buf[32..40], self.recv_timestamp);
        BigEndian::write_u64(&mut buf[40..48], self.tx_timestamp);
        buf
    }

    pub fn from_bytes(buf: &[u8]) -> Result<Self, NtpError> {
        if buf.len() < 48 {
            return Err(NtpError::InvalidResponse);
        }
        Ok(NtpPacket {
            li_vn_mode: LiVnMode::try_from(buf[0])?,
            stratum: buf[1],
            poll: buf[2] as i8,
            precision: buf[3] as i8,
            root_delay: BigEndian::read_u32(&buf[4..8]),
            root_dispersion: BigEndian::read_u32(&buf[8..12]),
            ref_id: BigEndian::read_u32(&buf[12..16]),
            ref_timestamp: BigEndian::read_u64(&buf[16..24]),
            orig_timestamp: BigEndian::read_u64(&buf[24..32]),
            recv_timestamp: BigEndian::read_u64(&buf[32..40]),
            tx_timestamp: BigEndian::read_u64(&buf[40..48]),
        })
    }
}

fn current_ntp_time() -> u64 {
    let unix_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    (unix_time + 2_208_988_800) << 32
}
