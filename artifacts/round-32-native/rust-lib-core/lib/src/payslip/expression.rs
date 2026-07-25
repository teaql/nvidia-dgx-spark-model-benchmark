#[derive(Clone)]
pub struct PayslipExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Payslip>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PayslipExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Payslip>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Payslip> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Payslip> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Payslip {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_issue_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("issue_date", |entity| entity.eval_issue_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_gross_amount(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("gross_amount", |entity| entity.eval_gross_amount());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_net_amount(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("net_amount", |entity| entity.eval_net_amount());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payment_method(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("payment_method", |entity| entity.eval_payment_method());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PayslipListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Payslip>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PayslipListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Payslip>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Payslip>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Payslip>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Payslip> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PayslipExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PayslipExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PayslipExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PayslipExpression::new(next, self.root_desc.clone())
    }
}