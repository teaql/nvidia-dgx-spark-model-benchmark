use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct OperationalHookBehavior;

impl EntityDataServiceBehavior for OperationalHookBehavior {}