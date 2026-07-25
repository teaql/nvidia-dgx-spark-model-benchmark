use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct TwoFactorAuthBehavior;

impl EntityDataServiceBehavior for TwoFactorAuthBehavior {}