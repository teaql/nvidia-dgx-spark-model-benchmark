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

    pub fn get_review_period(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("review_period", |entity| entity.eval_review_period());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_comments(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("comments", |entity| entity.eval_comments());
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
    pub fn get_grade_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("grade_id", |entity| entity.eval_grade_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("employee_id", |entity| entity.eval_employee_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_grade(self) -> crate::ReviewGradeExpression<'a> {
        let next = self.result.and_then("grade", |entity| entity.eval_grade());
        crate::ReviewGradeExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee(self) -> crate::EmployeeExpression<'a> {
        let next = self.result.and_then("employee", |entity| entity.eval_employee());
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn grade_is_excellent(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("grade_id", |entity| {
            if !entity.is_loaded("grade_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "grade_id".to_string(), attempted_path: "grade_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.grade_is_excellent())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn grade_is_good(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("grade_id", |entity| {
            if !entity.is_loaded("grade_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "grade_id".to_string(), attempted_path: "grade_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.grade_is_good())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn grade_is_needs_improvement(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("grade_id", |entity| {
            if !entity.is_loaded("grade_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "grade_id".to_string(), attempted_path: "grade_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.grade_is_needs_improvement())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
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