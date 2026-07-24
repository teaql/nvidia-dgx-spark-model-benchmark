#[derive(Clone)]
pub struct JobApplicationExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::JobApplication>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> JobApplicationExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::JobApplication>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::JobApplication> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::JobApplication> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::JobApplication {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_candidate_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("candidate_name", |entity| entity.eval_candidate_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_resume_url(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("resume_url", |entity| entity.eval_resume_url());
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
    pub fn get_status_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("status_id", |entity| entity.eval_status_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_recruitment_post_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("recruitment_post_id", |entity| entity.eval_recruitment_post_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_status(self) -> crate::ApplicationStatusExpression<'a> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::ApplicationStatusExpression::new(next, self.root_desc.clone())
    }

    pub fn get_recruitment_post(self) -> crate::RecruitmentPostExpression<'a> {
        let next = self.result.and_then("recruitment_post", |entity| entity.eval_recruitment_post());
        crate::RecruitmentPostExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn status_is_applied(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_applied())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_interviewing(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_interviewing())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn status_is_offered(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("status_id", |entity| {
            if !entity.is_loaded("status_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.status_is_offered())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_interview_list(self) -> crate::InterviewListExpression<'a> {
        let next = self.result.and_then("interview_list", |entity| entity.eval_interview_list());
        crate::InterviewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_offer_letter_list(self) -> crate::OfferLetterListExpression<'a> {
        let next = self.result.and_then("offer_letter_list", |entity| entity.eval_offer_letter_list());
        crate::OfferLetterListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct JobApplicationListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::JobApplication>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> JobApplicationListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::JobApplication>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::JobApplication>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::JobApplication>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::JobApplication> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::JobApplicationExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::JobApplicationExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::JobApplicationExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::JobApplicationExpression::new(next, self.root_desc.clone())
    }
}