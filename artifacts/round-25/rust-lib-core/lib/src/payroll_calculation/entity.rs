// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/payroll_calculation
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
#[teaql(entity = "PayrollCalculation", table = "payroll_calculation_data", data_service = "sqlite")]
pub struct PayrollCalculation {
#[teaql(id)]
    id: u64,
#[teaql(version)]
    version: i64,
// @source module_2.xml:39
#[teaql(column = "payroll_period")]
    payroll_period_id: u64,
// @source module_2.xml:39
#[teaql(relation(target = "PayrollPeriod", local_key = "payroll_period_id", foreign_key = "id"))]
    payroll_period: Option<crate::PayrollPeriod>,
#[teaql(relation(target = "Payslip", local_key = "id", foreign_key = "payroll_calculation_id", many))]
    payslip_list: SmartList<crate::Payslip>,
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
            version: 0_i64,
            payroll_period_id: 0_u64,
            payroll_period: None,
            payslip_list: Default::default(),
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
        if let Some(entity) = &mut self.payroll_period {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payslip_list {
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
    pub fn payroll_period_id(&self) -> u64 {
        self.changed_payroll_period_id().and_then(|value| value.try_u64()).unwrap_or(self.payroll_period_id)
    }

    pub fn update_payroll_period_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.payroll_period_id = value.try_u64().unwrap_or(self.payroll_period_id.clone());
        self.root.set(self.entity_key(), "payroll_period_id", value);
        self
    }

    pub fn changed_payroll_period_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "payroll_period_id")
    }

    pub fn eval_payroll_period_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("payroll_period_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_period_id".to_string(), attempted_path: "payroll_period_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.payroll_period_id())
                }}
    pub fn payroll_period(&self) -> Option<&crate::PayrollPeriod> {
        self.payroll_period.as_ref()
    }

    pub fn eval_payroll_period(&self) -> teaql_core::eval::EvalResult<&crate::PayrollPeriod> {
        if !self.is_loaded("payroll_period") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_period".to_string(), attempted_path: "payroll_period".to_string() }
        } else {
            match &self.payroll_period {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn payslip_list(&self) -> &SmartList<crate::Payslip> {
        &self.payslip_list
    }

    pub fn payslip_list_mut(&mut self) -> &mut SmartList<crate::Payslip> {
        &mut self.payslip_list
    }

    pub fn eval_payslip_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Payslip>> {
        if !self.is_loaded("payslip_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payslip_list".to_string(), attempted_path: "payslip_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.payslip_list)
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

