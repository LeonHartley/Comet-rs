use navigator::service::NavigatorServiceContext;
use navigator::search::handler::SearchHandler;

pub struct MyWorldView(pub String);

impl SearchHandler<MyWorldView> for NavigatorServiceContext {
    fn search(&self, view: MyWorldView) {

    }
}
