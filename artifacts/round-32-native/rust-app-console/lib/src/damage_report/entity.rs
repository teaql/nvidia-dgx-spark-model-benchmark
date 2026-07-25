// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/damage_report
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "DamageReport", table = "damage_report_data", data_service = "sqlite")]
pub struct DamageReport {
#[teaql(id)]
    id: u64,

// @source module_1.xml:2
    item_description: String,

// @source module_1.xml:2
    severity: String,

// @source module_1.xml:2
    date_reported: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl DamageReport {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            item_description: String::new(),
            severity: String::new(),
            date_reported: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("DamageReport", self.id)
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

    pub fn item_description(&self) -> String {
        self.changed_item_description().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.item_description.clone())
    }

    pub fn update_item_description(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.item_description = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.item_description.clone());
        self.root.set(self.entity_key(), "item_description", value);
        self
    }

    pub fn changed_item_description(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "item_description")
    }

    pub fn eval_item_description(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("item_description") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "item_description".to_string(), attempted_path: "item_description".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.item_description())
                }}

    pub fn severity(&self) -> String {
        self.changed_severity().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.severity.clone())
    }

    pub fn update_severity(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.severity = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.severity.clone());
        self.root.set(self.entity_key(), "severity", value);
        self
    }

    pub fn changed_severity(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "severity")
    }

    pub fn eval_severity(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("severity") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "severity".to_string(), attempted_path: "severity".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.severity())
                }}

    pub fn date_reported(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_date_reported().and_then(|value| value.try_timestamp()).unwrap_or(self.date_reported)
    }

    pub fn update_date_reported(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.date_reported = value.try_timestamp().unwrap_or(self.date_reported.clone());
        self.root.set(self.entity_key(), "date_reported", value);
        self
    }

    pub fn changed_date_reported(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "date_reported")
    }

    pub fn eval_date_reported(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("date_reported") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "date_reported".to_string(), attempted_path: "date_reported".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.date_reported())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::DamageReportRepository<'a>>>
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
            .damage_report_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("DamageReport"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

