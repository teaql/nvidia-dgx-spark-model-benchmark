// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/union_dues
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "UnionDues", table = "union_dues_data", data_service = "sqlite")]
pub struct UnionDues {
#[teaql(id)]
    id: u64,

// @source module_2.xml:16
    dues_amount: String,

// @source module_2.xml:16
    deduction_date: chrono::NaiveDate,

// @source module_2.xml:16
    union_title: String,

// @source module_2.xml:16
    status: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl UnionDues {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            dues_amount: String::new(),
            deduction_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            union_title: String::new(),
            status: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("UnionDues", self.id)
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

    pub fn dues_amount(&self) -> String {
        self.changed_dues_amount().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.dues_amount.clone())
    }

    pub fn update_dues_amount(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.dues_amount = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.dues_amount.clone());
        self.root.set(self.entity_key(), "dues_amount", value);
        self
    }

    pub fn changed_dues_amount(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "dues_amount")
    }

    pub fn eval_dues_amount(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("dues_amount") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "dues_amount".to_string(), attempted_path: "dues_amount".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.dues_amount())
                }}

    pub fn deduction_date(&self) -> chrono::NaiveDate {
        self.changed_deduction_date().and_then(|value| value.try_date()).unwrap_or(self.deduction_date)
    }

    pub fn update_deduction_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.deduction_date = value.try_date().unwrap_or(self.deduction_date.clone());
        self.root.set(self.entity_key(), "deduction_date", value);
        self
    }

    pub fn changed_deduction_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "deduction_date")
    }

    pub fn eval_deduction_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("deduction_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "deduction_date".to_string(), attempted_path: "deduction_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.deduction_date())
                }}

    pub fn union_title(&self) -> String {
        self.changed_union_title().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.union_title.clone())
    }

    pub fn update_union_title(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.union_title = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.union_title.clone());
        self.root.set(self.entity_key(), "union_title", value);
        self
    }

    pub fn changed_union_title(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "union_title")
    }

    pub fn eval_union_title(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("union_title") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "union_title".to_string(), attempted_path: "union_title".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.union_title())
                }}

    pub fn status(&self) -> String {
        self.changed_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.status.clone())
    }

    pub fn update_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.status.clone());
        self.root.set(self.entity_key(), "status", value);
        self
    }

    pub fn changed_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status")
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::UnionDuesRepository<'a>>>
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
            .union_dues_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("UnionDues"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

