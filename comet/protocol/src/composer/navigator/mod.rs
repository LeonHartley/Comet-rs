use buffer::Buffer;
use model::navigator::Category;

pub fn room_categories_composer(categories: Vec<Category>, rank: i32) -> Buffer {
    Buffer::empty(2986)
        .write_vec(categories, |c, buf| buf
            .write_i32(c.id)
            .write_str(&c.category)
            .write_bool(c.player_rank <= rank)
            .write_bool(false)
            .write_i16(0)
            .write_i16(0)
            .write_bool(false))
}