use buffer::Buffer;

pub fn policy_file() -> Buffer {
    Buffer::from_bytes(b"<?xml version=\"1.0\"?>\r\n<!DOCTYPE cross-domain-policy \\ SYSTEM \"/xml/dtds/cross-domain-policy.dtd\">\r\n<cross-domain-policy>\r\n<allow-access-from domain=\"*\" to-ports=\"*\" />\r\n</cross-domain-policy>\0")
}

pub fn motd_composer(motd: String) -> Buffer {
    Buffer::empty(408)
        .write_i32(1)
        .write_str(&motd)
}

pub fn auth_ok_composer() -> Buffer {
    Buffer::empty(3054)
}

pub fn availability_status_composer() -> Buffer {
    Buffer::empty(1769)
        .write_bool(false)
        .write_bool(false)
        .write_bool(true)
}