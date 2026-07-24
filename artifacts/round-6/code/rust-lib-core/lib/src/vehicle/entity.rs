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

// @source model.xml:102
    license_plate: String,

// @source model.xml:102
    model: String,

// @source model.xml:102
    capacity_kg: String,

// @source model.xml:102
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:102
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:102
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:102
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
#[teaql(relation(target = "VehicleAssignment", local_key = "id", foreign_key = "vehicle_id", many))]
    vehicle_assignment_list: SmartList<crate::VehicleAssignment>,
#[teaql(relation(target = "FuelLog", local_key = "id", foreign_key = "vehicle_id", many))]
    fuel_log_list: SmartList<crate::FuelLog>,
#[teaql(relation(target = "MaintenanceRecord", local_key = "id", foreign_key = "vehicle_id", many))]
    maintenance_record_list: SmartList<crate::MaintenanceRecord>,
#[teaql(relation(target = "TollReceipt", local_key = "id", foreign_key = "vehicle_id", many))]
    toll_receipt_list: SmartList<crate::TollReceipt>,
#[teaql(relation(target = "IncidentReport", local_key = "id", foreign_key = "vehicle_id", many))]
    incident_report_list: SmartList<crate::IncidentReport>,
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
            license_plate: String::new(),
            model: String::new(),
            capacity_kg: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            merchant_id: 0_u64,
            merchant: None,
            vehicle_assignment_list: Default::default(),
            fuel_log_list: Default::default(),
            maintenance_record_list: Default::default(),
            toll_receipt_list: Default::default(),
            incident_report_list: Default::default(),
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
        if let Some(entity) = &mut self.merchant {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.fuel_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.maintenance_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.toll_receipt_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.incident_report_list {
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

    pub fn license_plate(&self) -> String {
        self.changed_license_plate().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.license_plate.clone())
    }

    pub fn update_license_plate(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.license_plate = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.license_plate.clone());
        self.root.set(self.entity_key(), "license_plate", value);
        self
    }

    pub fn changed_license_plate(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "license_plate")
    }

    pub fn eval_license_plate(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("license_plate") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "license_plate".to_string(), attempted_path: "license_plate".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.license_plate())
                }}

    pub fn model(&self) -> String {
        self.changed_model().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.model.clone())
    }

    pub fn update_model(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.model = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.model.clone());
        self.root.set(self.entity_key(), "model", value);
        self
    }

    pub fn changed_model(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "model")
    }

    pub fn eval_model(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("model") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "model".to_string(), attempted_path: "model".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.model())
                }}

    pub fn capacity_kg(&self) -> String {
        self.changed_capacity_kg().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.capacity_kg.clone())
    }

    pub fn update_capacity_kg(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.capacity_kg = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.capacity_kg.clone());
        self.root.set(self.entity_key(), "capacity_kg", value);
        self
    }

    pub fn changed_capacity_kg(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "capacity_kg")
    }

    pub fn eval_capacity_kg(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("capacity_kg") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "capacity_kg".to_string(), attempted_path: "capacity_kg".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.capacity_kg())
                }}

    pub fn create_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.create_time = value.try_timestamp().unwrap_or(self.create_time.clone());
        self.root.set(self.entity_key(), "create_time", value);
        self
    }

    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}

    pub fn update_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.update_time = value.try_timestamp().unwrap_or(self.update_time.clone());
        self.root.set(self.entity_key(), "update_time", value);
        self
    }

    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("update_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "update_time".to_string(), attempted_path: "update_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.update_time())
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
    pub fn merchant_id(&self) -> u64 {
        self.changed_merchant_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_id)
    }

    pub fn update_merchant_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_id = value.try_u64().unwrap_or(self.merchant_id.clone());
        self.root.set(self.entity_key(), "merchant_id", value);
        self
    }

    pub fn changed_merchant_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_id")
    }

    pub fn eval_merchant_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_id".to_string(), attempted_path: "merchant_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_id())
                }}
    pub fn merchant(&self) -> Option<&crate::Merchant> {
        self.merchant.as_ref()
    }

    pub fn eval_merchant(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant".to_string(), attempted_path: "merchant".to_string() }
        } else {
            match &self.merchant {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn vehicle_assignment_list(&self) -> &SmartList<crate::VehicleAssignment> {
        &self.vehicle_assignment_list
    }

    pub fn vehicle_assignment_list_mut(&mut self) -> &mut SmartList<crate::VehicleAssignment> {
        &mut self.vehicle_assignment_list
    }

    pub fn eval_vehicle_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::VehicleAssignment>> {
        if !self.is_loaded("vehicle_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_assignment_list".to_string(), attempted_path: "vehicle_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.vehicle_assignment_list)
        }
    }

    pub fn fuel_log_list(&self) -> &SmartList<crate::FuelLog> {
        &self.fuel_log_list
    }

    pub fn fuel_log_list_mut(&mut self) -> &mut SmartList<crate::FuelLog> {
        &mut self.fuel_log_list
    }

    pub fn eval_fuel_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::FuelLog>> {
        if !self.is_loaded("fuel_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "fuel_log_list".to_string(), attempted_path: "fuel_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.fuel_log_list)
        }
    }

    pub fn maintenance_record_list(&self) -> &SmartList<crate::MaintenanceRecord> {
        &self.maintenance_record_list
    }

    pub fn maintenance_record_list_mut(&mut self) -> &mut SmartList<crate::MaintenanceRecord> {
        &mut self.maintenance_record_list
    }

    pub fn eval_maintenance_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MaintenanceRecord>> {
        if !self.is_loaded("maintenance_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "maintenance_record_list".to_string(), attempted_path: "maintenance_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.maintenance_record_list)
        }
    }

    pub fn toll_receipt_list(&self) -> &SmartList<crate::TollReceipt> {
        &self.toll_receipt_list
    }

    pub fn toll_receipt_list_mut(&mut self) -> &mut SmartList<crate::TollReceipt> {
        &mut self.toll_receipt_list
    }

    pub fn eval_toll_receipt_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TollReceipt>> {
        if !self.is_loaded("toll_receipt_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "toll_receipt_list".to_string(), attempted_path: "toll_receipt_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.toll_receipt_list)
        }
    }

    pub fn incident_report_list(&self) -> &SmartList<crate::IncidentReport> {
        &self.incident_report_list
    }

    pub fn incident_report_list_mut(&mut self) -> &mut SmartList<crate::IncidentReport> {
        &mut self.incident_report_list
    }

    pub fn eval_incident_report_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::IncidentReport>> {
        if !self.is_loaded("incident_report_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "incident_report_list".to_string(), attempted_path: "incident_report_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.incident_report_list)
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

