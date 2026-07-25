use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct AuditTrailBehavior;

impl EntityDataServiceBehavior for AuditTrailBehavior {}