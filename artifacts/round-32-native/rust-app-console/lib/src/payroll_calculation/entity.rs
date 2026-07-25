// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/payroll_calculation
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "PayrollCalculation", table = "payroll_calculation_data", data_service = "sqlite")]
pub struct PayrollCalculation {
#[teaql(id)]
    id: u64,

// @source module_2.xml:9
    gross_pay: String,

// @source module_2.xml:9
    net_pay: String,

// @source module_2.xml:9
    total_deductions: String,

// @source module_2.xml:9
    calculation_date: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl PayrollCalculation {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            gross_pay: String::new(),
            net_pay: String::new(),
            total_deductions: String::new(),
            calculation_date: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("PayrollCalculation", self.id)
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

    pub fn gross_pay(&self) -> String {
        self.changed_gross_pay().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.gross_pay.clone())
    }

    pub fn update_gross_pay(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.gross_pay = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.gross_pay.clone());
        self.root.set(self.entity_key(), "gross_pay", value);
        self
    }

    pub fn changed_gross_pay(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "gross_pay")
    }

    pub fn eval_gross_pay(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("gross_pay") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "gross_pay".to_string(), attempted_path: "gross_pay".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.gross_pay())
                }}

    pub fn net_pay(&self) -> String {
        self.changed_net_pay().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.net_pay.clone())
    }

    pub fn update_net_pay(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.net_pay = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.net_pay.clone());
        self.root.set(self.entity_key(), "net_pay", value);
        self
    }

    pub fn changed_net_pay(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "net_pay")
    }

    pub fn eval_net_pay(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("net_pay") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "net_pay".to_string(), attempted_path: "net_pay".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.net_pay())
                }}

    pub fn total_deductions(&self) -> String {
        self.changed_total_deductions().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.total_deductions.clone())
    }

    pub fn update_total_deductions(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.total_deductions = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.total_deductions.clone());
        self.root.set(self.entity_key(), "total_deductions", value);
        self
    }

    pub fn changed_total_deductions(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "total_deductions")
    }

    pub fn eval_total_deductions(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("total_deductions") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "total_deductions".to_string(), attempted_path: "total_deductions".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.total_deductions())
                }}

    pub fn calculation_date(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_calculation_date().and_then(|value| value.try_timestamp()).unwrap_or(self.calculation_date)
    }

    pub fn update_calculation_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.calculation_date = value.try_timestamp().unwrap_or(self.calculation_date.clone());
        self.root.set(self.entity_key(), "calculation_date", value);
        self
    }

    pub fn changed_calculation_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "calculation_date")
    }

    pub fn eval_calculation_date(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("calculation_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "calculation_date".to_string(), attempted_path: "calculation_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.calculation_date())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
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
            .payroll_calculation_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("PayrollCalculation"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

