//! Query filters value object representation
// TODO: Add tests and improve documentation

use std::collections::HashMap;
use std::str::FromStr;

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
