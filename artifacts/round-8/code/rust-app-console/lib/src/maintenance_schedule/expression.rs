#[derive(Clone)]
pub struct MaintenanceScheduleExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::MaintenanceSchedule>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MaintenanceScheduleExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::MaintenanceSchedule>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::MaintenanceSchedule> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::MaintenanceSchedule> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::MaintenanceSchedule {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_schedule_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("schedule_id", |entity| entity.eval_schedule_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_interval_days(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("interval_days", |entity| entity.eval_interval_days());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_vehicle_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("vehicle_ref_id", |entity| entity.eval_vehicle_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_vehicle_ref(self) -> crate::VehicleExpression<'a> {
        let next = self.result.and_then("vehicle_ref", |entity| entity.eval_vehicle_ref());
        crate::VehicleExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MaintenanceScheduleListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MaintenanceSchedule>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MaintenanceScheduleListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::MaintenanceSchedule>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::MaintenanceSchedule>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::MaintenanceSchedule>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::MaintenanceSchedule> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MaintenanceScheduleExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MaintenanceScheduleExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MaintenanceScheduleExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MaintenanceScheduleExpression::new(next, self.root_desc.clone())
    }
}