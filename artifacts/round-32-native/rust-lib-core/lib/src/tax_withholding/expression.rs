#[derive(Clone)]
pub struct TaxWithholdingExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::TaxWithholding>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaxWithholdingExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::TaxWithholding>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::TaxWithholding> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::TaxWithholding> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::TaxWithholding {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_year(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("tax_year", |entity| entity.eval_tax_year());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_federal_withholding(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("federal_withholding", |entity| entity.eval_federal_withholding());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_state_withholding(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("state_withholding", |entity| entity.eval_state_withholding());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_filing_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("filing_status", |entity| entity.eval_filing_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct TaxWithholdingListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaxWithholding>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> TaxWithholdingListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::TaxWithholding>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::TaxWithholding>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::TaxWithholding>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::TaxWithholding> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::TaxWithholdingExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaxWithholdingExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::TaxWithholdingExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::TaxWithholdingExpression::new(next, self.root_desc.clone())
    }
}