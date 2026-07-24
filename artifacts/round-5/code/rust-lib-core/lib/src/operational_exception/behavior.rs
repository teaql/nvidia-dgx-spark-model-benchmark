use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct OperationalExceptionBehavior;

impl EntityDataServiceBehavior for OperationalExceptionBehavior {}