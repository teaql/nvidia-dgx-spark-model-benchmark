// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/vehicle_load_plan
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "VehicleLoadPlan", table = "vehicle_load_plan_data", data_service = "sqlite")]
pub struct VehicleLoadPlan {
#[teaql(id)]
    id: u64,

// @source module_1.xml:6
    vehicle_reference: String,

// @source module_1.xml:6
    total_weight: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl VehicleLoadPlan {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            vehicle_reference: String::new(),
            total_weight: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("VehicleLoadPlan", self.id)
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

    pub fn vehicle_reference(&self) -> String {
        self.changed_vehicle_reference().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.vehicle_reference.clone())
    }

    pub fn update_vehicle_reference(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.vehicle_reference = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.vehicle_reference.clone());
        self.root.set(self.entity_key(), "vehicle_reference", value);
        self
    }

    pub fn changed_vehicle_reference(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "vehicle_reference")
    }

    pub fn eval_vehicle_reference(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("vehicle_reference") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_reference".to_string(), attempted_path: "vehicle_reference".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.vehicle_reference())
                }}

    pub fn total_weight(&self) -> String {
        self.changed_total_weight().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.total_weight.clone())
    }

    pub fn update_total_weight(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.total_weight = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.total_weight.clone());
        self.root.set(self.entity_key(), "total_weight", value);
        self
    }

    pub fn changed_total_weight(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "total_weight")
    }

    pub fn eval_total_weight(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("total_weight") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "total_weight".to_string(), attempted_path: "total_weight".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.total_weight())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::VehicleLoadPlanRepository<'a>>>
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
            .vehicle_load_plan_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("VehicleLoadPlan"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

