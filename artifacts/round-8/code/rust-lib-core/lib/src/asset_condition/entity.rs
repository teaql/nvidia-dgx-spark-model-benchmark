// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/asset_condition
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "AssetCondition", table = "asset_condition_data", data_service = "sqlite")]
pub struct AssetCondition {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    condition_id: String,

// @source model.xml:2
    rating: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "vehicle_ref")]
    vehicle_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Vehicle", local_key = "vehicle_ref_id", foreign_key = "id"))]
    vehicle_ref: Option<crate::Vehicle>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl AssetCondition {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            condition_id: String::new(),
            rating: String::new(),
            version: 0_i64,
            vehicle_ref_id: 0_u64,
            vehicle_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("AssetCondition", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.vehicle_ref {
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

    pub fn condition_id(&self) -> String {
        self.changed_condition_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.condition_id.clone())
    }

    pub fn update_condition_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.condition_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.condition_id.clone());
        self.root.set(self.entity_key(), "condition_id", value);
        self
    }

    pub fn changed_condition_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "condition_id")
    }

    pub fn eval_condition_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("condition_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "condition_id".to_string(), attempted_path: "condition_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.condition_id())
                }}

    pub fn rating(&self) -> String {
        self.changed_rating().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.rating.clone())
    }

    pub fn update_rating(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.rating = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.rating.clone());
        self.root.set(self.entity_key(), "rating", value);
        self
    }

    pub fn changed_rating(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "rating")
    }

    pub fn eval_rating(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("rating") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "rating".to_string(), attempted_path: "rating".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.rating())
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
    pub fn vehicle_ref_id(&self) -> u64 {
        self.changed_vehicle_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.vehicle_ref_id)
    }

    pub fn update_vehicle_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.vehicle_ref_id = value.try_u64().unwrap_or(self.vehicle_ref_id.clone());
        self.root.set(self.entity_key(), "vehicle_ref_id", value);
        self
    }

    pub fn changed_vehicle_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "vehicle_ref_id")
    }

    pub fn eval_vehicle_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("vehicle_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_ref_id".to_string(), attempted_path: "vehicle_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.vehicle_ref_id())
                }}
    pub fn vehicle_ref(&self) -> Option<&crate::Vehicle> {
        self.vehicle_ref.as_ref()
    }

    pub fn eval_vehicle_ref(&self) -> teaql_core::eval::EvalResult<&crate::Vehicle> {
        if !self.is_loaded("vehicle_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_ref".to_string(), attempted_path: "vehicle_ref".to_string() }
        } else {
            match &self.vehicle_ref {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::AssetConditionRepository<'a>>>
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
            .asset_condition_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("AssetCondition"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

