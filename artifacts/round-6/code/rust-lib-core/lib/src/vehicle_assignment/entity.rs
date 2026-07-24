// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/vehicle_assignment
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "VehicleAssignment", table = "vehicle_assignment_data", data_service = "sqlite")]
pub struct VehicleAssignment {
#[teaql(id)]
    id: u64,

// @source model.xml:110
    assignment_date: String,

// @source model.xml:110
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:110
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:110
#[teaql(column = "crew")]
    crew_id: u64,

// @source model.xml:110
#[teaql(column = "vehicle")]
    vehicle_id: u64,

// @source model.xml:110
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:110
#[teaql(relation(target = "Crew", local_key = "crew_id", foreign_key = "id"))]
    crew: Option<crate::Crew>,

// @source model.xml:110
#[teaql(relation(target = "Vehicle", local_key = "vehicle_id", foreign_key = "id"))]
    vehicle: Option<crate::Vehicle>,

// @source model.xml:110
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl VehicleAssignment {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            assignment_date: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            crew_id: 0_u64,
            vehicle_id: 0_u64,
            merchant_id: 0_u64,
            crew: None,
            vehicle: None,
            merchant: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("VehicleAssignment", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.crew {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.vehicle {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
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

    pub fn assignment_date(&self) -> String {
        self.changed_assignment_date().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.assignment_date.clone())
    }

    pub fn update_assignment_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.assignment_date = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.assignment_date.clone());
        self.root.set(self.entity_key(), "assignment_date", value);
        self
    }

    pub fn changed_assignment_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "assignment_date")
    }

    pub fn eval_assignment_date(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("assignment_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "assignment_date".to_string(), attempted_path: "assignment_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.assignment_date())
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
    pub fn crew_id(&self) -> u64 {
        self.changed_crew_id().and_then(|value| value.try_u64()).unwrap_or(self.crew_id)
    }

    pub fn update_crew_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.crew_id = value.try_u64().unwrap_or(self.crew_id.clone());
        self.root.set(self.entity_key(), "crew_id", value);
        self
    }

    pub fn changed_crew_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "crew_id")
    }

    pub fn eval_crew_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("crew_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "crew_id".to_string(), attempted_path: "crew_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.crew_id())
                }}

    pub fn vehicle_id(&self) -> u64 {
        self.changed_vehicle_id().and_then(|value| value.try_u64()).unwrap_or(self.vehicle_id)
    }

    pub fn update_vehicle_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.vehicle_id = value.try_u64().unwrap_or(self.vehicle_id.clone());
        self.root.set(self.entity_key(), "vehicle_id", value);
        self
    }

    pub fn changed_vehicle_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "vehicle_id")
    }

    pub fn eval_vehicle_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("vehicle_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_id".to_string(), attempted_path: "vehicle_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.vehicle_id())
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
    pub fn crew(&self) -> Option<&crate::Crew> {
        self.crew.as_ref()
    }

    pub fn eval_crew(&self) -> teaql_core::eval::EvalResult<&crate::Crew> {
        if !self.is_loaded("crew") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "crew".to_string(), attempted_path: "crew".to_string() }
        } else {
            match &self.crew {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn vehicle(&self) -> Option<&crate::Vehicle> {
        self.vehicle.as_ref()
    }

    pub fn eval_vehicle(&self) -> teaql_core::eval::EvalResult<&crate::Vehicle> {
        if !self.is_loaded("vehicle") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle".to_string(), attempted_path: "vehicle".to_string() }
        } else {
            match &self.vehicle {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::VehicleAssignmentRepository<'a>>>
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
            .vehicle_assignment_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("VehicleAssignment"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

