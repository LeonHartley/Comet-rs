pub enum CategorySearchOption {
    Nothing,
    Back,
    ShowMore,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CategoryViewMode {
    Regular,
    Thumbnail,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum CategoryType {
    Category,
    Public,
    StaffPicks,
    Popular,
    Recommended,
    PlayerRooms,
    PlayerFavourites,
    PlayerHistory,
    PlayerFriendRooms,
    PlayerFrequentRooms,
    PlayerRights,
    TopPromotions,
    Promotion,
}

pub struct Category {
    id: i32,
    category: String,
    name: String,
    colour: i32,
    player_rank: i32,
    view_mode: CategoryViewMode,
    category_type: CategoryType,
    search_option: CategorySearchOption,
    room_count: i32,
    room_count_expanded: i32,
    visible: bool,
}