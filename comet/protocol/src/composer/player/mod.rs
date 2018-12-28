use buffer::Buffer;

pub fn credits_composer(credits: i32) -> Buffer {
    Buffer::empty(1556)
        .write_string(format!("{}.0", credits))
}