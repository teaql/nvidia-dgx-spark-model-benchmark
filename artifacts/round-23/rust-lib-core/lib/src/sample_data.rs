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

    load_root_audit_trails(ctx, &mut state).await?; //depth: 0
    load_root_benefits_plans(ctx, &mut state).await?; //depth: 0
    load_root_billing_cycles(ctx, &mut state).await?; //depth: 0
    load_root_budget_allocations(ctx, &mut state).await?; //depth: 0
    load_root_cancellation_requests(ctx, &mut state).await?; //depth: 0
    load_root_cargo_securements(ctx, &mut state).await?; //depth: 0
    load_root_certification_records(ctx, &mut state).await?; //depth: 0
    load_root_competency_matrixes(ctx, &mut state).await?; //depth: 0
    load_root_compliance_audits(ctx, &mut state).await?; //depth: 0
    load_root_contract_termses(ctx, &mut state).await?; //depth: 0
    load_root_cost_centers(ctx, &mut state).await?; //depth: 0
    load_root_credit_notes(ctx, &mut state).await?; //depth: 0
    load_root_crew_assignments(ctx, &mut state).await?; //depth: 0
    load_root_currency_conversions(ctx, &mut state).await?; //depth: 0
    load_root_customer_accounts(ctx, &mut state).await?; //depth: 0
    load_root_customer_addresses(ctx, &mut state).await?; //depth: 0
    load_root_customer_contacts(ctx, &mut state).await?; //depth: 0
    load_root_customer_feedback(ctx, &mut state).await?; //depth: 0
    load_root_customer_preferences(ctx, &mut state).await?; //depth: 0
    load_root_customer_profiles(ctx, &mut state).await?; //depth: 0
    load_root_customer_segments(ctx, &mut state).await?; //depth: 0
    load_root_customs_documentations(ctx, &mut state).await?; //depth: 0
    load_root_debit_notes(ctx, &mut state).await?; //depth: 0
    load_root_decommission_records(ctx, &mut state).await?; //depth: 0
    load_root_delivery_locations(ctx, &mut state).await?; //depth: 0
    load_root_disciplinary_actions(ctx, &mut state).await?; //depth: 0
    load_root_discount_policies(ctx, &mut state).await?; //depth: 0
    load_root_dispute_records(ctx, &mut state).await?; //depth: 0
    load_root_driver_assignments(ctx, &mut state).await?; //depth: 0
    load_root_driver_licenses(ctx, &mut state).await?; //depth: 0
    load_root_driver_trainings(ctx, &mut state).await?; //depth: 0
    load_root_employee_records(ctx, &mut state).await?; //depth: 0
    load_root_equipment_allocations(ctx, &mut state).await?; //depth: 0
    load_root_exit_interviews(ctx, &mut state).await?; //depth: 0
    load_root_expense_categories(ctx, &mut state).await?; //depth: 0
    load_root_financial_reports(ctx, &mut state).await?; //depth: 0
    load_root_fiscal_periods(ctx, &mut state).await?; //depth: 0
    load_root_fuel_records(ctx, &mut state).await?; //depth: 0
    load_root_gps_trackings(ctx, &mut state).await?; //depth: 0
    load_root_grievance_logs(ctx, &mut state).await?; //depth: 0
    load_root_hazard_assessments(ctx, &mut state).await?; //depth: 0
    load_root_incident_reports(ctx, &mut state).await?; //depth: 0
    load_root_inspection_checklists(ctx, &mut state).await?; //depth: 0
    load_root_insurance_coverages(ctx, &mut state).await?; //depth: 0
    load_root_invoice_histories(ctx, &mut state).await?; //depth: 0
    load_root_invoice_templates(ctx, &mut state).await?; //depth: 0
    load_root_job_orders(ctx, &mut state).await?; //depth: 0
    load_root_lead_sources(ctx, &mut state).await?; //depth: 0
    load_root_liability_waivers(ctx, &mut state).await?; //depth: 0
    load_root_load_capacities(ctx, &mut state).await?; //depth: 0
    load_root_loading_docks(ctx, &mut state).await?; //depth: 0
    load_root_loyalty_programs(ctx, &mut state).await?; //depth: 0
    load_root_maintenance_logs(ctx, &mut state).await?; //depth: 0
    load_root_marketing_campaigns(ctx, &mut state).await?; //depth: 0
    load_root_move_schedules(ctx, &mut state).await?; //depth: 0
    load_root_notification_templates(ctx, &mut state).await?; //depth: 0
    load_root_offboarding_checklists(ctx, &mut state).await?; //depth: 0
    load_root_onboarding_checklists(ctx, &mut state).await?; //depth: 0
    load_root_payment_gateways(ctx, &mut state).await?; //depth: 0
    load_root_payment_methods(ctx, &mut state).await?; //depth: 0
    load_root_payroll_info(ctx, &mut state).await?; //depth: 0
    load_root_performance_kpis(ctx, &mut state).await?; //depth: 0
    load_root_performance_reviews(ctx, &mut state).await?; //depth: 0
    load_root_permit_requireds(ctx, &mut state).await?; //depth: 0
    load_root_pickup_locations(ctx, &mut state).await?; //depth: 0
    load_root_policy_acknowledgments(ctx, &mut state).await?; //depth: 0
    load_root_reconciliation_entries(ctx, &mut state).await?; //depth: 0
    load_root_referral_codes(ctx, &mut state).await?; //depth: 0
    load_root_refund_processes(ctx, &mut state).await?; //depth: 0
    load_root_renewal_notices(ctx, &mut state).await?; //depth: 0
    load_root_route_plans(ctx, &mut state).await?; //depth: 0
    load_root_safety_incidents(ctx, &mut state).await?; //depth: 0
    load_root_service_agreements(ctx, &mut state).await?; //depth: 0
    load_root_service_schedules(ctx, &mut state).await?; //depth: 0
    load_root_shift_schedules(ctx, &mut state).await?; //depth: 0
    load_root_sla_metrics(ctx, &mut state).await?; //depth: 0
    load_root_status_updates(ctx, &mut state).await?; //depth: 0
    load_root_tax_jurisdictions(ctx, &mut state).await?; //depth: 0
    load_root_tax_rates(ctx, &mut state).await?; //depth: 0
    load_root_telematics_data(ctx, &mut state).await?; //depth: 0
    load_root_time_off_requests(ctx, &mut state).await?; //depth: 0
    load_root_tire_inventory(ctx, &mut state).await?; //depth: 0
    load_root_tracking_numbers(ctx, &mut state).await?; //depth: 0
    load_root_training_courses(ctx, &mut state).await?; //depth: 0
    load_root_transaction_logs(ctx, &mut state).await?; //depth: 0
    load_root_transit_time_estimates(ctx, &mut state).await?; //depth: 0
    load_root_unloading_docks(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_registries(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_specs(ctx, &mut state).await?; //depth: 0
    load_root_warranty_info(ctx, &mut state).await?; //depth: 0



    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
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

async fn load_root_benefits_plans<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::benefits_plans().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Benefits Plan", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_billing_cycles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::billing_cycles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Billing Cycle", item.id().into_u64());
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

async fn load_root_cancellation_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cancellation_requests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Cancellation Request", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_cargo_securements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cargo_securements().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Cargo Securement", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_certification_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::certification_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Certification Record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_competency_matrixes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::competency_matrixes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Competency Matrix", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_compliance_audits<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::compliance_audits().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Compliance Audit", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_contract_termses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::contract_termses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Contract Terms", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_cost_centers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cost_centers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Cost Center", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_credit_notes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::credit_notes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Credit Note", item.id().into_u64());
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

async fn load_root_currency_conversions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::currency_conversions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Currency Conversion", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_accounts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_accounts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Account", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_addresses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_addresses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Address", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_contacts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_contacts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Contact", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_feedback<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_feedback().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Feedback", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_preferences<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_preferences().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Preference", item.id().into_u64());
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

