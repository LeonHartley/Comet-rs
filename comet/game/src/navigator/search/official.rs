use navigator::service::NavigatorServiceContext;
use navigator::search::handler::SearchHandler;

pub struct OfficialView(pub String);

impl SearchHandler<OfficialView> for NavigatorServiceContext {
    fn search(&self, view: OfficialView) {

    }
}
