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

    load_root_addresses(ctx, &mut state).await?; //depth: 0
    load_root_api_keys(ctx, &mut state).await?; //depth: 0
    load_root_audit_trails(ctx, &mut state).await?; //depth: 0
    load_root_barcodes(ctx, &mut state).await?; //depth: 0
    load_root_branches(ctx, &mut state).await?; //depth: 0
    load_root_claims(ctx, &mut state).await?; //depth: 0
    load_root_communication_logs(ctx, &mut state).await?; //depth: 0
    load_root_containers(ctx, &mut state).await?; //depth: 0
    load_root_customers(ctx, &mut state).await?; //depth: 0
    load_root_customs_documents(ctx, &mut state).await?; //depth: 0
    load_root_dashboards(ctx, &mut state).await?; //depth: 0
    load_root_drivers(ctx, &mut state).await?; //depth: 0
    load_root_employees(ctx, &mut state).await?; //depth: 0
    load_root_equipment(ctx, &mut state).await?; //depth: 0
    load_root_feedback(ctx, &mut state).await?; //depth: 0
    load_root_fuel_logs(ctx, &mut state).await?; //depth: 0
    load_root_insurance_policies(ctx, &mut state).await?; //depth: 0
    load_root_inventory_items(ctx, &mut state).await?; //depth: 0
    load_root_invoices(ctx, &mut state).await?; //depth: 0
    load_root_labels(ctx, &mut state).await?; //depth: 0
    load_root_licenses(ctx, &mut state).await?; //depth: 0
    load_root_loading_unloadings(ctx, &mut state).await?; //depth: 0
    load_root_move_orders(ctx, &mut state).await?; //depth: 0
    load_root_notifications(ctx, &mut state).await?; //depth: 0
    load_root_packing_materials(ctx, &mut state).await?; //depth: 0
    load_root_pallets(ctx, &mut state).await?; //depth: 0
    load_root_payments(ctx, &mut state).await?; //depth: 0
    load_root_permissions(ctx, &mut state).await?; //depth: 0
    load_root_permits(ctx, &mut state).await?; //depth: 0
    load_root_reports(ctx, &mut state).await?; //depth: 0
    load_root_routes(ctx, &mut state).await?; //depth: 0
    load_root_schedules(ctx, &mut state).await?; //depth: 0
    load_root_settingses(ctx, &mut state).await?; //depth: 0
    load_root_storage_facilities(ctx, &mut state).await?; //depth: 0
    load_root_tools(ctx, &mut state).await?; //depth: 0
    load_root_tracking_numbers(ctx, &mut state).await?; //depth: 0
    load_root_trucks(ctx, &mut state).await?; //depth: 0
    load_root_user_roles(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_maintenances(ctx, &mut state).await?; //depth: 0
    load_root_warehouses(ctx, &mut state).await?; //depth: 0



    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
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

async fn load_root_api_keys<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::api_keys().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("API Key", item.id().into_u64());
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

async fn load_root_barcodes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::barcodes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Barcode", item.id().into_u64());
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

async fn load_root_claims<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::claims().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Claim", item.id().into_u64());
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

async fn load_root_containers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::containers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Container", item.id().into_u64());
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

async fn load_root_customs_documents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::customs_documents().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Customs Document", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_dashboards<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::dashboards().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Dashboard", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_drivers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::drivers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Driver", item.id().into_u64());
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

async fn load_root_feedback<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::feedback().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Feedback", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_fuel_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::fuel_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Fuel Log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_insurance_policies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::insurance_policies().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Insurance Policy", item.id().into_u64());
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

async fn load_root_labels<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::labels().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Label", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_licenses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::licenses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("License", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_loading_unloadings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::loading_unloadings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Loading Unloading", item.id().into_u64());
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

async fn load_root_notifications<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::notifications().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Notification", item.id().into_u64());
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

async fn load_root_pallets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::pallets().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Pallet", item.id().into_u64());
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

async fn load_root_permissions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::permissions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Permission", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_permits<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::permits().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Permit", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::reports().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Report", item.id().into_u64());
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

async fn load_root_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::schedules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Schedule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_settingses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::settingses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Settings", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_storage_facilities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::storage_facilities().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Storage Facility", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_tools<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::tools().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Tool", item.id().into_u64());
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

async fn load_root_trucks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::trucks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Truck", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_user_roles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::user_roles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("User Role", item.id().into_u64());
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

async fn load_root_warehouses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::warehouses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Warehouse", item.id().into_u64());
    }
    Ok(())
}