async fn load_root_customer_segments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_segments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Segment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customs_documentations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customs_documentations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customs Documentation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_debit_notes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::debit_notes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Debit Note", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_decommission_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::decommission_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Decommission Record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_delivery_locations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::delivery_locations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Delivery Location", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_disciplinary_actions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::disciplinary_actions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Disciplinary Action", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_discount_policies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::discount_policies().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Discount Policy", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_dispute_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::dispute_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Dispute Record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_driver_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_driver_licenses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_licenses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver License", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_driver_trainings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_trainings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver Training", item.id().into_u64());
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

async fn load_root_exit_interviews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::exit_interviews().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Exit Interview", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_expense_categories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::expense_categories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Expense Category", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_financial_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::financial_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Financial Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fiscal_periods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fiscal_periods().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fiscal Period", item.id().into_u64());
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

async fn load_root_gps_trackings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::gps_trackings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("GPS Tracking", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_grievance_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::grievance_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Grievance Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_hazard_assessments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::hazard_assessments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Hazard Assessment", item.id().into_u64());
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

async fn load_root_insurance_coverages<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::insurance_coverages().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Insurance Coverage", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_invoice_histories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::invoice_histories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Invoice History", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_invoice_templates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::invoice_templates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Invoice Template", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_job_orders<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::job_orders().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Job Order", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_lead_sources<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::lead_sources().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Lead Source", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_liability_waivers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::liability_waivers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Liability Waiver", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_load_capacities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::load_capacities().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Load Capacity", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_loading_docks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::loading_docks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Loading Dock", item.id().into_u64());
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

