use ctx::DbContext;
use model::navigator::Category;

pub trait NavigatorRepository {
    fn get_navigator_categories(&self) -> Option<Vec<Category>>;
}

impl NavigatorRepository for DbContext {
    fn get_navigator_categories(&self) -> Option<Vec<Category>> {
        Some(vec![])
    }
}

struct CategoryQueryResult {
    id: i32,
    category: String,
    name: String,
    colour: i32,
    player_rank: i32,
    view_mode: String,
    category_type: String,
    search_option: String,
    room_count: i32,
    room_count_expanded: i32,
    visible: bool,
}

impl Into<Category> for CategoryQueryResult {
    fn into(self) -> Category {
        Category {
            id: self.id,
            category: self.category,
            name: self.name,
            colour: self.colour,
            player_rank: self.player_rank,
            view_mode: self.view_mode.into(),
            category_type: self.category_type.into(),
            search_option: self.search_option.into(),
            room_count: self.room_count,
            room_count_expanded: self.room_count_expanded,
            visible: self.visible,
        }
    }
}