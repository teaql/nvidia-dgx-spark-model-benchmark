// The `E` expression wrapper provides zero-cost AST traversal
// and will automatically panic if it encounters a NotLoaded error.
pub struct E;

impl E {
    pub fn leave_type<'a>(value: &'a crate::LeaveType) -> crate::LeaveTypeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeaveType(id={})", value.id()));
        crate::LeaveTypeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee_status<'a>(value: &'a crate::EmployeeStatus) -> crate::EmployeeStatusExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("EmployeeStatus(id={})", value.id()));
        crate::EmployeeStatusExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contract_type<'a>(value: &'a crate::ContractType) -> crate::ContractTypeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ContractType(id={})", value.id()));
        crate::ContractTypeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn review_grade<'a>(value: &'a crate::ReviewGrade) -> crate::ReviewGradeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ReviewGrade(id={})", value.id()));
        crate::ReviewGradeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn application_status<'a>(value: &'a crate::ApplicationStatus) -> crate::ApplicationStatusExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ApplicationStatus(id={})", value.id()));
        crate::ApplicationStatusExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn platform<'a>(value: &'a crate::Platform) -> crate::PlatformExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Platform(id={})", value.id()));
        crate::PlatformExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn merchant<'a>(value: &'a crate::Merchant) -> crate::MerchantExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Merchant(id={})", value.id()));
        crate::MerchantExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn department<'a>(value: &'a crate::Department) -> crate::DepartmentExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Department(id={})", value.id()));
        crate::DepartmentExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn position<'a>(value: &'a crate::Position) -> crate::PositionExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Position(id={})", value.id()));
        crate::PositionExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn employee<'a>(value: &'a crate::Employee) -> crate::EmployeeExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Employee(id={})", value.id()));
        crate::EmployeeExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn salary_record<'a>(value: &'a crate::SalaryRecord) -> crate::SalaryRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("SalaryRecord(id={})", value.id()));
        crate::SalaryRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn attendance_log<'a>(value: &'a crate::AttendanceLog) -> crate::AttendanceLogExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("AttendanceLog(id={})", value.id()));
        crate::AttendanceLogExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn leave_request<'a>(value: &'a crate::LeaveRequest) -> crate::LeaveRequestExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("LeaveRequest(id={})", value.id()));
        crate::LeaveRequestExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn performance_review<'a>(value: &'a crate::PerformanceReview) -> crate::PerformanceReviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PerformanceReview(id={})", value.id()));
        crate::PerformanceReviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn training_record<'a>(value: &'a crate::TrainingRecord) -> crate::TrainingRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TrainingRecord(id={})", value.id()));
        crate::TrainingRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn benefit_plan<'a>(value: &'a crate::BenefitPlan) -> crate::BenefitPlanExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BenefitPlan(id={})", value.id()));
        crate::BenefitPlanExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn expense_claim<'a>(value: &'a crate::ExpenseClaim) -> crate::ExpenseClaimExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ExpenseClaim(id={})", value.id()));
        crate::ExpenseClaimExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn payroll_run<'a>(value: &'a crate::PayrollRun) -> crate::PayrollRunExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("PayrollRun(id={})", value.id()));
        crate::PayrollRunExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn tax_form<'a>(value: &'a crate::TaxForm) -> crate::TaxFormExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TaxForm(id={})", value.id()));
        crate::TaxFormExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn contract<'a>(value: &'a crate::Contract) -> crate::ContractExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Contract(id={})", value.id()));
        crate::ContractExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn resignation<'a>(value: &'a crate::Resignation) -> crate::ResignationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Resignation(id={})", value.id()));
        crate::ResignationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn warning_letter<'a>(value: &'a crate::WarningLetter) -> crate::WarningLetterExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("WarningLetter(id={})", value.id()));
        crate::WarningLetterExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn bonus_record<'a>(value: &'a crate::BonusRecord) -> crate::BonusRecordExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("BonusRecord(id={})", value.id()));
        crate::BonusRecordExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn shift_schedule<'a>(value: &'a crate::ShiftSchedule) -> crate::ShiftScheduleExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("ShiftSchedule(id={})", value.id()));
        crate::ShiftScheduleExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn time_off_balance<'a>(value: &'a crate::TimeOffBalance) -> crate::TimeOffBalanceExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("TimeOffBalance(id={})", value.id()));
        crate::TimeOffBalanceExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn recruitment_post<'a>(value: &'a crate::RecruitmentPost) -> crate::RecruitmentPostExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("RecruitmentPost(id={})", value.id()));
        crate::RecruitmentPostExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn job_application<'a>(value: &'a crate::JobApplication) -> crate::JobApplicationExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("JobApplication(id={})", value.id()));
        crate::JobApplicationExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn interview<'a>(value: &'a crate::Interview) -> crate::InterviewExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("Interview(id={})", value.id()));
        crate::InterviewExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn offer_letter<'a>(value: &'a crate::OfferLetter) -> crate::OfferLetterExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OfferLetter(id={})", value.id()));
        crate::OfferLetterExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn onboarding_checklist<'a>(value: &'a crate::OnboardingChecklist) -> crate::OnboardingChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OnboardingChecklist(id={})", value.id()));
        crate::OnboardingChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
    }

    pub fn offboarding_checklist<'a>(value: &'a crate::OffboardingChecklist) -> crate::OffboardingChecklistExpression<'a> {
        let root_desc = std::sync::Arc::new(format!("OffboardingChecklist(id={})", value.id()));
        crate::OffboardingChecklistExpression::new(teaql_core::eval::EvalResult::Value(value), root_desc)
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

