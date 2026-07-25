use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct AuditAdjustmentBehavior;

impl EntityDataServiceBehavior for AuditAdjustmentBehavior {}