//! Filter for pagination and sorting

// TODO: Add filtering (https://www.moesif.com/blog/technical/api-design/REST-API-Design-Filtering-Sorting-and-Pagination/)
// Try something like this: /applications/test?price=gte:10,lte:200

use crate::domain::utils::query_sort::{Filter, Sorts};
use crate::domain::value_objects::pagination::Pagination;
use crate::infrastructure::api::response::ApiError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FilterRequest {
    #[serde(rename(deserialize = "p"))]
    pub page: Option<u32>,

    #[serde(rename(deserialize = "l"))]
    pub limit: Option<u32>,

    #[serde(rename(deserialize = "s"))]
    pub sort: Option<String>,
}

impl TryFrom<FilterRequest> for Option<Filter> {
    type Error = ApiError;

    fn try_from(value: FilterRequest) -> Result<Self, Self::Error> {
        let filter = match (value.page, value.limit, value.sort) {
            (Some(page), Some(limit), sort) => {
                let pagination = Some(Pagination::new(page, limit));
                let sorts = sort.map(|sort| Sorts::from(sort.as_str()));

                Some(Filter { pagination, sorts })
            }
            (_, _, Some(sort)) => Some(Filter {
                pagination: Some(Pagination::default()),
                sorts: Some(Sorts::from(sort.as_str())),
            }),
            _ => None,
        };

        Ok(filter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::utils::query_sort::SortDirection;
    use crate::domain::value_objects::pagination::PAGINATION_MAX_LIMIT;

    #[test]
    fn test_filter_request_try_from_filter() {
        let filter_request = FilterRequest {
            page: None,
            limit: None,
            sort: None,
        };
        let filter: Option<Filter> = filter_request.try_into().unwrap();
        assert!(filter.is_none());

        let filter_request = FilterRequest {
            page: Some(1),
            limit: Some(10),
            sort: None,
        };
        let filter: Option<Filter> = filter_request.try_into().unwrap();
        assert!(filter.is_some());
        if let Some(filter) = filter {
            assert_eq!(filter.pagination, Some(Pagination::new(1, 10)));
            assert!(filter.sorts.is_none());
        }

        let filter_request = FilterRequest {
            page: Some(0),
            limit: Some(1000),
            sort: None,
        };
        let filter: Option<Filter> = filter_request.try_into().unwrap();
        assert!(filter.is_some());
        if let Some(filter) = filter {
            assert_eq!(filter.pagination, Some(Pagination::new(1, PAGINATION_MAX_LIMIT)));
            assert!(filter.sorts.is_none());
        }

        let filter_request = FilterRequest {
            page: None,
            limit: None,
            sort: Some("+name,-created_at".to_owned()),
        };
        let filter: Option<Filter> = filter_request.try_into().unwrap();
        assert!(filter.is_some());
        if let Some(filter) = filter {
            assert_eq!(filter.pagination, Some(Pagination::default()));
            assert_eq!(
                filter.sorts,
                Some(Sorts(vec![
                    ("name".to_owned(), SortDirection::Asc),
                    ("created_at".to_owned(), SortDirection::Desc),
                ]))
            );
        }

        let filter_request = FilterRequest {
            page: Some(2),
            limit: Some(100),
            sort: Some("-name".to_owned()),
        };
        let filter: Option<Filter> = filter_request.try_into().unwrap();
        assert!(filter.is_some());
        if let Some(filter) = filter {
            assert_eq!(filter.pagination, Some(Pagination::new(2, 100)));
            assert_eq!(
                filter.sorts,
                Some(Sorts(vec![("name".to_owned(), SortDirection::Desc),]))
            );
        }
    }
}
