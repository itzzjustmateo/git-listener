pub mod engine;
pub mod predicates;


#[derive(Debug, Clone)]
pub enum FilterAction {
    Include,
    Exclude,
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub filter_type: FilterType,
    pub filter_value: String,
    pub action: FilterAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterType {
    Branch,
    Tag,
    User,
    File,
    Label,
    Milestone,
    EventType,
    Workflow,
    DeploymentEnvironment,
    CommitMessage,
}

impl std::str::FromStr for FilterType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "branch" => Ok(Self::Branch),
            "tag" => Ok(Self::Tag),
            "user" => Ok(Self::User),
            "file" => Ok(Self::File),
            "label" => Ok(Self::Label),
            "milestone" => Ok(Self::Milestone),
            "event_type" => Ok(Self::EventType),
            "workflow" => Ok(Self::Workflow),
            "deployment_environment" => Ok(Self::DeploymentEnvironment),
            "commit_message" => Ok(Self::CommitMessage),
            _ => Err(()),
        }
    }
}
