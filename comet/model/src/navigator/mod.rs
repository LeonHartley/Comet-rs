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
    pub id: i32,
    pub category: String,
    pub name: String,
    pub colour: i32,
    pub player_rank: i32,
    pub view_mode: CategoryViewMode,
    pub category_type: CategoryType,
    pub search_option: CategorySearchOption,
    pub room_count: i32,
    pub room_count_expanded: i32,
    pub visible: bool,
}

impl From<String> for CategoryViewMode {
    fn from(view_mode: String) -> Self {
        match view_mode.to_lowercase().as_ref() {
            "thumbnail" => CategoryViewMode::Thumbnail,
            _ => CategoryViewMode::Regular
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