use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct ReturnsProcessBehavior;

impl EntityDataServiceBehavior for ReturnsProcessBehavior {}