use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct LoadingBehavior;

impl EntityDataServiceBehavior for LoadingBehavior {}