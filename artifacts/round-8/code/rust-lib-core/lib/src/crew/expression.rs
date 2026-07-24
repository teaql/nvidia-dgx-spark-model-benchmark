#[derive(Clone)]
pub struct CrewExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Crew>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CrewExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Crew>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Crew> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Crew> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Crew {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("crew_id", |entity| entity.eval_crew_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_size(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("size", |entity| entity.eval_size());
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
    pub fn get_dispatch_assignment_list(self) -> crate::DispatchAssignmentListExpression<'a> {
        let next = self.result.and_then("dispatch_assignment_list", |entity| entity.eval_dispatch_assignment_list());
        crate::DispatchAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_assignment_list(self) -> crate::VehicleAssignmentListExpression<'a> {
        let next = self.result.and_then("vehicle_assignment_list", |entity| entity.eval_vehicle_assignment_list());
        crate::VehicleAssignmentListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct CrewListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Crew>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CrewListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Crew>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Crew>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Crew>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Crew> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::CrewExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CrewExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::CrewExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CrewExpression::new(next, self.root_desc.clone())
    }
}