use crate::providers::types::RenderData;

use super::FilterType;

pub fn evaluate(filter_type: FilterType, filter_value: &str, data: &RenderData) -> bool {
    match filter_type {
        FilterType::Branch => {
            data.branch
                .as_deref()
                .map(|b| glob_match(filter_value, b))
                .unwrap_or(false)
        }
        FilterType::Tag => {
            data.tag
                .as_deref()
                .map(|t| glob_match(filter_value, t))
                .unwrap_or(false)
        }
        FilterType::User => {
            data.sender
                .as_ref()
                .map(|s| glob_match(filter_value, &s.login))
                .unwrap_or(false)
        }
        FilterType::EventType => {
            glob_match(filter_value, &data.event_type)
        }
        FilterType::Label => {
            if let Some(pr) = &data.pull_request {
                pr.labels.iter().any(|l| glob_match(filter_value, &l.name))
            } else if let Some(issue) = &data.issue {
                issue.labels.iter().any(|l| glob_match(filter_value, &l.name))
            } else {
                false
            }
        }
        FilterType::Milestone => {
            let milestone = data
                .pull_request
                .as_ref()
                .and_then(|pr| pr.milestone.as_ref())
                .or_else(|| data.issue.as_ref().and_then(|i| i.milestone.as_ref()));

            milestone
                .map(|m| glob_match(filter_value, &m.title))
                .unwrap_or(false)
        }
        FilterType::File => {
            data.commits.iter().any(|c| {
                c.added.iter().any(|f| glob_match(filter_value, f))
                    || c.modified.iter().any(|f| glob_match(filter_value, f))
                    || c.removed.iter().any(|f| glob_match(filter_value, f))
            })
        }
        FilterType::Workflow => {
            data.workflow
                .as_ref()
                .map(|w| glob_match(filter_value, &w.name))
                .unwrap_or(false)
        }
        FilterType::DeploymentEnvironment => {
            data.deployment
                .as_ref()
                .map(|d| glob_match(filter_value, &d.environment))
                .unwrap_or(false)
        }
        FilterType::CommitMessage => {
            data.commits.iter().any(|c| {
                c.message.to_lowercase().contains(&filter_value.to_lowercase())
            })
        }
    }
}

fn glob_match(pattern: &str, value: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.contains('*') || pattern.contains('?') {
        let regex_pattern = pattern
            .replace('.', "\\.")
            .replace('*', ".*")
            .replace('?', ".");
        if let Ok(re) = regex::Regex::new(&format!("^{regex_pattern}$")) {
            return re.is_match(value);
        }
    }
    pattern.eq_ignore_ascii_case(value)
}
