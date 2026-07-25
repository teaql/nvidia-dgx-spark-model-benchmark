use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct SafetyCheckBehavior;

impl EntityDataServiceBehavior for SafetyCheckBehavior {}