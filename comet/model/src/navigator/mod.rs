#[derive(Debug, Copy)]
pub enum CategorySearchOption {
    Nothing,
    Back,
    ShowMore,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy)]
pub enum CategoryViewMode {
    Regular,
    Thumbnail,
}

#[derive(Debug)]
pub enum NavigatorView {
    MyWorld,
    Official,
    Hotel,
    Events,
    Query
}

#[derive(Debug, PartialEq, Eq, Hash, Copy)]
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

#[derive(Debug)]
pub struct Category {
    pub id: i32,
    pub category: String,
    pub category_id: String,
    pub name: String,
    pub player_rank: i16,
    pub view_mode: CategoryViewMode,
    pub category_type: CategoryType,
    pub search_option: CategorySearchOption,
    pub room_count: i32,
    pub room_count_expanded: i32,
    pub visible: bool,
}

impl Clone for Category {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            category: self.category.clone(),
            category_id: self.category_id.clone(),
            name: self.name.clone(),
            player_rank: self.player_rank,
            view_mode: self.view_mode,
            category_type: self.category_type,
            search_option: self.search_option.clone(),
            room_count: self.room_count,
            room_count_expanded: self.room_count_expanded,
            visible: self.visible,
        }
    }
}

impl Clone for CategoryViewMode {
    fn clone(&self) -> Self {
        *self
    }
}


impl Clone for CategorySearchOption {
    fn clone(&self) -> Self {
        *self
    }
}


impl Clone for CategoryType {
    fn clone(&self) -> Self {
        *self
    }
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
