// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/vehicle
use std::collections::BTreeMap;

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

// @source module_7.xml:16
    make: String,

// @source module_7.xml:16
    vehicle_model: String,

// @source module_7.xml:16
    year: String,

// @source module_7.xml:16
    license_plate: String,
#[teaql(version)]
    version: i64,
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
            make: String::new(),
            vehicle_model: String::new(),
            year: String::new(),
            license_plate: String::new(),
            version: 0_i64,
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

    pub fn vehicle_model(&self) -> String {
        self.changed_vehicle_model().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.vehicle_model.clone())
    }

    pub fn update_vehicle_model(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.vehicle_model = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.vehicle_model.clone());
        self.root.set(self.entity_key(), "vehicle_model", value);
        self
    }

    pub fn changed_vehicle_model(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "vehicle_model")
    }

    pub fn eval_vehicle_model(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("vehicle_model") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_model".to_string(), attempted_path: "vehicle_model".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.vehicle_model())
                }}

    pub fn year(&self) -> String {
        self.changed_year().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.year.clone())
    }

    pub fn update_year(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.year = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.year.clone());
        self.root.set(self.entity_key(), "year", value);
        self
    }

    pub fn changed_year(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "year")
    }

    pub fn eval_year(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("year") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "year".to_string(), attempted_path: "year".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.year())
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

