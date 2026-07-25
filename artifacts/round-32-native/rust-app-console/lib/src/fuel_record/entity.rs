// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/fuel_record
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "FuelRecord", table = "fuel_record_data", data_service = "sqlite")]
pub struct FuelRecord {
#[teaql(id)]
    id: u64,

// @source module_8.xml:8
    recorded_at: chrono::DateTime<chrono::Utc>,

// @source module_8.xml:8
    gallons: String,

// @source module_8.xml:8
    unit_price: String,

// @source module_8.xml:8
    total_cost: String,

// @source module_8.xml:8
    odometer: i64,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl FuelRecord {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            recorded_at: chrono::Utc::now(),
            gallons: String::new(),
            unit_price: String::new(),
            total_cost: String::new(),
            odometer: 0_i64,
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("FuelRecord", self.id)
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

    pub fn recorded_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_recorded_at().and_then(|value| value.try_timestamp()).unwrap_or(self.recorded_at)
    }

    pub fn update_recorded_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.recorded_at = value.try_timestamp().unwrap_or(self.recorded_at.clone());
        self.root.set(self.entity_key(), "recorded_at", value);
        self
    }

    pub fn changed_recorded_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "recorded_at")
    }

    pub fn eval_recorded_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("recorded_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "recorded_at".to_string(), attempted_path: "recorded_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.recorded_at())
                }}

    pub fn gallons(&self) -> String {
        self.changed_gallons().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.gallons.clone())
    }

    pub fn update_gallons(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.gallons = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.gallons.clone());
        self.root.set(self.entity_key(), "gallons", value);
        self
    }

    pub fn changed_gallons(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "gallons")
    }

    pub fn eval_gallons(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("gallons") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "gallons".to_string(), attempted_path: "gallons".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.gallons())
                }}

    pub fn unit_price(&self) -> String {
        self.changed_unit_price().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.unit_price.clone())
    }

    pub fn update_unit_price(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.unit_price = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.unit_price.clone());
        self.root.set(self.entity_key(), "unit_price", value);
        self
    }

    pub fn changed_unit_price(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "unit_price")
    }

    pub fn eval_unit_price(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("unit_price") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "unit_price".to_string(), attempted_path: "unit_price".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.unit_price())
                }}

    pub fn total_cost(&self) -> String {
        self.changed_total_cost().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.total_cost.clone())
    }

    pub fn update_total_cost(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.total_cost = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.total_cost.clone());
        self.root.set(self.entity_key(), "total_cost", value);
        self
    }

    pub fn changed_total_cost(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "total_cost")
    }

    pub fn eval_total_cost(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("total_cost") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "total_cost".to_string(), attempted_path: "total_cost".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.total_cost())
                }}

    pub fn odometer(&self) -> i64 {
        self.changed_odometer().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.odometer)
    }

    pub fn update_odometer(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.odometer = value.try_i64().map(|value| value as i64).unwrap_or(self.odometer.clone());
        self.root.set(self.entity_key(), "odometer", value);
        self
    }

    pub fn changed_odometer(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "odometer")
    }

    pub fn eval_odometer(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("odometer") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "odometer".to_string(), attempted_path: "odometer".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.odometer())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
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
            .fuel_record_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("FuelRecord"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

