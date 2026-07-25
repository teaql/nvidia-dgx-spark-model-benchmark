use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct BackgroundCheckBehavior;

impl EntityDataServiceBehavior for BackgroundCheckBehavior {}