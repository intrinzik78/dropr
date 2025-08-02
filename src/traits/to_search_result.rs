use crate::enums::{ ImageType, SearchResult };

pub trait ToSearchResult<T> {
    fn to_search_result<'a>(self, search_opt: Option<&T>) -> SearchResult;
}

impl <T> ToSearchResult <T> for Option<ImageType>
where T: PartialEq<ImageType>
{
    fn to_search_result(self, search_opt: Option<&T>) -> SearchResult {
        if let Some(val) = self {
            if search_opt.is_none() {
                // early exit, no search criteria to match against
                SearchResult::NoSearchFilter
            } else {
                // search match test
                if let Some(search) = search_opt {
                    if *search == val {
                        SearchResult::Found
                    } else {
                        SearchResult::NotFound
                    }
                } else {
                    SearchResult::NoSearchFilter
                }
            }
        } else {
            // early exit, nothing found to match search criteria to match against
            SearchResult::NotFound
        }
    }
}