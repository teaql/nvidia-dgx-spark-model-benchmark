#[derive(Clone)]
pub struct PlatformExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Platform>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PlatformExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Platform>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Platform> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Platform> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Platform {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
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
    pub fn get_route_status_type_list(self) -> crate::RouteStatusTypeListExpression<'a> {
        let next = self.result.and_then("route_status_type_list", |entity| entity.eval_route_status_type_list());
        crate::RouteStatusTypeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_inventory_condition_type_list(self) -> crate::InventoryConditionTypeListExpression<'a> {
        let next = self.result.and_then("inventory_condition_type_list", |entity| entity.eval_inventory_condition_type_list());
        crate::InventoryConditionTypeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_exception_severity_list(self) -> crate::ExceptionSeverityListExpression<'a> {
        let next = self.result.and_then("exception_severity_list", |entity| entity.eval_exception_severity_list());
        crate::ExceptionSeverityListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_order_status_list(self) -> crate::OrderStatusListExpression<'a> {
        let next = self.result.and_then("order_status_list", |entity| entity.eval_order_status_list());
        crate::OrderStatusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_role_list(self) -> crate::CrewRoleListExpression<'a> {
        let next = self.result.and_then("crew_role_list", |entity| entity.eval_crew_role_list());
        crate::CrewRoleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_list(self) -> crate::MerchantListExpression<'a> {
        let next = self.result.and_then("merchant_list", |entity| entity.eval_merchant_list());
        crate::MerchantListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PlatformListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Platform>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PlatformListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Platform>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Platform>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Platform>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Platform> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PlatformExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PlatformExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PlatformExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PlatformExpression::new(next, self.root_desc.clone())
    }
}