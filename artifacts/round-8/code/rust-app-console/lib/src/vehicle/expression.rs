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

    pub fn get_make(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("make", |entity| entity.eval_make());
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
    pub fn get_asset_assignment_list(self) -> crate::AssetAssignmentListExpression<'a> {
        let next = self.result.and_then("asset_assignment_list", |entity| entity.eval_asset_assignment_list());
        crate::AssetAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_asset_inspection_list(self) -> crate::AssetInspectionListExpression<'a> {
        let next = self.result.and_then("asset_inspection_list", |entity| entity.eval_asset_inspection_list());
        crate::AssetInspectionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_maintenance_schedule_list(self) -> crate::MaintenanceScheduleListExpression<'a> {
        let next = self.result.and_then("maintenance_schedule_list", |entity| entity.eval_maintenance_schedule_list());
        crate::MaintenanceScheduleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_maintenance_event_list(self) -> crate::MaintenanceEventListExpression<'a> {
        let next = self.result.and_then("maintenance_event_list", |entity| entity.eval_maintenance_event_list());
        crate::MaintenanceEventListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fuel_record_list(self) -> crate::FuelRecordListExpression<'a> {
        let next = self.result.and_then("fuel_record_list", |entity| entity.eval_fuel_record_list());
        crate::FuelRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_registration_list(self) -> crate::VehicleRegistrationListExpression<'a> {
        let next = self.result.and_then("vehicle_registration_list", |entity| entity.eval_vehicle_registration_list());
        crate::VehicleRegistrationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_asset_condition_list(self) -> crate::AssetConditionListExpression<'a> {
        let next = self.result.and_then("asset_condition_list", |entity| entity.eval_asset_condition_list());
        crate::AssetConditionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_depreciation_record_list(self) -> crate::DepreciationRecordListExpression<'a> {
        let next = self.result.and_then("depreciation_record_list", |entity| entity.eval_depreciation_record_list());
        crate::DepreciationRecordListExpression::new(next, self.root_desc.clone())
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