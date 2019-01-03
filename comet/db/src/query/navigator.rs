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
