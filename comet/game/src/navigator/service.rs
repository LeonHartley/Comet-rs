use std::sync::RwLock;

use db::ctx::DbContext;
use db::query::navigator::NavigatorRepository;
use model::navigator::Category;
use container::Component;

pub trait NavigatorService {}

pub struct NavigatorServiceContext {
    categories: RwLock<Vec<Category>>
}

impl Component for NavigatorServiceContext {}

impl NavigatorServiceContext {
    pub fn new(db: DbContext) -> NavigatorServiceContext {
        NavigatorServiceContext {
            categories: RwLock::new(db
                .get_navigator_categories()
                .expect("load navigator categories"))
        }
    }
}