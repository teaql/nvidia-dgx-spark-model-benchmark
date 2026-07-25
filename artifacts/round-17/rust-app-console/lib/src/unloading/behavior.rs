use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct UnloadingBehavior;

impl EntityDataServiceBehavior for UnloadingBehavior {}