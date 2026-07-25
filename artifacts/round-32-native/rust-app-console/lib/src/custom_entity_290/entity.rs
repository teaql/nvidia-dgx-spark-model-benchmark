// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/custom_entity_290
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CustomEntity290", table = "custom_entity_290_data", data_service = "sqlite")]
pub struct CustomEntity290 {
#[teaql(id)]
    id: u64,

// @source module_19.xml:7
    score: i64,

// @source module_19.xml:7
    max_score: i64,

// @source module_19.xml:7
    evaluated_at: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CustomEntity290 {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            score: 0_i64,
            max_score: 0_i64,
            evaluated_at: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CustomEntity290", self.id)
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

    pub fn score(&self) -> i64 {
        self.changed_score().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.score)
    }

    pub fn update_score(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.score = value.try_i64().map(|value| value as i64).unwrap_or(self.score.clone());
        self.root.set(self.entity_key(), "score", value);
        self
    }

    pub fn changed_score(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "score")
    }

    pub fn eval_score(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("score") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "score".to_string(), attempted_path: "score".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.score())
                }}

    pub fn max_score(&self) -> i64 {
        self.changed_max_score().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.max_score)
    }

    pub fn update_max_score(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.max_score = value.try_i64().map(|value| value as i64).unwrap_or(self.max_score.clone());
        self.root.set(self.entity_key(), "max_score", value);
        self
    }

    pub fn changed_max_score(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "max_score")
    }

    pub fn eval_max_score(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("max_score") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "max_score".to_string(), attempted_path: "max_score".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.max_score())
                }}

    pub fn evaluated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_evaluated_at().and_then(|value| value.try_timestamp()).unwrap_or(self.evaluated_at)
    }

    pub fn update_evaluated_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.evaluated_at = value.try_timestamp().unwrap_or(self.evaluated_at.clone());
        self.root.set(self.entity_key(), "evaluated_at", value);
        self
    }

    pub fn changed_evaluated_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "evaluated_at")
    }

    pub fn eval_evaluated_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("evaluated_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "evaluated_at".to_string(), attempted_path: "evaluated_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.evaluated_at())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CustomEntity290Repository<'a>>>
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
            .custom_entity_290_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CustomEntity290"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

