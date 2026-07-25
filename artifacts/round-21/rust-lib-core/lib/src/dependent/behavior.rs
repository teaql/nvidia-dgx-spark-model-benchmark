use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct DependentBehavior;

impl EntityDataServiceBehavior for DependentBehavior {}