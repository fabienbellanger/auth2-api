//! Query filters value object representation
//!
//! Example:
//! <base_url>?price=gt:100,le:200&name=in:John&created_at=gte:2021-01-01

// TODO: Add tests and improve documentation

use std::collections::HashMap;
use std::str::FromStr;

/// Query filter operator
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryFilterOperator {
    /// Equal
    Eq,

    /// Not equal
    Ne,

    /// Greater than
    Gt,

    /// Greater than or equal
    Gte,

    /// Less than
    Lt,

    /// Less than or equal
    Lte,

    /// In
    In,

    /// Not in
    Nin,
}

impl FromStr for QueryFilterOperator {
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

/// Query filter field name (Ex.: "name", "email", "created_at")
pub type QueryFilterFieldName = String;

/// Query filter field value containing the operator and the value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryFilterFieldValue {
    pub operator: QueryFilterOperator,
    pub value: String,
}

impl QueryFilterFieldValue {
    /// Create a new filter field value
    pub fn new(operator: QueryFilterOperator, value: String) -> Self {
        Self { operator, value }
    }
}

/// List of query filter field values for a single field
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryFilterFieldValues(Vec<QueryFilterFieldValue>);

impl QueryFilterFieldValues {
    /// Create a new filter field values
    pub fn new(values: Vec<QueryFilterFieldValue>) -> Self {
        Self(values)
    }

    /// Get the filter field values
    pub fn value(&self) -> &Vec<QueryFilterFieldValue> {
        &self.0
    }
}

impl FromStr for QueryFilterFieldValues {
    type Err = String; // TODO: Change to a custom error type

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Query filters value object
pub struct QueryFilters(HashMap<QueryFilterFieldName, QueryFilterFieldValue>);

impl QueryFilters {
    /// Create a new query filters value object
    pub fn new(filters: HashMap<QueryFilterFieldName, QueryFilterFieldValue>) -> Self {
        Self(filters)
    }

    /// Get the query filters
    pub fn value(&self) -> &HashMap<QueryFilterFieldName, QueryFilterFieldValue> {
        &self.0
    }
}
