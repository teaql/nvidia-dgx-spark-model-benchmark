// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/compensation_adjustment
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CompensationAdjustment", table = "compensation_adjustment_data", data_service = "sqlite")]
pub struct CompensationAdjustment {
#[teaql(id)]
    id: u64,

// @source hr_payroll.xml:76
    adjustment_type: String,

// @source hr_payroll.xml:76
    amount: String,

// @source hr_payroll.xml:76
    effective_date: chrono::NaiveDate,
#[teaql(version)]
    version: i64,
// @source hr_payroll.xml:76
#[teaql(column = "employee")]
    employee_id: u64,

// @source hr_payroll.xml:76
#[teaql(column = "approved_by")]
    approved_by_id: u64,
// @source hr_payroll.xml:76
#[teaql(relation(target = "EmployeeRecord", local_key = "employee_id", foreign_key = "id"))]
    employee: Option<crate::EmployeeRecord>,

// @source hr_payroll.xml:76
#[teaql(relation(target = "EmployeeRecord", local_key = "approved_by_id", foreign_key = "id"))]
    approved_by: Option<crate::EmployeeRecord>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CompensationAdjustment {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            adjustment_type: String::new(),
            amount: String::new(),
            effective_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            version: 0_i64,
            employee_id: 0_u64,
            approved_by_id: 0_u64,
            employee: None,
            approved_by: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CompensationAdjustment", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.employee {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.approved_by {
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

    pub fn adjustment_type(&self) -> String {
        self.changed_adjustment_type().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.adjustment_type.clone())
    }

    pub fn update_adjustment_type(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.adjustment_type = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.adjustment_type.clone());
        self.root.set(self.entity_key(), "adjustment_type", value);
        self
    }

    pub fn changed_adjustment_type(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "adjustment_type")
    }

    pub fn eval_adjustment_type(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("adjustment_type") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "adjustment_type".to_string(), attempted_path: "adjustment_type".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.adjustment_type())
                }}

    pub fn amount(&self) -> String {
        self.changed_amount().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.amount.clone())
    }

    pub fn update_amount(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.amount = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.amount.clone());
        self.root.set(self.entity_key(), "amount", value);
        self
    }

    pub fn changed_amount(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "amount")
    }

    pub fn eval_amount(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("amount") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "amount".to_string(), attempted_path: "amount".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.amount())
                }}

    pub fn effective_date(&self) -> chrono::NaiveDate {
        self.changed_effective_date().and_then(|value| value.try_date()).unwrap_or(self.effective_date)
    }

    pub fn update_effective_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.effective_date = value.try_date().unwrap_or(self.effective_date.clone());
        self.root.set(self.entity_key(), "effective_date", value);
        self
    }

    pub fn changed_effective_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "effective_date")
    }

    pub fn eval_effective_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("effective_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "effective_date".to_string(), attempted_path: "effective_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.effective_date())
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
    pub fn employee_id(&self) -> u64 {
        self.changed_employee_id().and_then(|value| value.try_u64()).unwrap_or(self.employee_id)
    }

    pub fn update_employee_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employee_id = value.try_u64().unwrap_or(self.employee_id.clone());
        self.root.set(self.entity_key(), "employee_id", value);
        self
    }

    pub fn changed_employee_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employee_id")
    }

    pub fn eval_employee_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("employee_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_id".to_string(), attempted_path: "employee_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employee_id())
                }}

    pub fn approved_by_id(&self) -> u64 {
        self.changed_approved_by_id().and_then(|value| value.try_u64()).unwrap_or(self.approved_by_id)
    }

    pub fn update_approved_by_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.approved_by_id = value.try_u64().unwrap_or(self.approved_by_id.clone());
        self.root.set(self.entity_key(), "approved_by_id", value);
        self
    }

    pub fn changed_approved_by_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "approved_by_id")
    }

    pub fn eval_approved_by_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("approved_by_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "approved_by_id".to_string(), attempted_path: "approved_by_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.approved_by_id())
                }}
    pub fn employee(&self) -> Option<&crate::EmployeeRecord> {
        self.employee.as_ref()
    }

    pub fn eval_employee(&self) -> teaql_core::eval::EvalResult<&crate::EmployeeRecord> {
        if !self.is_loaded("employee") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee".to_string(), attempted_path: "employee".to_string() }
        } else {
            match &self.employee {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn approved_by(&self) -> Option<&crate::EmployeeRecord> {
        self.approved_by.as_ref()
    }

    pub fn eval_approved_by(&self) -> teaql_core::eval::EvalResult<&crate::EmployeeRecord> {
        if !self.is_loaded("approved_by") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "approved_by".to_string(), attempted_path: "approved_by".to_string() }
        } else {
            match &self.approved_by {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CompensationAdjustmentRepository<'a>>>
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
            .compensation_adjustment_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CompensationAdjustment"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

