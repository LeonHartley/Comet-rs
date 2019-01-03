use std::sync::RwLock;

use container::Component;
use db::ctx::DbContext;
use db::query::navigator::NavigatorRepository;
use model::navigator::Category;

pub trait NavigatorService {}

pub struct NavigatorServiceContext {
    categories: RwLock<Vec<Category>>
}

impl Component for NavigatorServiceContext {
    fn registered(&self) {
        info!("Loaded {} navigator categories",
              self
                  .categories
                  .read()
                  .expect("categories lock")
                  .len());
    }
}

impl NavigatorServiceContext {
    pub fn new(mut db: DbContext) -> NavigatorServiceContext {
        NavigatorServiceContext {
            categories: RwLock::new(db
                .get_navigator_categories()
                .expect("load navigator categories"))
        }
    }
}