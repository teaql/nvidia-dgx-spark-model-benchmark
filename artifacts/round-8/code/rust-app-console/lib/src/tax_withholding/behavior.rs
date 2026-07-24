use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct TaxWithholdingBehavior;

impl EntityDataServiceBehavior for TaxWithholdingBehavior {}