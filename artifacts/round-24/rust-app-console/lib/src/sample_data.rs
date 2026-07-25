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

    load_root_aging_reports(ctx, &mut state).await?; //depth: 0
    load_root_annual_performances(ctx, &mut state).await?; //depth: 0
    load_root_audit_trails(ctx, &mut state).await?; //depth: 0
    load_root_billing_accuracies(ctx, &mut state).await?; //depth: 0
    load_root_billing_addresses(ctx, &mut state).await?; //depth: 0
    load_root_billing_adjustments(ctx, &mut state).await?; //depth: 0
    load_root_billing_approvals(ctx, &mut state).await?; //depth: 0
    load_root_billing_cycles(ctx, &mut state).await?; //depth: 0
    load_root_claim_rates(ctx, &mut state).await?; //depth: 0
    load_root_compliance_checks(ctx, &mut state).await?; //depth: 0
    load_root_cost_analyses(ctx, &mut state).await?; //depth: 0
    load_root_credit_notes(ctx, &mut state).await?; //depth: 0
    load_root_crew_assignments(ctx, &mut state).await?; //depth: 0
    load_root_currency_rates(ctx, &mut state).await?; //depth: 0
    load_root_customer_accounts(ctx, &mut state).await?; //depth: 0
    load_root_customer_addresses(ctx, &mut state).await?; //depth: 0
    load_root_customer_claims(ctx, &mut state).await?; //depth: 0
    load_root_customer_contacts(ctx, &mut state).await?; //depth: 0
    load_root_customer_contracts(ctx, &mut state).await?; //depth: 0
    load_root_customer_feedback(ctx, &mut state).await?; //depth: 0
    load_root_customer_invoices(ctx, &mut state).await?; //depth: 0
    load_root_customer_leads(ctx, &mut state).await?; //depth: 0
    load_root_customer_loyalties(ctx, &mut state).await?; //depth: 0
    load_root_customer_move_histories(ctx, &mut state).await?; //depth: 0
    load_root_customer_notifications(ctx, &mut state).await?; //depth: 0
    load_root_customer_payments(ctx, &mut state).await?; //depth: 0
    load_root_customer_preferences(ctx, &mut state).await?; //depth: 0
    load_root_customer_preferred_times(ctx, &mut state).await?; //depth: 0
    load_root_customer_profiles(ctx, &mut state).await?; //depth: 0
    load_root_customer_quotes(ctx, &mut state).await?; //depth: 0
    load_root_customer_satisfactions(ctx, &mut state).await?; //depth: 0
    load_root_customer_segments(ctx, &mut state).await?; //depth: 0
    load_root_customer_services(ctx, &mut state).await?; //depth: 0
    load_root_customer_support_tickets(ctx, &mut state).await?; //depth: 0
    load_root_customer_vehicles(ctx, &mut state).await?; //depth: 0
    load_root_customs_documentations(ctx, &mut state).await?; //depth: 0
    load_root_daily_summaries(ctx, &mut state).await?; //depth: 0
    load_root_debit_notes(ctx, &mut state).await?; //depth: 0
    load_root_delivery_confirmations(ctx, &mut state).await?; //depth: 0
    load_root_discount_rules(ctx, &mut state).await?; //depth: 0
    load_root_dock_schedulings(ctx, &mut state).await?; //depth: 0
    load_root_driver_availabilities(ctx, &mut state).await?; //depth: 0
    load_root_driver_certifications(ctx, &mut state).await?; //depth: 0
    load_root_driver_licenses(ctx, &mut state).await?; //depth: 0
    load_root_driver_performances(ctx, &mut state).await?; //depth: 0
    load_root_driver_productivities(ctx, &mut state).await?; //depth: 0
    load_root_driver_profiles(ctx, &mut state).await?; //depth: 0
    load_root_driver_trainings(ctx, &mut state).await?; //depth: 0
    load_root_equipment_checklists(ctx, &mut state).await?; //depth: 0
    load_root_exception_handlings(ctx, &mut state).await?; //depth: 0
    load_root_executive_dashboards(ctx, &mut state).await?; //depth: 0
    load_root_expense_variances(ctx, &mut state).await?; //depth: 0
    load_root_financial_periods(ctx, &mut state).await?; //depth: 0
    load_root_fleet_dispatches(ctx, &mut state).await?; //depth: 0
    load_root_fleet_efficiencies(ctx, &mut state).await?; //depth: 0
    load_root_fleet_operators(ctx, &mut state).await?; //depth: 0
    load_root_fleet_vehicles(ctx, &mut state).await?; //depth: 0
    load_root_forecast_vs_actuals(ctx, &mut state).await?; //depth: 0
    load_root_geographic_distributions(ctx, &mut state).await?; //depth: 0
    load_root_inventory_snapshots(ctx, &mut state).await?; //depth: 0
    load_root_invoice_agings(ctx, &mut state).await?; //depth: 0
    load_root_invoice_headers(ctx, &mut state).await?; //depth: 0
    load_root_invoice_line_items(ctx, &mut state).await?; //depth: 0
    load_root_load_plans(ctx, &mut state).await?; //depth: 0
    load_root_loading_procedures(ctx, &mut state).await?; //depth: 0
    load_root_monthly_kpis(ctx, &mut state).await?; //depth: 0
    load_root_move_orders(ctx, &mut state).await?; //depth: 0
    load_root_move_schedules(ctx, &mut state).await?; //depth: 0
    load_root_move_volume_trends(ctx, &mut state).await?; //depth: 0
    load_root_on_time_deliveries(ctx, &mut state).await?; //depth: 0
    load_root_operations_dashboards(ctx, &mut state).await?; //depth: 0
    load_root_outstanding_balances(ctx, &mut state).await?; //depth: 0
    load_root_payment_methods(ctx, &mut state).await?; //depth: 0
    load_root_payment_reminders(ctx, &mut state).await?; //depth: 0
    load_root_payment_transactions(ctx, &mut state).await?; //depth: 0
    load_root_performance_metrics(ctx, &mut state).await?; //depth: 0
    load_root_profit_margins(ctx, &mut state).await?; //depth: 0
    load_root_refund_requests(ctx, &mut state).await?; //depth: 0
    load_root_revenue_recognitions(ctx, &mut state).await?; //depth: 0
    load_root_route_plans(ctx, &mut state).await?; //depth: 0
    load_root_safety_incidents(ctx, &mut state).await?; //depth: 0
    load_root_service_line_performances(ctx, &mut state).await?; //depth: 0
    load_root_tax_codes(ctx, &mut state).await?; //depth: 0
    load_root_transit_monitorings(ctx, &mut state).await?; //depth: 0
    load_root_unloading_procedures(ctx, &mut state).await?; //depth: 0
    load_root_utilization_reports(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_assignments(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_cleaning_schedules(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_damage_reports(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_fuel_logs(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_inspections(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_insurances(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_maintenances(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_odometers(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_registrations(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_specs(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_utilizations(ctx, &mut state).await?; //depth: 0
    load_root_warehouse_allocations(ctx, &mut state).await?; //depth: 0
    load_root_weekly_reports(ctx, &mut state).await?; //depth: 0
    load_root_yard_managements(ctx, &mut state).await?; //depth: 0



    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
}

async fn load_root_aging_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::aging_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Aging Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_annual_performances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::annual_performances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Annual Performance", item.id().into_u64());
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

async fn load_root_billing_accuracies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::billing_accuracies().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Billing Accuracy", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_billing_addresses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::billing_addresses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Billing Address", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_billing_adjustments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::billing_adjustments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Billing Adjustment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_billing_approvals<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::billing_approvals().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Billing Approval", item.id().into_u64());
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

async fn load_root_claim_rates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::claim_rates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Claim Rate", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_compliance_checks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::compliance_checks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Compliance Check", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_cost_analyses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cost_analyses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Cost Analysis", item.id().into_u64());
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

async fn load_root_currency_rates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::currency_rates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Currency Rate", item.id().into_u64());
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

async fn load_root_customer_claims<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_claims().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Claim", item.id().into_u64());
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

async fn load_root_customer_contracts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_contracts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Contract", item.id().into_u64());
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

async fn load_root_customer_invoices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_invoices().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Invoice", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_leads<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_leads().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Lead", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_loyalties<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_loyalties().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Loyalty", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_move_histories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_move_histories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Move History", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_notifications<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_notifications().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Notification", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_payments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_payments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Payment", item.id().into_u64());
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

async fn load_root_customer_preferred_times<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_preferred_times().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Preferred Time", item.id().into_u64());
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

async fn load_root_customer_quotes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_quotes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Quote", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_satisfactions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_satisfactions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Satisfaction", item.id().into_u64());
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

async fn load_root_customer_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_services().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Service", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_support_tickets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_support_tickets().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Support Ticket", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_vehicles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_vehicles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Vehicle", item.id().into_u64());
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

async fn load_root_daily_summaries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::daily_summaries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Daily Summary", item.id().into_u64());
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

async fn load_root_delivery_confirmations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::delivery_confirmations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Delivery Confirmation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_discount_rules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::discount_rules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Discount Rule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_dock_schedulings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::dock_schedulings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Dock Scheduling", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_driver_availabilities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_availabilities().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver Availability", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_driver_certifications<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_certifications().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver Certification", item.id().into_u64());
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

async fn load_root_driver_performances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_performances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver Performance", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_driver_productivities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::driver_productivities().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver Productivity", item.id().into_u64());
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

async fn load_root_equipment_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::equipment_checklists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Equipment Checklist", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_exception_handlings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::exception_handlings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Exception Handling", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_executive_dashboards<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::executive_dashboards().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Executive Dashboard", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_expense_variances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::expense_variances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Expense Variance", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_financial_periods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::financial_periods().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Financial Period", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fleet_dispatches<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fleet_dispatches().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fleet Dispatch", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fleet_efficiencies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fleet_efficiencies().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fleet Efficiency", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fleet_operators<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fleet_operators().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fleet Operator", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fleet_vehicles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fleet_vehicles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fleet Vehicle", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_forecast_vs_actuals<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::forecast_vs_actuals().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Forecast vs Actual", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_geographic_distributions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::geographic_distributions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Geographic Distribution", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_inventory_snapshots<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::inventory_snapshots().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Inventory Snapshot", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_invoice_agings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::invoice_agings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Invoice Aging", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_invoice_headers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::invoice_headers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Invoice Header", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_invoice_line_items<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::invoice_line_items().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Invoice Line Item", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_load_plans<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::load_plans().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Load Plan", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_loading_procedures<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::loading_procedures().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Loading Procedure", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_monthly_kpis<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::monthly_kpis().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Monthly KPI", item.id().into_u64());
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

async fn load_root_move_volume_trends<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::move_volume_trends().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Move Volume Trend", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_on_time_deliveries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::on_time_deliveries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("On Time Delivery", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_operations_dashboards<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::operations_dashboards().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Operations Dashboard", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_outstanding_balances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::outstanding_balances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Outstanding Balance", item.id().into_u64());
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

async fn load_root_payment_reminders<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payment_reminders().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payment Reminder", item.id().into_u64());
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

async fn load_root_performance_metrics<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::performance_metrics().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Performance Metric", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_profit_margins<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::profit_margins().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Profit Margin", item.id().into_u64());
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

async fn load_root_revenue_recognitions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::revenue_recognitions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Revenue Recognition", item.id().into_u64());
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

async fn load_root_service_line_performances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_line_performances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service Line Performance", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tax_codes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tax_codes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tax Code", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_transit_monitorings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::transit_monitorings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Transit Monitoring", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_unloading_procedures<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::unloading_procedures().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Unloading Procedure", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_utilization_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::utilization_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Utilization Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_cleaning_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_cleaning_schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Cleaning Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_damage_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_damage_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Damage Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_fuel_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_fuel_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Fuel Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_inspections<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_inspections().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Inspection", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_insurances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_insurances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Insurance", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_maintenances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_maintenances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Maintenance", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_odometers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_odometers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Odometer", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_registrations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_registrations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Registration", item.id().into_u64());
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

async fn load_root_vehicle_utilizations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_utilizations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Utilization", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_warehouse_allocations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::warehouse_allocations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Warehouse Allocation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_weekly_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::weekly_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Weekly Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_yard_managements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::yard_managements().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Yard Management", item.id().into_u64());
    }
    Ok(())
}


