#[derive(Clone)]
pub struct OfferLetterExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::OfferLetter>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OfferLetterExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::OfferLetter>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::OfferLetter> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::OfferLetter> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::OfferLetter {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_offer_amount(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("offer_amount", |entity| entity.eval_offer_amount());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_valid_until(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("valid_until", |entity| entity.eval_valid_until());
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
    pub fn get_job_application_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("job_application_id", |entity| entity.eval_job_application_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_job_application(self) -> crate::JobApplicationExpression<'a> {
        let next = self.result.and_then("job_application", |entity| entity.eval_job_application());
        crate::JobApplicationExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct OfferLetterListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OfferLetter>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> OfferLetterListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::OfferLetter>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::OfferLetter>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::OfferLetter>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::OfferLetter> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::OfferLetterExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OfferLetterExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::OfferLetterExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::OfferLetterExpression::new(next, self.root_desc.clone())
    }
}