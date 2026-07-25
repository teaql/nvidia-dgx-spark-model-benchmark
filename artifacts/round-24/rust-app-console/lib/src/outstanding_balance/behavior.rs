use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct OutstandingBalanceBehavior;

impl EntityDataServiceBehavior for OutstandingBalanceBehavior {}