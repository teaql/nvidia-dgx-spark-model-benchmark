use std::collections::BTreeMap;
use crate::TeaqlRuntime;
use crate::Q;
use teaql_core::Entity;
use crate::request_support::TeaqlUserContextExt;
use crate::request_support::AuditedSave;

pub trait IntoU64 {
    fn into_u64(self) -> u64;
}

impl IntoU64 for u64 {
    fn into_u64(self) -> u64 {
        self
    }
}

impl IntoU64 for Option<&teaql_core::Value> {
    fn into_u64(self) -> u64 {
        self.and_then(|v| v.try_u64()).unwrap_or_default()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SampleDataScale {
    Tiny,
    Small,
    Medium,
}

pub struct SampleDataPlan {
    pub scale: SampleDataScale,
    pub seed: u64,
}

impl SampleDataPlan {
    pub fn small() -> Self {
        Self {
            scale: SampleDataScale::Small,
            seed: 0,
        }
    }
}

pub struct SampleDataReport {
    pub generated: BTreeMap<&'static str, usize>,
    pub skipped: Vec<SampleDataSkipped>,
}

pub struct SampleDataSkipped {
    pub entity: &'static str,
    pub reason: String,
}

pub struct SampleDataState {
    pub plan: SampleDataPlan,
    pub references: BTreeMap<&'static str, Vec<u64>>,
    pub generated: BTreeMap<&'static str, usize>,
    pub skipped: Vec<SampleDataSkipped>,
}

impl SampleDataState {
    pub fn new(plan: SampleDataPlan) -> Self {
        Self {
            plan,
            references: BTreeMap::new(),
            generated: BTreeMap::new(),
            skipped: Vec::new(),
        }
    }

    pub fn add_reference(&mut self, entity: &'static str, id: u64) {
        self.references.entry(entity).or_default().push(id);
    }

    pub fn ids(&self, entity: &'static str) -> &[u64] {
        self.references.get(entity).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn pick_id(&self, entity: &'static str, salt: usize) -> Option<u64> {
        let ids = self.ids(entity);
        if ids.is_empty() {
            None
        } else {
            Some(ids[salt % ids.len()])
        }
    }

    pub fn pick_unused_id(&self, entity: &'static str, salt: usize, used: &std::collections::HashSet<u64>) -> Option<u64> {
        let ids = self.ids(entity);
        if ids.is_empty() {
            return None;
        }

        let best_id = ids[salt % ids.len()];
        if !used.contains(&best_id) {
            return Some(best_id);
        }

        for id in ids {
            if !used.contains(id) {
                return Some(*id);
            }
        }

        Some(best_id)
    }

    pub fn record_generated(&mut self, entity: &'static str) {
        *self.generated.entry(entity).or_default() += 1;
    }

    pub fn record_skipped(&mut self, entity: &'static str, reason: String) {
        self.skipped.push(SampleDataSkipped { entity, reason });
    }

    pub fn into_report(self) -> SampleDataReport {
        SampleDataReport {
            generated: self.generated,
            skipped: self.skipped,
        }
    }
}

pub async fn generate_sample_data<C>(
    ctx: &C,
    plan: SampleDataPlan,
) -> Result<SampleDataReport, String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    log::info!("Starting sample data generation. Scale: {:?}, Seed: {}", plan.scale, plan.seed);
    let mut state = SampleDataState::new(plan);

    load_root_account_settingses(ctx, &mut state).await?; //depth: 0
    load_root_address_books(ctx, &mut state).await?; //depth: 0
    load_root_audit_trails(ctx, &mut state).await?; //depth: 0
    load_root_benefit_plans(ctx, &mut state).await?; //depth: 0
    load_root_billing_contacts(ctx, &mut state).await?; //depth: 0
    load_root_budget_allocations(ctx, &mut state).await?; //depth: 0
    load_root_cancellation_policies(ctx, &mut state).await?; //depth: 0
    load_root_compliance_certificates(ctx, &mut state).await?; //depth: 0
    load_root_contact_persons(ctx, &mut state).await?; //depth: 0
    load_root_credit_memoes(ctx, &mut state).await?; //depth: 0
    load_root_crew_assignments(ctx, &mut state).await?; //depth: 0
    load_root_currency_exchanges(ctx, &mut state).await?; //depth: 0
    load_root_customer_profiles(ctx, &mut state).await?; //depth: 0
    load_root_deposit_receipts(ctx, &mut state).await?; //depth: 0
    load_root_dispute_cases(ctx, &mut state).await?; //depth: 0
    load_root_document_uploads(ctx, &mut state).await?; //depth: 0
    load_root_driver_profiles(ctx, &mut state).await?; //depth: 0
    load_root_employee_handbooks(ctx, &mut state).await?; //depth: 0
    load_root_employee_records(ctx, &mut state).await?; //depth: 0
    load_root_equipment_allocations(ctx, &mut state).await?; //depth: 0
    load_root_equipment_inventory(ctx, &mut state).await?; //depth: 0
    load_root_expense_reports(ctx, &mut state).await?; //depth: 0
    load_root_feedback_reviews(ctx, &mut state).await?; //depth: 0
    load_root_financial_statements(ctx, &mut state).await?; //depth: 0
    load_root_follow_up_tasks(ctx, &mut state).await?; //depth: 0
    load_root_fuel_records(ctx, &mut state).await?; //depth: 0
    load_root_garage_assignments(ctx, &mut state).await?; //depth: 0
    load_root_incident_reports(ctx, &mut state).await?; //depth: 0
    load_root_inspection_checklists(ctx, &mut state).await?; //depth: 0
    load_root_invoices(ctx, &mut state).await?; //depth: 0
    load_root_job_schedules(ctx, &mut state).await?; //depth: 0
    load_root_load_manifests(ctx, &mut state).await?; //depth: 0
    load_root_loyalty_programs(ctx, &mut state).await?; //depth: 0
    load_root_maintenance_logs(ctx, &mut state).await?; //depth: 0
    load_root_move_orders(ctx, &mut state).await?; //depth: 0
    load_root_notification_prefs(ctx, &mut state).await?; //depth: 0
    load_root_payment_transactions(ctx, &mut state).await?; //depth: 0
    load_root_payroll_runs(ctx, &mut state).await?; //depth: 0
    load_root_preference_centers(ctx, &mut state).await?; //depth: 0
    load_root_receivable_agings(ctx, &mut state).await?; //depth: 0
    load_root_refund_requests(ctx, &mut state).await?; //depth: 0
    load_root_reschedule_requests(ctx, &mut state).await?; //depth: 0
    load_root_route_plans(ctx, &mut state).await?; //depth: 0
    load_root_satisfaction_surveys(ctx, &mut state).await?; //depth: 0
    load_root_service_histories(ctx, &mut state).await?; //depth: 0
    load_root_service_locations(ctx, &mut state).await?; //depth: 0
    load_root_special_instructionses(ctx, &mut state).await?; //depth: 0
    load_root_status_updates(ctx, &mut state).await?; //depth: 0
    load_root_tax_calculations(ctx, &mut state).await?; //depth: 0
    load_root_telematics_data(ctx, &mut state).await?; //depth: 0
    load_root_time_slots(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_registries(ctx, &mut state).await?; //depth: 0


    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_compensation_adjustments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_leave_requests(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_offboarding_processes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_onboarding_checklists(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_performance_reviews(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_tax_withholdings(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_timesheet_entries(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_training_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;


    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
}

async fn load_root_account_settingses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::account_settingses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Account Settings", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_address_books<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::address_books().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Address Book", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_audit_trails<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::audit_trails().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Audit Trail", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_benefit_plans<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::benefit_plans().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Benefit Plan", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_billing_contacts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::billing_contacts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Billing Contact", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_budget_allocations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::budget_allocations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Budget Allocation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_cancellation_policies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cancellation_policies().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Cancellation Policy", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_compliance_certificates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::compliance_certificates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Compliance Certificate", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_contact_persons<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::contact_persons().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Contact Person", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_credit_memoes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::credit_memoes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Credit Memo", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_crew_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::crew_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Crew Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_currency_exchanges<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::currency_exchanges().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Currency Exchange", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_profiles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Profile", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_deposit_receipts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::deposit_receipts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Deposit Receipt", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_dispute_cases<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::dispute_cases().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Dispute Case", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_document_uploads<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::document_uploads().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Document Upload", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_driver_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_profiles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver Profile", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_employee_handbooks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::employee_handbooks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Employee Handbook", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_employee_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::employee_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Employee Record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_equipment_allocations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::equipment_allocations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Equipment Allocation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_equipment_inventory<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::equipment_inventory().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Equipment Inventory", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_expense_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::expense_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Expense Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_feedback_reviews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::feedback_reviews().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Feedback Review", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_financial_statements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::financial_statements().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Financial Statement", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_follow_up_tasks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::follow_up_tasks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Follow Up Task", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fuel_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fuel_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fuel Record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_garage_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::garage_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Garage Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_incident_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::incident_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Incident Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_inspection_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::inspection_checklists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Inspection Checklist", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_invoices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::invoices().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Invoice", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_job_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::job_schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Job Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_load_manifests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::load_manifests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Load Manifest", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_loyalty_programs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::loyalty_programs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Loyalty Program", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_maintenance_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::maintenance_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Maintenance Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_move_orders<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::move_orders().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Move Order", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_notification_prefs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::notification_prefs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Notification Preference", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payment_transactions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payment_transactions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payment Transaction", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payroll_runs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payroll_runs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payroll Run", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_preference_centers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::preference_centers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Preference Center", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_receivable_agings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::receivable_agings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Receivable Aging", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_refund_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::refund_requests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Refund Request", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_reschedule_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::reschedule_requests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Reschedule Request", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_route_plans<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::route_plans().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Route Plan", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_satisfaction_surveys<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::satisfaction_surveys().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Satisfaction Survey", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_service_histories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_histories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service History", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_service_locations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_locations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service Location", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_special_instructionses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::special_instructionses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Special Instructions", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_status_updates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::status_updates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Status Update", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tax_calculations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tax_calculations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tax Calculation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_telematics_data<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::telematics_data().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Telematics Data", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_time_slots<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::time_slots().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Time Slot", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_registries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_registries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Registry", item.id().into_u64());
    }
    Ok(())
}


async fn generate_compensation_adjustments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Compensation Adjustment", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Compensation Adjustment: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Compensation Adjustment", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Compensation Adjustment: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Compensation Adjustment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::compensation_adjustments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_approved_by_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_adjustment_type(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "decimal()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_effective_date(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Compensation Adjustment");

        if i % 20 == 0 {
            log::info!("Generating Compensation Adjustment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Compensation Adjustment.");
    Ok(())
}


async fn generate_leave_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Leave Request", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Leave Request: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Leave Request (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::leave_requests().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_leave_type(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_start_date(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_end_date(past.format("%Y-%m-%d").to_string());
                }

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Leave Request");

        if i % 20 == 0 {
            log::info!("Generating Leave Request: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Leave Request.");
    Ok(())
}


async fn generate_offboarding_processes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Offboarding Process", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Offboarding Process: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Offboarding Process (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::offboarding_processes().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_process_step(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_due_date(past.format("%Y-%m-%d").to_string());
                }

                entity.update_completed(format!("{} {}", "boolean()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Offboarding Process");

        if i % 20 == 0 {
            log::info!("Generating Offboarding Process: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Offboarding Process.");
    Ok(())
}


async fn generate_onboarding_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Onboarding Checklist", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Onboarding Checklist: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Onboarding Checklist (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::onboarding_checklists().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_task_description(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_due_date(past.format("%Y-%m-%d").to_string());
                }

                entity.update_completed(format!("{} {}", "boolean()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Onboarding Checklist");

        if i % 20 == 0 {
            log::info!("Generating Onboarding Checklist: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Onboarding Checklist.");
    Ok(())
}


async fn generate_performance_reviews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Performance Review", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Performance Review: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Performance Review", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Performance Review: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Performance Review (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::performance_reviews().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_reviewer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_review_date(past.format("%Y-%m-%d").to_string());
                }

                entity.update_rating(format!("{} {}", "decimal()", i + 1));

                entity.update_comments(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Performance Review");

        if i % 20 == 0 {
            log::info!("Generating Performance Review: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Performance Review.");
    Ok(())
}


async fn generate_tax_withholdings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Tax Withholding", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Tax Withholding: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Tax Withholding (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::tax_withholdings().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_tax_type(format!("{} {}", "string()", i + 1));

                entity.update_rate(format!("{} {}", "decimal()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_effective_date(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Tax Withholding");

        if i % 20 == 0 {
            log::info!("Generating Tax Withholding: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Tax Withholding.");
    Ok(())
}


async fn generate_timesheet_entries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Timesheet Entry", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Timesheet Entry: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Timesheet Entry (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::timesheet_entries().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_work_date(past.format("%Y-%m-%d").to_string());
                }

                entity.update_hours_worked(format!("{} {}", "decimal()", i + 1));

                entity.update_description(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Timesheet Entry");

        if i % 20 == 0 {
            log::info!("Generating Timesheet Entry: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Timesheet Entry.");
    Ok(())
}


async fn generate_training_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Record").is_empty() {
            state.record_skipped("Training Record", "Required dependency Employee Record is missing in reference pool".to_string());
            log::info!("Skipped generating Training Record: Required dependency Employee Record is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Training Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::training_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee Record", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_training_name(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_completion_date(past.format("%Y-%m-%d").to_string());
                }

                entity.update_provider(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Training Record");

        if i % 20 == 0 {
            log::info!("Generating Training Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Training Record.");
    Ok(())
}
