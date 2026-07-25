#[derive(Clone)]
pub struct VehicleTransportExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::VehicleTransport>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> VehicleTransportExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::VehicleTransport>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::VehicleTransport> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::VehicleTransport> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::VehicleTransport {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_make(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("vehicle_make", |entity| entity.eval_vehicle_make());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_model(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("vehicle_model", |entity| entity.eval_vehicle_model());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_transport_fee(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("transport_fee", |entity| entity.eval_transport_fee());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct VehicleTransportListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::VehicleTransport>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> VehicleTransportListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::VehicleTransport>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::VehicleTransport>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::VehicleTransport>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::VehicleTransport> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::VehicleTransportExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::VehicleTransportExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::VehicleTransportExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::VehicleTransportExpression::new(next, self.root_desc.clone())
    }
}