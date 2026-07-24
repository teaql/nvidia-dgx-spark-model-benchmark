use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct PayableBehavior;

impl EntityDataServiceBehavior for PayableBehavior {}