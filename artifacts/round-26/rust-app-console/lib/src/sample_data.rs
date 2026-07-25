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

    load_root_accounts(ctx, &mut state).await?; //depth: 0
    load_root_ad_spends(ctx, &mut state).await?; //depth: 0
    load_root_addresses(ctx, &mut state).await?; //depth: 0
    load_root_asset_assignments(ctx, &mut state).await?; //depth: 0
    load_root_asset_inspections(ctx, &mut state).await?; //depth: 0
    load_root_audit_adjustments(ctx, &mut state).await?; //depth: 0
    load_root_background_checks(ctx, &mut state).await?; //depth: 0
    load_root_bank_transactions(ctx, &mut state).await?; //depth: 0
    load_root_billing_profiles(ctx, &mut state).await?; //depth: 0
    load_root_bonuses(ctx, &mut state).await?; //depth: 0
    load_root_box_rentals(ctx, &mut state).await?; //depth: 0
    load_root_branches(ctx, &mut state).await?; //depth: 0
    load_root_campaigns(ctx, &mut state).await?; //depth: 0
    load_root_chargeback_records(ctx, &mut state).await?; //depth: 0
    load_root_cleaning_services(ctx, &mut state).await?; //depth: 0
    load_root_communication_logs(ctx, &mut state).await?; //depth: 0
    load_root_competitor_analyses(ctx, &mut state).await?; //depth: 0
    load_root_complaint_tickets(ctx, &mut state).await?; //depth: 0
    load_root_consumables(ctx, &mut state).await?; //depth: 0
    load_root_conversion_events(ctx, &mut state).await?; //depth: 0
    load_root_conversion_metrics(ctx, &mut state).await?; //depth: 0
    load_root_corporate_customer_profiles(ctx, &mut state).await?; //depth: 0
    load_root_credit_notes(ctx, &mut state).await?; //depth: 0
    load_root_crews(ctx, &mut state).await?; //depth: 0
    load_root_customers(ctx, &mut state).await?; //depth: 0
    load_root_customer_consents(ctx, &mut state).await?; //depth: 0
    load_root_customer_contacts(ctx, &mut state).await?; //depth: 0
    load_root_customer_histories(ctx, &mut state).await?; //depth: 0
    load_root_customer_notes(ctx, &mut state).await?; //depth: 0
    load_root_customer_preferences(ctx, &mut state).await?; //depth: 0
    load_root_customer_signatures(ctx, &mut state).await?; //depth: 0
    load_root_damage_reports(ctx, &mut state).await?; //depth: 0
    load_root_dashcam_footages(ctx, &mut state).await?; //depth: 0
    load_root_debit_notes(ctx, &mut state).await?; //depth: 0
    load_root_departments(ctx, &mut state).await?; //depth: 0
    load_root_depreciation_schedules(ctx, &mut state).await?; //depth: 0
    load_root_detour_logs(ctx, &mut state).await?; //depth: 0
    load_root_direct_deposit_info(ctx, &mut state).await?; //depth: 0
    load_root_discount_codes(ctx, &mut state).await?; //depth: 0
    load_root_dispatch_assignments(ctx, &mut state).await?; //depth: 0
    load_root_do_not_contact_lists(ctx, &mut state).await?; //depth: 0
    load_root_email_blasts(ctx, &mut state).await?; //depth: 0
    load_root_emergency_contacts(ctx, &mut state).await?; //depth: 0
    load_root_employees(ctx, &mut state).await?; //depth: 0
    load_root_employee_certifications(ctx, &mut state).await?; //depth: 0
    load_root_equipment(ctx, &mut state).await?; //depth: 0
    load_root_expenses(ctx, &mut state).await?; //depth: 0
    load_root_expense_reimbursements(ctx, &mut state).await?; //depth: 0
    load_root_financial_summaries(ctx, &mut state).await?; //depth: 0
    load_root_fiscal_years(ctx, &mut state).await?; //depth: 0
    load_root_franchises(ctx, &mut state).await?; //depth: 0
    load_root_fuel_records(ctx, &mut state).await?; //depth: 0
    load_root_fuel_stops(ctx, &mut state).await?; //depth: 0
    load_root_fulfillment_events(ctx, &mut state).await?; //depth: 0
    load_root_gps_trackers(ctx, &mut state).await?; //depth: 0
    load_root_hoisting_services(ctx, &mut state).await?; //depth: 0
    load_root_insurance_addons(ctx, &mut state).await?; //depth: 0
    load_root_insurance_cards(ctx, &mut state).await?; //depth: 0
    load_root_inventory_items(ctx, &mut state).await?; //depth: 0
    load_root_invoices(ctx, &mut state).await?; //depth: 0
    load_root_invoice_lines(ctx, &mut state).await?; //depth: 0
    load_root_job_assignments(ctx, &mut state).await?; //depth: 0
    load_root_journal_entries(ctx, &mut state).await?; //depth: 0
    load_root_leads(ctx, &mut state).await?; //depth: 0
    load_root_lead_activities(ctx, &mut state).await?; //depth: 0
    load_root_leave_requests(ctx, &mut state).await?; //depth: 0
    load_root_long_carry_fees(ctx, &mut state).await?; //depth: 0
    load_root_loyalty_tiers(ctx, &mut state).await?; //depth: 0
    load_root_maintenance_events(ctx, &mut state).await?; //depth: 0
    load_root_maintenance_schedules(ctx, &mut state).await?; //depth: 0
    load_root_merchants(ctx, &mut state).await?; //depth: 0
    load_root_merchant_fees(ctx, &mut state).await?; //depth: 0
    load_root_move_orders(ctx, &mut state).await?; //depth: 0
    load_root_move_quotes(ctx, &mut state).await?; //depth: 0
    load_root_moving_services(ctx, &mut state).await?; //depth: 0
    load_root_objection_handling_guides(ctx, &mut state).await?; //depth: 0
    load_root_oil_change_logs(ctx, &mut state).await?; //depth: 0
    load_root_operations_manager_overrides(ctx, &mut state).await?; //depth: 0
    load_root_overtime_approvals(ctx, &mut state).await?; //depth: 0
    load_root_packing_lists(ctx, &mut state).await?; //depth: 0
    load_root_packing_materials(ctx, &mut state).await?; //depth: 0
    load_root_parking_permits(ctx, &mut state).await?; //depth: 0
    load_root_payments(ctx, &mut state).await?; //depth: 0
    load_root_payroll_calculations(ctx, &mut state).await?; //depth: 0
    load_root_payroll_periods(ctx, &mut state).await?; //depth: 0
    load_root_payslips(ctx, &mut state).await?; //depth: 0
    load_root_performance_reviews(ctx, &mut state).await?; //depth: 0
    load_root_pet_relocation_services(ctx, &mut state).await?; //depth: 0
    load_root_piano_handlings(ctx, &mut state).await?; //depth: 0
    load_root_platforms(ctx, &mut state).await?; //depth: 0
    load_root_platform_configs(ctx, &mut state).await?; //depth: 0
    load_root_post_move_surveys(ctx, &mut state).await?; //depth: 0
    load_root_price_lists(ctx, &mut state).await?; //depth: 0
    load_root_private_customer_profiles(ctx, &mut state).await?; //depth: 0
    load_root_products(ctx, &mut state).await?; //depth: 0
    load_root_proof_of_deliveries(ctx, &mut state).await?; //depth: 0
    load_root_referral_codes(ctx, &mut state).await?; //depth: 0
    load_root_refunds(ctx, &mut state).await?; //depth: 0
    load_root_registration_renewals(ctx, &mut state).await?; //depth: 0
    load_root_resolution_offers(ctx, &mut state).await?; //depth: 0
    load_root_routes(ctx, &mut state).await?; //depth: 0
    load_root_route_stops(ctx, &mut state).await?; //depth: 0
    load_root_sales_opportunities(ctx, &mut state).await?; //depth: 0
    load_root_sales_scripts(ctx, &mut state).await?; //depth: 0
    load_root_sales_territories(ctx, &mut state).await?; //depth: 0
    load_root_scrap_records(ctx, &mut state).await?; //depth: 0
    load_root_services(ctx, &mut state).await?; //depth: 0
    load_root_service_bundles(ctx, &mut state).await?; //depth: 0
    load_root_service_configurations(ctx, &mut state).await?; //depth: 0
    load_root_service_prices(ctx, &mut state).await?; //depth: 0
    load_root_sms_campaigns(ctx, &mut state).await?; //depth: 0
    load_root_social_media_posts(ctx, &mut state).await?; //depth: 0
    load_root_stair_fees(ctx, &mut state).await?; //depth: 0
    load_root_storage_units(ctx, &mut state).await?; //depth: 0
    load_root_suppliers(ctx, &mut state).await?; //depth: 0
    load_root_tax_documents(ctx, &mut state).await?; //depth: 0
    load_root_tax_withholdings(ctx, &mut state).await?; //depth: 0
    load_root_tenant_registries(ctx, &mut state).await?; //depth: 0
    load_root_termination_records(ctx, &mut state).await?; //depth: 0
    load_root_time_slots(ctx, &mut state).await?; //depth: 0
    load_root_tire_replacements(ctx, &mut state).await?; //depth: 0
    load_root_toll_receipts(ctx, &mut state).await?; //depth: 0
    load_root_traffic_violations(ctx, &mut state).await?; //depth: 0
    load_root_uniform_assignments(ctx, &mut state).await?; //depth: 0
    load_root_union_dueses(ctx, &mut state).await?; //depth: 0
    load_root_vat_rates(ctx, &mut state).await?; //depth: 0
    load_root_vehicles(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_load_plans(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_transports(ctx, &mut state).await?; //depth: 0
    load_root_vip_statuses(ctx, &mut state).await?; //depth: 0
    load_root_walkthrough_checklists(ctx, &mut state).await?; //depth: 0
    load_root_warning_letters(ctx, &mut state).await?; //depth: 0
    load_root_weather_delays(ctx, &mut state).await?; //depth: 0
    load_root_weigh_station_tickets(ctx, &mut state).await?; //depth: 0
    load_root_work_shifts(ctx, &mut state).await?; //depth: 0
    load_root_worked_hourses(ctx, &mut state).await?; //depth: 0



    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
}

async fn load_root_accounts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::accounts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Account", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_ad_spends<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::ad_spends().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Ad Spend", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_addresses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::addresses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Address", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_asset_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::asset_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Asset Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_asset_inspections<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::asset_inspections().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Asset Inspection", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_audit_adjustments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::audit_adjustments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Audit Adjustment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_background_checks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::background_checks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Background Check", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_bank_transactions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::bank_transactions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Bank Transaction", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_billing_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::billing_profiles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Billing Profile", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_bonuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::bonuses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Bonus", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_box_rentals<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::box_rentals().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Box Rental", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_branches<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::branches().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Branch", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_campaigns<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::campaigns().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Campaign", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_chargeback_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::chargeback_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Chargeback Record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_cleaning_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cleaning_services().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Cleaning Service", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_communication_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::communication_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Communication Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_competitor_analyses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::competitor_analyses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Competitor Analysis", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_complaint_tickets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::complaint_tickets().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Complaint Ticket", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_consumables<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::consumables().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Consumable", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_conversion_events<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::conversion_events().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Conversion Event", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_conversion_metrics<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::conversion_metrics().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Conversion Metric", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_corporate_customer_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::corporate_customer_profiles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Corporate Customer Profile", item.id().into_u64());
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

