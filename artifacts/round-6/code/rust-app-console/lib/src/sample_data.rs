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

    load_root_platforms(ctx, &mut state).await?; //depth: 0

    load_constant_crew_roles(ctx, &mut state).await?;
    load_constant_exception_severities(ctx, &mut state).await?;
    load_constant_inventory_condition_types(ctx, &mut state).await?;
    load_constant_order_statuses(ctx, &mut state).await?;
    load_constant_route_status_types(ctx, &mut state).await?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_merchants(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_crews(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_logistics_providers(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_quotes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_vehicles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_crew_member_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_fuel_logs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_incident_reports(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_maintenance_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_orders(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_shift_logs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_toll_receipts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_vehicle_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customer_feedback(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_damage_reports(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_dispatch_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_inventory(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_operational_exceptions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_packaging_items(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_proof_of_deliveries(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_route_stops(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_third_party_dispatches(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_delivery_instructions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_pickup_instructions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;


    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
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
        state.add_reference("System Platform", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_crew_roles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::crew_roles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Crew Role", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_exception_severities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::exception_severities().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Exception Severity", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_inventory_condition_types<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::inventory_condition_types().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Inventory Condition Type", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_order_statuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::order_statuses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Order Status", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_route_status_types<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::route_status_types().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Route Status Type", item.id().into_u64());
    }
    Ok(())
}

async fn generate_merchants<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("System Platform").is_empty() {
            state.record_skipped("Merchant", "Required dependency System Platform is missing in reference pool".to_string());
            log::info!("Skipped generating Merchant: Required dependency System Platform is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Merchant (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::merchants().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("System Platform", i as usize, &used_refs) {
                    entity.update_platform_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_name(format!("{} {}", "Nordic Moving Services", i + 1));

                entity.update_tax_number(format!("{} {}", "FI12345678", i + 1));

                entity.update_address(format!("{} {}", "Mannerheimintie 10", i + 1));

                entity.update_external_id(format!("{} {}", "MERCHANT_NORDIC_001", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Merchant");

        if i % 20 == 0 {
            log::info!("Generating Merchant: {}/{}", i, fanout);
        }

        state.add_reference("Merchant", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Merchant.");
    Ok(())
}


async fn generate_crews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Crew", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Crew: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Crew (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::crews().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_crew_name(format!("{} {}", "string()", i + 1));

                entity.update_max_capacity(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Crew");

        if i % 20 == 0 {
            log::info!("Generating Crew: {}/{}", i, fanout);
        }

        state.add_reference("Crew", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Crew.");
    Ok(())
}


async fn generate_logistics_providers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Logistics Provider", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Logistics Provider: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Logistics Provider (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::logistics_providers().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_provider_name(format!("{} {}", "string()", i + 1));

                entity.update_contact_number(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Logistics Provider");

        if i % 20 == 0 {
            log::info!("Generating Logistics Provider: {}/{}", i, fanout);
        }

        state.add_reference("Logistics Provider", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Logistics Provider.");
    Ok(())
}


async fn generate_move_quotes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Move Quote", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Move Quote: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Move Quote (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::move_quotes().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_quote_number(format!("{} {}", "string()", i + 1));

                entity.update_estimated_price(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Move Quote");

        if i % 20 == 0 {
            log::info!("Generating Move Quote: {}/{}", i, fanout);
        }

        state.add_reference("Move Quote", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Move Quote.");
    Ok(())
}


async fn generate_vehicles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Vehicle", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Vehicle: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Vehicle (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::vehicles().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_license_plate(format!("{} {}", "string()", i + 1));

                entity.update_model(format!("{} {}", "string()", i + 1));

                entity.update_capacity_kg(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Vehicle");

        if i % 20 == 0 {
            log::info!("Generating Vehicle: {}/{}", i, fanout);
        }

        state.add_reference("Vehicle", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Vehicle.");
    Ok(())
}


async fn generate_crew_member_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Crew Role").is_empty() {
            state.record_skipped("Crew Member Assignment", "Required dependency Crew Role is missing in reference pool".to_string());
            log::info!("Skipped generating Crew Member Assignment: Required dependency Crew Role is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Crew").is_empty() {
            state.record_skipped("Crew Member Assignment", "Required dependency Crew is missing in reference pool".to_string());
            log::info!("Skipped generating Crew Member Assignment: Required dependency Crew is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Crew Member Assignment", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Crew Member Assignment: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Crew Member Assignment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::crew_member_assignments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Crew Role", i as usize, &used_refs) {
                    entity.update_role_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Crew", i as usize, &used_refs) {
                    entity.update_crew_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_employee_name(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Crew Member Assignment");

        if i % 20 == 0 {
            log::info!("Generating Crew Member Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Crew Member Assignment.");
    Ok(())
}


async fn generate_fuel_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Fuel Log", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Fuel Log: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Fuel Log", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Fuel Log: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Fuel Log (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::fuel_logs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_gallons_filled(format!("{} {}", "string()", i + 1));

                entity.update_cost(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Fuel Log");

        if i % 20 == 0 {
            log::info!("Generating Fuel Log: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Fuel Log.");
    Ok(())
}


async fn generate_incident_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Incident Report", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Incident Report: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Incident Report", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Incident Report: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Incident Report (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::incident_reports().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_incident_date(format!("{} {}", "string()", i + 1));

                entity.update_police_report_number(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Incident Report");

        if i % 20 == 0 {
            log::info!("Generating Incident Report: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Incident Report.");
    Ok(())
}


async fn generate_maintenance_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Maintenance Record", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Maintenance Record: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Maintenance Record", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Maintenance Record: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Maintenance Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::maintenance_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_service_type(format!("{} {}", "string()", i + 1));

                entity.update_service_date(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Maintenance Record");

        if i % 20 == 0 {
            log::info!("Generating Maintenance Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Maintenance Record.");
    Ok(())
}


async fn generate_move_orders<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Order Status").is_empty() {
            state.record_skipped("Move Order", "Required dependency Order Status is missing in reference pool".to_string());
            log::info!("Skipped generating Move Order: Required dependency Order Status is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Move Quote").is_empty() {
            state.record_skipped("Move Order", "Required dependency Move Quote is missing in reference pool".to_string());
            log::info!("Skipped generating Move Order: Required dependency Move Quote is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Move Order", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Move Order: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Order Status", i as usize, &used_refs) {
                    entity.update_status_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Quote", i as usize, &used_refs) {
                    entity.update_quote_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_order_number(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Move Order");

        if i % 20 == 0 {
            log::info!("Generating Move Order: {}/{}", i, fanout);
        }

        state.add_reference("Move Order", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Move Order.");
    Ok(())
}


async fn generate_shift_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Crew").is_empty() {
            state.record_skipped("Shift Log", "Required dependency Crew is missing in reference pool".to_string());
            log::info!("Skipped generating Shift Log: Required dependency Crew is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Shift Log", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Shift Log: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Shift Log (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::shift_logs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Crew", i as usize, &used_refs) {
                    entity.update_crew_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_clock_in(format!("{} {}", "string()", i + 1));

                entity.update_clock_out(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Shift Log");

        if i % 20 == 0 {
            log::info!("Generating Shift Log: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Shift Log.");
    Ok(())
}


async fn generate_toll_receipts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Toll Receipt", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Toll Receipt: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Toll Receipt", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Toll Receipt: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Toll Receipt (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::toll_receipts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_toll_amount(format!("{} {}", "string()", i + 1));

                entity.update_toll_location(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Toll Receipt");

        if i % 20 == 0 {
            log::info!("Generating Toll Receipt: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Toll Receipt.");
    Ok(())
}


async fn generate_vehicle_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Crew").is_empty() {
            state.record_skipped("Vehicle Assignment", "Required dependency Crew is missing in reference pool".to_string());
            log::info!("Skipped generating Vehicle Assignment: Required dependency Crew is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Vehicle Assignment", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Vehicle Assignment: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Vehicle Assignment", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Vehicle Assignment: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Vehicle Assignment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::vehicle_assignments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Crew", i as usize, &used_refs) {
                    entity.update_crew_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_assignment_date(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Vehicle Assignment");

        if i % 20 == 0 {
            log::info!("Generating Vehicle Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Vehicle Assignment.");
    Ok(())
}


async fn generate_customer_feedback<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Customer Feedback", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Customer Feedback: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Customer Feedback", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Customer Feedback: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Customer Feedback (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::customer_feedback().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_rating(format!("{} {}", "string()", i + 1));

                entity.update_comments(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer Feedback");

        if i % 20 == 0 {
            log::info!("Generating Customer Feedback: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Customer Feedback.");
    Ok(())
}


async fn generate_damage_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Damage Report", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Damage Report: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Damage Report", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Damage Report: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Damage Report (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::damage_reports().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_report_details(format!("{} {}", "string()", i + 1));

                entity.update_reported_at(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Damage Report");

        if i % 20 == 0 {
            log::info!("Generating Damage Report: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Damage Report.");
    Ok(())
}


async fn generate_dispatch_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Crew").is_empty() {
            state.record_skipped("Dispatch Assignment", "Required dependency Crew is missing in reference pool".to_string());
            log::info!("Skipped generating Dispatch Assignment: Required dependency Crew is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Move Order").is_empty() {
            state.record_skipped("Dispatch Assignment", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Dispatch Assignment: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Dispatch Assignment", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Dispatch Assignment: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Dispatch Assignment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::dispatch_assignments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Crew", i as usize, &used_refs) {
                    entity.update_crew_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_dispatch_time(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Dispatch Assignment");

        if i % 20 == 0 {
            log::info!("Generating Dispatch Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Dispatch Assignment.");
    Ok(())
}


async fn generate_move_inventory<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Inventory Condition Type").is_empty() {
            state.record_skipped("Move Inventory", "Required dependency Inventory Condition Type is missing in reference pool".to_string());
            log::info!("Skipped generating Move Inventory: Required dependency Inventory Condition Type is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Move Order").is_empty() {
            state.record_skipped("Move Inventory", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Move Inventory: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Move Inventory", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Move Inventory: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Move Inventory (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::move_inventory().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Inventory Condition Type", i as usize, &used_refs) {
                    entity.update_condition_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_item_name(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Move Inventory");

        if i % 20 == 0 {
            log::info!("Generating Move Inventory: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Move Inventory.");
    Ok(())
}


async fn generate_operational_exceptions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Exception Severity").is_empty() {
            state.record_skipped("Operational Exception", "Required dependency Exception Severity is missing in reference pool".to_string());
            log::info!("Skipped generating Operational Exception: Required dependency Exception Severity is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Move Order").is_empty() {
            state.record_skipped("Operational Exception", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Operational Exception: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Operational Exception", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Operational Exception: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Operational Exception (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::operational_exceptions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Exception Severity", i as usize, &used_refs) {
                    entity.update_severity_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_description(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Operational Exception");

        if i % 20 == 0 {
            log::info!("Generating Operational Exception: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Operational Exception.");
    Ok(())
}


async fn generate_packaging_items<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Packaging Item", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Packaging Item: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Packaging Item", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Packaging Item: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Packaging Item (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::packaging_items().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_material_name(format!("{} {}", "string()", i + 1));

                entity.update_quantity_used(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Packaging Item");

        if i % 20 == 0 {
            log::info!("Generating Packaging Item: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Packaging Item.");
    Ok(())
}


async fn generate_proof_of_deliveries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Proof Of Delivery", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Proof Of Delivery: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Proof Of Delivery", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Proof Of Delivery: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Proof Of Delivery (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::proof_of_deliveries().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_signature_url(format!("{} {}", "string()", i + 1));

                entity.update_signed_at(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Proof Of Delivery");

        if i % 20 == 0 {
            log::info!("Generating Proof Of Delivery: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Proof Of Delivery.");
    Ok(())
}


async fn generate_route_stops<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Route Status Type").is_empty() {
            state.record_skipped("Route Stop", "Required dependency Route Status Type is missing in reference pool".to_string());
            log::info!("Skipped generating Route Stop: Required dependency Route Status Type is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Move Order").is_empty() {
            state.record_skipped("Route Stop", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Route Stop: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Route Stop", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Route Stop: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Route Stop (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::route_stops().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Route Status Type", i as usize, &used_refs) {
                    entity.update_status_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_stop_sequence(format!("{} {}", "string()", i + 1));

                entity.update_address(format!("{} {}", "string()", i + 1));

                entity.update_arrival_time(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Route Stop");

        if i % 20 == 0 {
            log::info!("Generating Route Stop: {}/{}", i, fanout);
        }

        state.add_reference("Route Stop", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Route Stop.");
    Ok(())
}


async fn generate_third_party_dispatches<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Logistics Provider").is_empty() {
            state.record_skipped("Third Party Dispatch", "Required dependency Logistics Provider is missing in reference pool".to_string());
            log::info!("Skipped generating Third Party Dispatch: Required dependency Logistics Provider is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Move Order").is_empty() {
            state.record_skipped("Third Party Dispatch", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Third Party Dispatch: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Third Party Dispatch", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Third Party Dispatch: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Third Party Dispatch (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::third_party_dispatches().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Logistics Provider", i as usize, &used_refs) {
                    entity.update_logistics_provider_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_tracking_id(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Third Party Dispatch");

        if i % 20 == 0 {
            log::info!("Generating Third Party Dispatch: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Third Party Dispatch.");
    Ok(())
}


async fn generate_delivery_instructions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Route Stop").is_empty() {
            state.record_skipped("Delivery Instruction", "Required dependency Route Stop is missing in reference pool".to_string());
            log::info!("Skipped generating Delivery Instruction: Required dependency Route Stop is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Delivery Instruction", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Delivery Instruction: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Delivery Instruction (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::delivery_instructions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Route Stop", i as usize, &used_refs) {
                    entity.update_route_stop_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_instruction_text(format!("{} {}", "string()", i + 1));

                entity.update_floor_number(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Delivery Instruction");

        if i % 20 == 0 {
            log::info!("Generating Delivery Instruction: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Delivery Instruction.");
    Ok(())
}


async fn generate_pickup_instructions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Route Stop").is_empty() {
            state.record_skipped("Pickup Instruction", "Required dependency Route Stop is missing in reference pool".to_string());
            log::info!("Skipped generating Pickup Instruction: Required dependency Route Stop is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Pickup Instruction", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Pickup Instruction: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Pickup Instruction (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::pickup_instructions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Route Stop", i as usize, &used_refs) {
                    entity.update_route_stop_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_instruction_text(format!("{} {}", "string()", i + 1));

                entity.update_gate_code(format!("{} {}", "string()", i + 1));

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_create_time(past.format("%Y-%m-%d").to_string());
                }

                {
                    let days = ((i as u64 + state.plan.seed) % (365 * 3)) as i64;
                    let past = chrono::Utc::now().naive_utc() - chrono::Duration::try_days(days).unwrap_or_default();
                    entity.update_update_time(past.format("%Y-%m-%d").to_string());
                }



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Pickup Instruction");

        if i % 20 == 0 {
            log::info!("Generating Pickup Instruction: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Pickup Instruction.");
    Ok(())
}
