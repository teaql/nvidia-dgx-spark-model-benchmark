// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/tax_withholding
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "TaxWithholding", table = "tax_withholding_data", data_service = "sqlite")]
pub struct TaxWithholding {
#[teaql(id)]
    id: u64,

// @source module_2.xml:14
    tax_year: i64,

// @source module_2.xml:14
    federal_withholding: String,

// @source module_2.xml:14
    state_withholding: String,

// @source module_2.xml:14
    filing_status: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl TaxWithholding {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            tax_year: 0_i64,
            federal_withholding: String::new(),
            state_withholding: String::new(),
            filing_status: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("TaxWithholding", self.id)
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

    pub fn tax_year(&self) -> i64 {
        self.changed_tax_year().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.tax_year)
    }

    pub fn update_tax_year(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tax_year = value.try_i64().map(|value| value as i64).unwrap_or(self.tax_year.clone());
        self.root.set(self.entity_key(), "tax_year", value);
        self
    }

    pub fn changed_tax_year(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tax_year")
    }

    pub fn eval_tax_year(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("tax_year") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_year".to_string(), attempted_path: "tax_year".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tax_year())
                }}

    pub fn federal_withholding(&self) -> String {
        self.changed_federal_withholding().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.federal_withholding.clone())
    }

    pub fn update_federal_withholding(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.federal_withholding = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.federal_withholding.clone());
        self.root.set(self.entity_key(), "federal_withholding", value);
        self
    }

    pub fn changed_federal_withholding(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "federal_withholding")
    }

    pub fn eval_federal_withholding(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("federal_withholding") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "federal_withholding".to_string(), attempted_path: "federal_withholding".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.federal_withholding())
                }}

    pub fn state_withholding(&self) -> String {
        self.changed_state_withholding().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.state_withholding.clone())
    }

    pub fn update_state_withholding(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.state_withholding = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.state_withholding.clone());
        self.root.set(self.entity_key(), "state_withholding", value);
        self
    }

    pub fn changed_state_withholding(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "state_withholding")
    }

    pub fn eval_state_withholding(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("state_withholding") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "state_withholding".to_string(), attempted_path: "state_withholding".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.state_withholding())
                }}

    pub fn filing_status(&self) -> String {
        self.changed_filing_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.filing_status.clone())
    }

    pub fn update_filing_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.filing_status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.filing_status.clone());
        self.root.set(self.entity_key(), "filing_status", value);
        self
    }

    pub fn changed_filing_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "filing_status")
    }

    pub fn eval_filing_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("filing_status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "filing_status".to_string(), attempted_path: "filing_status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.filing_status())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::TaxWithholdingRepository<'a>>>
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
            .tax_withholding_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("TaxWithholding"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

