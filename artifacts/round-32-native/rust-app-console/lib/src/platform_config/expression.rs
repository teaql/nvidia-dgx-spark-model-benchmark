#[derive(Clone)]
pub struct PlatformConfigExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::PlatformConfig>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PlatformConfigExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::PlatformConfig>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::PlatformConfig> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::PlatformConfig> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::PlatformConfig {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_config_key(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("config_key", |entity| entity.eval_config_key());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_config_value(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("config_value", |entity| entity.eval_config_value());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_category(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("category", |entity| entity.eval_category());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_is_enabled(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("is_enabled", |entity| entity.eval_is_enabled());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_secret_key(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("secret_key", |entity| entity.eval_secret_key());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PlatformConfigListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PlatformConfig>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PlatformConfigListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PlatformConfig>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::PlatformConfig>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::PlatformConfig>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::PlatformConfig> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PlatformConfigExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PlatformConfigExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PlatformConfigExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PlatformConfigExpression::new(next, self.root_desc.clone())
    }
}