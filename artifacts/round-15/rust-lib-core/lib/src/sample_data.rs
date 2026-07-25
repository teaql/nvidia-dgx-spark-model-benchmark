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

    load_root_customers(ctx, &mut state).await?; //depth: 0


    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_employees(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_feedback(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_inventory_items(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_invoices(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_orders(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_routes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_schedules(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_trucks(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;


    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
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


async fn generate_employees<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Employee", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Employee: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Employee (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::employees().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Employee");

        if i % 20 == 0 {
            log::info!("Generating Employee: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Employee.");
    Ok(())
}


async fn generate_feedback<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Feedback", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Feedback: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Feedback (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::feedback().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Feedback");

        if i % 20 == 0 {
            log::info!("Generating Feedback: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Feedback.");
    Ok(())
}


async fn generate_inventory_items<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Inventory Item", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Inventory Item: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Inventory Item (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::inventory_items().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Inventory Item");

        if i % 20 == 0 {
            log::info!("Generating Inventory Item: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Inventory Item.");
    Ok(())
}


async fn generate_invoices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Invoice", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Invoice: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Invoice (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::invoices().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Invoice");

        if i % 20 == 0 {
            log::info!("Generating Invoice: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Invoice.");
    Ok(())
}


async fn generate_move_orders<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Move Order", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Move Order: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Move Order (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::move_orders().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Move Order");

        if i % 20 == 0 {
            log::info!("Generating Move Order: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Move Order.");
    Ok(())
}


async fn generate_payments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Payment", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Payment: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payment");

        if i % 20 == 0 {
            log::info!("Generating Payment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Payment.");
    Ok(())
}


async fn generate_routes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Route", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Route: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Route (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::routes().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Route");

        if i % 20 == 0 {
            log::info!("Generating Route: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Route.");
    Ok(())
}


async fn generate_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Schedule", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Schedule: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Schedule (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::schedules().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Schedule");

        if i % 20 == 0 {
            log::info!("Generating Schedule: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Schedule.");
    Ok(())
}


async fn generate_trucks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Truck", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Truck: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Truck (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::trucks().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }

entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Truck");

        if i % 20 == 0 {
            log::info!("Generating Truck: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Truck.");
    Ok(())
}
