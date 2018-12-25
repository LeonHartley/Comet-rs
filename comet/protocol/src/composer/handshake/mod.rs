
use buffer::Buffer;

pub fn motd_composer(motd: String) -> Buffer {
    let mut buf = Buffer::empty(408);

    buf.write_i32(1);
    buf.write_string(String::from("Yooooo"));

    buf
}

pub fn auth_ok_composer() -> Buffer {
    Buffer::empty(3054)
}