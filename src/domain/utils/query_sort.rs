//! Module used for query parameters sorts

use crate::domain::value_objects::pagination::Pagination;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

/// Filter sort field
pub type SortField = String;

/// Filter sort direction (ASC or DESC)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SortDirection {
    /// Ascending sort (`'+'` prefix)
    /// Example: ?sort=+id
    Asc,

    /// Descending sort (`'-'` prefix)
    /// Example: ?sort=-name
    Desc,
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Asc => "ASC",
                Self::Desc => "DESC",
            }
        )
    }
}

/// Filter sorts
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sorts(pub Vec<(SortField, SortDirection)>);

impl From<&str> for Sorts {
    fn from(value: &str) -> Self {
        let mut sorts = Vec::new();
        let parts = value.split(',');

        for part in parts {
            let prefix = part.chars().next();
            if let Some(prefix) = prefix {
                if prefix == '+' {
                    sorts.push((part[1..].to_string(), SortDirection::Asc));
                } else if prefix == '-' {
                    sorts.push((part[1..].to_string(), SortDirection::Desc));
                }
            }
        }

        Self(sorts)
    }
}

/// Filter operator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterOperator {
    Eq,
    Ne,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    Nin,
}

impl FromStr for FilterOperator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(Self::Eq),
            "ne" => Ok(Self::Ne),
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            "in" => Ok(Self::In),
            "nin" => Ok(Self::Nin),
            err => Err(format!("{err} in not a valid filter operator")),
        }
    }
}

pub type FilterFieldName = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilterFieldValue {
    pub operator: FilterOperator,
    pub value: String,
}

pub type Filters = HashMap<FilterFieldName, Vec<FilterFieldValue>>;

/// Filter pagination and sorts
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Filter {
    pub pagination: Option<Pagination>,
    pub sorts: Option<Sorts>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_sorts_from_str() {
        let sorts = Sorts::from("");
        assert!(sorts.0.is_empty());

        let sorts = Sorts::from("+id,-name");
        assert_eq!(sorts.0.len(), 2);
        assert_eq!(
            sorts.0,
            vec![
                ("id".to_string(), SortDirection::Asc),
                ("name".to_string(), SortDirection::Desc),
            ]
        );

        let sorts = Sorts::from("id");
        assert!(sorts.0.is_empty());
    }
}
