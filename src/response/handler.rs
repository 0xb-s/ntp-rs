use crate::error::NtpError;
use crate::packet::NtpPacket;

pub fn process_response(packet: &NtpPacket) -> Result<(), NtpError> {
    let originate_time = ntp_to_unix(packet.orig_timestamp);
    let receive_time = ntp_to_unix(packet.recv_timestamp);
    let transmit_time = ntp_to_unix(packet.tx_timestamp);

    println!("Received NTP response:");
    println!("Originate Timestamp: {}", originate_time);
    println!("Receive Timestamp: {}", receive_time);
    println!("Transmit Timestamp: {}", transmit_time);

    Ok(())
}

fn ntp_to_unix(ntp_time: u64) -> u64 {
    (ntp_time >> 32) - 2_208_988_800
}
