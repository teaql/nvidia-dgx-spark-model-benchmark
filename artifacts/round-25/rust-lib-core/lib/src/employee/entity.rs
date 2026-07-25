// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/employee
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
#[teaql(entity = "Employee", table = "employee_data", data_service = "sqlite")]
pub struct Employee {
#[teaql(id)]
    id: u64,
#[teaql(version)]
    version: i64,
// @source module_2.xml:16
#[teaql(column = "department")]
    department_id: u64,
// @source module_2.xml:16
#[teaql(relation(target = "Department", local_key = "department_id", foreign_key = "id"))]
    department: Option<crate::Department>,
#[teaql(relation(target = "OperationsManagerOverride", local_key = "id", foreign_key = "employee_id", many))]
    operations_manager_override_list: SmartList<crate::OperationsManagerOverride>,
#[teaql(relation(target = "JobAssignment", local_key = "id", foreign_key = "employee_id", many))]
    job_assignment_list: SmartList<crate::JobAssignment>,
#[teaql(relation(target = "Bonus", local_key = "id", foreign_key = "employee_id", many))]
    bonus_list: SmartList<crate::Bonus>,
#[teaql(relation(target = "LeaveRequest", local_key = "id", foreign_key = "employee_id", many))]
    leave_request_list: SmartList<crate::LeaveRequest>,
#[teaql(relation(target = "EmployeeCertification", local_key = "id", foreign_key = "employee_id", many))]
    employee_certification_list: SmartList<crate::EmployeeCertification>,
#[teaql(relation(target = "TaxWithholding", local_key = "id", foreign_key = "employee_id", many))]
    tax_withholding_list: SmartList<crate::TaxWithholding>,
#[teaql(relation(target = "DirectDepositInfo", local_key = "id", foreign_key = "employee_id", many))]
    direct_deposit_info_list: SmartList<crate::DirectDepositInfo>,
#[teaql(relation(target = "UnionDues", local_key = "id", foreign_key = "employee_id", many))]
    union_dues_list: SmartList<crate::UnionDues>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Employee {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            version: 0_i64,
            department_id: 0_u64,
            department: None,
            operations_manager_override_list: Default::default(),
            job_assignment_list: Default::default(),
            bonus_list: Default::default(),
            leave_request_list: Default::default(),
            employee_certification_list: Default::default(),
            tax_withholding_list: Default::default(),
            direct_deposit_info_list: Default::default(),
            union_dues_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Employee", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.department {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.operations_manager_override_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.job_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.bonus_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.leave_request_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.employee_certification_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tax_withholding_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.direct_deposit_info_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.union_dues_list {
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
    pub fn department_id(&self) -> u64 {
        self.changed_department_id().and_then(|value| value.try_u64()).unwrap_or(self.department_id)
    }

    pub fn update_department_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.department_id = value.try_u64().unwrap_or(self.department_id.clone());
        self.root.set(self.entity_key(), "department_id", value);
        self
    }

    pub fn changed_department_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "department_id")
    }

    pub fn eval_department_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("department_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "department_id".to_string(), attempted_path: "department_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.department_id())
                }}
    pub fn department(&self) -> Option<&crate::Department> {
        self.department.as_ref()
    }

    pub fn eval_department(&self) -> teaql_core::eval::EvalResult<&crate::Department> {
        if !self.is_loaded("department") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "department".to_string(), attempted_path: "department".to_string() }
        } else {
            match &self.department {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn operations_manager_override_list(&self) -> &SmartList<crate::OperationsManagerOverride> {
        &self.operations_manager_override_list
    }

    pub fn operations_manager_override_list_mut(&mut self) -> &mut SmartList<crate::OperationsManagerOverride> {
        &mut self.operations_manager_override_list
    }

    pub fn eval_operations_manager_override_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OperationsManagerOverride>> {
        if !self.is_loaded("operations_manager_override_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "operations_manager_override_list".to_string(), attempted_path: "operations_manager_override_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.operations_manager_override_list)
        }
    }

    pub fn job_assignment_list(&self) -> &SmartList<crate::JobAssignment> {
        &self.job_assignment_list
    }

    pub fn job_assignment_list_mut(&mut self) -> &mut SmartList<crate::JobAssignment> {
        &mut self.job_assignment_list
    }

    pub fn eval_job_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::JobAssignment>> {
        if !self.is_loaded("job_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_assignment_list".to_string(), attempted_path: "job_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.job_assignment_list)
        }
    }

    pub fn bonus_list(&self) -> &SmartList<crate::Bonus> {
        &self.bonus_list
    }

    pub fn bonus_list_mut(&mut self) -> &mut SmartList<crate::Bonus> {
        &mut self.bonus_list
    }

    pub fn eval_bonus_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Bonus>> {
        if !self.is_loaded("bonus_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "bonus_list".to_string(), attempted_path: "bonus_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.bonus_list)
        }
    }

    pub fn leave_request_list(&self) -> &SmartList<crate::LeaveRequest> {
        &self.leave_request_list
    }

    pub fn leave_request_list_mut(&mut self) -> &mut SmartList<crate::LeaveRequest> {
        &mut self.leave_request_list
    }

    pub fn eval_leave_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeaveRequest>> {
        if !self.is_loaded("leave_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_request_list".to_string(), attempted_path: "leave_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.leave_request_list)
        }
    }

    pub fn employee_certification_list(&self) -> &SmartList<crate::EmployeeCertification> {
        &self.employee_certification_list
    }

    pub fn employee_certification_list_mut(&mut self) -> &mut SmartList<crate::EmployeeCertification> {
        &mut self.employee_certification_list
    }

    pub fn eval_employee_certification_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EmployeeCertification>> {
        if !self.is_loaded("employee_certification_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_certification_list".to_string(), attempted_path: "employee_certification_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.employee_certification_list)
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

    pub fn direct_deposit_info_list(&self) -> &SmartList<crate::DirectDepositInfo> {
        &self.direct_deposit_info_list
    }

    pub fn direct_deposit_info_list_mut(&mut self) -> &mut SmartList<crate::DirectDepositInfo> {
        &mut self.direct_deposit_info_list
    }

    pub fn eval_direct_deposit_info_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DirectDepositInfo>> {
        if !self.is_loaded("direct_deposit_info_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "direct_deposit_info_list".to_string(), attempted_path: "direct_deposit_info_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.direct_deposit_info_list)
        }
    }

    pub fn union_dues_list(&self) -> &SmartList<crate::UnionDues> {
        &self.union_dues_list
    }

    pub fn union_dues_list_mut(&mut self) -> &mut SmartList<crate::UnionDues> {
        &mut self.union_dues_list
    }

    pub fn eval_union_dues_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UnionDues>> {
        if !self.is_loaded("union_dues_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "union_dues_list".to_string(), attempted_path: "union_dues_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.union_dues_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
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
            .employee_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Employee"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

