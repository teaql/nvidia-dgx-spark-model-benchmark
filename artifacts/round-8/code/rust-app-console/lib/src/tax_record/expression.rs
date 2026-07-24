#[derive(Clone)]
pub struct TaxRecordExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TaxRecord>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaxRecordExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TaxRecord>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TaxRecord> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TaxRecord> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TaxRecord {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_record_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("record_id", |entity| entity.eval_record_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_record_type(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("record_type", |entity| entity.eval_record_type());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_invoice_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("invoice_ref_id", |entity| entity.eval_invoice_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_invoice_ref(self) -> crate::InvoiceExpression<'a> {
        let next = self.result.and_then("invoice_ref", |entity| entity.eval_invoice_ref());
        crate::InvoiceExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TaxRecordListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaxRecord>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaxRecordListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaxRecord>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TaxRecord>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TaxRecord>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TaxRecord> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TaxRecordExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaxRecordExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TaxRecordExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaxRecordExpression::new(next, self.root_desc.clone())
    }
}