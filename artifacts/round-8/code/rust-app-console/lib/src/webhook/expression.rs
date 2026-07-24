#[derive(Clone)]
pub struct WebhookExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Webhook>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> WebhookExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Webhook>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Webhook> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Webhook> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Webhook {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_webhook_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("webhook_id", |entity| entity.eval_webhook_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_url(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("url", |entity| entity.eval_url());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_api_client_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("api_client_ref_id", |entity| entity.eval_api_client_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_api_client_ref(self) -> crate::ApiClientExpression<'a> {
        let next = self.result.and_then("api_client_ref", |entity| entity.eval_api_client_ref());
        crate::ApiClientExpression::new(next, self.root_desc.clone())
    }
    pub fn get_webhook_delivery_list(self) -> crate::WebhookDeliveryListExpression<'a> {
        let next = self.result.and_then("webhook_delivery_list", |entity| entity.eval_webhook_delivery_list());
        crate::WebhookDeliveryListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct WebhookListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Webhook>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> WebhookListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Webhook>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Webhook>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Webhook>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Webhook> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::WebhookExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::WebhookExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::WebhookExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::WebhookExpression::new(next, self.root_desc.clone())
    }
}