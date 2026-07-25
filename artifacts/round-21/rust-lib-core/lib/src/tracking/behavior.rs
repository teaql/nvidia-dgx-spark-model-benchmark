use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct TrackingBehavior;

impl EntityDataServiceBehavior for TrackingBehavior {}