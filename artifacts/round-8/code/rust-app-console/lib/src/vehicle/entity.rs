// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/vehicle
use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Vehicle", table = "vehicle_data", data_service = "sqlite")]
pub struct Vehicle {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    vehicle_id: String,

// @source model.xml:2
    make: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "merchant_ref")]
    merchant_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Merchant", local_key = "merchant_ref_id", foreign_key = "id"))]
    merchant_ref: Option<crate::Merchant>,
#[teaql(relation(target = "AssetAssignment", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    asset_assignment_list: SmartList<crate::AssetAssignment>,
#[teaql(relation(target = "AssetInspection", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    asset_inspection_list: SmartList<crate::AssetInspection>,
#[teaql(relation(target = "MaintenanceSchedule", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    maintenance_schedule_list: SmartList<crate::MaintenanceSchedule>,
#[teaql(relation(target = "MaintenanceEvent", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    maintenance_event_list: SmartList<crate::MaintenanceEvent>,
#[teaql(relation(target = "FuelRecord", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    fuel_record_list: SmartList<crate::FuelRecord>,
#[teaql(relation(target = "VehicleRegistration", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    vehicle_registration_list: SmartList<crate::VehicleRegistration>,
#[teaql(relation(target = "AssetCondition", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    asset_condition_list: SmartList<crate::AssetCondition>,
#[teaql(relation(target = "DepreciationRecord", local_key = "id", foreign_key = "vehicle_ref_id", many))]
    depreciation_record_list: SmartList<crate::DepreciationRecord>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Vehicle {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            vehicle_id: String::new(),
            make: String::new(),
            version: 0_i64,
            merchant_ref_id: 0_u64,
            merchant_ref: None,
            asset_assignment_list: Default::default(),
            asset_inspection_list: Default::default(),
            maintenance_schedule_list: Default::default(),
            maintenance_event_list: Default::default(),
            fuel_record_list: Default::default(),
            vehicle_registration_list: Default::default(),
            asset_condition_list: Default::default(),
            depreciation_record_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Vehicle", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.merchant_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.asset_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.asset_inspection_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.maintenance_schedule_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.maintenance_event_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.fuel_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_registration_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.asset_condition_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.depreciation_record_list {
            entity.attach_root_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn vehicle_id(&self) -> String {
        self.changed_vehicle_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.vehicle_id.clone())
    }

    pub fn update_vehicle_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.vehicle_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.vehicle_id.clone());
        self.root.set(self.entity_key(), "vehicle_id", value);
        self
    }

    pub fn changed_vehicle_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "vehicle_id")
    }

    pub fn eval_vehicle_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("vehicle_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_id".to_string(), attempted_path: "vehicle_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.vehicle_id())
                }}

    pub fn make(&self) -> String {
        self.changed_make().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.make.clone())
    }

    pub fn update_make(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.make = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.make.clone());
        self.root.set(self.entity_key(), "make", value);
        self
    }

    pub fn changed_make(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "make")
    }

    pub fn eval_make(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("make") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "make".to_string(), attempted_path: "make".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.make())
                }}

    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn merchant_ref_id(&self) -> u64 {
        self.changed_merchant_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_ref_id)
    }

    pub fn update_merchant_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_ref_id = value.try_u64().unwrap_or(self.merchant_ref_id.clone());
        self.root.set(self.entity_key(), "merchant_ref_id", value);
        self
    }

    pub fn changed_merchant_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_ref_id")
    }

    pub fn eval_merchant_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref_id".to_string(), attempted_path: "merchant_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_ref_id())
                }}
    pub fn merchant_ref(&self) -> Option<&crate::Merchant> {
        self.merchant_ref.as_ref()
    }

    pub fn eval_merchant_ref(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref".to_string(), attempted_path: "merchant_ref".to_string() }
        } else {
            match &self.merchant_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn asset_assignment_list(&self) -> &SmartList<crate::AssetAssignment> {
        &self.asset_assignment_list
    }

    pub fn asset_assignment_list_mut(&mut self) -> &mut SmartList<crate::AssetAssignment> {
        &mut self.asset_assignment_list
    }

    pub fn eval_asset_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AssetAssignment>> {
        if !self.is_loaded("asset_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "asset_assignment_list".to_string(), attempted_path: "asset_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.asset_assignment_list)
        }
    }

    pub fn asset_inspection_list(&self) -> &SmartList<crate::AssetInspection> {
        &self.asset_inspection_list
    }

    pub fn asset_inspection_list_mut(&mut self) -> &mut SmartList<crate::AssetInspection> {
        &mut self.asset_inspection_list
    }

    pub fn eval_asset_inspection_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AssetInspection>> {
        if !self.is_loaded("asset_inspection_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "asset_inspection_list".to_string(), attempted_path: "asset_inspection_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.asset_inspection_list)
        }
    }

    pub fn maintenance_schedule_list(&self) -> &SmartList<crate::MaintenanceSchedule> {
        &self.maintenance_schedule_list
    }

    pub fn maintenance_schedule_list_mut(&mut self) -> &mut SmartList<crate::MaintenanceSchedule> {
        &mut self.maintenance_schedule_list
    }

    pub fn eval_maintenance_schedule_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MaintenanceSchedule>> {
        if !self.is_loaded("maintenance_schedule_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "maintenance_schedule_list".to_string(), attempted_path: "maintenance_schedule_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.maintenance_schedule_list)
        }
    }

    pub fn maintenance_event_list(&self) -> &SmartList<crate::MaintenanceEvent> {
        &self.maintenance_event_list
    }

    pub fn maintenance_event_list_mut(&mut self) -> &mut SmartList<crate::MaintenanceEvent> {
        &mut self.maintenance_event_list
    }

    pub fn eval_maintenance_event_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MaintenanceEvent>> {
        if !self.is_loaded("maintenance_event_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "maintenance_event_list".to_string(), attempted_path: "maintenance_event_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.maintenance_event_list)
        }
    }

    pub fn fuel_record_list(&self) -> &SmartList<crate::FuelRecord> {
        &self.fuel_record_list
    }

    pub fn fuel_record_list_mut(&mut self) -> &mut SmartList<crate::FuelRecord> {
        &mut self.fuel_record_list
    }

    pub fn eval_fuel_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::FuelRecord>> {
        if !self.is_loaded("fuel_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "fuel_record_list".to_string(), attempted_path: "fuel_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.fuel_record_list)
        }
    }

    pub fn vehicle_registration_list(&self) -> &SmartList<crate::VehicleRegistration> {
        &self.vehicle_registration_list
    }

    pub fn vehicle_registration_list_mut(&mut self) -> &mut SmartList<crate::VehicleRegistration> {
        &mut self.vehicle_registration_list
    }

    pub fn eval_vehicle_registration_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::VehicleRegistration>> {
        if !self.is_loaded("vehicle_registration_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_registration_list".to_string(), attempted_path: "vehicle_registration_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.vehicle_registration_list)
        }
    }

    pub fn asset_condition_list(&self) -> &SmartList<crate::AssetCondition> {
        &self.asset_condition_list
    }

    pub fn asset_condition_list_mut(&mut self) -> &mut SmartList<crate::AssetCondition> {
        &mut self.asset_condition_list
    }

    pub fn eval_asset_condition_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AssetCondition>> {
        if !self.is_loaded("asset_condition_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "asset_condition_list".to_string(), attempted_path: "asset_condition_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.asset_condition_list)
        }
    }

    pub fn depreciation_record_list(&self) -> &SmartList<crate::DepreciationRecord> {
        &self.depreciation_record_list
    }

    pub fn depreciation_record_list_mut(&mut self) -> &mut SmartList<crate::DepreciationRecord> {
        &mut self.depreciation_record_list
    }

    pub fn eval_depreciation_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DepreciationRecord>> {
        if !self.is_loaded("depreciation_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "depreciation_record_list".to_string(), attempted_path: "depreciation_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.depreciation_record_list)
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }

    pub(crate) async fn save<'a, C>(
        &self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let root = ctx.user_context().entity_root();
        let key = self.entity_key();
        let has_ledger_change = (self.id != 0)
            && (root.current_change_set().changes().contains_key(&key)
                || root.is_marked_as_delete(&key)
                || root.is_new(&key));
        let repository = ctx
            .vehicle_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Vehicle"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

