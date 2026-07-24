#[derive(Clone)]
pub struct OperationalExceptionExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::OperationalException>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OperationalExceptionExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::OperationalException>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::OperationalException> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::OperationalException> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::OperationalException {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_description(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("description", |entity| entity.eval_description());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_severity_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("severity_id", |entity| entity.eval_severity_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("move_order_id", |entity| entity.eval_move_order_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_severity(self) -> crate::ExceptionSeverityExpression<'a> {
        let next = self.result.and_then("severity", |entity| entity.eval_severity());
        crate::ExceptionSeverityExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order(self) -> crate::MoveOrderExpression<'a> {
        let next = self.result.and_then("move_order", |entity| entity.eval_move_order());
        crate::MoveOrderExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn severity_is_low(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("severity_id", |entity| {
            if !entity.is_loaded("severity_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "severity_id".to_string(), attempted_path: "severity_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.severity_is_low())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn severity_is_medium(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("severity_id", |entity| {
            if !entity.is_loaded("severity_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "severity_id".to_string(), attempted_path: "severity_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.severity_is_medium())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn severity_is_high(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("severity_id", |entity| {
            if !entity.is_loaded("severity_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "severity_id".to_string(), attempted_path: "severity_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.severity_is_high())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct OperationalExceptionListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OperationalException>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OperationalExceptionListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OperationalException>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::OperationalException>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::OperationalException>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::OperationalException> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::OperationalExceptionExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OperationalExceptionExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::OperationalExceptionExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OperationalExceptionExpression::new(next, self.root_desc.clone())
    }
}