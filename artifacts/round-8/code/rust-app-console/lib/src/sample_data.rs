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


    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_merchants(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_platform_settings(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_accounts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_addresses(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_api_clients(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_audit_logs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_automation_rules(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_branch_offices(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_budgets(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_campaigns(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_compliance_certificates(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_compliance_checks(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_consumables(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_contracts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_crews(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_currency_rates(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customers(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customer_segments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_data_retention_policies(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_departments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_department_hierarchies(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_employees(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_equipment(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_expenses(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_financial_periods(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_financial_summaries(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_incident_reports(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_insurance_policies(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_inventory_items(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_legal_entities(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_loading_zones(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_marketing_channels(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_statuses(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_notification_templates(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_organization_units(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_packing_materials(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payables(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payment_methods(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payroll_periods(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_permissions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_policy_documents(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_price_lists(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_products(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_regulatory_requirements(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_roles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_route_optimization_rules(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_services(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_service_areas(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_service_bundles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_service_categories(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_storage_locations(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_storage_units(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_suppliers(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_system_events(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_tenant_configurations(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_unloading_zones(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_vat_rates(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_vehicles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_account_statuses(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_add_on_services(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_api_endpoints(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_api_keys(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_asset_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_asset_conditions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_asset_inspections(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_attendance_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_attribution_models(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_audience_segments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_audit_trails(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_automation_actions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_automation_triggers(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_availability_calendars(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_availability_schedules(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_benefit_enrollments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_billing_profiles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_bonuses(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_box_rentals(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_campaign_budgets(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_cleaning_services(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_commission_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_communication_logs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_contact_methods(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_conversion_metrics(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_conversion_reports(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_corporate_customer_profiles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customer_consents(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customer_contacts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customer_feedback(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customer_histories(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_customer_preferences(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_depreciation_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_discount_codes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_disposal_services(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_documents(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_employee_certifications(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_equipment_serials(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_financial_hooks(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_fuel_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_insurance_claims(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_integration_mappings(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_inventory_stocks(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_invoices(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_job_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_leads(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_leave_requests(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_loyalty_tiers(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_maintenance_events(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_maintenance_schedules(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_orders(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_moving_services(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_operational_hooks(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_overtime_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_packing_kits(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payroll_calculations(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_performance_reviews(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_private_customer_profiles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_promotional_offers(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_referral_codes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_refunds(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_role_permissions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_service_configurations(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_service_level_agreements(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_service_prices(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_service_ratings(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_settlements(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_shift_swap_requests(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_skill_profiles(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_supplier_contracts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_training_modules(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_user_accounts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_vehicle_registrations(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_warranty_claims(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_webhooks(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_work_shifts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_access_tokens(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_activity_logs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_cargo_weight_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_damage_reports(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_data_exports(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_deductions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_delay_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_delivery_windows(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_dispatch_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_document_versions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_fulfillment_events(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_inventory_lists(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_invoice_lines(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_journal_entries(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_lead_activities(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_lead_scores(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_login_attempts(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_magic_links(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_maintenance_costs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_items(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_move_quotes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_notifications(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payroll_adjustments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_payslips(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_proof_of_deliveries(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_receivables(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_recovery_requests(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_routes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_sales_funnels(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_sales_opportunities(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_special_handling_instructions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_synchronization_runs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_tax_records(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_tax_withholdings(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_time_slots(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_transit_logs(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_two_factor_auths(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_user_role_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_user_sessions(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_vehicle_assignments(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_webhook_deliveries(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_worked_hourses(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_conversion_events(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_entity_changes(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_route_stops(ctx, &mut state)).await.map_err(|e| {
            teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(e))
        })
    }).await.map_err(|e| e.to_string())?;

    ctx.user_context().transaction_data(|| async {
        Box::pin(generate_change_sets(ctx, &mut state)).await.map_err(|e| {
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
        state.add_reference("Platform", item.id().into_u64());
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
        if state.ids("Platform").is_empty() {
            state.record_skipped("Merchant", "Required dependency Platform is missing in reference pool".to_string());
            log::info!("Skipped generating Merchant: Required dependency Platform is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Platform", i as usize, &used_refs) {
                    entity.update_platform_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_name(format!("{} {}", "string()", i + 1));

                entity.update_tax_id(format!("{} {}", "string()", i + 1));



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


async fn generate_platform_settings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Platform").is_empty() {
            state.record_skipped("Platform Setting", "Required dependency Platform is missing in reference pool".to_string());
            log::info!("Skipped generating Platform Setting: Required dependency Platform is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Platform Setting (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::platform_settings().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Platform", i as usize, &used_refs) {
                    entity.update_platform_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_key(format!("{} {}", "string()", i + 1));

                entity.update_value(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Platform Setting");

        if i % 20 == 0 {
            log::info!("Generating Platform Setting: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Platform Setting.");
    Ok(())
}


async fn generate_accounts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Account", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Account: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Account (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::accounts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_account_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Account");

        if i % 20 == 0 {
            log::info!("Generating Account: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Account.");
    Ok(())
}


async fn generate_addresses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Address", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Address: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Address (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::addresses().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_street(format!("{} {}", "string()", i + 1));

                entity.update_city(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Address");

        if i % 20 == 0 {
            log::info!("Generating Address: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Address.");
    Ok(())
}


async fn generate_api_clients<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("API Client", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating API Client: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for API Client (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::api_clients().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_client_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("API Client");

        if i % 20 == 0 {
            log::info!("Generating API Client: {}/{}", i, fanout);
        }

        state.add_reference("API Client", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for API Client.");
    Ok(())
}


async fn generate_audit_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Audit Log", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Audit Log: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Audit Log (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::audit_logs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_log_id(format!("{} {}", "string()", i + 1));

                entity.update_level(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Audit Log");

        if i % 20 == 0 {
            log::info!("Generating Audit Log: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Audit Log.");
    Ok(())
}


async fn generate_automation_rules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Automation Rule", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Automation Rule: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Automation Rule (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::automation_rules().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_rule_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Automation Rule");

        if i % 20 == 0 {
            log::info!("Generating Automation Rule: {}/{}", i, fanout);
        }

        state.add_reference("Automation Rule", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Automation Rule.");
    Ok(())
}


async fn generate_branch_offices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Branch Office", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Branch Office: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Branch Office (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::branch_offices().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_address(format!("{} {}", "string()", i + 1));

                entity.update_capacity(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Branch Office");

        if i % 20 == 0 {
            log::info!("Generating Branch Office: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Branch Office.");
    Ok(())
}


async fn generate_budgets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Budget", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Budget: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Budget (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::budgets().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_budget_id(format!("{} {}", "string()", i + 1));

                entity.update_limit(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Budget");

        if i % 20 == 0 {
            log::info!("Generating Budget: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Budget.");
    Ok(())
}


async fn generate_campaigns<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Campaign", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Campaign: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Campaign (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::campaigns().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_campaign_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Campaign");

        if i % 20 == 0 {
            log::info!("Generating Campaign: {}/{}", i, fanout);
        }

        state.add_reference("Campaign", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Campaign.");
    Ok(())
}


async fn generate_compliance_certificates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Compliance Certificate", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Compliance Certificate: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Compliance Certificate (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::compliance_certificates().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_cert_id(format!("{} {}", "string()", i + 1));

                entity.update_issued_date(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Compliance Certificate");

        if i % 20 == 0 {
            log::info!("Generating Compliance Certificate: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Compliance Certificate.");
    Ok(())
}


async fn generate_compliance_checks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Compliance Check", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Compliance Check: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Compliance Check (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::compliance_checks().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_check_id(format!("{} {}", "string()", i + 1));

                entity.update_result(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Compliance Check");

        if i % 20 == 0 {
            log::info!("Generating Compliance Check: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Compliance Check.");
    Ok(())
}


async fn generate_consumables<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Consumable", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Consumable: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Consumable (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::consumables().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_consumable_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Consumable");

        if i % 20 == 0 {
            log::info!("Generating Consumable: {}/{}", i, fanout);
        }

        state.add_reference("Consumable", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Consumable.");
    Ok(())
}


async fn generate_contracts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Contract", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Contract: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_contract_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Contract");

        if i % 20 == 0 {
            log::info!("Generating Contract: {}/{}", i, fanout);
        }

        state.add_reference("Contract", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Contract.");
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
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_crew_id(format!("{} {}", "string()", i + 1));

                entity.update_size(format!("{} {}", "string()", i + 1));



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


async fn generate_currency_rates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Currency Rate", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Currency Rate: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Currency Rate (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::currency_rates().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_rate_id(format!("{} {}", "string()", i + 1));

                entity.update_pair(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Currency Rate");

        if i % 20 == 0 {
            log::info!("Generating Currency Rate: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Currency Rate.");
    Ok(())
}


async fn generate_customers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Customer", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Customer: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Customer (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::customers().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_customer_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer");

        if i % 20 == 0 {
            log::info!("Generating Customer: {}/{}", i, fanout);
        }

        state.add_reference("Customer", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Customer.");
    Ok(())
}


async fn generate_customer_segments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Customer Segment", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Customer Segment: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Customer Segment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::customer_segments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_segment_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer Segment");

        if i % 20 == 0 {
            log::info!("Generating Customer Segment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Customer Segment.");
    Ok(())
}


async fn generate_data_retention_policies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Data Retention Policy", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Data Retention Policy: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Data Retention Policy (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::data_retention_policies().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_policy_id(format!("{} {}", "string()", i + 1));

                entity.update_duration_years(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Data Retention Policy");

        if i % 20 == 0 {
            log::info!("Generating Data Retention Policy: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Data Retention Policy.");
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
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_name(format!("{} {}", "string()", i + 1));

                entity.update_code(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Department");

        if i % 20 == 0 {
            log::info!("Generating Department: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Department.");
    Ok(())
}


async fn generate_department_hierarchies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Department Hierarchy", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Department Hierarchy: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Department Hierarchy (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::department_hierarchies().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_level(format!("{} {}", "string()", i + 1));

                entity.update_parent(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Department Hierarchy");

        if i % 20 == 0 {
            log::info!("Generating Department Hierarchy: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Department Hierarchy.");
    Ok(())
}


async fn generate_employees<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Employee", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Employee: Required dependency Merchant is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_name(format!("{} {}", "string()", i + 1));

                entity.update_role(format!("{} {}", "string()", i + 1));



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


async fn generate_equipment<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Equipment", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Equipment: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Equipment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::equipment().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_equipment_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Equipment");

        if i % 20 == 0 {
            log::info!("Generating Equipment: {}/{}", i, fanout);
        }

        state.add_reference("Equipment", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Equipment.");
    Ok(())
}


async fn generate_expenses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Expense", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Expense: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Expense (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::expenses().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_expense_id(format!("{} {}", "string()", i + 1));

                entity.update_category(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Expense");

        if i % 20 == 0 {
            log::info!("Generating Expense: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Expense.");
    Ok(())
}


async fn generate_financial_periods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Financial Period", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Financial Period: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Financial Period (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::financial_periods().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_period_id(format!("{} {}", "string()", i + 1));

                entity.update_start(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Financial Period");

        if i % 20 == 0 {
            log::info!("Generating Financial Period: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Financial Period.");
    Ok(())
}


async fn generate_financial_summaries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Financial Summary", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Financial Summary: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Financial Summary (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::financial_summaries().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_summary_id(format!("{} {}", "string()", i + 1));

                entity.update_period(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Financial Summary");

        if i % 20 == 0 {
            log::info!("Generating Financial Summary: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Financial Summary.");
    Ok(())
}


async fn generate_incident_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Incident Report", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Incident Report: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_report_id(format!("{} {}", "string()", i + 1));

                entity.update_severity(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Incident Report");

        if i % 20 == 0 {
            log::info!("Generating Incident Report: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Incident Report.");
    Ok(())
}


async fn generate_insurance_policies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Insurance Policy", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Insurance Policy: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Insurance Policy (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::insurance_policies().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_policy_id(format!("{} {}", "string()", i + 1));

                entity.update_provider(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Insurance Policy");

        if i % 20 == 0 {
            log::info!("Generating Insurance Policy: {}/{}", i, fanout);
        }

        state.add_reference("Insurance Policy", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Insurance Policy.");
    Ok(())
}


async fn generate_inventory_items<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Inventory Item", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Inventory Item: Required dependency Merchant is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_item_id(format!("{} {}", "string()", i + 1));

                entity.update_sku(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Inventory Item");

        if i % 20 == 0 {
            log::info!("Generating Inventory Item: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Inventory Item.");
    Ok(())
}


async fn generate_legal_entities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Legal Entity", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Legal Entity: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Legal Entity (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::legal_entities().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_entity_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Legal Entity");

        if i % 20 == 0 {
            log::info!("Generating Legal Entity: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Legal Entity.");
    Ok(())
}


async fn generate_loading_zones<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Loading Zone", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Loading Zone: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Loading Zone (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::loading_zones().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_zone_id(format!("{} {}", "string()", i + 1));

                entity.update_capacity(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Loading Zone");

        if i % 20 == 0 {
            log::info!("Generating Loading Zone: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Loading Zone.");
    Ok(())
}


async fn generate_marketing_channels<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Marketing Channel", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Marketing Channel: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Marketing Channel (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::marketing_channels().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_channel_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Marketing Channel");

        if i % 20 == 0 {
            log::info!("Generating Marketing Channel: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Marketing Channel.");
    Ok(())
}


async fn generate_move_statuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Move Status", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Move Status: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Move Status (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::move_statuses().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_status_code(format!("{} {}", "string()", i + 1));

                entity.update_description(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Move Status");

        if i % 20 == 0 {
            log::info!("Generating Move Status: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Move Status.");
    Ok(())
}


async fn generate_notification_templates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Notification Template", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Notification Template: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Notification Template (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::notification_templates().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_template_id(format!("{} {}", "string()", i + 1));

                entity.update_subject(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Notification Template");

        if i % 20 == 0 {
            log::info!("Generating Notification Template: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Notification Template.");
    Ok(())
}


async fn generate_organization_units<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Organization Unit", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Organization Unit: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Organization Unit (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::organization_units().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_name(format!("{} {}", "string()", i + 1));

                entity.update_code(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Organization Unit");

        if i % 20 == 0 {
            log::info!("Generating Organization Unit: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Organization Unit.");
    Ok(())
}


async fn generate_packing_materials<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Packing Material", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Packing Material: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Packing Material (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::packing_materials().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_material_type(format!("{} {}", "string()", i + 1));

                entity.update_quantity(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Packing Material");

        if i % 20 == 0 {
            log::info!("Generating Packing Material: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Packing Material.");
    Ok(())
}


async fn generate_payables<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Payable", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Payable: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payable (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payables().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_payable_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payable");

        if i % 20 == 0 {
            log::info!("Generating Payable: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Payable.");
    Ok(())
}


async fn generate_payments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Payment", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Payment: Required dependency Merchant is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_payment_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payment");

        if i % 20 == 0 {
            log::info!("Generating Payment: {}/{}", i, fanout);
        }

        state.add_reference("Payment", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Payment.");
    Ok(())
}


async fn generate_payment_methods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Payment Method", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Payment Method: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payment Method (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payment_methods().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_method_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payment Method");

        if i % 20 == 0 {
            log::info!("Generating Payment Method: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Payment Method.");
    Ok(())
}


async fn generate_payroll_periods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Payroll Period", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Payroll Period: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payroll Period (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payroll_periods().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_period_id(format!("{} {}", "string()", i + 1));

                entity.update_start(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payroll Period");

        if i % 20 == 0 {
            log::info!("Generating Payroll Period: {}/{}", i, fanout);
        }

        state.add_reference("Payroll Period", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Payroll Period.");
    Ok(())
}


async fn generate_permissions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Permission", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Permission: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Permission (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::permissions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_permission_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Permission");

        if i % 20 == 0 {
            log::info!("Generating Permission: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Permission.");
    Ok(())
}


async fn generate_policy_documents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Policy Document", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Policy Document: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Policy Document (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::policy_documents().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_doc_id(format!("{} {}", "string()", i + 1));

                entity.update_title(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Policy Document");

        if i % 20 == 0 {
            log::info!("Generating Policy Document: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Policy Document.");
    Ok(())
}


async fn generate_price_lists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Price List", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Price List: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Price List (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::price_lists().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_list_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Price List");

        if i % 20 == 0 {
            log::info!("Generating Price List: {}/{}", i, fanout);
        }

        state.add_reference("Price List", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Price List.");
    Ok(())
}


async fn generate_products<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Product", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Product: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Product (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::products().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_product_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Product");

        if i % 20 == 0 {
            log::info!("Generating Product: {}/{}", i, fanout);
        }

        state.add_reference("Product", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Product.");
    Ok(())
}


async fn generate_regulatory_requirements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Regulatory Requirement", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Regulatory Requirement: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Regulatory Requirement (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::regulatory_requirements().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_req_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Regulatory Requirement");

        if i % 20 == 0 {
            log::info!("Generating Regulatory Requirement: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Regulatory Requirement.");
    Ok(())
}


async fn generate_roles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Role", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Role: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Role (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::roles().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_role_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Role");

        if i % 20 == 0 {
            log::info!("Generating Role: {}/{}", i, fanout);
        }

        state.add_reference("Role", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Role.");
    Ok(())
}


async fn generate_route_optimization_rules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Route Optimization Rule", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Route Optimization Rule: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Route Optimization Rule (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::route_optimization_rules().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_rule_name(format!("{} {}", "string()", i + 1));

                entity.update_enabled(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Route Optimization Rule");

        if i % 20 == 0 {
            log::info!("Generating Route Optimization Rule: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Route Optimization Rule.");
    Ok(())
}


async fn generate_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Service", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Service: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::services().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_service_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service");

        if i % 20 == 0 {
            log::info!("Generating Service: {}/{}", i, fanout);
        }

        state.add_reference("Service", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Service.");
    Ok(())
}


async fn generate_service_areas<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Service Area", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Service Area: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service Area (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::service_areas().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_area_id(format!("{} {}", "string()", i + 1));

                entity.update_region(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service Area");

        if i % 20 == 0 {
            log::info!("Generating Service Area: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Service Area.");
    Ok(())
}


async fn generate_service_bundles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Service Bundle", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Service Bundle: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service Bundle (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::service_bundles().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_bundle_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service Bundle");

        if i % 20 == 0 {
            log::info!("Generating Service Bundle: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Service Bundle.");
    Ok(())
}


async fn generate_service_categories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Service Category", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Service Category: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service Category (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::service_categories().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_category_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service Category");

        if i % 20 == 0 {
            log::info!("Generating Service Category: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Service Category.");
    Ok(())
}


async fn generate_storage_locations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Storage Location", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Storage Location: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Storage Location (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::storage_locations().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_location_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Storage Location");

        if i % 20 == 0 {
            log::info!("Generating Storage Location: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Storage Location.");
    Ok(())
}


async fn generate_storage_units<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Storage Unit", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Storage Unit: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Storage Unit (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::storage_units().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_unit_id(format!("{} {}", "string()", i + 1));

                entity.update_size_sqft(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Storage Unit");

        if i % 20 == 0 {
            log::info!("Generating Storage Unit: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Storage Unit.");
    Ok(())
}


async fn generate_suppliers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Supplier", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Supplier: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Supplier (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::suppliers().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_supplier_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Supplier");

        if i % 20 == 0 {
            log::info!("Generating Supplier: {}/{}", i, fanout);
        }

        state.add_reference("Supplier", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Supplier.");
    Ok(())
}


async fn generate_system_events<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("System Event", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating System Event: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for System Event (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::system_events().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_event_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("System Event");

        if i % 20 == 0 {
            log::info!("Generating System Event: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for System Event.");
    Ok(())
}


async fn generate_tenant_configurations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Tenant Configuration", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Tenant Configuration: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Tenant Configuration (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::tenant_configurations().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_region(format!("{} {}", "string()", i + 1));

                entity.update_tier(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Tenant Configuration");

        if i % 20 == 0 {
            log::info!("Generating Tenant Configuration: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Tenant Configuration.");
    Ok(())
}


async fn generate_unloading_zones<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Unloading Zone", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Unloading Zone: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Unloading Zone (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::unloading_zones().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_zone_id(format!("{} {}", "string()", i + 1));

                entity.update_capacity(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Unloading Zone");

        if i % 20 == 0 {
            log::info!("Generating Unloading Zone: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Unloading Zone.");
    Ok(())
}


async fn generate_vat_rates<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("VAT Rate", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating VAT Rate: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for VAT Rate (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::vat_rates().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_rate_id(format!("{} {}", "string()", i + 1));

                entity.update_percentage(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("VAT Rate");

        if i % 20 == 0 {
            log::info!("Generating VAT Rate: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for VAT Rate.");
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
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_vehicle_id(format!("{} {}", "string()", i + 1));

                entity.update_make(format!("{} {}", "string()", i + 1));



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


async fn generate_account_statuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Account Status", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Account Status: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Account Status (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::account_statuses().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_status_id(format!("{} {}", "string()", i + 1));

                entity.update_state(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Account Status");

        if i % 20 == 0 {
            log::info!("Generating Account Status: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Account Status.");
    Ok(())
}


async fn generate_add_on_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Service").is_empty() {
            state.record_skipped("Add On Service", "Required dependency Service is missing in reference pool".to_string());
            log::info!("Skipped generating Add On Service: Required dependency Service is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Add On Service (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::add_on_services().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Service", i as usize, &used_refs) {
                    entity.update_service_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_addon_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Add On Service");

        if i % 20 == 0 {
            log::info!("Generating Add On Service: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Add On Service.");
    Ok(())
}


async fn generate_api_endpoints<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("API Client").is_empty() {
            state.record_skipped("API Endpoint", "Required dependency API Client is missing in reference pool".to_string());
            log::info!("Skipped generating API Endpoint: Required dependency API Client is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for API Endpoint (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::api_endpoints().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("API Client", i as usize, &used_refs) {
                    entity.update_api_client_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_endpoint_id(format!("{} {}", "string()", i + 1));

                entity.update_path(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("API Endpoint");

        if i % 20 == 0 {
            log::info!("Generating API Endpoint: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for API Endpoint.");
    Ok(())
}


async fn generate_api_keys<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("API Client").is_empty() {
            state.record_skipped("API Key", "Required dependency API Client is missing in reference pool".to_string());
            log::info!("Skipped generating API Key: Required dependency API Client is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for API Key (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::api_keys().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("API Client", i as usize, &used_refs) {
                    entity.update_api_client_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_key_id(format!("{} {}", "string()", i + 1));

                entity.update_prefix(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("API Key");

        if i % 20 == 0 {
            log::info!("Generating API Key: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for API Key.");
    Ok(())
}


async fn generate_asset_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Asset Assignment", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Asset Assignment: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Asset Assignment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::asset_assignments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_assignment_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Asset Assignment");

        if i % 20 == 0 {
            log::info!("Generating Asset Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Asset Assignment.");
    Ok(())
}


async fn generate_asset_conditions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Asset Condition", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Asset Condition: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Asset Condition (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::asset_conditions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_condition_id(format!("{} {}", "string()", i + 1));

                entity.update_rating(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Asset Condition");

        if i % 20 == 0 {
            log::info!("Generating Asset Condition: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Asset Condition.");
    Ok(())
}


async fn generate_asset_inspections<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Asset Inspection", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Asset Inspection: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Asset Inspection (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::asset_inspections().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_inspection_id(format!("{} {}", "string()", i + 1));

                entity.update_result(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Asset Inspection");

        if i % 20 == 0 {
            log::info!("Generating Asset Inspection: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Asset Inspection.");
    Ok(())
}


async fn generate_attendance_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Attendance Record", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Attendance Record: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Attendance Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::attendance_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_record_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Attendance Record");

        if i % 20 == 0 {
            log::info!("Generating Attendance Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Attendance Record.");
    Ok(())
}


async fn generate_attribution_models<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Attribution Model", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Attribution Model: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Attribution Model (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::attribution_models().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_model_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Attribution Model");

        if i % 20 == 0 {
            log::info!("Generating Attribution Model: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Attribution Model.");
    Ok(())
}


async fn generate_audience_segments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Audience Segment", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Audience Segment: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Audience Segment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::audience_segments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_segment_id(format!("{} {}", "string()", i + 1));

                entity.update_criteria(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Audience Segment");

        if i % 20 == 0 {
            log::info!("Generating Audience Segment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Audience Segment.");
    Ok(())
}


async fn generate_audit_trails<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Contract").is_empty() {
            state.record_skipped("Audit Trail", "Required dependency Contract is missing in reference pool".to_string());
            log::info!("Skipped generating Audit Trail: Required dependency Contract is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Audit Trail (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::audit_trails().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Contract", i as usize, &used_refs) {
                    entity.update_contract_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_trail_id(format!("{} {}", "string()", i + 1));

                entity.update_action(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Audit Trail");

        if i % 20 == 0 {
            log::info!("Generating Audit Trail: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Audit Trail.");
    Ok(())
}


async fn generate_automation_actions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Automation Rule").is_empty() {
            state.record_skipped("Automation Action", "Required dependency Automation Rule is missing in reference pool".to_string());
            log::info!("Skipped generating Automation Action: Required dependency Automation Rule is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Automation Action (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::automation_actions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Automation Rule", i as usize, &used_refs) {
                    entity.update_automation_rule_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_action_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Automation Action");

        if i % 20 == 0 {
            log::info!("Generating Automation Action: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Automation Action.");
    Ok(())
}


async fn generate_automation_triggers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Automation Rule").is_empty() {
            state.record_skipped("Automation Trigger", "Required dependency Automation Rule is missing in reference pool".to_string());
            log::info!("Skipped generating Automation Trigger: Required dependency Automation Rule is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Automation Trigger (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::automation_triggers().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Automation Rule", i as usize, &used_refs) {
                    entity.update_automation_rule_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_trigger_id(format!("{} {}", "string()", i + 1));

                entity.update_event(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Automation Trigger");

        if i % 20 == 0 {
            log::info!("Generating Automation Trigger: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Automation Trigger.");
    Ok(())
}


async fn generate_availability_calendars<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Service").is_empty() {
            state.record_skipped("Availability Calendar", "Required dependency Service is missing in reference pool".to_string());
            log::info!("Skipped generating Availability Calendar: Required dependency Service is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Availability Calendar (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::availability_calendars().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Service", i as usize, &used_refs) {
                    entity.update_service_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_calendar_id(format!("{} {}", "string()", i + 1));

                entity.update_month(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Availability Calendar");

        if i % 20 == 0 {
            log::info!("Generating Availability Calendar: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Availability Calendar.");
    Ok(())
}


async fn generate_availability_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Availability Schedule", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Availability Schedule: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Availability Schedule (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::availability_schedules().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_schedule_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Availability Schedule");

        if i % 20 == 0 {
            log::info!("Generating Availability Schedule: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Availability Schedule.");
    Ok(())
}


async fn generate_benefit_enrollments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Benefit Enrollment", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Benefit Enrollment: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Benefit Enrollment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::benefit_enrollments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_enrollment_id(format!("{} {}", "string()", i + 1));

                entity.update_plan(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Benefit Enrollment");

        if i % 20 == 0 {
            log::info!("Generating Benefit Enrollment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Benefit Enrollment.");
    Ok(())
}


async fn generate_billing_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Billing Profile", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Billing Profile: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Billing Profile (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::billing_profiles().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_profile_id(format!("{} {}", "string()", i + 1));

                entity.update_method(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Billing Profile");

        if i % 20 == 0 {
            log::info!("Generating Billing Profile: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Billing Profile.");
    Ok(())
}


async fn generate_bonuses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Bonus", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Bonus: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Bonus (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::bonuses().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_bonus_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Bonus");

        if i % 20 == 0 {
            log::info!("Generating Bonus: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Bonus.");
    Ok(())
}


async fn generate_box_rentals<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Product").is_empty() {
            state.record_skipped("Box Rental", "Required dependency Product is missing in reference pool".to_string());
            log::info!("Skipped generating Box Rental: Required dependency Product is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Box Rental (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::box_rentals().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Product", i as usize, &used_refs) {
                    entity.update_product_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_rental_id(format!("{} {}", "string()", i + 1));

                entity.update_duration_days(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Box Rental");

        if i % 20 == 0 {
            log::info!("Generating Box Rental: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Box Rental.");
    Ok(())
}


async fn generate_campaign_budgets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Campaign Budget", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Campaign Budget: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Campaign Budget (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::campaign_budgets().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_budget_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Campaign Budget");

        if i % 20 == 0 {
            log::info!("Generating Campaign Budget: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Campaign Budget.");
    Ok(())
}


async fn generate_cleaning_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Service").is_empty() {
            state.record_skipped("Cleaning Service", "Required dependency Service is missing in reference pool".to_string());
            log::info!("Skipped generating Cleaning Service: Required dependency Service is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Cleaning Service (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::cleaning_services().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Service", i as usize, &used_refs) {
                    entity.update_service_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_service_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Cleaning Service");

        if i % 20 == 0 {
            log::info!("Generating Cleaning Service: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Cleaning Service.");
    Ok(())
}


async fn generate_commission_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Commission Record", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Commission Record: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Commission Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::commission_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_commission_id(format!("{} {}", "string()", i + 1));

                entity.update_percentage(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Commission Record");

        if i % 20 == 0 {
            log::info!("Generating Commission Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Commission Record.");
    Ok(())
}


async fn generate_communication_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Communication Log", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Communication Log: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Communication Log (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::communication_logs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_log_id(format!("{} {}", "string()", i + 1));

                entity.update_channel(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Communication Log");

        if i % 20 == 0 {
            log::info!("Generating Communication Log: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Communication Log.");
    Ok(())
}


async fn generate_contact_methods<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Contact Method", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Contact Method: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Contact Method (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::contact_methods().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_method_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Contact Method");

        if i % 20 == 0 {
            log::info!("Generating Contact Method: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Contact Method.");
    Ok(())
}


async fn generate_conversion_metrics<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Conversion Metric", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Conversion Metric: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Conversion Metric (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::conversion_metrics().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_metric_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Conversion Metric");

        if i % 20 == 0 {
            log::info!("Generating Conversion Metric: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Conversion Metric.");
    Ok(())
}


async fn generate_conversion_reports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Conversion Report", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Conversion Report: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Conversion Report (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::conversion_reports().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_report_id(format!("{} {}", "string()", i + 1));

                entity.update_period(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Conversion Report");

        if i % 20 == 0 {
            log::info!("Generating Conversion Report: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Conversion Report.");
    Ok(())
}


async fn generate_corporate_customer_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Corporate Customer Profile", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Corporate Customer Profile: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Corporate Customer Profile (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::corporate_customer_profiles().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_profile_id(format!("{} {}", "string()", i + 1));

                entity.update_company_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Corporate Customer Profile");

        if i % 20 == 0 {
            log::info!("Generating Corporate Customer Profile: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Corporate Customer Profile.");
    Ok(())
}


async fn generate_customer_consents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Customer Consent", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Customer Consent: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Customer Consent (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::customer_consents().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_consent_id(format!("{} {}", "string()", i + 1));

                entity.update_scope(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer Consent");

        if i % 20 == 0 {
            log::info!("Generating Customer Consent: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Customer Consent.");
    Ok(())
}


async fn generate_customer_contacts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Customer Contact", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Customer Contact: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Customer Contact (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::customer_contacts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_contact_id(format!("{} {}", "string()", i + 1));

                entity.update_email(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer Contact");

        if i % 20 == 0 {
            log::info!("Generating Customer Contact: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Customer Contact.");
    Ok(())
}


async fn generate_customer_feedback<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Customer Feedback", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Customer Feedback: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_feedback_id(format!("{} {}", "string()", i + 1));

                entity.update_rating(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer Feedback");

        if i % 20 == 0 {
            log::info!("Generating Customer Feedback: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Customer Feedback.");
    Ok(())
}


async fn generate_customer_histories<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Customer History", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Customer History: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Customer History (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::customer_histories().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_history_id(format!("{} {}", "string()", i + 1));

                entity.update_total_moves(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer History");

        if i % 20 == 0 {
            log::info!("Generating Customer History: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Customer History.");
    Ok(())
}


async fn generate_customer_preferences<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Customer Preference", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Customer Preference: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Customer Preference (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::customer_preferences().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_preference_id(format!("{} {}", "string()", i + 1));

                entity.update_key(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Customer Preference");

        if i % 20 == 0 {
            log::info!("Generating Customer Preference: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Customer Preference.");
    Ok(())
}


async fn generate_depreciation_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Depreciation Record", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Depreciation Record: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Depreciation Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::depreciation_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_record_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Depreciation Record");

        if i % 20 == 0 {
            log::info!("Generating Depreciation Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Depreciation Record.");
    Ok(())
}


async fn generate_discount_codes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Discount Code", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Discount Code: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Discount Code (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::discount_codes().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_code(format!("{} {}", "string()", i + 1));

                entity.update_value(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Discount Code");

        if i % 20 == 0 {
            log::info!("Generating Discount Code: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Discount Code.");
    Ok(())
}


async fn generate_disposal_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Service").is_empty() {
            state.record_skipped("Disposal Service", "Required dependency Service is missing in reference pool".to_string());
            log::info!("Skipped generating Disposal Service: Required dependency Service is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Disposal Service (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::disposal_services().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Service", i as usize, &used_refs) {
                    entity.update_service_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_service_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Disposal Service");

        if i % 20 == 0 {
            log::info!("Generating Disposal Service: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Disposal Service.");
    Ok(())
}


async fn generate_documents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Contract").is_empty() {
            state.record_skipped("Document", "Required dependency Contract is missing in reference pool".to_string());
            log::info!("Skipped generating Document: Required dependency Contract is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Document (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::documents().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Contract", i as usize, &used_refs) {
                    entity.update_contract_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_document_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Document");

        if i % 20 == 0 {
            log::info!("Generating Document: {}/{}", i, fanout);
        }

        state.add_reference("Document", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Document.");
    Ok(())
}


async fn generate_employee_certifications<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Employee Certification", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Employee Certification: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Employee Certification (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::employee_certifications().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_cert_id(format!("{} {}", "string()", i + 1));

                entity.update_name(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Employee Certification");

        if i % 20 == 0 {
            log::info!("Generating Employee Certification: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Employee Certification.");
    Ok(())
}


async fn generate_equipment_serials<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Equipment").is_empty() {
            state.record_skipped("Equipment Serial", "Required dependency Equipment is missing in reference pool".to_string());
            log::info!("Skipped generating Equipment Serial: Required dependency Equipment is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Equipment Serial (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::equipment_serials().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Equipment", i as usize, &used_refs) {
                    entity.update_equipment_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_serial_id(format!("{} {}", "string()", i + 1));

                entity.update_code(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Equipment Serial");

        if i % 20 == 0 {
            log::info!("Generating Equipment Serial: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Equipment Serial.");
    Ok(())
}


async fn generate_financial_hooks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Automation Rule").is_empty() {
            state.record_skipped("Financial Hook", "Required dependency Automation Rule is missing in reference pool".to_string());
            log::info!("Skipped generating Financial Hook: Required dependency Automation Rule is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Financial Hook (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::financial_hooks().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Automation Rule", i as usize, &used_refs) {
                    entity.update_automation_rule_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_hook_id(format!("{} {}", "string()", i + 1));

                entity.update_endpoint(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Financial Hook");

        if i % 20 == 0 {
            log::info!("Generating Financial Hook: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Financial Hook.");
    Ok(())
}


async fn generate_fuel_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Fuel Record", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Fuel Record: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Fuel Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::fuel_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_record_id(format!("{} {}", "string()", i + 1));

                entity.update_gallons(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Fuel Record");

        if i % 20 == 0 {
            log::info!("Generating Fuel Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Fuel Record.");
    Ok(())
}


async fn generate_insurance_claims<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Insurance Policy").is_empty() {
            state.record_skipped("Insurance Claim", "Required dependency Insurance Policy is missing in reference pool".to_string());
            log::info!("Skipped generating Insurance Claim: Required dependency Insurance Policy is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Insurance Claim (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::insurance_claims().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Insurance Policy", i as usize, &used_refs) {
                    entity.update_insurance_policy_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_claim_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Insurance Claim");

        if i % 20 == 0 {
            log::info!("Generating Insurance Claim: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Insurance Claim.");
    Ok(())
}


async fn generate_integration_mappings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("API Client").is_empty() {
            state.record_skipped("Integration Mapping", "Required dependency API Client is missing in reference pool".to_string());
            log::info!("Skipped generating Integration Mapping: Required dependency API Client is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Integration Mapping (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::integration_mappings().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("API Client", i as usize, &used_refs) {
                    entity.update_api_client_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_mapping_id(format!("{} {}", "string()", i + 1));

                entity.update_source(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Integration Mapping");

        if i % 20 == 0 {
            log::info!("Generating Integration Mapping: {}/{}", i, fanout);
        }

        state.add_reference("Integration Mapping", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Integration Mapping.");
    Ok(())
}


async fn generate_inventory_stocks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Consumable").is_empty() {
            state.record_skipped("Inventory Stock", "Required dependency Consumable is missing in reference pool".to_string());
            log::info!("Skipped generating Inventory Stock: Required dependency Consumable is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Inventory Stock (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::inventory_stocks().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Consumable", i as usize, &used_refs) {
                    entity.update_consumable_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_stock_id(format!("{} {}", "string()", i + 1));

                entity.update_quantity(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Inventory Stock");

        if i % 20 == 0 {
            log::info!("Generating Inventory Stock: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Inventory Stock.");
    Ok(())
}


async fn generate_invoices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payment").is_empty() {
            state.record_skipped("Invoice", "Required dependency Payment is missing in reference pool".to_string());
            log::info!("Skipped generating Invoice: Required dependency Payment is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Payment", i as usize, &used_refs) {
                    entity.update_payment_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_invoice_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Invoice");

        if i % 20 == 0 {
            log::info!("Generating Invoice: {}/{}", i, fanout);
        }

        state.add_reference("Invoice", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Invoice.");
    Ok(())
}


async fn generate_job_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Job Assignment", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Job Assignment: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Job Assignment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::job_assignments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_title(format!("{} {}", "string()", i + 1));

                entity.update_level(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Job Assignment");

        if i % 20 == 0 {
            log::info!("Generating Job Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Job Assignment.");
    Ok(())
}


async fn generate_leads<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Lead", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Lead: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Lead (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::leads().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_lead_id(format!("{} {}", "string()", i + 1));

                entity.update_source(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Lead");

        if i % 20 == 0 {
            log::info!("Generating Lead: {}/{}", i, fanout);
        }

        state.add_reference("Lead", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Lead.");
    Ok(())
}


async fn generate_leave_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Leave Request", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Leave Request: Required dependency Employee is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_request_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Leave Request");

        if i % 20 == 0 {
            log::info!("Generating Leave Request: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Leave Request.");
    Ok(())
}


async fn generate_loyalty_tiers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Loyalty Tier", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Loyalty Tier: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Loyalty Tier (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::loyalty_tiers().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_tier_id(format!("{} {}", "string()", i + 1));

                entity.update_level(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Loyalty Tier");

        if i % 20 == 0 {
            log::info!("Generating Loyalty Tier: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Loyalty Tier.");
    Ok(())
}


async fn generate_maintenance_events<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Maintenance Event", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Maintenance Event: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Maintenance Event (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::maintenance_events().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_event_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Maintenance Event");

        if i % 20 == 0 {
            log::info!("Generating Maintenance Event: {}/{}", i, fanout);
        }

        state.add_reference("Maintenance Event", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Maintenance Event.");
    Ok(())
}


async fn generate_maintenance_schedules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Maintenance Schedule", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Maintenance Schedule: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Maintenance Schedule (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::maintenance_schedules().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_schedule_id(format!("{} {}", "string()", i + 1));

                entity.update_interval_days(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Maintenance Schedule");

        if i % 20 == 0 {
            log::info!("Generating Maintenance Schedule: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Maintenance Schedule.");
    Ok(())
}


async fn generate_move_orders<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Merchant").is_empty() {
            state.record_skipped("Move Order", "Required dependency Merchant is missing in reference pool".to_string());
            log::info!("Skipped generating Move Order: Required dependency Merchant is missing in reference pool.");
            return Ok(());
        }

        if state.ids("Customer").is_empty() {
            state.record_skipped("Move Order", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Move Order: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Merchant", i as usize, &used_refs) {
                    entity.update_merchant_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_order_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



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


async fn generate_moving_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Service").is_empty() {
            state.record_skipped("Moving Service", "Required dependency Service is missing in reference pool".to_string());
            log::info!("Skipped generating Moving Service: Required dependency Service is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Moving Service (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::moving_services().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Service", i as usize, &used_refs) {
                    entity.update_service_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_service_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Moving Service");

        if i % 20 == 0 {
            log::info!("Generating Moving Service: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Moving Service.");
    Ok(())
}


async fn generate_operational_hooks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Automation Rule").is_empty() {
            state.record_skipped("Operational Hook", "Required dependency Automation Rule is missing in reference pool".to_string());
            log::info!("Skipped generating Operational Hook: Required dependency Automation Rule is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Operational Hook (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::operational_hooks().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Automation Rule", i as usize, &used_refs) {
                    entity.update_automation_rule_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_hook_id(format!("{} {}", "string()", i + 1));

                entity.update_endpoint(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Operational Hook");

        if i % 20 == 0 {
            log::info!("Generating Operational Hook: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Operational Hook.");
    Ok(())
}


async fn generate_overtime_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Overtime Record", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Overtime Record: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Overtime Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::overtime_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_record_id(format!("{} {}", "string()", i + 1));

                entity.update_hours(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Overtime Record");

        if i % 20 == 0 {
            log::info!("Generating Overtime Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Overtime Record.");
    Ok(())
}


async fn generate_packing_kits<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Product").is_empty() {
            state.record_skipped("Packing Kit", "Required dependency Product is missing in reference pool".to_string());
            log::info!("Skipped generating Packing Kit: Required dependency Product is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Packing Kit (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::packing_kits().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Product", i as usize, &used_refs) {
                    entity.update_product_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_kit_id(format!("{} {}", "string()", i + 1));

                entity.update_contents(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Packing Kit");

        if i % 20 == 0 {
            log::info!("Generating Packing Kit: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Packing Kit.");
    Ok(())
}


async fn generate_payroll_calculations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payroll Period").is_empty() {
            state.record_skipped("Payroll Calculation", "Required dependency Payroll Period is missing in reference pool".to_string());
            log::info!("Skipped generating Payroll Calculation: Required dependency Payroll Period is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payroll Calculation (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payroll_calculations().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Payroll Period", i as usize, &used_refs) {
                    entity.update_payroll_period_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_calc_id(format!("{} {}", "string()", i + 1));

                entity.update_gross_pay(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payroll Calculation");

        if i % 20 == 0 {
            log::info!("Generating Payroll Calculation: {}/{}", i, fanout);
        }

        state.add_reference("Payroll Calculation", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Payroll Calculation.");
    Ok(())
}


async fn generate_performance_reviews<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Performance Review", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Performance Review: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_review_id(format!("{} {}", "string()", i + 1));

                entity.update_score(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Performance Review");

        if i % 20 == 0 {
            log::info!("Generating Performance Review: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Performance Review.");
    Ok(())
}


async fn generate_private_customer_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Private Customer Profile", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Private Customer Profile: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Private Customer Profile (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::private_customer_profiles().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_profile_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Private Customer Profile");

        if i % 20 == 0 {
            log::info!("Generating Private Customer Profile: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Private Customer Profile.");
    Ok(())
}


async fn generate_promotional_offers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Campaign").is_empty() {
            state.record_skipped("Promotional Offer", "Required dependency Campaign is missing in reference pool".to_string());
            log::info!("Skipped generating Promotional Offer: Required dependency Campaign is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Promotional Offer (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::promotional_offers().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Campaign", i as usize, &used_refs) {
                    entity.update_campaign_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_offer_id(format!("{} {}", "string()", i + 1));

                entity.update_description(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Promotional Offer");

        if i % 20 == 0 {
            log::info!("Generating Promotional Offer: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Promotional Offer.");
    Ok(())
}


async fn generate_referral_codes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Referral Code", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Referral Code: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Referral Code (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::referral_codes().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_code(format!("{} {}", "string()", i + 1));

                entity.update_uses(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Referral Code");

        if i % 20 == 0 {
            log::info!("Generating Referral Code: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Referral Code.");
    Ok(())
}


async fn generate_refunds<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payment").is_empty() {
            state.record_skipped("Refund", "Required dependency Payment is missing in reference pool".to_string());
            log::info!("Skipped generating Refund: Required dependency Payment is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Refund (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::refunds().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Payment", i as usize, &used_refs) {
                    entity.update_payment_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_refund_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Refund");

        if i % 20 == 0 {
            log::info!("Generating Refund: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Refund.");
    Ok(())
}


async fn generate_role_permissions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Role").is_empty() {
            state.record_skipped("Role Permission", "Required dependency Role is missing in reference pool".to_string());
            log::info!("Skipped generating Role Permission: Required dependency Role is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Role Permission (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::role_permissions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Role", i as usize, &used_refs) {
                    entity.update_role_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_link_id(format!("{} {}", "string()", i + 1));

                entity.update_scope(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Role Permission");

        if i % 20 == 0 {
            log::info!("Generating Role Permission: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Role Permission.");
    Ok(())
}


async fn generate_service_configurations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Service").is_empty() {
            state.record_skipped("Service Configuration", "Required dependency Service is missing in reference pool".to_string());
            log::info!("Skipped generating Service Configuration: Required dependency Service is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service Configuration (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::service_configurations().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Service", i as usize, &used_refs) {
                    entity.update_service_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_config_id(format!("{} {}", "string()", i + 1));

                entity.update_key(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service Configuration");

        if i % 20 == 0 {
            log::info!("Generating Service Configuration: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Service Configuration.");
    Ok(())
}


async fn generate_service_level_agreements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Service").is_empty() {
            state.record_skipped("Service Level Agreement", "Required dependency Service is missing in reference pool".to_string());
            log::info!("Skipped generating Service Level Agreement: Required dependency Service is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service Level Agreement (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::service_level_agreements().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Service", i as usize, &used_refs) {
                    entity.update_service_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_sla_id(format!("{} {}", "string()", i + 1));

                entity.update_target_hours(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service Level Agreement");

        if i % 20 == 0 {
            log::info!("Generating Service Level Agreement: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Service Level Agreement.");
    Ok(())
}


async fn generate_service_prices<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Price List").is_empty() {
            state.record_skipped("Service Price", "Required dependency Price List is missing in reference pool".to_string());
            log::info!("Skipped generating Service Price: Required dependency Price List is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service Price (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::service_prices().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Price List", i as usize, &used_refs) {
                    entity.update_price_list_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_price_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service Price");

        if i % 20 == 0 {
            log::info!("Generating Service Price: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Service Price.");
    Ok(())
}


async fn generate_service_ratings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Customer").is_empty() {
            state.record_skipped("Service Rating", "Required dependency Customer is missing in reference pool".to_string());
            log::info!("Skipped generating Service Rating: Required dependency Customer is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Service Rating (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::service_ratings().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Customer", i as usize, &used_refs) {
                    entity.update_customer_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_rating_id(format!("{} {}", "string()", i + 1));

                entity.update_score(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Service Rating");

        if i % 20 == 0 {
            log::info!("Generating Service Rating: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Service Rating.");
    Ok(())
}


async fn generate_settlements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payment").is_empty() {
            state.record_skipped("Settlement", "Required dependency Payment is missing in reference pool".to_string());
            log::info!("Skipped generating Settlement: Required dependency Payment is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Settlement (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::settlements().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Payment", i as usize, &used_refs) {
                    entity.update_payment_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_settlement_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Settlement");

        if i % 20 == 0 {
            log::info!("Generating Settlement: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Settlement.");
    Ok(())
}


async fn generate_shift_swap_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Shift Swap Request", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Shift Swap Request: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Shift Swap Request (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::shift_swap_requests().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_swap_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Shift Swap Request");

        if i % 20 == 0 {
            log::info!("Generating Shift Swap Request: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Shift Swap Request.");
    Ok(())
}


async fn generate_skill_profiles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Skill Profile", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Skill Profile: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Skill Profile (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::skill_profiles().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_skill(format!("{} {}", "string()", i + 1));

                entity.update_proficiency(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Skill Profile");

        if i % 20 == 0 {
            log::info!("Generating Skill Profile: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Skill Profile.");
    Ok(())
}


async fn generate_supplier_contracts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Supplier").is_empty() {
            state.record_skipped("Supplier Contract", "Required dependency Supplier is missing in reference pool".to_string());
            log::info!("Skipped generating Supplier Contract: Required dependency Supplier is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Supplier Contract (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::supplier_contracts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Supplier", i as usize, &used_refs) {
                    entity.update_supplier_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_contract_id(format!("{} {}", "string()", i + 1));

                entity.update_term(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Supplier Contract");

        if i % 20 == 0 {
            log::info!("Generating Supplier Contract: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Supplier Contract.");
    Ok(())
}


async fn generate_training_modules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Training Module", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Training Module: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Training Module (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::training_modules().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_module_id(format!("{} {}", "string()", i + 1));

                entity.update_duration_hrs(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Training Module");

        if i % 20 == 0 {
            log::info!("Generating Training Module: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Training Module.");
    Ok(())
}


async fn generate_user_accounts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("User Account", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating User Account: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for User Account (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::user_accounts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_account_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("User Account");

        if i % 20 == 0 {
            log::info!("Generating User Account: {}/{}", i, fanout);
        }

        state.add_reference("User Account", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for User Account.");
    Ok(())
}


async fn generate_vehicle_registrations<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Vehicle").is_empty() {
            state.record_skipped("Vehicle Registration", "Required dependency Vehicle is missing in reference pool".to_string());
            log::info!("Skipped generating Vehicle Registration: Required dependency Vehicle is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Vehicle Registration (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::vehicle_registrations().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Vehicle", i as usize, &used_refs) {
                    entity.update_vehicle_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_reg_id(format!("{} {}", "string()", i + 1));

                entity.update_plate(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Vehicle Registration");

        if i % 20 == 0 {
            log::info!("Generating Vehicle Registration: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Vehicle Registration.");
    Ok(())
}


async fn generate_warranty_claims<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Equipment").is_empty() {
            state.record_skipped("Warranty Claim", "Required dependency Equipment is missing in reference pool".to_string());
            log::info!("Skipped generating Warranty Claim: Required dependency Equipment is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Warranty Claim (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::warranty_claims().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Equipment", i as usize, &used_refs) {
                    entity.update_equipment_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_claim_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Warranty Claim");

        if i % 20 == 0 {
            log::info!("Generating Warranty Claim: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Warranty Claim.");
    Ok(())
}


async fn generate_webhooks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("API Client").is_empty() {
            state.record_skipped("Webhook", "Required dependency API Client is missing in reference pool".to_string());
            log::info!("Skipped generating Webhook: Required dependency API Client is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Webhook (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::webhooks().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("API Client", i as usize, &used_refs) {
                    entity.update_api_client_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_webhook_id(format!("{} {}", "string()", i + 1));

                entity.update_url(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Webhook");

        if i % 20 == 0 {
            log::info!("Generating Webhook: {}/{}", i, fanout);
        }

        state.add_reference("Webhook", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Webhook.");
    Ok(())
}


async fn generate_work_shifts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Employee").is_empty() {
            state.record_skipped("Work Shift", "Required dependency Employee is missing in reference pool".to_string());
            log::info!("Skipped generating Work Shift: Required dependency Employee is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Work Shift (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::work_shifts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Employee", i as usize, &used_refs) {
                    entity.update_employee_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_shift_id(format!("{} {}", "string()", i + 1));

                entity.update_start(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Work Shift");

        if i % 20 == 0 {
            log::info!("Generating Work Shift: {}/{}", i, fanout);
        }

        state.add_reference("Work Shift", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Work Shift.");
    Ok(())
}


async fn generate_access_tokens<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("Access Token", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating Access Token: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Access Token (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::access_tokens().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_token_id(format!("{} {}", "string()", i + 1));

                entity.update_scope(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Access Token");

        if i % 20 == 0 {
            log::info!("Generating Access Token: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Access Token.");
    Ok(())
}


async fn generate_activity_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("Activity Log", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating Activity Log: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Activity Log (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::activity_logs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_log_id(format!("{} {}", "string()", i + 1));

                entity.update_action(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Activity Log");

        if i % 20 == 0 {
            log::info!("Generating Activity Log: {}/{}", i, fanout);
        }

        state.add_reference("Activity Log", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Activity Log.");
    Ok(())
}


async fn generate_cargo_weight_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Cargo Weight Record", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Cargo Weight Record: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Cargo Weight Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::cargo_weight_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_record_id(format!("{} {}", "string()", i + 1));

                entity.update_total_weight(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Cargo Weight Record");

        if i % 20 == 0 {
            log::info!("Generating Cargo Weight Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Cargo Weight Record.");
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


    let object_fields_count = 0 + 1;
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
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_report_id(format!("{} {}", "string()", i + 1));

                entity.update_severity(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Damage Report");

        if i % 20 == 0 {
            log::info!("Generating Damage Report: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Damage Report.");
    Ok(())
}


async fn generate_data_exports<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("Data Export", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating Data Export: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Data Export (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::data_exports().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_export_id(format!("{} {}", "string()", i + 1));

                entity.update_format(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Data Export");

        if i % 20 == 0 {
            log::info!("Generating Data Export: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Data Export.");
    Ok(())
}


async fn generate_deductions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payroll Calculation").is_empty() {
            state.record_skipped("Deduction", "Required dependency Payroll Calculation is missing in reference pool".to_string());
            log::info!("Skipped generating Deduction: Required dependency Payroll Calculation is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Deduction (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::deductions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Payroll Calculation", i as usize, &used_refs) {
                    entity.update_payroll_calculation_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_deduction_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Deduction");

        if i % 20 == 0 {
            log::info!("Generating Deduction: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Deduction.");
    Ok(())
}


async fn generate_delay_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Delay Record", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Delay Record: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Delay Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::delay_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_delay_id(format!("{} {}", "string()", i + 1));

                entity.update_duration_min(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Delay Record");

        if i % 20 == 0 {
            log::info!("Generating Delay Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Delay Record.");
    Ok(())
}


async fn generate_delivery_windows<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Delivery Window", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Delivery Window: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Delivery Window (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::delivery_windows().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_window_id(format!("{} {}", "string()", i + 1));

                entity.update_start(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Delivery Window");

        if i % 20 == 0 {
            log::info!("Generating Delivery Window: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Delivery Window.");
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


    let object_fields_count = 0 + 1 + 1;
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
                    entity.update_crew_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_assignment_id(format!("{} {}", "string()", i + 1));

                entity.update_priority(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Dispatch Assignment");

        if i % 20 == 0 {
            log::info!("Generating Dispatch Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Dispatch Assignment.");
    Ok(())
}


async fn generate_document_versions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Document").is_empty() {
            state.record_skipped("Document Version", "Required dependency Document is missing in reference pool".to_string());
            log::info!("Skipped generating Document Version: Required dependency Document is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Document Version (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::document_versions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Document", i as usize, &used_refs) {
                    entity.update_document_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_version_id(format!("{} {}", "string()", i + 1));

                entity.update_version(format!("{} {}", "string()", i + 1));


entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Document Version");

        if i % 20 == 0 {
            log::info!("Generating Document Version: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Document Version.");
    Ok(())
}


async fn generate_fulfillment_events<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Fulfillment Event", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Fulfillment Event: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Fulfillment Event (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::fulfillment_events().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_event_type(format!("{} {}", "string()", i + 1));

                entity.update_timestamp(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Fulfillment Event");

        if i % 20 == 0 {
            log::info!("Generating Fulfillment Event: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Fulfillment Event.");
    Ok(())
}


async fn generate_inventory_lists<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Inventory List", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Inventory List: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Inventory List (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::inventory_lists().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_list_id(format!("{} {}", "string()", i + 1));

                entity.update_total_items(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Inventory List");

        if i % 20 == 0 {
            log::info!("Generating Inventory List: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Inventory List.");
    Ok(())
}


async fn generate_invoice_lines<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Invoice").is_empty() {
            state.record_skipped("Invoice Line", "Required dependency Invoice is missing in reference pool".to_string());
            log::info!("Skipped generating Invoice Line: Required dependency Invoice is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Invoice Line (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::invoice_lines().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Invoice", i as usize, &used_refs) {
                    entity.update_invoice_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_line_id(format!("{} {}", "string()", i + 1));

                entity.update_description(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Invoice Line");

        if i % 20 == 0 {
            log::info!("Generating Invoice Line: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Invoice Line.");
    Ok(())
}


async fn generate_journal_entries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Invoice").is_empty() {
            state.record_skipped("Journal Entry", "Required dependency Invoice is missing in reference pool".to_string());
            log::info!("Skipped generating Journal Entry: Required dependency Invoice is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Journal Entry (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::journal_entries().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Invoice", i as usize, &used_refs) {
                    entity.update_invoice_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_entry_id(format!("{} {}", "string()", i + 1));

                entity.update_date(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Journal Entry");

        if i % 20 == 0 {
            log::info!("Generating Journal Entry: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Journal Entry.");
    Ok(())
}


async fn generate_lead_activities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Lead").is_empty() {
            state.record_skipped("Lead Activity", "Required dependency Lead is missing in reference pool".to_string());
            log::info!("Skipped generating Lead Activity: Required dependency Lead is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Lead Activity (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::lead_activities().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Lead", i as usize, &used_refs) {
                    entity.update_lead_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_activity_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Lead Activity");

        if i % 20 == 0 {
            log::info!("Generating Lead Activity: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Lead Activity.");
    Ok(())
}


async fn generate_lead_scores<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Lead").is_empty() {
            state.record_skipped("Lead Score", "Required dependency Lead is missing in reference pool".to_string());
            log::info!("Skipped generating Lead Score: Required dependency Lead is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Lead Score (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::lead_scores().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Lead", i as usize, &used_refs) {
                    entity.update_lead_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_score_id(format!("{} {}", "string()", i + 1));

                entity.update_value(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Lead Score");

        if i % 20 == 0 {
            log::info!("Generating Lead Score: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Lead Score.");
    Ok(())
}


async fn generate_login_attempts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("Login Attempt", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating Login Attempt: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Login Attempt (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::login_attempts().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_attempt_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Login Attempt");

        if i % 20 == 0 {
            log::info!("Generating Login Attempt: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Login Attempt.");
    Ok(())
}


async fn generate_magic_links<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("Magic Link", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating Magic Link: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Magic Link (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::magic_links().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_link_id(format!("{} {}", "string()", i + 1));

                entity.update_expires(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Magic Link");

        if i % 20 == 0 {
            log::info!("Generating Magic Link: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Magic Link.");
    Ok(())
}


async fn generate_maintenance_costs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Maintenance Event").is_empty() {
            state.record_skipped("Maintenance Cost", "Required dependency Maintenance Event is missing in reference pool".to_string());
            log::info!("Skipped generating Maintenance Cost: Required dependency Maintenance Event is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Maintenance Cost (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::maintenance_costs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Maintenance Event", i as usize, &used_refs) {
                    entity.update_maintenance_event_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_cost_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Maintenance Cost");

        if i % 20 == 0 {
            log::info!("Generating Maintenance Cost: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Maintenance Cost.");
    Ok(())
}


async fn generate_move_items<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Move Item", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Move Item: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Move Item (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::move_items().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_item_name(format!("{} {}", "string()", i + 1));

                entity.update_weight_kg(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Move Item");

        if i % 20 == 0 {
            log::info!("Generating Move Item: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Move Item.");
    Ok(())
}


async fn generate_move_quotes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Move Quote", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Move Quote: Required dependency Move Order is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_quote_id(format!("{} {}", "string()", i + 1));

                entity.update_total(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Move Quote");

        if i % 20 == 0 {
            log::info!("Generating Move Quote: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Move Quote.");
    Ok(())
}


async fn generate_notifications<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("Notification", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating Notification: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Notification (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::notifications().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_notification_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Notification");

        if i % 20 == 0 {
            log::info!("Generating Notification: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Notification.");
    Ok(())
}


async fn generate_payroll_adjustments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payroll Calculation").is_empty() {
            state.record_skipped("Payroll Adjustment", "Required dependency Payroll Calculation is missing in reference pool".to_string());
            log::info!("Skipped generating Payroll Adjustment: Required dependency Payroll Calculation is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payroll Adjustment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payroll_adjustments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Payroll Calculation", i as usize, &used_refs) {
                    entity.update_payroll_calculation_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_adjustment_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payroll Adjustment");

        if i % 20 == 0 {
            log::info!("Generating Payroll Adjustment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Payroll Adjustment.");
    Ok(())
}


async fn generate_payslips<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payroll Calculation").is_empty() {
            state.record_skipped("Payslip", "Required dependency Payroll Calculation is missing in reference pool".to_string());
            log::info!("Skipped generating Payslip: Required dependency Payroll Calculation is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Payslip (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::payslips().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Payroll Calculation", i as usize, &used_refs) {
                    entity.update_payroll_calculation_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_payslip_id(format!("{} {}", "string()", i + 1));

                entity.update_net_pay(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Payslip");

        if i % 20 == 0 {
            log::info!("Generating Payslip: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Payslip.");
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
            state.record_skipped("Proof of Delivery", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Proof of Delivery: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Proof of Delivery (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::proof_of_deliveries().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_pod_id(format!("{} {}", "string()", i + 1));

                entity.update_signed_by(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Proof of Delivery");

        if i % 20 == 0 {
            log::info!("Generating Proof of Delivery: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Proof of Delivery.");
    Ok(())
}


async fn generate_receivables<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Invoice").is_empty() {
            state.record_skipped("Receivable", "Required dependency Invoice is missing in reference pool".to_string());
            log::info!("Skipped generating Receivable: Required dependency Invoice is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Receivable (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::receivables().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Invoice", i as usize, &used_refs) {
                    entity.update_invoice_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_receivable_id(format!("{} {}", "string()", i + 1));

                entity.update_amount(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Receivable");

        if i % 20 == 0 {
            log::info!("Generating Receivable: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Receivable.");
    Ok(())
}


async fn generate_recovery_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Document").is_empty() {
            state.record_skipped("Recovery Request", "Required dependency Document is missing in reference pool".to_string());
            log::info!("Skipped generating Recovery Request: Required dependency Document is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Recovery Request (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::recovery_requests().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Document", i as usize, &used_refs) {
                    entity.update_document_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_request_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Recovery Request");

        if i % 20 == 0 {
            log::info!("Generating Recovery Request: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Recovery Request.");
    Ok(())
}


async fn generate_routes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Route", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Route: Required dependency Move Order is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_route_id(format!("{} {}", "string()", i + 1));

                entity.update_distance_km(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Route");

        if i % 20 == 0 {
            log::info!("Generating Route: {}/{}", i, fanout);
        }

        state.add_reference("Route", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Route.");
    Ok(())
}


async fn generate_sales_funnels<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Lead").is_empty() {
            state.record_skipped("Sales Funnel", "Required dependency Lead is missing in reference pool".to_string());
            log::info!("Skipped generating Sales Funnel: Required dependency Lead is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Sales Funnel (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::sales_funnels().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Lead", i as usize, &used_refs) {
                    entity.update_lead_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_funnel_id(format!("{} {}", "string()", i + 1));

                entity.update_stage(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Sales Funnel");

        if i % 20 == 0 {
            log::info!("Generating Sales Funnel: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Sales Funnel.");
    Ok(())
}


async fn generate_sales_opportunities<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Lead").is_empty() {
            state.record_skipped("Sales Opportunity", "Required dependency Lead is missing in reference pool".to_string());
            log::info!("Skipped generating Sales Opportunity: Required dependency Lead is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Sales Opportunity (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::sales_opportunities().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Lead", i as usize, &used_refs) {
                    entity.update_lead_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_opp_id(format!("{} {}", "string()", i + 1));

                entity.update_value(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Sales Opportunity");

        if i % 20 == 0 {
            log::info!("Generating Sales Opportunity: {}/{}", i, fanout);
        }

        state.add_reference("Sales Opportunity", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Sales Opportunity.");
    Ok(())
}


async fn generate_special_handling_instructions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Special Handling Instruction", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Special Handling Instruction: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Special Handling Instruction (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::special_handling_instructions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_instruction(format!("{} {}", "string()", i + 1));

                entity.update_applies_to(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Special Handling Instruction");

        if i % 20 == 0 {
            log::info!("Generating Special Handling Instruction: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Special Handling Instruction.");
    Ok(())
}


async fn generate_synchronization_runs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Integration Mapping").is_empty() {
            state.record_skipped("Synchronization Run", "Required dependency Integration Mapping is missing in reference pool".to_string());
            log::info!("Skipped generating Synchronization Run: Required dependency Integration Mapping is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Synchronization Run (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::synchronization_runs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Integration Mapping", i as usize, &used_refs) {
                    entity.update_integration_mapping_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_run_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Synchronization Run");

        if i % 20 == 0 {
            log::info!("Generating Synchronization Run: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Synchronization Run.");
    Ok(())
}


async fn generate_tax_records<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Invoice").is_empty() {
            state.record_skipped("Tax Record", "Required dependency Invoice is missing in reference pool".to_string());
            log::info!("Skipped generating Tax Record: Required dependency Invoice is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Tax Record (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::tax_records().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Invoice", i as usize, &used_refs) {
                    entity.update_invoice_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_record_id(format!("{} {}", "string()", i + 1));

                entity.update_record_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Tax Record");

        if i % 20 == 0 {
            log::info!("Generating Tax Record: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Tax Record.");
    Ok(())
}


async fn generate_tax_withholdings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Payroll Calculation").is_empty() {
            state.record_skipped("Tax Withholding", "Required dependency Payroll Calculation is missing in reference pool".to_string());
            log::info!("Skipped generating Tax Withholding: Required dependency Payroll Calculation is missing in reference pool.");
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

                if let Some(ref_id) = state.pick_unused_id("Payroll Calculation", i as usize, &used_refs) {
                    entity.update_payroll_calculation_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_withholding_id(format!("{} {}", "string()", i + 1));

                entity.update_rate(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Tax Withholding");

        if i % 20 == 0 {
            log::info!("Generating Tax Withholding: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Tax Withholding.");
    Ok(())
}


async fn generate_time_slots<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Time Slot", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Time Slot: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Time Slot (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::time_slots().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_start(format!("{} {}", "string()", i + 1));

                entity.update_end(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Time Slot");

        if i % 20 == 0 {
            log::info!("Generating Time Slot: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Time Slot.");
    Ok(())
}


async fn generate_transit_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Move Order").is_empty() {
            state.record_skipped("Transit Log", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Transit Log: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Transit Log (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::transit_logs().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_log_id(format!("{} {}", "string()", i + 1));

                entity.update_mileage(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Transit Log");

        if i % 20 == 0 {
            log::info!("Generating Transit Log: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Transit Log.");
    Ok(())
}


async fn generate_two_factor_auths<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("Two Factor Auth", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating Two Factor Auth: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Two Factor Auth (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::two_factor_auths().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_auth_id(format!("{} {}", "string()", i + 1));

                entity.update_method(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Two Factor Auth");

        if i % 20 == 0 {
            log::info!("Generating Two Factor Auth: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Two Factor Auth.");
    Ok(())
}


async fn generate_user_role_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("User Role Assignment", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating User Role Assignment: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for User Role Assignment (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::user_role_assignments().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_assignment_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("User Role Assignment");

        if i % 20 == 0 {
            log::info!("Generating User Role Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for User Role Assignment.");
    Ok(())
}


async fn generate_user_sessions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("User Account").is_empty() {
            state.record_skipped("User Session", "Required dependency User Account is missing in reference pool".to_string());
            log::info!("Skipped generating User Session: Required dependency User Account is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for User Session (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::user_sessions().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("User Account", i as usize, &used_refs) {
                    entity.update_user_account_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_session_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("User Session");

        if i % 20 == 0 {
            log::info!("Generating User Session: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for User Session.");
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

        if state.ids("Move Order").is_empty() {
            state.record_skipped("Vehicle Assignment", "Required dependency Move Order is missing in reference pool".to_string());
            log::info!("Skipped generating Vehicle Assignment: Required dependency Move Order is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1 + 1;
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
                    entity.update_crew_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                if let Some(ref_id) = state.pick_unused_id("Move Order", i as usize, &used_refs) {
                    entity.update_move_order_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_assignment_id(format!("{} {}", "string()", i + 1));

                entity.update_vehicle_type(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Vehicle Assignment");

        if i % 20 == 0 {
            log::info!("Generating Vehicle Assignment: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Vehicle Assignment.");
    Ok(())
}


async fn generate_webhook_deliveries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Webhook").is_empty() {
            state.record_skipped("Webhook Delivery", "Required dependency Webhook is missing in reference pool".to_string());
            log::info!("Skipped generating Webhook Delivery: Required dependency Webhook is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Webhook Delivery (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::webhook_deliveries().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Webhook", i as usize, &used_refs) {
                    entity.update_webhook_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_delivery_id(format!("{} {}", "string()", i + 1));

                entity.update_status(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Webhook Delivery");

        if i % 20 == 0 {
            log::info!("Generating Webhook Delivery: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Webhook Delivery.");
    Ok(())
}


async fn generate_worked_hourses<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Work Shift").is_empty() {
            state.record_skipped("Worked Hours", "Required dependency Work Shift is missing in reference pool".to_string());
            log::info!("Skipped generating Worked Hours: Required dependency Work Shift is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Worked Hours (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::worked_hourses().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Work Shift", i as usize, &used_refs) {
                    entity.update_work_shift_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_hours(format!("{} {}", "string()", i + 1));

                entity.update_date(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Worked Hours");

        if i % 20 == 0 {
            log::info!("Generating Worked Hours: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Worked Hours.");
    Ok(())
}


async fn generate_conversion_events<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Sales Opportunity").is_empty() {
            state.record_skipped("Conversion Event", "Required dependency Sales Opportunity is missing in reference pool".to_string());
            log::info!("Skipped generating Conversion Event: Required dependency Sales Opportunity is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Conversion Event (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::conversion_events().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Sales Opportunity", i as usize, &used_refs) {
                    entity.update_sales_opportunity_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_event_id(format!("{} {}", "string()", i + 1));

                entity.update_timestamp(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Conversion Event");

        if i % 20 == 0 {
            log::info!("Generating Conversion Event: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Conversion Event.");
    Ok(())
}


async fn generate_entity_changes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Activity Log").is_empty() {
            state.record_skipped("Entity Change", "Required dependency Activity Log is missing in reference pool".to_string());
            log::info!("Skipped generating Entity Change: Required dependency Activity Log is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Entity Change (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::entity_changes().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Activity Log", i as usize, &used_refs) {
                    entity.update_activity_log_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_change_id(format!("{} {}", "string()", i + 1));

                entity.update_field(format!("{} {}", "string()", i + 1));



        let entity = entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Entity Change");

        if i % 20 == 0 {
            log::info!("Generating Entity Change: {}/{}", i, fanout);
        }

        state.add_reference("Entity Change", entity.id().into_u64());
    }

    log::info!("Successfully generated sample records for Entity Change.");
    Ok(())
}


async fn generate_route_stops<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Route").is_empty() {
            state.record_skipped("Route Stop", "Required dependency Route is missing in reference pool".to_string());
            log::info!("Skipped generating Route Stop: Required dependency Route is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
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

                if let Some(ref_id) = state.pick_unused_id("Route", i as usize, &used_refs) {
                    entity.update_route_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_stop_id(format!("{} {}", "string()", i + 1));

                entity.update_sequence(format!("{} {}", "string()", i + 1));



entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Route Stop");

        if i % 20 == 0 {
            log::info!("Generating Route Stop: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Route Stop.");
    Ok(())
}


async fn generate_change_sets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
        if state.ids("Entity Change").is_empty() {
            state.record_skipped("Change Set", "Required dependency Entity Change is missing in reference pool".to_string());
            log::info!("Skipped generating Change Set: Required dependency Entity Change is missing in reference pool.");
            return Ok(());
        }


    let object_fields_count = 0 + 1;
    let base_fanout = std::cmp::max(1, object_fields_count) * 20;

    let fanout = match state.plan.scale {
        SampleDataScale::Tiny => base_fanout,
        SampleDataScale::Small => base_fanout * 5,
        SampleDataScale::Medium => base_fanout * 50,
    };

    log::info!("Generating sample data for Change Set (expected: {})...", fanout);

    for i in 0..fanout {
        let mut entity = Q::change_sets().purpose("Init Sample Data").new_entity(ctx);
        let mut used_refs = std::collections::HashSet::new();

                if let Some(ref_id) = state.pick_unused_id("Entity Change", i as usize, &used_refs) {
                    entity.update_entity_change_ref_id(ref_id);
                    used_refs.insert(ref_id);
                } else {
                    // Optional relation was missing in reference pool
                }
                entity.update_set_id(format!("{} {}", "string()", i + 1));

                entity.update_version(format!("{} {}", "string()", i + 1));


entity.audit_as("Init Sample Data").save(ctx).await.map_err(|e| e.to_string())?;

        state.record_generated("Change Set");

        if i % 20 == 0 {
            log::info!("Generating Change Set: {}/{}", i, fanout);
        }

    }

    log::info!("Successfully generated sample records for Change Set.");
    Ok(())
}
