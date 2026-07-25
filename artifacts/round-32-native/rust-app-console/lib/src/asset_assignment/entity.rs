// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/asset_assignment
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "AssetAssignment", table = "asset_assignment_data", data_service = "sqlite")]
pub struct AssetAssignment {
#[teaql(id)]
    id: u64,

// @source module_8.xml:4
    assigned_to: String,

// @source module_8.xml:4
    assignment_date: chrono::NaiveDate,

// @source module_8.xml:4
    return_date: chrono::NaiveDate,

// @source module_8.xml:4
    notes: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl AssetAssignment {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            assigned_to: String::new(),
            assignment_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            return_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            notes: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("AssetAssignment", self.id)
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

    pub fn assigned_to(&self) -> String {
        self.changed_assigned_to().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.assigned_to.clone())
    }

    pub fn update_assigned_to(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.assigned_to = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.assigned_to.clone());
        self.root.set(self.entity_key(), "assigned_to", value);
        self
    }

    pub fn changed_assigned_to(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "assigned_to")
    }

    pub fn eval_assigned_to(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("assigned_to") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "assigned_to".to_string(), attempted_path: "assigned_to".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.assigned_to())
                }}

    pub fn assignment_date(&self) -> chrono::NaiveDate {
        self.changed_assignment_date().and_then(|value| value.try_date()).unwrap_or(self.assignment_date)
    }

    pub fn update_assignment_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.assignment_date = value.try_date().unwrap_or(self.assignment_date.clone());
        self.root.set(self.entity_key(), "assignment_date", value);
        self
    }

    pub fn changed_assignment_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "assignment_date")
    }

    pub fn eval_assignment_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("assignment_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "assignment_date".to_string(), attempted_path: "assignment_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.assignment_date())
                }}

    pub fn return_date(&self) -> chrono::NaiveDate {
        self.changed_return_date().and_then(|value| value.try_date()).unwrap_or(self.return_date)
    }

    pub fn update_return_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.return_date = value.try_date().unwrap_or(self.return_date.clone());
        self.root.set(self.entity_key(), "return_date", value);
        self
    }

    pub fn changed_return_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "return_date")
    }

    pub fn eval_return_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("return_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "return_date".to_string(), attempted_path: "return_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.return_date())
                }}

    pub fn notes(&self) -> String {
        self.changed_notes().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.notes.clone())
    }

    pub fn update_notes(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.notes = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.notes.clone());
        self.root.set(self.entity_key(), "notes", value);
        self
    }

    pub fn changed_notes(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "notes")
    }

    pub fn eval_notes(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("notes") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "notes".to_string(), attempted_path: "notes".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.notes())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::AssetAssignmentRepository<'a>>>
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
            .asset_assignment_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("AssetAssignment"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