async fn load_root_marketing_campaigns<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::marketing_campaigns().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Marketing Campaign", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_move_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::move_schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Move Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_notification_templates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::notification_templates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Notification Template", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_offboarding_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::offboarding_checklists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Offboarding Checklist", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_onboarding_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::onboarding_checklists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Onboarding Checklist", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payment_gateways<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payment_gateways().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payment Gateway", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payment_methods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payment_methods().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payment Method", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payroll_info<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payroll_info().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payroll Info", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_performance_kpis<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::performance_kpis().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Performance KPI", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_performance_reviews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::performance_reviews().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Performance Review", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_permit_requireds<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::permit_requireds().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Permit Required", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_pickup_locations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::pickup_locations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Pickup Location", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_policy_acknowledgments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::policy_acknowledgments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Policy Acknowledgment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_reconciliation_entries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::reconciliation_entries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Reconciliation Entry", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_referral_codes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::referral_codes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Referral Code", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_refund_processes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::refund_processes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Refund Process", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_renewal_notices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::renewal_notices().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Renewal Notice", item.id().into_u64());
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

async fn load_root_safety_incidents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::safety_incidents().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Safety Incident", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_service_agreements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_agreements().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service Agreement", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_service_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_shift_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::shift_schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Shift Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_sla_metrics<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::sla_metrics().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("SLA Metric", item.id().into_u64());
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

async fn load_root_tax_jurisdictions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tax_jurisdictions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tax Jurisdiction", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tax_rates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tax_rates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tax Rate", item.id().into_u64());
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

async fn load_root_time_off_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::time_off_requests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Time Off Request", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tire_inventory<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tire_inventory().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tire Inventory", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tracking_numbers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tracking_numbers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tracking Number", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_training_courses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::training_courses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Training Course", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_transaction_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::transaction_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Transaction Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_transit_time_estimates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::transit_time_estimates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Transit Time Estimate", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_unloading_docks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::unloading_docks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Unloading Dock", item.id().into_u64());
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

async fn load_root_vehicle_specs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_specs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Spec", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_warranty_info<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::warranty_info().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Warranty Info", item.id().into_u64());
    }
    Ok(())
}


