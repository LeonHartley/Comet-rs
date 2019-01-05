use container::Component;
use container::Container;
use ctx::GameContext;
use db::ctx::DbContext;
use db::query::navigator::NavigatorRepository;
use model::navigator::Category;
use model::navigator::CategoryType;
use std::sync::RwLock;

pub trait NavigatorService {
    fn get_room_categories(&self) -> Vec<Category>;
}

pub struct NavigatorServiceContext {
    categories: RwLock<Vec<Category>>,
    room_categories: RwLock<Vec<Category>>,
}

impl Component for NavigatorServiceContext {
    fn registered(&self) {
        info!("Loaded {} navigator categories", self.categories.read().expect("categories lock").len());
        info!("Loaded {} room categories", self.room_categories.read().expect("categories lock").len());
    }
}

impl NavigatorServiceContext {
    pub fn new(mut db: DbContext) -> NavigatorServiceContext {
        let categories = db
            .get_navigator_categories()
            .expect("load navigator categories");

        let room_categories = categories
            .iter()
            .filter(|c| c.category_type == CategoryType::Category)
            .map(|c| c.clone())
            .collect();

        NavigatorServiceContext {
            categories: RwLock::new(categories),
            room_categories: RwLock::new(room_categories),
        }
    }
}

impl NavigatorService for NavigatorServiceContext {
    fn get_room_categories(&self) -> Vec<Category> {
        self.room_categories
            .read()
            .expect("room categories lock")
            .clone()
    }
}

impl NavigatorService for GameContext {
    fn get_room_categories(&self) -> Vec<Category> {
        self.component::<NavigatorServiceContext>()
            .get_room_categories()
    }
}