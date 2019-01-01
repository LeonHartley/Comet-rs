use buffer::Buffer;

pub fn messenger_config_composer() -> Buffer {
    Buffer::empty(913)
        .write_i32(300)//max friends
        .write_i32(300)
        .write_i32(800)
        .write_i32(0)
}