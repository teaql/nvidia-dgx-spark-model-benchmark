#[derive(Clone)]
pub struct ServiceExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Service>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ServiceExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Service>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Service> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Service> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Service {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("service_id", |entity| entity.eval_service_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_ref_id", |entity| entity.eval_merchant_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant_ref", |entity| entity.eval_merchant_ref());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn get_moving_service_list(self) -> crate::MovingServiceListExpression<'a> {
        let next = self.result.and_then("moving_service_list", |entity| entity.eval_moving_service_list());
        crate::MovingServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cleaning_service_list(self) -> crate::CleaningServiceListExpression<'a> {
        let next = self.result.and_then("cleaning_service_list", |entity| entity.eval_cleaning_service_list());
        crate::CleaningServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_configuration_list(self) -> crate::ServiceConfigurationListExpression<'a> {
        let next = self.result.and_then("service_configuration_list", |entity| entity.eval_service_configuration_list());
        crate::ServiceConfigurationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_disposal_service_list(self) -> crate::DisposalServiceListExpression<'a> {
        let next = self.result.and_then("disposal_service_list", |entity| entity.eval_disposal_service_list());
        crate::DisposalServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_availability_calendar_list(self) -> crate::AvailabilityCalendarListExpression<'a> {
        let next = self.result.and_then("availability_calendar_list", |entity| entity.eval_availability_calendar_list());
        crate::AvailabilityCalendarListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_level_agreement_list(self) -> crate::ServiceLevelAgreementListExpression<'a> {
        let next = self.result.and_then("service_level_agreement_list", |entity| entity.eval_service_level_agreement_list());
        crate::ServiceLevelAgreementListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_add_on_service_list(self) -> crate::AddOnServiceListExpression<'a> {
        let next = self.result.and_then("add_on_service_list", |entity| entity.eval_add_on_service_list());
        crate::AddOnServiceListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct ServiceListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Service>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> ServiceListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Service>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Service>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Service>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Service> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::ServiceExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ServiceExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::ServiceExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::ServiceExpression::new(next, self.root_desc.clone())
    }
}