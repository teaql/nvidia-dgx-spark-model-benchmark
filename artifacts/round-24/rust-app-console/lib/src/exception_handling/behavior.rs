use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct ExceptionHandlingBehavior;

impl EntityDataServiceBehavior for ExceptionHandlingBehavior {}