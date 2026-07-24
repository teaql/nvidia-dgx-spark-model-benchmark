#[derive(Clone)]
pub struct RecruitmentPostExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::RecruitmentPost>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> RecruitmentPostExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::RecruitmentPost>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::RecruitmentPost> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::RecruitmentPost> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::RecruitmentPost {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_job_description(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("job_description", |entity| entity.eval_job_description());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_posting_date(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("posting_date", |entity| entity.eval_posting_date());
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
    pub fn get_position_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("position_id", |entity| entity.eval_position_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_position(self) -> crate::PositionExpression<'a> {
        let next = self.result.and_then("position", |entity| entity.eval_position());
        crate::PositionExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn get_job_application_list(self) -> crate::JobApplicationListExpression<'a> {
        let next = self.result.and_then("job_application_list", |entity| entity.eval_job_application_list());
        crate::JobApplicationListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct RecruitmentPostListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::RecruitmentPost>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> RecruitmentPostListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::RecruitmentPost>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::RecruitmentPost>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::RecruitmentPost>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::RecruitmentPost> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::RecruitmentPostExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::RecruitmentPostExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::RecruitmentPostExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::RecruitmentPostExpression::new(next, self.root_desc.clone())
    }
}