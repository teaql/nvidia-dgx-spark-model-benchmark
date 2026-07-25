use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct EmailBounceLogBehavior;

impl EntityDataServiceBehavior for EmailBounceLogBehavior {}