async fn load_root_crews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::crews().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Crew", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_consents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_consents().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Consent", item.id().into_u64());
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

async fn load_root_customer_histories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_histories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer History", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_customer_notes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_notes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Note", item.id().into_u64());
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

async fn load_root_customer_signatures<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customer_signatures().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customer Signature", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_damage_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::damage_reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Damage Report", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_dashcam_footages<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::dashcam_footages().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Dashcam Footage", item.id().into_u64());
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

async fn load_root_departments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::departments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Department", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_depreciation_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::depreciation_schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Depreciation Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_detour_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::detour_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Detour Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_direct_deposit_info<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::direct_deposit_info().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Direct Deposit Info", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_discount_codes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::discount_codes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Discount Code", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_dispatch_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::dispatch_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Dispatch Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_do_not_contact_lists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::do_not_contact_lists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Do Not Contact List", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_email_blasts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::email_blasts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Email Blast", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_emergency_contacts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::emergency_contacts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Emergency Contact", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_employees<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::employees().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Employee", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_employee_certifications<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::employee_certifications().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Employee Certification", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_equipment<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::equipment().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Equipment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_expenses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::expenses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Expense", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_expense_reimbursements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::expense_reimbursements().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Expense Reimbursement", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_financial_summaries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::financial_summaries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Financial Summary", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fiscal_years<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fiscal_years().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fiscal Year", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_franchises<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::franchises().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Franchise", item.id().into_u64());
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

