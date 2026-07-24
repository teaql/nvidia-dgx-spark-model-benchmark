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

// @source model.xml:2
    calc_id: String,

// @source model.xml:2
    gross_pay: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "payroll_period_ref")]
    payroll_period_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "PayrollPeriod", local_key = "payroll_period_ref_id", foreign_key = "id"))]
    payroll_period_ref: Option<crate::PayrollPeriod>,
#[teaql(relation(target = "Payslip", local_key = "id", foreign_key = "payroll_calculation_ref_id", many))]
    payslip_list: SmartList<crate::Payslip>,
#[teaql(relation(target = "Deduction", local_key = "id", foreign_key = "payroll_calculation_ref_id", many))]
    deduction_list: SmartList<crate::Deduction>,
#[teaql(relation(target = "TaxWithholding", local_key = "id", foreign_key = "payroll_calculation_ref_id", many))]
    tax_withholding_list: SmartList<crate::TaxWithholding>,
#[teaql(relation(target = "PayrollAdjustment", local_key = "id", foreign_key = "payroll_calculation_ref_id", many))]
    payroll_adjustment_list: SmartList<crate::PayrollAdjustment>,
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
            calc_id: String::new(),
            gross_pay: String::new(),
            version: 0_i64,
            payroll_period_ref_id: 0_u64,
            payroll_period_ref: None,
            payslip_list: Default::default(),
            deduction_list: Default::default(),
            tax_withholding_list: Default::default(),
            payroll_adjustment_list: Default::default(),
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
        if let Some(entity) = &mut self.payroll_period_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payslip_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.deduction_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tax_withholding_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payroll_adjustment_list {
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

    pub fn calc_id(&self) -> String {
        self.changed_calc_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.calc_id.clone())
    }

    pub fn update_calc_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.calc_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.calc_id.clone());
        self.root.set(self.entity_key(), "calc_id", value);
        self
    }

    pub fn changed_calc_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "calc_id")
    }

    pub fn eval_calc_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("calc_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "calc_id".to_string(), attempted_path: "calc_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.calc_id())
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
    pub fn payroll_period_ref_id(&self) -> u64 {
        self.changed_payroll_period_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.payroll_period_ref_id)
    }

    pub fn update_payroll_period_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.payroll_period_ref_id = value.try_u64().unwrap_or(self.payroll_period_ref_id.clone());
        self.root.set(self.entity_key(), "payroll_period_ref_id", value);
        self
    }

    pub fn changed_payroll_period_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "payroll_period_ref_id")
    }

    pub fn eval_payroll_period_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("payroll_period_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_period_ref_id".to_string(), attempted_path: "payroll_period_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.payroll_period_ref_id())
                }}
    pub fn payroll_period_ref(&self) -> Option<&crate::PayrollPeriod> {
        self.payroll_period_ref.as_ref()
    }

    pub fn eval_payroll_period_ref(&self) -> teaql_core::eval::EvalResult<&crate::PayrollPeriod> {
        if !self.is_loaded("payroll_period_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_period_ref".to_string(), attempted_path: "payroll_period_ref".to_string() }
        } else {
            match &self.payroll_period_ref {
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

    pub fn deduction_list(&self) -> &SmartList<crate::Deduction> {
        &self.deduction_list
    }

    pub fn deduction_list_mut(&mut self) -> &mut SmartList<crate::Deduction> {
        &mut self.deduction_list
    }

    pub fn eval_deduction_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Deduction>> {
        if !self.is_loaded("deduction_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "deduction_list".to_string(), attempted_path: "deduction_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.deduction_list)
        }
    }

    pub fn tax_withholding_list(&self) -> &SmartList<crate::TaxWithholding> {
        &self.tax_withholding_list
    }

    pub fn tax_withholding_list_mut(&mut self) -> &mut SmartList<crate::TaxWithholding> {
        &mut self.tax_withholding_list
    }

    pub fn eval_tax_withholding_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TaxWithholding>> {
        if !self.is_loaded("tax_withholding_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_withholding_list".to_string(), attempted_path: "tax_withholding_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.tax_withholding_list)
        }
    }

    pub fn payroll_adjustment_list(&self) -> &SmartList<crate::PayrollAdjustment> {
        &self.payroll_adjustment_list
    }

    pub fn payroll_adjustment_list_mut(&mut self) -> &mut SmartList<crate::PayrollAdjustment> {
        &mut self.payroll_adjustment_list
    }

    pub fn eval_payroll_adjustment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PayrollAdjustment>> {
        if !self.is_loaded("payroll_adjustment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_adjustment_list".to_string(), attempted_path: "payroll_adjustment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.payroll_adjustment_list)
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

