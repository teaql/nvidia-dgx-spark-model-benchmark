use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct ApiRateLimitBehavior;

impl EntityDataServiceBehavior for ApiRateLimitBehavior {}