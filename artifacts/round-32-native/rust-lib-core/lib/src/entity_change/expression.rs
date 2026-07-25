#[derive(Clone)]
pub struct EntityChangeExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::EntityChange>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EntityChangeExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::EntityChange>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::EntityChange> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::EntityChange> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::EntityChange {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_activity_log_ref(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("activity_log_ref", |entity| entity.eval_activity_log_ref());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_field_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("field_name", |entity| entity.eval_field_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_old_value(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("old_value", |entity| entity.eval_old_value());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_new_value(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("new_value", |entity| entity.eval_new_value());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct EntityChangeListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::EntityChange>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EntityChangeListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::EntityChange>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::EntityChange>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::EntityChange>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::EntityChange> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::EntityChangeExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EntityChangeExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::EntityChangeExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EntityChangeExpression::new(next, self.root_desc.clone())
    }
}