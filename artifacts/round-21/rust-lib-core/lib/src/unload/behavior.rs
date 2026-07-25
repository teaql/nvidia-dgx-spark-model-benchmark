use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct UnloadBehavior;

impl EntityDataServiceBehavior for UnloadBehavior {}