async fn load_root_fuel_stops<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fuel_stops().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fuel Stop", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fulfillment_events<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fulfillment_events().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fulfillment Event", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_gps_trackers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::gps_trackers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("GPS Tracker", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_hoisting_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::hoisting_services().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Hoisting Service", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_insurance_addons<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::insurance_addons().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Insurance Addon", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_insurance_cards<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::insurance_cards().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Insurance Card", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_inventory_items<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::inventory_items().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Inventory Item", item.id().into_u64());
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

async fn load_root_invoice_lines<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::invoice_lines().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Invoice Line", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_job_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::job_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Job Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_journal_entries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::journal_entries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Journal Entry", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_leads<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::leads().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Lead", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_lead_activities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::lead_activities().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Lead Activity", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_leave_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::leave_requests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Leave Request", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_long_carry_fees<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::long_carry_fees().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Long Carry Fee", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_loyalty_tiers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::loyalty_tiers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Loyalty Tier", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_maintenance_events<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::maintenance_events().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Maintenance Event", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_maintenance_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::maintenance_schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Maintenance Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_merchants<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::merchants().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Merchant", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_merchant_fees<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::merchant_fees().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Merchant Fee", item.id().into_u64());
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

async fn load_root_move_quotes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::move_quotes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Move Quote", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_moving_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::moving_services().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Moving Service", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_objection_handling_guides<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::objection_handling_guides().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Objection Handling Guide", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_oil_change_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::oil_change_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Oil Change Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_operations_manager_overrides<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::operations_manager_overrides().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Operations Manager Override", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_overtime_approvals<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::overtime_approvals().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Overtime Approval", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_packing_lists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::packing_lists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Packing List", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_packing_materials<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::packing_materials().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Packing Material", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_parking_permits<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::parking_permits().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Parking Permit", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payroll_calculations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payroll_calculations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payroll Calculation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payroll_periods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payroll_periods().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payroll Period", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_payslips<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::payslips().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Payslip", item.id().into_u64());
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

