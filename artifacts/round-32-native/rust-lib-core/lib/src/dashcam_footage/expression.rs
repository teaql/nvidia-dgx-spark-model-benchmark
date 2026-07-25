#[derive(Clone)]
pub struct DashcamFootageExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::DashcamFootage>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DashcamFootageExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::DashcamFootage>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::DashcamFootage> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::DashcamFootage> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::DashcamFootage {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_recorded_at(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("recorded_at", |entity| entity.eval_recorded_at());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_duration_seconds(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("duration_seconds", |entity| entity.eval_duration_seconds());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_file_path(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("file_path", |entity| entity.eval_file_path());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_resolution(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("resolution", |entity| entity.eval_resolution());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DashcamFootageListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DashcamFootage>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DashcamFootageListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DashcamFootage>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::DashcamFootage>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::DashcamFootage>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::DashcamFootage> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DashcamFootageExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DashcamFootageExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DashcamFootageExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DashcamFootageExpression::new(next, self.root_desc.clone())
    }
}