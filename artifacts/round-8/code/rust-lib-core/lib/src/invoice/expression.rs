#[derive(Clone)]
pub struct InvoiceExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Invoice>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InvoiceExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Invoice>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Invoice> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Invoice> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Invoice {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_invoice_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("invoice_id", |entity| entity.eval_invoice_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_payment_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("payment_ref_id", |entity| entity.eval_payment_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_payment_ref(self) -> crate::PaymentExpression<'a> {
        let next = self.result.and_then("payment_ref", |entity| entity.eval_payment_ref());
        crate::PaymentExpression::new(next, self.root_desc.clone())
    }
    pub fn get_invoice_line_list(self) -> crate::InvoiceLineListExpression<'a> {
        let next = self.result.and_then("invoice_line_list", |entity| entity.eval_invoice_line_list());
        crate::InvoiceLineListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_journal_entry_list(self) -> crate::JournalEntryListExpression<'a> {
        let next = self.result.and_then("journal_entry_list", |entity| entity.eval_journal_entry_list());
        crate::JournalEntryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_receivable_list(self) -> crate::ReceivableListExpression<'a> {
        let next = self.result.and_then("receivable_list", |entity| entity.eval_receivable_list());
        crate::ReceivableListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_record_list(self) -> crate::TaxRecordListExpression<'a> {
        let next = self.result.and_then("tax_record_list", |entity| entity.eval_tax_record_list());
        crate::TaxRecordListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct InvoiceListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Invoice>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> InvoiceListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Invoice>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Invoice>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Invoice>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Invoice> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::InvoiceExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InvoiceExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::InvoiceExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::InvoiceExpression::new(next, self.root_desc.clone())
    }
}