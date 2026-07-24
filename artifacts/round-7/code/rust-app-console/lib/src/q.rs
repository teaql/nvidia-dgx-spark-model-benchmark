use teaql_core::Expr;

use crate::*;

pub struct PurposedQuery<T> {
    pub inner: T,
    pub purpose: String,
}

impl<T> PurposedQuery<T> {
    pub fn new(inner: T, purpose: impl Into<String>) -> Self {
        Self { inner, purpose: purpose.into() }
    }
}

pub struct Q;

impl Q {
    pub fn leave_types() -> LeaveTypeRequest {
        LeaveTypeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leave_types_minimal() -> LeaveTypeRequest {
        LeaveTypeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leave_types_with_children() -> LeaveTypeRequest {
        LeaveTypeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employee_statuses() -> EmployeeStatusRequest {
        EmployeeStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_statuses_minimal() -> EmployeeStatusRequest {
        EmployeeStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employee_statuses_with_children() -> EmployeeStatusRequest {
        EmployeeStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contract_types() -> ContractTypeRequest {
        ContractTypeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contract_types_minimal() -> ContractTypeRequest {
        ContractTypeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contract_types_with_children() -> ContractTypeRequest {
        ContractTypeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn review_grades() -> ReviewGradeRequest {
        ReviewGradeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn review_grades_minimal() -> ReviewGradeRequest {
        ReviewGradeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn review_grades_with_children() -> ReviewGradeRequest {
        ReviewGradeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn application_statuses() -> ApplicationStatusRequest {
        ApplicationStatusRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn application_statuses_minimal() -> ApplicationStatusRequest {
        ApplicationStatusRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn application_statuses_with_children() -> ApplicationStatusRequest {
        ApplicationStatusRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn platforms() -> PlatformRequest {
        PlatformRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_minimal() -> PlatformRequest {
        PlatformRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_with_children() -> PlatformRequest {
        PlatformRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn merchants() -> MerchantRequest {
        MerchantRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchants_minimal() -> MerchantRequest {
        MerchantRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchants_with_children() -> MerchantRequest {
        MerchantRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn departments() -> DepartmentRequest {
        DepartmentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn departments_minimal() -> DepartmentRequest {
        DepartmentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn departments_with_children() -> DepartmentRequest {
        DepartmentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn positions() -> PositionRequest {
        PositionRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn positions_minimal() -> PositionRequest {
        PositionRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn positions_with_children() -> PositionRequest {
        PositionRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employees() -> EmployeeRequest {
        EmployeeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_minimal() -> EmployeeRequest {
        EmployeeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_with_children() -> EmployeeRequest {
        EmployeeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn salary_records() -> SalaryRecordRequest {
        SalaryRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn salary_records_minimal() -> SalaryRecordRequest {
        SalaryRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn salary_records_with_children() -> SalaryRecordRequest {
        SalaryRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn attendance_logs() -> AttendanceLogRequest {
        AttendanceLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attendance_logs_minimal() -> AttendanceLogRequest {
        AttendanceLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn attendance_logs_with_children() -> AttendanceLogRequest {
        AttendanceLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn leave_requests() -> LeaveRequestRequest {
        LeaveRequestRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leave_requests_minimal() -> LeaveRequestRequest {
        LeaveRequestRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn leave_requests_with_children() -> LeaveRequestRequest {
        LeaveRequestRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn performance_reviews() -> PerformanceReviewRequest {
        PerformanceReviewRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_reviews_minimal() -> PerformanceReviewRequest {
        PerformanceReviewRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn performance_reviews_with_children() -> PerformanceReviewRequest {
        PerformanceReviewRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn training_records() -> TrainingRecordRequest {
        TrainingRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_records_minimal() -> TrainingRecordRequest {
        TrainingRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn training_records_with_children() -> TrainingRecordRequest {
        TrainingRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn benefit_plans() -> BenefitPlanRequest {
        BenefitPlanRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefit_plans_minimal() -> BenefitPlanRequest {
        BenefitPlanRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn benefit_plans_with_children() -> BenefitPlanRequest {
        BenefitPlanRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn expense_claims() -> ExpenseClaimRequest {
        ExpenseClaimRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_claims_minimal() -> ExpenseClaimRequest {
        ExpenseClaimRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn expense_claims_with_children() -> ExpenseClaimRequest {
        ExpenseClaimRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payroll_runs() -> PayrollRunRequest {
        PayrollRunRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_runs_minimal() -> PayrollRunRequest {
        PayrollRunRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payroll_runs_with_children() -> PayrollRunRequest {
        PayrollRunRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn tax_forms() -> TaxFormRequest {
        TaxFormRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_forms_minimal() -> TaxFormRequest {
        TaxFormRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn tax_forms_with_children() -> TaxFormRequest {
        TaxFormRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contracts() -> ContractRequest {
        ContractRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contracts_minimal() -> ContractRequest {
        ContractRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contracts_with_children() -> ContractRequest {
        ContractRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn resignations() -> ResignationRequest {
        ResignationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn resignations_minimal() -> ResignationRequest {
        ResignationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn resignations_with_children() -> ResignationRequest {
        ResignationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn warning_letters() -> WarningLetterRequest {
        WarningLetterRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warning_letters_minimal() -> WarningLetterRequest {
        WarningLetterRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn warning_letters_with_children() -> WarningLetterRequest {
        WarningLetterRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn bonus_records() -> BonusRecordRequest {
        BonusRecordRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bonus_records_minimal() -> BonusRecordRequest {
        BonusRecordRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn bonus_records_with_children() -> BonusRecordRequest {
        BonusRecordRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn shift_schedules() -> ShiftScheduleRequest {
        ShiftScheduleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_schedules_minimal() -> ShiftScheduleRequest {
        ShiftScheduleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn shift_schedules_with_children() -> ShiftScheduleRequest {
        ShiftScheduleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn time_off_balances() -> TimeOffBalanceRequest {
        TimeOffBalanceRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn time_off_balances_minimal() -> TimeOffBalanceRequest {
        TimeOffBalanceRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn time_off_balances_with_children() -> TimeOffBalanceRequest {
        TimeOffBalanceRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn recruitment_posts() -> RecruitmentPostRequest {
        RecruitmentPostRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn recruitment_posts_minimal() -> RecruitmentPostRequest {
        RecruitmentPostRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn recruitment_posts_with_children() -> RecruitmentPostRequest {
        RecruitmentPostRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn job_applications() -> JobApplicationRequest {
        JobApplicationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_applications_minimal() -> JobApplicationRequest {
        JobApplicationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn job_applications_with_children() -> JobApplicationRequest {
        JobApplicationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn interviews() -> InterviewRequest {
        InterviewRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn interviews_minimal() -> InterviewRequest {
        InterviewRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn interviews_with_children() -> InterviewRequest {
        InterviewRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn offer_letters() -> OfferLetterRequest {
        OfferLetterRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offer_letters_minimal() -> OfferLetterRequest {
        OfferLetterRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offer_letters_with_children() -> OfferLetterRequest {
        OfferLetterRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn onboarding_checklists() -> OnboardingChecklistRequest {
        OnboardingChecklistRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn onboarding_checklists_minimal() -> OnboardingChecklistRequest {
        OnboardingChecklistRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn onboarding_checklists_with_children() -> OnboardingChecklistRequest {
        OnboardingChecklistRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn offboarding_checklists() -> OffboardingChecklistRequest {
        OffboardingChecklistRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offboarding_checklists_minimal() -> OffboardingChecklistRequest {
        OffboardingChecklistRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn offboarding_checklists_with_children() -> OffboardingChecklistRequest {
        OffboardingChecklistRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}