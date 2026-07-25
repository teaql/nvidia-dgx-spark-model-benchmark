#[derive(Clone)]
pub struct PerformanceReviewExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::PerformanceReview>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PerformanceReviewExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::PerformanceReview>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::PerformanceReview> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::PerformanceReview> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::PerformanceReview {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_review_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("review_date", |entity| entity.eval_review_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_rating(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("rating", |entity| entity.eval_rating());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_comments(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("comments", |entity| entity.eval_comments());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("employee_id", |entity| entity.eval_employee_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_reviewer_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("reviewer_id", |entity| entity.eval_reviewer_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee(self) -> crate::EmployeeRecordExpression<'a> {
        let next = self.result.and_then("employee", |entity| entity.eval_employee());
        crate::EmployeeRecordExpression::new(next, self.root_desc.clone())
    }

    pub fn get_reviewer(self) -> crate::EmployeeRecordExpression<'a> {
        let next = self.result.and_then("reviewer", |entity| entity.eval_reviewer());
        crate::EmployeeRecordExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PerformanceReviewListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PerformanceReview>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PerformanceReviewListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PerformanceReview>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::PerformanceReview>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::PerformanceReview>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::PerformanceReview> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PerformanceReviewExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PerformanceReviewExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PerformanceReviewExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PerformanceReviewExpression::new(next, self.root_desc.clone())
    }
}