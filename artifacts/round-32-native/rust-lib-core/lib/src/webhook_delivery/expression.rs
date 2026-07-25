#[derive(Clone)]
pub struct WebhookDeliveryExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::WebhookDelivery>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> WebhookDeliveryExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::WebhookDelivery>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::WebhookDelivery> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::WebhookDelivery> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::WebhookDelivery {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_delivery_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("delivery_status", |entity| entity.eval_delivery_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_response_code(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("response_code", |entity| entity.eval_response_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_attempted_at(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("attempted_at", |entity| entity.eval_attempted_at());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_response_body(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("response_body", |entity| entity.eval_response_body());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct WebhookDeliveryListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::WebhookDelivery>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> WebhookDeliveryListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::WebhookDelivery>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::WebhookDelivery>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::WebhookDelivery>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::WebhookDelivery> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::WebhookDeliveryExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::WebhookDeliveryExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::WebhookDeliveryExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::WebhookDeliveryExpression::new(next, self.root_desc.clone())
    }
}