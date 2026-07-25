// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/custom_entity_215
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CustomEntity215", table = "custom_entity_215_data", data_service = "sqlite")]
pub struct CustomEntity215 {
#[teaql(id)]
    id: u64,

// @source module_14.xml:7
    title: String,

// @source module_14.xml:7
    summary: String,

// @source module_14.xml:7
    date_created: chrono::DateTime<chrono::Utc>,

// @source module_14.xml:7
    active: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CustomEntity215 {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            title: String::new(),
            summary: String::new(),
            date_created: chrono::Utc::now(),
            active: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CustomEntity215", self.id)
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

    pub fn title(&self) -> String {
        self.changed_title().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.title.clone())
    }

    pub fn update_title(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.title = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.title.clone());
        self.root.set(self.entity_key(), "title", value);
        self
    }

    pub fn changed_title(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "title")
    }

    pub fn eval_title(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("title") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "title".to_string(), attempted_path: "title".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.title())
                }}

    pub fn summary(&self) -> String {
        self.changed_summary().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.summary.clone())
    }

    pub fn update_summary(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.summary = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.summary.clone());
        self.root.set(self.entity_key(), "summary", value);
        self
    }

    pub fn changed_summary(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "summary")
    }

    pub fn eval_summary(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("summary") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "summary".to_string(), attempted_path: "summary".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.summary())
                }}

    pub fn date_created(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_date_created().and_then(|value| value.try_timestamp()).unwrap_or(self.date_created)
    }

    pub fn update_date_created(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.date_created = value.try_timestamp().unwrap_or(self.date_created.clone());
        self.root.set(self.entity_key(), "date_created", value);
        self
    }

    pub fn changed_date_created(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "date_created")
    }

    pub fn eval_date_created(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("date_created") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "date_created".to_string(), attempted_path: "date_created".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.date_created())
                }}

    pub fn active(&self) -> String {
        self.changed_active().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.active.clone())
    }

    pub fn update_active(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.active = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.active.clone());
        self.root.set(self.entity_key(), "active", value);
        self
    }

    pub fn changed_active(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "active")
    }

    pub fn eval_active(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("active") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "active".to_string(), attempted_path: "active".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.active())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CustomEntity215Repository<'a>>>
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
            .custom_entity_215_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CustomEntity215"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

