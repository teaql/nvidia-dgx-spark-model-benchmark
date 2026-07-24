#[derive(Clone)]
pub struct LogisticsProviderExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::LogisticsProvider>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> LogisticsProviderExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::LogisticsProvider>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::LogisticsProvider> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::LogisticsProvider> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::LogisticsProvider {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_provider_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("provider_name", |entity| entity.eval_provider_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contact_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("contact_number", |entity| entity.eval_contact_number());
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
    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn get_third_party_dispatch_list(self) -> crate::ThirdPartyDispatchListExpression<'a> {
        let next = self.result.and_then("third_party_dispatch_list", |entity| entity.eval_third_party_dispatch_list());
        crate::ThirdPartyDispatchListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct LogisticsProviderListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::LogisticsProvider>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> LogisticsProviderListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::LogisticsProvider>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::LogisticsProvider>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::LogisticsProvider>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::LogisticsProvider> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::LogisticsProviderExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::LogisticsProviderExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::LogisticsProviderExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::LogisticsProviderExpression::new(next, self.root_desc.clone())
    }
}