use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct PermitRequiredBehavior;

impl EntityDataServiceBehavior for PermitRequiredBehavior {}