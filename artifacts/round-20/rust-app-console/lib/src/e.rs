// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn customer_profile<'a>(value: &'a crate::CustomerProfile) -> crate::CustomerProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CustomerProfile(id={})", value.id()));
        crate::CustomerProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn address_book<'a>(value: &'a crate::AddressBook) -> crate::AddressBookExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AddressBook(id={})", value.id()));
        crate::AddressBookExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contact_person<'a>(value: &'a crate::ContactPerson) -> crate::ContactPersonExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ContactPerson(id={})", value.id()));
        crate::ContactPersonExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn account_settings<'a>(value: &'a crate::AccountSettings) -> crate::AccountSettingsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AccountSettings(id={})", value.id()));
        crate::AccountSettingsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn loyalty_program<'a>(value: &'a crate::LoyaltyProgram) -> crate::LoyaltyProgramExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoyaltyProgram(id={})", value.id()));
        crate::LoyaltyProgramExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_history<'a>(value: &'a crate::ServiceHistory) -> crate::ServiceHistoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceHistory(id={})", value.id()));
        crate::ServiceHistoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn feedback_review<'a>(value: &'a crate::FeedbackReview) -> crate::FeedbackReviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FeedbackReview(id={})", value.id()));
        crate::FeedbackReviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn dispute_case<'a>(value: &'a crate::DisputeCase) -> crate::DisputeCaseExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DisputeCase(id={})", value.id()));
        crate::DisputeCaseExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn document_upload<'a>(value: &'a crate::DocumentUpload) -> crate::DocumentUploadExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DocumentUpload(id={})", value.id()));
        crate::DocumentUploadExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn preference_center<'a>(value: &'a crate::PreferenceCenter) -> crate::PreferenceCenterExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PreferenceCenter(id={})", value.id()));
        crate::PreferenceCenterExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn notification_pref<'a>(value: &'a crate::NotificationPref) -> crate::NotificationPrefExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("NotificationPref(id={})", value.id()));
        crate::NotificationPrefExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn billing_contact<'a>(value: &'a crate::BillingContact) -> crate::BillingContactExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BillingContact(id={})", value.id()));
        crate::BillingContactExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn vehicle_registry<'a>(value: &'a crate::VehicleRegistry) -> crate::VehicleRegistryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("VehicleRegistry(id={})", value.id()));
        crate::VehicleRegistryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn driver_profile<'a>(value: &'a crate::DriverProfile) -> crate::DriverProfileExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DriverProfile(id={})", value.id()));
        crate::DriverProfileExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn maintenance_log<'a>(value: &'a crate::MaintenanceLog) -> crate::MaintenanceLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MaintenanceLog(id={})", value.id()));
        crate::MaintenanceLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn fuel_record<'a>(value: &'a crate::FuelRecord) -> crate::FuelRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FuelRecord(id={})", value.id()));
        crate::FuelRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn inspection_checklist<'a>(value: &'a crate::InspectionChecklist) -> crate::InspectionChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("InspectionChecklist(id={})", value.id()));
        crate::InspectionChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn route_plan<'a>(value: &'a crate::RoutePlan) -> crate::RoutePlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RoutePlan(id={})", value.id()));
        crate::RoutePlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn load_manifest<'a>(value: &'a crate::LoadManifest) -> crate::LoadManifestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LoadManifest(id={})", value.id()));
        crate::LoadManifestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment_inventory<'a>(value: &'a crate::EquipmentInventory) -> crate::EquipmentInventoryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EquipmentInventory(id={})", value.id()));
        crate::EquipmentInventoryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn garage_assignment<'a>(value: &'a crate::GarageAssignment) -> crate::GarageAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("GarageAssignment(id={})", value.id()));
        crate::GarageAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn incident_report<'a>(value: &'a crate::IncidentReport) -> crate::IncidentReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("IncidentReport(id={})", value.id()));
        crate::IncidentReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn compliance_certificate<'a>(value: &'a crate::ComplianceCertificate) -> crate::ComplianceCertificateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ComplianceCertificate(id={})", value.id()));
        crate::ComplianceCertificateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn telematics_data<'a>(value: &'a crate::TelematicsData) -> crate::TelematicsDataExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TelematicsData(id={})", value.id()));
        crate::TelematicsDataExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn invoice<'a>(value: &'a crate::Invoice) -> crate::InvoiceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Invoice(id={})", value.id()));
        crate::InvoiceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payment_transaction<'a>(value: &'a crate::PaymentTransaction) -> crate::PaymentTransactionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PaymentTransaction(id={})", value.id()));
        crate::PaymentTransactionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_calculation<'a>(value: &'a crate::TaxCalculation) -> crate::TaxCalculationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxCalculation(id={})", value.id()));
        crate::TaxCalculationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn credit_memo<'a>(value: &'a crate::CreditMemo) -> crate::CreditMemoExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CreditMemo(id={})", value.id()));
        crate::CreditMemoExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn deposit_receipt<'a>(value: &'a crate::DepositReceipt) -> crate::DepositReceiptExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("DepositReceipt(id={})", value.id()));
        crate::DepositReceiptExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn refund_request<'a>(value: &'a crate::RefundRequest) -> crate::RefundRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RefundRequest(id={})", value.id()));
        crate::RefundRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_report<'a>(value: &'a crate::ExpenseReport) -> crate::ExpenseReportExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseReport(id={})", value.id()));
        crate::ExpenseReportExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn budget_allocation<'a>(value: &'a crate::BudgetAllocation) -> crate::BudgetAllocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BudgetAllocation(id={})", value.id()));
        crate::BudgetAllocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn financial_statement<'a>(value: &'a crate::FinancialStatement) -> crate::FinancialStatementExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FinancialStatement(id={})", value.id()));
        crate::FinancialStatementExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn audit_trail<'a>(value: &'a crate::AuditTrail) -> crate::AuditTrailExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AuditTrail(id={})", value.id()));
        crate::AuditTrailExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn currency_exchange<'a>(value: &'a crate::CurrencyExchange) -> crate::CurrencyExchangeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CurrencyExchange(id={})", value.id()));
        crate::CurrencyExchangeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn receivable_aging<'a>(value: &'a crate::ReceivableAging) -> crate::ReceivableAgingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReceivableAging(id={})", value.id()));
        crate::ReceivableAgingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee_record<'a>(value: &'a crate::EmployeeRecord) -> crate::EmployeeRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeRecord(id={})", value.id()));
        crate::EmployeeRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll_run<'a>(value: &'a crate::PayrollRun) -> crate::PayrollRunExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PayrollRun(id={})", value.id()));
        crate::PayrollRunExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn timesheet_entry<'a>(value: &'a crate::TimesheetEntry) -> crate::TimesheetEntryExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TimesheetEntry(id={})", value.id()));
        crate::TimesheetEntryExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn benefit_plan<'a>(value: &'a crate::BenefitPlan) -> crate::BenefitPlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BenefitPlan(id={})", value.id()));
        crate::BenefitPlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_withholding<'a>(value: &'a crate::TaxWithholding) -> crate::TaxWithholdingExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxWithholding(id={})", value.id()));
        crate::TaxWithholdingExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn leave_request<'a>(value: &'a crate::LeaveRequest) -> crate::LeaveRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeaveRequest(id={})", value.id()));
        crate::LeaveRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn training_record<'a>(value: &'a crate::TrainingRecord) -> crate::TrainingRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrainingRecord(id={})", value.id()));
        crate::TrainingRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_review<'a>(value: &'a crate::PerformanceReview) -> crate::PerformanceReviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceReview(id={})", value.id()));
        crate::PerformanceReviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn compensation_adjustment<'a>(value: &'a crate::CompensationAdjustment) -> crate::CompensationAdjustmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CompensationAdjustment(id={})", value.id()));
        crate::CompensationAdjustmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn onboarding_checklist<'a>(value: &'a crate::OnboardingChecklist) -> crate::OnboardingChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OnboardingChecklist(id={})", value.id()));
        crate::OnboardingChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn offboarding_process<'a>(value: &'a crate::OffboardingProcess) -> crate::OffboardingProcessExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OffboardingProcess(id={})", value.id()));
        crate::OffboardingProcessExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee_handbook<'a>(value: &'a crate::EmployeeHandbook) -> crate::EmployeeHandbookExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeHandbook(id={})", value.id()));
        crate::EmployeeHandbookExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn move_order<'a>(value: &'a crate::MoveOrder) -> crate::MoveOrderExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("MoveOrder(id={})", value.id()));
        crate::MoveOrderExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn job_schedule<'a>(value: &'a crate::JobSchedule) -> crate::JobScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("JobSchedule(id={})", value.id()));
        crate::JobScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn crew_assignment<'a>(value: &'a crate::CrewAssignment) -> crate::CrewAssignmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CrewAssignment(id={})", value.id()));
        crate::CrewAssignmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn equipment_allocation<'a>(value: &'a crate::EquipmentAllocation) -> crate::EquipmentAllocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EquipmentAllocation(id={})", value.id()));
        crate::EquipmentAllocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn time_slot<'a>(value: &'a crate::TimeSlot) -> crate::TimeSlotExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TimeSlot(id={})", value.id()));
        crate::TimeSlotExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn service_location<'a>(value: &'a crate::ServiceLocation) -> crate::ServiceLocationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ServiceLocation(id={})", value.id()));
        crate::ServiceLocationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn special_instructions<'a>(value: &'a crate::SpecialInstructions) -> crate::SpecialInstructionsExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SpecialInstructions(id={})", value.id()));
        crate::SpecialInstructionsExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn status_update<'a>(value: &'a crate::StatusUpdate) -> crate::StatusUpdateExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("StatusUpdate(id={})", value.id()));
        crate::StatusUpdateExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn cancellation_policy<'a>(value: &'a crate::CancellationPolicy) -> crate::CancellationPolicyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("CancellationPolicy(id={})", value.id()));
        crate::CancellationPolicyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn reschedule_request<'a>(value: &'a crate::RescheduleRequest) -> crate::RescheduleRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RescheduleRequest(id={})", value.id()));
        crate::RescheduleRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn satisfaction_survey<'a>(value: &'a crate::SatisfactionSurvey) -> crate::SatisfactionSurveyExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SatisfactionSurvey(id={})", value.id()));
        crate::SatisfactionSurveyExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn follow_up_task<'a>(value: &'a crate::FollowUpTask) -> crate::FollowUpTaskExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("FollowUpTask(id={})", value.id()));
        crate::FollowUpTaskExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }
}


