use buffer::Buffer;

pub fn fuserights_composer(rank: i16, club: bool) -> Buffer {
    Buffer::empty(1081)
        .write_i32(2)
        .write_i32(rank as i32)
        .write_bool(club)
}

pub fn allowances_composer() -> Buffer {
    let allowances: Vec<(String, String, bool)> = vec![
        ("USE_GUIDE_TOOL".to_string(), String::new(), false),
        ("GIVE_GUIdE_TOURS".to_string(), "requirement.unfulfilled.helper_le".to_string(), false),
        ("JUDGE_CHAT_REVIEWS".to_string(), String::new(), true),
        ("VOTE_IN_COMPETITIONS".to_string(), String::new(), true),
        ("CALL_ON_HELPERS".to_string(), String::new(), false),
        ("CITIZEN".to_string(), String::new(), true),
        ("TRADE".to_string(), String::new(), true),
        ("CAMERA".to_string(), String::new(), true),
        ("HEIGHTMAP_EDITOR_BETA".to_string(), String::new(), false),
        ("EXPERIMENTAL_CHAT_BETA".to_string(), String::new(), true),
        ("EXPERIMENTAL_TOOLBAR".to_string(), String::new(), true),
        ("BUILDER_AT_WORK".to_string(), String::new(), true),
        ("NAVIGATOR_PHASE_ONE_2014".to_string(), String::new(), true),
        ("NAVIGATOR_PHASE_TWO_2014".to_string(), String::new(), true),
        ("MOUSE_ZOOM".to_string(), String::new(), true),
        ("NAVIGATOR_ROOM_THUMBNAIL_CAMERA".to_string(), String::new(), true),
    ];

    let mut buf = Buffer::empty(3189)
        .write_i32(allowances.len() as i32);

    for (key, s, enabled) in allowances {
        buf = buf.write_str(&key)
            .write_str(&s)
            .write_bool(enabled);
    }

    buf
}