use crate::providers::types::RenderData;

use super::predicates;
use super::{Filter, FilterAction};

pub fn evaluate_filters(filters: &[Filter], data: &RenderData) -> FilterDecision {
    if filters.is_empty() {
        return FilterDecision::Include;
    }

    let mut included = false;
    let mut explicitly_excluded = false;

    for filter in filters {
        let matches = predicates::evaluate(filter.filter_type, &filter.filter_value, data);

        match filter.action {
            FilterAction::Exclude if matches => {
                explicitly_excluded = true;
            }
            FilterAction::Include if matches => {
                included = true;
            }
            _ => {}
        }
    }

    if explicitly_excluded {
        FilterDecision::Exclude
    } else if included || filters.iter().all(|f| matches!(f.action, FilterAction::Exclude)) {
        FilterDecision::Include
    } else {
        FilterDecision::Exclude
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterDecision {
    Include,
    Exclude,
}
