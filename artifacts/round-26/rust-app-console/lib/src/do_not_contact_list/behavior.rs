use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct DoNotContactListBehavior;

impl EntityDataServiceBehavior for DoNotContactListBehavior {}