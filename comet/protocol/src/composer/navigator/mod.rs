use buffer::Buffer;
use model::navigator::Category;
use model::player::settings::NavigatorSettings;

pub fn room_categories_composer(categories: Vec<Category>, rank: i16) -> Buffer {
    Buffer::empty(2986)
        .write_vec(&categories, |c, buf| buf
            .write_i32(c.id)
            .write_str(&c.category)
            .write_bool(c.player_rank <= rank)
            .write_bool(false)
            .write_i16(0)
            .write_i16(0)
            .write_bool(false))
}

pub fn home_room_composer(home_room: i64) -> Buffer {
    Buffer::empty(1776)
        .write_i32(home_room as i32)
        .write_i32(home_room as i32)
}

pub fn navigator_settings_composer(settings: NavigatorSettings) -> Buffer {
    Buffer::empty(2123)
        .write_i32(settings.x)
        .write_i32(settings.y)
        .write_i32(settings.width)
        .write_i32(settings.height)
        .write_bool(settings.searches_visible)
        .write_i32(0)//??
}

pub fn navigator_metadata_composer() -> Buffer {
    Buffer::empty(2631)
        .write_vec(
            &vec!["official_view", "hotel_view", "roomads_view", "myworld_view"],
            |v, buf| buf
                .write_str(&v.to_string())
                .write_i32(0))
}
