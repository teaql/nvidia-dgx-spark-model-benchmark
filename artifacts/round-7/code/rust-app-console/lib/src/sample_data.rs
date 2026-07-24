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

    load_constant_application_statuses(ctx, &mut state).await?;
    load_constant_contract_types(ctx, &mut state).await?;
    load_constant_employee_statuses(ctx, &mut state).await?;
    load_constant_leave_types(ctx, &mut state).await?;
    load_constant_review_grades(ctx, &mut state).await?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_merchants(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_departments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payroll_runs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_positions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_employees(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_recruitment_posts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_attendance_logs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_benefit_plans(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_bonus_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_contracts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_expense_claims(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_job_applications(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_leave_requests(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_offboarding_checklists(ctx, &mut state)).await.map_err(|e| {
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
        Box::pin(generate_resignations(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_salary_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_shift_schedules(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_tax_forms(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_time_off_balances(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_training_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_warning_letters(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_interviews(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_offer_letters(ctx, &mut state)).await.map_err(|e| {
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

async fn load_constant_application_statuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::application_statuses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Application Status", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_contract_types<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::contract_types().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Contract Type", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_employee_statuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::employee_statuses().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Employee Status", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_leave_types<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::leave_types().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Leave Type", item.id().into_u64());
    }
    Ok(())
}

async fn load_constant_review_grades<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::review_grades().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("Review Grade", item.id().into_u64());
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


async fn generate_departments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Department", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Department: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Department (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::departments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_dept_name(format!("{} {}", "string()", i + 1));

                entity.update_manager_name(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Department");

        if i % 20 == 0 {
            log::info!("Generating Department: {}/{}", i, fanout);
        }

        state.add_reference("Department", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Department.");
    Ok(())
}


async fn generate_payroll_runs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Payroll Run", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Payroll Run: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payroll Run (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payroll_runs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_run_period(format!("{} {}", "string()", i + 1));

                entity.update_total_amount(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Payroll Run");

        if i % 20 == 0 {
            log::info!("Generating Payroll Run: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Payroll Run.");
    Ok(())
}


async fn generate_positions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Department").is_empty() {
            state.record_skipped("Position", "Required dependency Department is missing in reference pool".to_string());
            log::info!("Skipped generating Position: Required dependency Department is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Position", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Position: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Position (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::positions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Department", i as usize, &used_refs) {
                    entity.update_department_id(ref_id);
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
                entity.update_title(format!("{} {}", "string()", i + 1));

                entity.update_level(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Position");

        if i % 20 == 0 {
            log::info!("Generating Position: {}/{}", i, fanout);
        }

        state.add_reference("Position", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Position.");
    Ok(())
}


async fn generate_employees<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee Status").is_empty() {
            state.record_skipped("Employee", "Required dependency Employee Status is missing in reference pool".to_string());
            log::info!("Skipped generating Employee: Required dependency Employee Status is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Position").is_empty() {
            state.record_skipped("Employee", "Required dependency Position is missing in reference pool".to_string());
            log::info!("Skipped generating Employee: Required dependency Position is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Employee", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Employee: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Employee Status", i as usize, &used_refs) {
                    entity.update_status_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Position", i as usize, &used_refs) {
                    entity.update_position_id(ref_id);
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
                entity.update_first_name(format!("{} {}", "string()", i + 1));

                entity.update_last_name(format!("{} {}", "string()", i + 1));

                entity.update_email(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Employee");

        if i % 20 == 0 {
            log::info!("Generating Employee: {}/{}", i, fanout);
        }

        state.add_reference("Employee", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Employee.");
    Ok(())
}


async fn generate_recruitment_posts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Position").is_empty() {
            state.record_skipped("Recruitment Post", "Required dependency Position is missing in reference pool".to_string());
            log::info!("Skipped generating Recruitment Post: Required dependency Position is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Recruitment Post", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Recruitment Post: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Recruitment Post (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::recruitment_posts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Position", i as usize, &used_refs) {
                    entity.update_position_id(ref_id);
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
                entity.update_job_description(format!("{} {}", "string()", i + 1));

                entity.update_posting_date(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Recruitment Post");

        if i % 20 == 0 {
            log::info!("Generating Recruitment Post: {}/{}", i, fanout);
        }

        state.add_reference("Recruitment Post", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Recruitment Post.");
    Ok(())
}


async fn generate_attendance_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Attendance Log", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Attendance Log: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Attendance Log", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Attendance Log: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Attendance Log (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::attendance_logs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_date_logged(format!("{} {}", "string()", i + 1));

                entity.update_check_in(format!("{} {}", "string()", i + 1));

                entity.update_check_out(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Attendance Log");

        if i % 20 == 0 {
            log::info!("Generating Attendance Log: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Attendance Log.");
    Ok(())
}


async fn generate_benefit_plans<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Benefit Plan", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Benefit Plan: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Benefit Plan", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Benefit Plan: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Benefit Plan (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::benefit_plans().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_plan_name(format!("{} {}", "string()", i + 1));

                entity.update_provider(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Benefit Plan");

        if i % 20 == 0 {
            log::info!("Generating Benefit Plan: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Benefit Plan.");
    Ok(())
}


async fn generate_bonus_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Bonus Record", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Bonus Record: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Bonus Record", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Bonus Record: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Bonus Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::bonus_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_amount(format!("{} {}", "string()", i + 1));

                entity.update_bonus_reason(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Bonus Record");

        if i % 20 == 0 {
            log::info!("Generating Bonus Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Bonus Record.");
    Ok(())
}


async fn generate_contracts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Contract Type").is_empty() {
            state.record_skipped("Contract", "Required dependency Contract Type is missing in reference pool".to_string());
            log::info!("Skipped generating Contract: Required dependency Contract Type is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Employee").is_empty() {
            state.record_skipped("Contract", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Contract: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Contract", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Contract: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Contract (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::contracts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Contract Type", i as usize, &used_refs) {
                    entity.update_contract_type_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_start_date(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Contract");

        if i % 20 == 0 {
            log::info!("Generating Contract: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Contract.");
    Ok(())
}


async fn generate_expense_claims<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Expense Claim", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Expense Claim: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Expense Claim", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Expense Claim: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Expense Claim (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::expense_claims().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_amount(format!("{} {}", "string()", i + 1));

                entity.update_description(format!("{} {}", "string()", i + 1));

                entity.update_claim_date(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Expense Claim");

        if i % 20 == 0 {
            log::info!("Generating Expense Claim: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Expense Claim.");
    Ok(())
}


async fn generate_job_applications<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Application Status").is_empty() {
            state.record_skipped("Job Application", "Required dependency Application Status is missing in reference pool".to_string());
            log::info!("Skipped generating Job Application: Required dependency Application Status is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Recruitment Post").is_empty() {
            state.record_skipped("Job Application", "Required dependency Recruitment Post is missing in reference pool".to_string());
            log::info!("Skipped generating Job Application: Required dependency Recruitment Post is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Job Application", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Job Application: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Job Application (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::job_applications().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Application Status", i as usize, &used_refs) {
                    entity.update_status_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Recruitment Post", i as usize, &used_refs) {
                    entity.update_recruitment_post_id(ref_id);
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
                entity.update_candidate_name(format!("{} {}", "string()", i + 1));

                entity.update_resume_url(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Job Application");

        if i % 20 == 0 {
            log::info!("Generating Job Application: {}/{}", i, fanout);
        }

        state.add_reference("Job Application", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Job Application.");
    Ok(())
}


async fn generate_leave_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Leave Type").is_empty() {
            state.record_skipped("Leave Request", "Required dependency Leave Type is missing in reference pool".to_string());
            log::info!("Skipped generating Leave Request: Required dependency Leave Type is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Employee").is_empty() {
            state.record_skipped("Leave Request", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Leave Request: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Leave Request", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Leave Request: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Leave Type", i as usize, &used_refs) {
                    entity.update_leave_type_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_start_date(format!("{} {}", "string()", i + 1));

                entity.update_end_date(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Leave Request");

        if i % 20 == 0 {
            log::info!("Generating Leave Request: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Leave Request.");
    Ok(())
}


async fn generate_offboarding_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Offboarding Checklist", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Offboarding Checklist: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Offboarding Checklist", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Offboarding Checklist: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Offboarding Checklist (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::offboarding_checklists().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_is_completed(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Offboarding Checklist");

        if i % 20 == 0 {
            log::info!("Generating Offboarding Checklist: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Offboarding Checklist.");
    Ok(())
}


async fn generate_onboarding_checklists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Onboarding Checklist", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Onboarding Checklist: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Onboarding Checklist", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Onboarding Checklist: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_is_completed(format!("{} {}", "string()", i + 1));

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
        if state.ids("Review Grade").is_empty() {
            state.record_skipped("Performance Review", "Required dependency Review Grade is missing in reference pool".to_string());
            log::info!("Skipped generating Performance Review: Required dependency Review Grade is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Employee").is_empty() {
            state.record_skipped("Performance Review", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Performance Review: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Performance Review", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Performance Review: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Review Grade", i as usize, &used_refs) {
                    entity.update_grade_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_review_period(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Performance Review");

        if i % 20 == 0 {
            log::info!("Generating Performance Review: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Performance Review.");
    Ok(())
}


async fn generate_resignations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Resignation", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Resignation: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Resignation", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Resignation: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Resignation (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::resignations().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_last_working_day(format!("{} {}", "string()", i + 1));

                entity.update_reason(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Resignation");

        if i % 20 == 0 {
            log::info!("Generating Resignation: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Resignation.");
    Ok(())
}


async fn generate_salary_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Salary Record", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Salary Record: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Salary Record", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Salary Record: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Salary Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::salary_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_base_salary(format!("{} {}", "string()", i + 1));

                entity.update_currency(format!("{} {}", "string()", i + 1));

                entity.update_effective_date(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Salary Record");

        if i % 20 == 0 {
            log::info!("Generating Salary Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Salary Record.");
    Ok(())
}


async fn generate_shift_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Shift Schedule", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Shift Schedule: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Shift Schedule", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Shift Schedule: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Shift Schedule (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::shift_schedules().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_shift_date(format!("{} {}", "string()", i + 1));

                entity.update_start_time(format!("{} {}", "string()", i + 1));

                entity.update_end_time(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Shift Schedule");

        if i % 20 == 0 {
            log::info!("Generating Shift Schedule: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Shift Schedule.");
    Ok(())
}


async fn generate_tax_forms<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Tax Form", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Tax Form: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Tax Form", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Tax Form: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Tax Form (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::tax_forms().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_form_type(format!("{} {}", "string()", i + 1));

                entity.update_tax_year(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Tax Form");

        if i % 20 == 0 {
            log::info!("Generating Tax Form: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Tax Form.");
    Ok(())
}


async fn generate_time_off_balances<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Time Off Balance", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Time Off Balance: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Time Off Balance", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Time Off Balance: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Time Off Balance (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::time_off_balances().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_remaining_days(format!("{} {}", "string()", i + 1));

                entity.update_accrued_days(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Time Off Balance");

        if i % 20 == 0 {
            log::info!("Generating Time Off Balance: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Time Off Balance.");
    Ok(())
}


async fn generate_training_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Training Record", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Training Record: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Training Record", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Training Record: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_course_name(format!("{} {}", "string()", i + 1));

                entity.update_completion_date(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Training Record");

        if i % 20 == 0 {
            log::info!("Generating Training Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Training Record.");
    Ok(())
}


async fn generate_warning_letters<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Warning Letter", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Warning Letter: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Warning Letter", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Warning Letter: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Warning Letter (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::warning_letters().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_id(ref_id);
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
                entity.update_issue_date(format!("{} {}", "string()", i + 1));

                entity.update_reason(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Warning Letter");

        if i % 20 == 0 {
            log::info!("Generating Warning Letter: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Warning Letter.");
    Ok(())
}


async fn generate_interviews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Job Application").is_empty() {
            state.record_skipped("Interview", "Required dependency Job Application is missing in reference pool".to_string());
            log::info!("Skipped generating Interview: Required dependency Job Application is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Interview", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Interview: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Interview (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::interviews().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Job Application", i as usize, &used_refs) {
                    entity.update_job_application_id(ref_id);
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
                entity.update_interview_date(format!("{} {}", "string()", i + 1));

                entity.update_feedback(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Interview");

        if i % 20 == 0 {
            log::info!("Generating Interview: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Interview.");
    Ok(())
}


async fn generate_offer_letters<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Job Application").is_empty() {
            state.record_skipped("Offer Letter", "Required dependency Job Application is missing in reference pool".to_string());
            log::info!("Skipped generating Offer Letter: Required dependency Job Application is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Merchant").is_empty() {
            state.record_skipped("Offer Letter", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Offer Letter: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Offer Letter (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::offer_letters().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Job Application", i as usize, &used_refs) {
                    entity.update_job_application_id(ref_id);
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
                entity.update_offer_amount(format!("{} {}", "string()", i + 1));

                entity.update_valid_until(format!("{} {}", "string()", i + 1));

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

        state.record_generated("Offer Letter");

        if i % 20 == 0 {
            log::info!("Generating Offer Letter: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Offer Letter.");
    Ok(())
}
