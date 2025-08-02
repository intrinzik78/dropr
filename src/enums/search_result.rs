#[derive(Debug,PartialEq)]
pub enum SearchResult {
    Found,          // filter applied, search matched
    NotFound,       // filter applied, search did not match
    NoSearchFilter  // no filter
}