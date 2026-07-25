use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct GdprRequestBehavior;

impl EntityDataServiceBehavior for GdprRequestBehavior {}