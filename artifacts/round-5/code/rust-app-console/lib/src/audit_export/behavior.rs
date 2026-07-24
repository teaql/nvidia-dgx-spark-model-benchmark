use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct AuditExportBehavior;

impl EntityDataServiceBehavior for AuditExportBehavior {}