use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct AuditBehavior;

impl EntityDataServiceBehavior for AuditBehavior {}