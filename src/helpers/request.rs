use chrono::NaiveDateTime;
// use sea_orm::{EntityTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Page {
    pub offset: usize,
    pub size: usize,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            offset: 0,
            size: 10,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct DateFilter {
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
    pub method: String,
}

impl Default for DateFilter {
    fn default() -> Self {
        Self {
            start: None,
            end: None,
            method: std::string::String::from("after"),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct Filter {
    // shared filters
    pub maintenance: Option<bool>,

    // outtages filters
    pub downtime: Option<String>,
    pub loss: Option<f32>,

    // modem_events filters
    pub description: Option<String>,
    pub priority: Option<i32>,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            description: None,
            downtime: None,
            loss: None,
            maintenance: None,
            priority: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct PaginatedRequest {
    pub date: DateFilter,
    pub filter: Filter,
    pub page: Page,
}

impl Default for PaginatedRequest {
    fn default() -> Self {
        Self {
            date: DateFilter::default(),
            filter: Filter::default(),
            page: Page::default(),
        }
    }
}

// Implement the trait for the paginated request
// impl PaginatedRequest {
//     pub fn use_filter<E: EntityTrait>(&self, mut select: &sea_orm::Select<E>) -> &sea_orm::Select<E> {
//         if let Some(created_at) = self.filter.created_at {
//             select = &select.filter(E::CreatedAt.gte(created_at));
//         }

//         if let Some(maintenance) = self.filter.maintenance {
//             select = &select.filter(E::Maintenance.eq(maintenance));
//         }

//         select
//     }
// }
