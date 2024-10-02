//! Query filters value object representation
//!
//! Example:
//! <base_url>?price=gt:100,le:200&name=in:John&created_at=gte:2021-01-01

// TODO: Add tests and improve documentation

use std::collections::HashMap;
use std::hash::Hash;
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
pub trait QueryFilterFieldName: PartialEq + Eq + Hash {}

/// Query filter field value containing the operator and the value
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryFilterFieldValue<V> {
    pub operator: QueryFilterOperator,
    pub value: V,
}

impl<V> QueryFilterFieldValue<V> {
    /// Create a new filter field value
    pub fn new(operator: QueryFilterOperator, value: V) -> Self {
        Self { operator, value }
    }
}

/// List of query filter field values for a single field
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryFilterFieldValues<V>(Vec<QueryFilterFieldValue<V>>);

impl<V> QueryFilterFieldValues<V> {
    /// Create a new filter field values
    pub fn new(values: Vec<QueryFilterFieldValue<V>>) -> Self {
        Self(values)
    }

    /// Get the filter field values
    pub fn value(&self) -> &Vec<QueryFilterFieldValue<V>> {
        &self.0
    }
}

impl<V> FromStr for QueryFilterFieldValues<V> {
    type Err = String; // TODO: Change to a custom error type

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Query filters value object
pub struct QueryFilters<F: QueryFilterFieldName, V>(HashMap<F, QueryFilterFieldValues<V>>);

impl<F: QueryFilterFieldName, V> QueryFilters<F, V> {
    /// Create a new query filters value object
    pub fn new(filters: HashMap<F, QueryFilterFieldValues<V>>) -> Self {
        Self(filters)
    }

    /// Get the query filters
    pub fn value(&self) -> &HashMap<F, QueryFilterFieldValues<V>> {
        &self.0
    }
}
