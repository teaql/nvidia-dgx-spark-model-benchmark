// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/activity_log
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
#[teaql(entity = "ActivityLog", table = "activity_log_data", data_service = "sqlite")]
pub struct ActivityLog {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    log_id: String,

// @source model.xml:2
    action: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "user_account_ref")]
    user_account_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "UserAccount", local_key = "user_account_ref_id", foreign_key = "id"))]
    user_account_ref: Option<crate::UserAccount>,
#[teaql(relation(target = "EntityChange", local_key = "id", foreign_key = "activity_log_ref_id", many))]
    entity_change_list: SmartList<crate::EntityChange>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl ActivityLog {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            log_id: String::new(),
            action: String::new(),
            version: 0_i64,
            user_account_ref_id: 0_u64,
            user_account_ref: None,
            entity_change_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("ActivityLog", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.user_account_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.entity_change_list {
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

    pub fn log_id(&self) -> String {
        self.changed_log_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.log_id.clone())
    }

    pub fn update_log_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.log_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.log_id.clone());
        self.root.set(self.entity_key(), "log_id", value);
        self
    }

    pub fn changed_log_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "log_id")
    }

    pub fn eval_log_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("log_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "log_id".to_string(), attempted_path: "log_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.log_id())
                }}

    pub fn action(&self) -> String {
        self.changed_action().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.action.clone())
    }

    pub fn update_action(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.action = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.action.clone());
        self.root.set(self.entity_key(), "action", value);
        self
    }

    pub fn changed_action(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "action")
    }

    pub fn eval_action(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("action") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "action".to_string(), attempted_path: "action".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.action())
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
    pub fn user_account_ref_id(&self) -> u64 {
        self.changed_user_account_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.user_account_ref_id)
    }

    pub fn update_user_account_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.user_account_ref_id = value.try_u64().unwrap_or(self.user_account_ref_id.clone());
        self.root.set(self.entity_key(), "user_account_ref_id", value);
        self
    }

    pub fn changed_user_account_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "user_account_ref_id")
    }

    pub fn eval_user_account_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("user_account_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_account_ref_id".to_string(), attempted_path: "user_account_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.user_account_ref_id())
                }}
    pub fn user_account_ref(&self) -> Option<&crate::UserAccount> {
        self.user_account_ref.as_ref()
    }

    pub fn eval_user_account_ref(&self) -> teaql_core::eval::EvalResult<&crate::UserAccount> {
        if !self.is_loaded("user_account_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_account_ref".to_string(), attempted_path: "user_account_ref".to_string() }
        } else {
            match &self.user_account_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn entity_change_list(&self) -> &SmartList<crate::EntityChange> {
        &self.entity_change_list
    }

    pub fn entity_change_list_mut(&mut self) -> &mut SmartList<crate::EntityChange> {
        &mut self.entity_change_list
    }

    pub fn eval_entity_change_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EntityChange>> {
        if !self.is_loaded("entity_change_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "entity_change_list".to_string(), attempted_path: "entity_change_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.entity_change_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ActivityLogRepository<'a>>>
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
            .activity_log_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("ActivityLog"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

