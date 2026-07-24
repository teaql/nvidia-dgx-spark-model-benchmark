// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/change_set
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "ChangeSet", table = "change_set_data", data_service = "sqlite")]
pub struct ChangeSet {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    set_id: String,

// @source model.xml:2
    version: String,
// @source model.xml:2
#[teaql(column = "entity_change_ref")]
    entity_change_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "EntityChange", local_key = "entity_change_ref_id", foreign_key = "id"))]
    entity_change_ref: Option<crate::EntityChange>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl ChangeSet {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            set_id: String::new(),
            version: String::new(),
            entity_change_ref_id: 0_u64,
            entity_change_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("ChangeSet", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.entity_change_ref {
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

    pub fn set_id(&self) -> String {
        self.changed_set_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.set_id.clone())
    }

    pub fn update_set_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.set_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.set_id.clone());
        self.root.set(self.entity_key(), "set_id", value);
        self
    }

    pub fn changed_set_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "set_id")
    }

    pub fn eval_set_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("set_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "set_id".to_string(), attempted_path: "set_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.set_id())
                }}

    pub fn version(&self) -> String {
        self.changed_version().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.version.clone())
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn entity_change_ref_id(&self) -> u64 {
        self.changed_entity_change_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.entity_change_ref_id)
    }

    pub fn update_entity_change_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.entity_change_ref_id = value.try_u64().unwrap_or(self.entity_change_ref_id.clone());
        self.root.set(self.entity_key(), "entity_change_ref_id", value);
        self
    }

    pub fn changed_entity_change_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "entity_change_ref_id")
    }

    pub fn eval_entity_change_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("entity_change_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "entity_change_ref_id".to_string(), attempted_path: "entity_change_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.entity_change_ref_id())
                }}
    pub fn entity_change_ref(&self) -> Option<&crate::EntityChange> {
        self.entity_change_ref.as_ref()
    }

    pub fn eval_entity_change_ref(&self) -> teaql_core::eval::EvalResult<&crate::EntityChange> {
        if !self.is_loaded("entity_change_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "entity_change_ref".to_string(), attempted_path: "entity_change_ref".to_string() }
        } else {
            match &self.entity_change_ref {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ChangeSetRepository<'a>>>
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
            .change_set_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("ChangeSet"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

