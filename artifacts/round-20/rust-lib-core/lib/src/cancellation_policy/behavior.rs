use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct CancellationPolicyBehavior;

impl EntityDataServiceBehavior for CancellationPolicyBehavior {}