async fn load_root_pet_relocation_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::pet_relocation_services().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Pet Relocation Service", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_piano_handlings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::piano_handlings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Piano Handling", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_platforms<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::platforms().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Platform", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_platform_configs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::platform_configs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Platform Config", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_post_move_surveys<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::post_move_surveys().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Post Move Survey", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_price_lists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::price_lists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Price List", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_private_customer_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::private_customer_profiles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Private Customer Profile", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_products<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::products().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Product", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_proof_of_deliveries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::proof_of_deliveries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Proof Of Delivery", item.id().into_u64());
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

async fn load_root_refunds<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::refunds().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Refund", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_registration_renewals<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::registration_renewals().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Registration Renewal", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_resolution_offers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::resolution_offers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Resolution Offer", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_routes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::routes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Route", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_route_stops<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::route_stops().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Route Stop", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_sales_opportunities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::sales_opportunities().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Sales Opportunity", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_sales_scripts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::sales_scripts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Sales Script", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_sales_territories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::sales_territories().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Sales Territory", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_scrap_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::scrap_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Scrap Record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::services().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_service_bundles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_bundles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service Bundle", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_service_configurations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_configurations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service Configuration", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_service_prices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::service_prices().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Service Price", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_sms_campaigns<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::sms_campaigns().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("SMS Campaign", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_social_media_posts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::social_media_posts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Social Media Post", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_stair_fees<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::stair_fees().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Stair Fee", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_storage_units<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::storage_units().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Storage Unit", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_suppliers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::suppliers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Supplier", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tax_documents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tax_documents().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tax Document", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tax_withholdings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tax_withholdings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tax Withholding", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tenant_registries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tenant_registries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tenant Registry", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_termination_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::termination_records().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Termination Record", item.id().into_u64());
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

async fn load_root_tire_replacements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tire_replacements().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tire Replacement", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_toll_receipts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::toll_receipts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Toll Receipt", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_traffic_violations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::traffic_violations().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Traffic Violation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_uniform_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::uniform_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Uniform Assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_union_dueses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::union_dueses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Union Dues", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vat_rates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vat_rates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("VAT Rate", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_load_plans<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_load_plans().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Load Plan", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vehicle_transports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vehicle_transports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Vehicle Transport", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_vip_statuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::vip_statuses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("VIP Status", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_walkthrough_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::walkthrough_checklists().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Walkthrough Checklist", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_warning_letters<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::warning_letters().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Warning Letter", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_weather_delays<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::weather_delays().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Weather Delay", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_weigh_station_tickets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::weigh_station_tickets().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Weigh Station Ticket", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_work_shifts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::work_shifts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Work Shift", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_worked_hourses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::worked_hourses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Worked Hours", item.id().into_u64());
    }
    Ok(())
}


