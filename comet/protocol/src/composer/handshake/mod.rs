
use buffer::Buffer;

pub fn policy_file() -> Buffer {
    Buffer::from_bytes(b"<?xml version=\"1.0\"?>\r\n<!DOCTYPE cross-domain-policy \\ SYSTEM \"/xml/dtds/cross-domain-policy.dtd\">\r\n<cross-domain-policy>\r\n<allow-access-from domain=\"*\" to-ports=\"*\" />\r\n</cross-domain-policy>\0")
}

pub fn motd_composer(motd: String) -> Buffer {
    let mut buf = Buffer::empty(408);

    buf.write_i32(1);
    buf.write_string(motd);

    buf
}

pub fn auth_ok_composer() -> Buffer {
    Buffer::empty(3054)
}