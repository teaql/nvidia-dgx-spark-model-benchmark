// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/warning_letter
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "WarningLetter", table = "warning_letter_data", data_service = "sqlite")]
pub struct WarningLetter {
#[teaql(id)]
    id: u64,

// @source module_3.xml:5
    date_issued: chrono::NaiveDate,

// @source module_3.xml:5
    reason: String,

// @source module_3.xml:5
    severity_level: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl WarningLetter {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            date_issued: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            reason: String::new(),
            severity_level: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("WarningLetter", self.id)
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

    pub fn date_issued(&self) -> chrono::NaiveDate {
        self.changed_date_issued().and_then(|value| value.try_date()).unwrap_or(self.date_issued)
    }

    pub fn update_date_issued(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.date_issued = value.try_date().unwrap_or(self.date_issued.clone());
        self.root.set(self.entity_key(), "date_issued", value);
        self
    }

    pub fn changed_date_issued(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "date_issued")
    }

    pub fn eval_date_issued(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("date_issued") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "date_issued".to_string(), attempted_path: "date_issued".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.date_issued())
                }}

    pub fn reason(&self) -> String {
        self.changed_reason().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.reason.clone())
    }

    pub fn update_reason(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.reason = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.reason.clone());
        self.root.set(self.entity_key(), "reason", value);
        self
    }

    pub fn changed_reason(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "reason")
    }

    pub fn eval_reason(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("reason") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "reason".to_string(), attempted_path: "reason".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.reason())
                }}

    pub fn severity_level(&self) -> String {
        self.changed_severity_level().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.severity_level.clone())
    }

    pub fn update_severity_level(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.severity_level = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.severity_level.clone());
        self.root.set(self.entity_key(), "severity_level", value);
        self
    }

    pub fn changed_severity_level(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "severity_level")
    }

    pub fn eval_severity_level(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("severity_level") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "severity_level".to_string(), attempted_path: "severity_level".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.severity_level())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::WarningLetterRepository<'a>>>
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
            .warning_letter_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("WarningLetter"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

