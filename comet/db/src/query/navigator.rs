use ctx::DbContext;
use model::navigator::Category;

pub trait NavigatorRepository {
    fn get_navigator_categories(&mut self) -> Option<Vec<Category>>;
}

struct CategoryQueryResult {
    id: i32,
    category: String,
    category_id: String,
    name: String,
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
            category_id: self.category_id,
            name: self.name,
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

impl NavigatorRepository for DbContext {
    fn get_navigator_categories(&mut self) -> Option<Vec<Category>> {
        match self
            .pool()
            .prep_exec("SELECT id, category, category_identifier AS category_id, public_name AS name, required_rank AS player_rank, view_mode, category_type, search_allowance AS search_option, room_count, room_count_expanded, visible FROM navigator_categories;", ())
            .map(|res| {
                res.map(|x| x.unwrap()).map(|row| {
                    let (id, category, category_id, name, player_rank, view_mode, category_type, search_option, room_count, room_count_expanded, visible) = mysql::from_row(row);
                    CategoryQueryResult { id, category, category_id, name, player_rank, view_mode, category_type, search_option, room_count, room_count_expanded, visible }.into()
                }).collect()
            }) {
            Ok(categories) => Some(categories),
            Err(e) => {
                error!("MySQL Error {:?}", e);
                None
            }
        }
    }
}
