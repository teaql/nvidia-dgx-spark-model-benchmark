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
    load_root_cargoes(ctx, &mut state).await?; //depth: 0
    load_root_contacts(ctx, &mut state).await?; //depth: 0
    load_root_contracts(ctx, &mut state).await?; //depth: 0
    load_root_customers(ctx, &mut state).await?; //depth: 0
    load_root_documents(ctx, &mut state).await?; //depth: 0
    load_root_drivers(ctx, &mut state).await?; //depth: 0
    load_root_emails(ctx, &mut state).await?; //depth: 0
    load_root_employees(ctx, &mut state).await?; //depth: 0
    load_root_estimates(ctx, &mut state).await?; //depth: 0
    load_root_expenses(ctx, &mut state).await?; //depth: 0
    load_root_feedback(ctx, &mut state).await?; //depth: 0
    load_root_fuel_logs(ctx, &mut state).await?; //depth: 0
    load_root_insurances(ctx, &mut state).await?; //depth: 0
    load_root_inventory(ctx, &mut state).await?; //depth: 0
    load_root_invoices(ctx, &mut state).await?; //depth: 0
    load_root_licenses(ctx, &mut state).await?; //depth: 0
    load_root_loadings(ctx, &mut state).await?; //depth: 0
    load_root_maintenances(ctx, &mut state).await?; //depth: 0
    load_root_move_orders(ctx, &mut state).await?; //depth: 0
    load_root_payments(ctx, &mut state).await?; //depth: 0
    load_root_phones(ctx, &mut state).await?; //depth: 0
    load_root_quotes(ctx, &mut state).await?; //depth: 0
    load_root_ratings(ctx, &mut state).await?; //depth: 0
    load_root_routes(ctx, &mut state).await?; //depth: 0
    load_root_schedules(ctx, &mut state).await?; //depth: 0
    load_root_timesheets(ctx, &mut state).await?; //depth: 0
    load_root_trucks(ctx, &mut state).await?; //depth: 0
    load_root_unloadings(ctx, &mut state).await?; //depth: 0
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

async fn load_root_cargoes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cargoes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Cargo", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_contacts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::contacts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Contact", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_contracts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::contracts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Contract", item.id().into_u64());
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

async fn load_root_documents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::documents().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Document", item.id().into_u64());
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

async fn load_root_emails<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::emails().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Email", item.id().into_u64());
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

async fn load_root_estimates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::estimates().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Estimate", item.id().into_u64());
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

async fn load_root_insurances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::insurances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Insurance", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_inventory<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::inventory().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Inventory", item.id().into_u64());
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

async fn load_root_loadings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::loadings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Loading", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_maintenances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::maintenances().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Maintenance", item.id().into_u64());
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

async fn load_root_phones<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::phones().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Phone", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_quotes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::quotes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Quote", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_ratings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::ratings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Rating", item.id().into_u64());
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

async fn load_root_timesheets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::timesheets().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Timesheet", item.id().into_u64());
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

async fn load_root_unloadings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::unloadings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Unloading", item.id().into_u64());
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


