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
    Query,
    PlayerRooms,
    PlayerGroups,
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

impl From<String> for CategoryViewMode {
    fn from(view_mode: String) -> Self {
        match view_mode.to_lowercase().as_ref() {
            "thumbnail" => CategoryViewMode::Thumbnail,
            _ => CategoryViewMode::Reglar
        }
    }
}

impl From<String> for CategoryType {
    fn from(category_type: String) -> Self {
        match category_type.to_lowercase().as_ref() {
            "public" => CategoryType::Public,
            "staff_picks" => CategoryType::StaffPicks,
            "popular" => CategoryType::Popular,
            "recommended" => CategoryType::Recommended,
            "query" => CategoryType::Query,
            "my_rooms" => CategoryType::PlayerRooms,
            "my_favorites" => CategoryType::PlayerFavourites,
            "my_groups" => CategoryType::PlayerGroups,
            "my_history" => CategoryType::PlayerHistory,
            "my_friends_rooms" => CategoryType::PlayerFriendRooms,
            "my_history_freq" => CategoryType::PlayerFrequentRooms,
            "top_promotions" => CategoryType::TopPromotions,
            "promotion_category" => CategoryType::Promotion,
            _ => CategoryType::Category,
        }
    }
}

impl From<String> for CategorySearchOption {
    fn from(search_option: String) -> Self {
        match search_option.to_lowercase().as_ref() {
            "go_back" => CategorySearchOption::Back,
            "show_more" => CategorySearchOption::ShowMore,
            _ => CategorySearchOption::Nothing
        }
    }
}