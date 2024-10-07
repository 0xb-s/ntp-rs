use crate::error::NtpError;
use crate::packet::{LiVnMode, NtpPacket};

pub fn handle_request(buf: &[u8]) -> Result<NtpPacket, NtpError> {
    if buf.len() < 48 {
        return Err(NtpError::InvalidResponse);
    }

    let received_packet = NtpPacket::from_bytes(buf)?;

    let mut response = NtpPacket::new();
    let (li, vn, _) = received_packet.li_vn_mode.components();
    response.li_vn_mode = LiVnMode::new(li, vn, 4);
    response.stratum = 2;
    response.ref_id = generate_ref_id();
    response.ref_timestamp = received_packet.tx_timestamp;
    response.orig_timestamp = received_packet.tx_timestamp;
    response.recv_timestamp = current_ntp_time();
    response.tx_timestamp = current_ntp_time();

    Ok(response)
}

fn generate_ref_id() -> u32 {
    0x4E545031 // "NTP1" in ASCII
}

fn current_ntp_time() -> u64 {
    let unix_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    (unix_time + 2_208_988_800) << 32
}
