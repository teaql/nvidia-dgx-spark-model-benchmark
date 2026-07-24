#[derive(Clone)]
pub struct CrewMemberAssignmentExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::CrewMemberAssignment>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CrewMemberAssignmentExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::CrewMemberAssignment>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::CrewMemberAssignment> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::CrewMemberAssignment> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::CrewMemberAssignment {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("employee_name", |entity| entity.eval_employee_name());
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
    pub fn get_role_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("role_id", |entity| entity.eval_role_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("crew_id", |entity| entity.eval_crew_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_id", |entity| entity.eval_merchant_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_role(self) -> crate::CrewRoleExpression<'a> {
        let next = self.result.and_then("role", |entity| entity.eval_role());
        crate::CrewRoleExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew(self) -> crate::CrewExpression<'a> {
        let next = self.result.and_then("crew", |entity| entity.eval_crew());
        crate::CrewExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant", |entity| entity.eval_merchant());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn role_is_driver(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("role_id", |entity| {
            if !entity.is_loaded("role_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "role_id".to_string(), attempted_path: "role_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.role_is_driver())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn role_is_mover(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("role_id", |entity| {
            if !entity.is_loaded("role_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "role_id".to_string(), attempted_path: "role_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.role_is_mover())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn role_is_supervisor(self) -> crate::ValueExpression<'a, bool> {
        let next = self.result.and_then("role_id", |entity| {
            if !entity.is_loaded("role_id") {
                teaql_core::eval::EvalResult::NotLoaded { failed_node: "role_id".to_string(), attempted_path: "role_id".to_string() }
            } else {
                teaql_core::eval::EvalResult::Value(entity.role_is_supervisor())
            }
        });
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct CrewMemberAssignmentListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::CrewMemberAssignment>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CrewMemberAssignmentListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::CrewMemberAssignment>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::CrewMemberAssignment>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::CrewMemberAssignment>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::CrewMemberAssignment> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::CrewMemberAssignmentExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CrewMemberAssignmentExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::CrewMemberAssignmentExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CrewMemberAssignmentExpression::new(next, self.root_desc.clone())
    }
}