pub fn trigger_logic_bug_panic(root_desc: &str, failed_node: &str, attempted_path: &str) -> ! {
    let parts: Vec<&str> = attempted_path.split('.').collect();
    let break_idx = parts.iter().position(|&p| p == failed_node).unwrap_or(0);

    let mut nested_fix = String::new();
    if break_idx < parts.len() - 1 {
        nested_fix.push_str(&format!("\"select_{}(", failed_node));
        let mut close_parens = 1;
        for i in (break_idx + 1)..parts.len() {
            let sub_field = parts[i];
            let prev_field = parts[i-1];
            let is_last = i == parts.len() - 1;
            if is_last {
                nested_fix.push_str(&format!("Q::{}s().select_{}()", prev_field, sub_field));
            } else {
                nested_fix.push_str(&format!("Q::{}s().select_{}(", prev_field, sub_field));
                close_parens += 1;
            }
        }
        for _ in 0..close_parens {
            nested_fix.push(')');
        }
        nested_fix.push('"');
    } else {
        nested_fix = "null".to_string();
    }

    let suggested_fix = format!("\"select_{}()\"", failed_node);

    let access_path_json = format!("[{}]", parts.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(", "));
    let missing_preload_json = format!("[\"{}\"]", failed_node);

    let human_nested = if nested_fix != "null" { format!(" 或完整嵌套加载 {}", nested_fix) } else { String::new() };
    let root_name = root_desc.split('(').next().unwrap_or("Unknown");

    let mut root_snake = String::new();
    for (i, c) in root_name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                root_snake.push('_');
            }
            root_snake.push(c.to_ascii_lowercase());
        } else {
            root_snake.push(c);
        }
    }
    let id_part = root_desc.split('(').nth(1).unwrap_or(")");
    let mut original_expr = format!("E::{}({}", root_snake, id_part);
    for p in &parts {
        original_expr.push_str(&format!(".get_{}()", p));
        if *p == failed_node {
            original_expr.push_str("<broken>");
        }
    }

    let human_message = format!("\"访问 {}.{} 时缺少预加载。请在查询中加入 {}{}\"", root_name, attempted_path, suggested_fix, human_nested);

    panic!("\n\n💥 [Coding Logic Bug]\n\noriginal_expr_with_broken_point: \"{}\"\nroot: {}\naccess_path: {}\nbreak_point: \"{}\"\nmissing_preload: {}\nsuggested_fix: {}\nnested_fix: {}\nseverity: \"error\"\nhuman_message: {}\n", 
        original_expr, root_desc, access_path_json, failed_node, missing_preload_json, suggested_fix, nested_fix, human_message);
}

#[derive(Clone)]
pub struct ValueExpression<'a, T> {
    result: teaql_core::eval::EvalResult<T>,
    root_desc: std::sync::Arc<String>,
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: Clone> ValueExpression<'a, T> {
    pub fn new(result: teaql_core::eval::EvalResult<T>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc, _phantom: std::marker::PhantomData }
    }

    fn resolve(self) -> Option<T> {
        match self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(self) -> Option<T> {
        self.resolve()
    }

    pub fn unwrap(self) -> T {
        self.resolve().expect("Value was legitimately null in database!")
    }

    pub fn or_else(self, default_value: T) -> T {
        self.eval().unwrap_or(default_value)
    }

    pub fn or_else_with(self, default_fn: impl FnOnce() -> T) -> T {
        self.eval().unwrap_or_else(default_fn)
    }

    pub fn or_default(self) -> T where T: Default {
        self.eval().unwrap_or_default()
    }
}

