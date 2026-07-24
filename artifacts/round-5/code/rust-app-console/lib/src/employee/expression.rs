#[derive(Clone)]
pub struct EmployeeExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Employee>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EmployeeExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Employee>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Employee> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Employee> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Employee {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("employee_number", |entity| entity.eval_employee_number());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_birth_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("birth_date", |entity| entity.eval_birth_date());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_mobile_phone(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("mobile_phone", |entity| entity.eval_mobile_phone());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_email(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("email", |entity| entity.eval_email());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_job_title(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("job_title", |entity| entity.eval_job_title());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_hiring_date(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("hiring_date", |entity| entity.eval_hiring_date());
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
    pub fn get_gender_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("gender_id", |entity| entity.eval_gender_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_gender(self) -> crate::GenderTypeExpression<'a> {
        let next = self.result.and_then("gender", |entity| entity.eval_gender());
        crate::GenderTypeExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn gender_is_male(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("gender_id", |entity| {
            if !entity.is_loaded("gender_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "gender_id".to_string(), attempted_path: "gender_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.gender_is_male())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn gender_is_female(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("gender_id", |entity| {
            if !entity.is_loaded("gender_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "gender_id".to_string(), attempted_path: "gender_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.gender_is_female())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct EmployeeListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Employee>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EmployeeListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Employee>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Employee>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Employee>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Employee> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::EmployeeExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::EmployeeExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }
}