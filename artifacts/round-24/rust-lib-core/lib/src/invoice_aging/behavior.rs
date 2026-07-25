use teaql_runtime::EntityDataServiceBehavior;

#[derive(Clone, Debug, Default)]
pub struct InvoiceAgingBehavior;

impl EntityDataServiceBehavior for InvoiceAgingBehavior {}