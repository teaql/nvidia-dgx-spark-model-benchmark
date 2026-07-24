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

// @source model.xml:2
    name: String,

// @source model.xml:2
    role: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "merchant_ref")]
    merchant_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Merchant", local_key = "merchant_ref_id", foreign_key = "id"))]
    merchant_ref: Option<crate::Merchant>,
    #[teaql(boxed_relations)]
    pub _relations: Box<EmployeeReverseRelations>,
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
            name: String::new(),
            role: String::new(),
            version: 0_i64,
            merchant_ref_id: 0_u64,
            merchant_ref: None,
            _relations: Box::new(EmployeeReverseRelations::new()),
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
        if let Some(entity) = &mut self.merchant_ref {
            entity.attach_root_recursive(root.clone());
        }
        self._relations.attach_root_recursive(root.clone());
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

    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.name.clone());
        self.root.set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
                }}

    pub fn role(&self) -> String {
        self.changed_role().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.role.clone())
    }

    pub fn update_role(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.role = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.role.clone());
        self.root.set(self.entity_key(), "role", value);
        self
    }

    pub fn changed_role(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "role")
    }

    pub fn eval_role(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("role") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "role".to_string(), attempted_path: "role".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.role())
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
    pub fn merchant_ref_id(&self) -> u64 {
        self.changed_merchant_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_ref_id)
    }

    pub fn update_merchant_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_ref_id = value.try_u64().unwrap_or(self.merchant_ref_id.clone());
        self.root.set(self.entity_key(), "merchant_ref_id", value);
        self
    }

    pub fn changed_merchant_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_ref_id")
    }

    pub fn eval_merchant_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref_id".to_string(), attempted_path: "merchant_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_ref_id())
                }}
    pub fn merchant_ref(&self) -> Option<&crate::Merchant> {
        self.merchant_ref.as_ref()
    }

    pub fn eval_merchant_ref(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref".to_string(), attempted_path: "merchant_ref".to_string() }
        } else {
            match &self.merchant_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn job_assignment_list(&self) -> &SmartList<crate::JobAssignment> {
        &self._relations.job_assignment_list
    }

    pub fn job_assignment_list_mut(&mut self) -> &mut SmartList<crate::JobAssignment> {
        &mut self._relations.job_assignment_list
    }

    pub fn eval_job_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::JobAssignment>> {
        if !self.is_loaded("job_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_assignment_list".to_string(), attempted_path: "job_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.job_assignment_list)
        }
    }

    pub fn work_shift_list(&self) -> &SmartList<crate::WorkShift> {
        &self._relations.work_shift_list
    }

    pub fn work_shift_list_mut(&mut self) -> &mut SmartList<crate::WorkShift> {
        &mut self._relations.work_shift_list
    }

    pub fn eval_work_shift_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::WorkShift>> {
        if !self.is_loaded("work_shift_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "work_shift_list".to_string(), attempted_path: "work_shift_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.work_shift_list)
        }
    }

    pub fn bonus_list(&self) -> &SmartList<crate::Bonus> {
        &self._relations.bonus_list
    }

    pub fn bonus_list_mut(&mut self) -> &mut SmartList<crate::Bonus> {
        &mut self._relations.bonus_list
    }

    pub fn eval_bonus_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Bonus>> {
        if !self.is_loaded("bonus_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "bonus_list".to_string(), attempted_path: "bonus_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.bonus_list)
        }
    }

    pub fn leave_request_list(&self) -> &SmartList<crate::LeaveRequest> {
        &self._relations.leave_request_list
    }

    pub fn leave_request_list_mut(&mut self) -> &mut SmartList<crate::LeaveRequest> {
        &mut self._relations.leave_request_list
    }

    pub fn eval_leave_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeaveRequest>> {
        if !self.is_loaded("leave_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_request_list".to_string(), attempted_path: "leave_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.leave_request_list)
        }
    }

    pub fn employee_certification_list(&self) -> &SmartList<crate::EmployeeCertification> {
        &self._relations.employee_certification_list
    }

    pub fn employee_certification_list_mut(&mut self) -> &mut SmartList<crate::EmployeeCertification> {
        &mut self._relations.employee_certification_list
    }

    pub fn eval_employee_certification_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EmployeeCertification>> {
        if !self.is_loaded("employee_certification_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_certification_list".to_string(), attempted_path: "employee_certification_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.employee_certification_list)
        }
    }

    pub fn training_module_list(&self) -> &SmartList<crate::TrainingModule> {
        &self._relations.training_module_list
    }

    pub fn training_module_list_mut(&mut self) -> &mut SmartList<crate::TrainingModule> {
        &mut self._relations.training_module_list
    }

    pub fn eval_training_module_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TrainingModule>> {
        if !self.is_loaded("training_module_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "training_module_list".to_string(), attempted_path: "training_module_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.training_module_list)
        }
    }

    pub fn availability_schedule_list(&self) -> &SmartList<crate::AvailabilitySchedule> {
        &self._relations.availability_schedule_list
    }

    pub fn availability_schedule_list_mut(&mut self) -> &mut SmartList<crate::AvailabilitySchedule> {
        &mut self._relations.availability_schedule_list
    }

    pub fn eval_availability_schedule_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AvailabilitySchedule>> {
        if !self.is_loaded("availability_schedule_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "availability_schedule_list".to_string(), attempted_path: "availability_schedule_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.availability_schedule_list)
        }
    }

    pub fn skill_profile_list(&self) -> &SmartList<crate::SkillProfile> {
        &self._relations.skill_profile_list
    }

    pub fn skill_profile_list_mut(&mut self) -> &mut SmartList<crate::SkillProfile> {
        &mut self._relations.skill_profile_list
    }

    pub fn eval_skill_profile_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SkillProfile>> {
        if !self.is_loaded("skill_profile_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "skill_profile_list".to_string(), attempted_path: "skill_profile_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.skill_profile_list)
        }
    }

    pub fn performance_review_list(&self) -> &SmartList<crate::PerformanceReview> {
        &self._relations.performance_review_list
    }

    pub fn performance_review_list_mut(&mut self) -> &mut SmartList<crate::PerformanceReview> {
        &mut self._relations.performance_review_list
    }

    pub fn eval_performance_review_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PerformanceReview>> {
        if !self.is_loaded("performance_review_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "performance_review_list".to_string(), attempted_path: "performance_review_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.performance_review_list)
        }
    }

    pub fn overtime_record_list(&self) -> &SmartList<crate::OvertimeRecord> {
        &self._relations.overtime_record_list
    }

    pub fn overtime_record_list_mut(&mut self) -> &mut SmartList<crate::OvertimeRecord> {
        &mut self._relations.overtime_record_list
    }

    pub fn eval_overtime_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OvertimeRecord>> {
        if !self.is_loaded("overtime_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "overtime_record_list".to_string(), attempted_path: "overtime_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.overtime_record_list)
        }
    }

    pub fn benefit_enrollment_list(&self) -> &SmartList<crate::BenefitEnrollment> {
        &self._relations.benefit_enrollment_list
    }

    pub fn benefit_enrollment_list_mut(&mut self) -> &mut SmartList<crate::BenefitEnrollment> {
        &mut self._relations.benefit_enrollment_list
    }

    pub fn eval_benefit_enrollment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::BenefitEnrollment>> {
        if !self.is_loaded("benefit_enrollment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "benefit_enrollment_list".to_string(), attempted_path: "benefit_enrollment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.benefit_enrollment_list)
        }
    }

    pub fn shift_swap_request_list(&self) -> &SmartList<crate::ShiftSwapRequest> {
        &self._relations.shift_swap_request_list
    }

    pub fn shift_swap_request_list_mut(&mut self) -> &mut SmartList<crate::ShiftSwapRequest> {
        &mut self._relations.shift_swap_request_list
    }

    pub fn eval_shift_swap_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ShiftSwapRequest>> {
        if !self.is_loaded("shift_swap_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "shift_swap_request_list".to_string(), attempted_path: "shift_swap_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.shift_swap_request_list)
        }
    }

    pub fn attendance_record_list(&self) -> &SmartList<crate::AttendanceRecord> {
        &self._relations.attendance_record_list
    }

    pub fn attendance_record_list_mut(&mut self) -> &mut SmartList<crate::AttendanceRecord> {
        &mut self._relations.attendance_record_list
    }

    pub fn eval_attendance_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AttendanceRecord>> {
        if !self.is_loaded("attendance_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "attendance_record_list".to_string(), attempted_path: "attendance_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.attendance_record_list)
        }
    }

    pub fn commission_record_list(&self) -> &SmartList<crate::CommissionRecord> {
        &self._relations.commission_record_list
    }

    pub fn commission_record_list_mut(&mut self) -> &mut SmartList<crate::CommissionRecord> {
        &mut self._relations.commission_record_list
    }

    pub fn eval_commission_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CommissionRecord>> {
        if !self.is_loaded("commission_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "commission_record_list".to_string(), attempted_path: "commission_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.commission_record_list)
        }
    }

    pub fn user_account_list(&self) -> &SmartList<crate::UserAccount> {
        &self._relations.user_account_list
    }

    pub fn user_account_list_mut(&mut self) -> &mut SmartList<crate::UserAccount> {
        &mut self._relations.user_account_list
    }

    pub fn eval_user_account_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UserAccount>> {
        if !self.is_loaded("user_account_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_account_list".to_string(), attempted_path: "user_account_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.user_account_list)
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

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct EmployeeReverseRelations {
#[teaql(relation(target = "JobAssignment", local_key = "id", foreign_key = "employee_ref_id", many))]
    job_assignment_list: SmartList<crate::JobAssignment>,
#[teaql(relation(target = "WorkShift", local_key = "id", foreign_key = "employee_ref_id", many))]
    work_shift_list: SmartList<crate::WorkShift>,
#[teaql(relation(target = "Bonus", local_key = "id", foreign_key = "employee_ref_id", many))]
    bonus_list: SmartList<crate::Bonus>,
#[teaql(relation(target = "LeaveRequest", local_key = "id", foreign_key = "employee_ref_id", many))]
    leave_request_list: SmartList<crate::LeaveRequest>,
#[teaql(relation(target = "EmployeeCertification", local_key = "id", foreign_key = "employee_ref_id", many))]
    employee_certification_list: SmartList<crate::EmployeeCertification>,
#[teaql(relation(target = "TrainingModule", local_key = "id", foreign_key = "employee_ref_id", many))]
    training_module_list: SmartList<crate::TrainingModule>,
#[teaql(relation(target = "AvailabilitySchedule", local_key = "id", foreign_key = "employee_ref_id", many))]
    availability_schedule_list: SmartList<crate::AvailabilitySchedule>,
#[teaql(relation(target = "SkillProfile", local_key = "id", foreign_key = "employee_ref_id", many))]
    skill_profile_list: SmartList<crate::SkillProfile>,
#[teaql(relation(target = "PerformanceReview", local_key = "id", foreign_key = "employee_ref_id", many))]
    performance_review_list: SmartList<crate::PerformanceReview>,
#[teaql(relation(target = "OvertimeRecord", local_key = "id", foreign_key = "employee_ref_id", many))]
    overtime_record_list: SmartList<crate::OvertimeRecord>,
#[teaql(relation(target = "BenefitEnrollment", local_key = "id", foreign_key = "employee_ref_id", many))]
    benefit_enrollment_list: SmartList<crate::BenefitEnrollment>,
#[teaql(relation(target = "ShiftSwapRequest", local_key = "id", foreign_key = "employee_ref_id", many))]
    shift_swap_request_list: SmartList<crate::ShiftSwapRequest>,
#[teaql(relation(target = "AttendanceRecord", local_key = "id", foreign_key = "employee_ref_id", many))]
    attendance_record_list: SmartList<crate::AttendanceRecord>,
#[teaql(relation(target = "CommissionRecord", local_key = "id", foreign_key = "employee_ref_id", many))]
    commission_record_list: SmartList<crate::CommissionRecord>,
#[teaql(relation(target = "UserAccount", local_key = "id", foreign_key = "employee_ref_id", many))]
    user_account_list: SmartList<crate::UserAccount>,
}

impl EmployeeReverseRelations {
    pub fn new() -> Self {
        Self {
            job_assignment_list: Default::default(),
            work_shift_list: Default::default(),
            bonus_list: Default::default(),
            leave_request_list: Default::default(),
            employee_certification_list: Default::default(),
            training_module_list: Default::default(),
            availability_schedule_list: Default::default(),
            skill_profile_list: Default::default(),
            performance_review_list: Default::default(),
            overtime_record_list: Default::default(),
            benefit_enrollment_list: Default::default(),
            shift_swap_request_list: Default::default(),
            attendance_record_list: Default::default(),
            commission_record_list: Default::default(),
            user_account_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.job_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.work_shift_list {
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
        for entity in &mut self.training_module_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.availability_schedule_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.skill_profile_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.performance_review_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.overtime_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.benefit_enrollment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.shift_swap_request_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.attendance_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.commission_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.user_account_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
