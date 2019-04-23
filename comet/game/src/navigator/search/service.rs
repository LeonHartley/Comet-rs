//use navigator::service::NavigatorServiceContext;
//use navigator::search::myworld::MyWorldView;
//use navigator::search::official::OfficialView;
//use navigator::search::handler::SearchHandler;
//
//#[derive(Debug)]
//pub enum SearchFilter {
//    MyWorld(String),
//    Official(String),
//    Hotel(String)
//}
//
//pub trait SearchService {}
//
//impl SearchService for NavigatorServiceContext {}
//
//impl SearchHandler<SearchFilter> for NavigatorServiceContext {
//    fn search(&self, filter: SearchFilter) {
//        match filter {
//            SearchFilter::MyWorld(query) => self.search(MyWorldView(query)),
//            SearchFilter::Official(query) => self.search(OfficialView(query))
//        };
//    }
//}
