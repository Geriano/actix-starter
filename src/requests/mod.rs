pub mod auth;
pub mod permission;
pub mod role;
pub mod user;

use sea_orm::ColumnTrait;
use sea_query::Order;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Deserialize, ToSchema, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
pub struct PaginationRequest<T> {
    #[schema(example = "1")]
    pub page: Option<u64>,
    #[schema(example = "10")]
    pub limit: Option<u64>,
    #[schema(example = "a")]
    pub search: Option<String>,
    #[schema()]
    pub order: Option<T>,
    #[schema(example = "asc")]
    pub sort: Option<Sort>,
}

impl<T: ColumnTrait> PaginationRequest<T> {
    pub fn page(&self) -> u64 {
        if let Some(page) = self.page {
            if page < 1 {
                1
            } else {
                page
            }
        } else {
            1
        }
    }

    pub fn limit(&self) -> u64 {
        if let Some(limit) = self.limit {
            if limit > 100 {
                100
            } else if limit < 10 {
                10
            } else {
                limit
            }
        } else {
            10
        }
    }

    pub fn offset(&self) -> u64 {
        (self.page() - 1) * self.limit()
    }

    pub fn search(&self) -> Option<String> {
        self.search
            .clone()
            .and_then(|search| Some(format!("%{}%", search.to_lowercase())))
    }

    pub fn order(&self, default: T) -> T {
        if let Some(order) = self.order {
            order.into()
        } else {
            default
        }
    }

    pub fn sort(&self) -> sea_query::Order {
        self.sort.unwrap_or(Sort::Asc).into()
    }
}

#[derive(Clone, Copy, Deserialize, ToSchema)]
pub enum Sort {
    Asc,
    Desc,
}

impl Into<Order> for Sort {
    fn into(self) -> Order {
        match self {
            Self::Asc => Order::Asc,
            Self::Desc => Order::Desc,
        }
    }
}
