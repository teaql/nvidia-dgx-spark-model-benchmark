#[derive(Clone)]
pub struct VehicleExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Vehicle>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> VehicleExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Vehicle>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Vehicle> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Vehicle> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Vehicle {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("vehicle_id", |entity| entity.eval_vehicle_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_license_plate(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("license_plate", |entity| entity.eval_license_plate());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_make(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("make", |entity| entity.eval_make());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_model(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("model", |entity| entity.eval_model());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_year(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("year", |entity| entity.eval_year());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, chrono::NaiveDate> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct VehicleListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Vehicle>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> VehicleListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Vehicle>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Vehicle>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Vehicle>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Vehicle> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::VehicleExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::VehicleExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::VehicleExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::VehicleExpression::new(next, self.root_desc.clone())
    }
}