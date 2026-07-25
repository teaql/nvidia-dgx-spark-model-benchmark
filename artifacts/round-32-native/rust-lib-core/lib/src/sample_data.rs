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

    load_root_access_tokens(ctx, &mut state).await?; //depth: 0
    load_root_accounts(ctx, &mut state).await?; //depth: 0
    load_root_activity_logs(ctx, &mut state).await?; //depth: 0
    load_root_ad_spends(ctx, &mut state).await?; //depth: 0
    load_root_addresses(ctx, &mut state).await?; //depth: 0
    load_root_api_clients(ctx, &mut state).await?; //depth: 0
    load_root_api_endpoints(ctx, &mut state).await?; //depth: 0
    load_root_api_rate_limits(ctx, &mut state).await?; //depth: 0
    load_root_asset_assignments(ctx, &mut state).await?; //depth: 0
    load_root_asset_inspections(ctx, &mut state).await?; //depth: 0
    load_root_audit_adjustments(ctx, &mut state).await?; //depth: 0
    load_root_audit_logs(ctx, &mut state).await?; //depth: 0
    load_root_automation_actions(ctx, &mut state).await?; //depth: 0
    load_root_automation_rules(ctx, &mut state).await?; //depth: 0
    load_root_automation_triggers(ctx, &mut state).await?; //depth: 0
    load_root_background_checks(ctx, &mut state).await?; //depth: 0
    load_root_bank_transactions(ctx, &mut state).await?; //depth: 0
    load_root_billing_profiles(ctx, &mut state).await?; //depth: 0
    load_root_bonuses(ctx, &mut state).await?; //depth: 0
    load_root_box_rentals(ctx, &mut state).await?; //depth: 0
    load_root_branches(ctx, &mut state).await?; //depth: 0
    load_root_campaigns(ctx, &mut state).await?; //depth: 0
    load_root_change_sets(ctx, &mut state).await?; //depth: 0
    load_root_chargeback_records(ctx, &mut state).await?; //depth: 0
    load_root_cleaning_services(ctx, &mut state).await?; //depth: 0
    load_root_communication_logs(ctx, &mut state).await?; //depth: 0
    load_root_competitor_analyses(ctx, &mut state).await?; //depth: 0
    load_root_complaint_tickets(ctx, &mut state).await?; //depth: 0
    load_root_compliance_checks(ctx, &mut state).await?; //depth: 0
    load_root_consumables(ctx, &mut state).await?; //depth: 0
    load_root_contracts(ctx, &mut state).await?; //depth: 0
    load_root_conversion_events(ctx, &mut state).await?; //depth: 0
    load_root_conversion_metrics(ctx, &mut state).await?; //depth: 0
    load_root_cookie_consents(ctx, &mut state).await?; //depth: 0
    load_root_corporate_customer_profiles(ctx, &mut state).await?; //depth: 0
    load_root_credit_notes(ctx, &mut state).await?; //depth: 0
    load_root_crews(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_180s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_181s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_182s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_183s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_184s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_185s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_186s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_187s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_188s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_189s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_190s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_191s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_192s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_193s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_194s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_195s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_196s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_197s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_198s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_199s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_200s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_201s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_202s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_203s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_204s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_205s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_206s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_207s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_208s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_209s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_210s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_211s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_212s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_213s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_214s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_215s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_216s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_217s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_218s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_219s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_220s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_221s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_222s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_223s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_224s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_225s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_226s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_227s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_228s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_229s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_230s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_231s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_232s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_233s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_234s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_235s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_236s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_237s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_238s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_239s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_240s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_241s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_242s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_243s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_244s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_245s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_246s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_247s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_248s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_249s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_250s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_251s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_252s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_253s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_254s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_255s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_256s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_257s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_258s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_259s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_260s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_261s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_262s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_263s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_264s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_265s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_266s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_267s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_268s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_269s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_270s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_271s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_272s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_273s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_274s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_275s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_276s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_277s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_278s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_279s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_280s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_281s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_282s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_283s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_284s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_285s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_286s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_287s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_288s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_289s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_290s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_291s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_292s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_293s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_294s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_295s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_296s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_297s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_298s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_299s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_300s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_301s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_302s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_303s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_304s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_305s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_306s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_307s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_308s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_309s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_310s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_311s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_312s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_313s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_314s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_315s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_316s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_317s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_318s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_319s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_320s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_321s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_322s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_323s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_324s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_325s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_326s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_327s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_328s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_329s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_330s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_331s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_332s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_333s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_334s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_335s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_336s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_337s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_338s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_339s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_340s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_341s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_342s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_343s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_344s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_345s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_346s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_347s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_348s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_349s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_350s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_351s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_352s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_353s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_354s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_355s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_356s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_357s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_358s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_359s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_360s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_361s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_362s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_363s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_364s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_365s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_366s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_367s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_368s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_369s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_370s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_371s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_372s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_373s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_374s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_375s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_376s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_377s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_378s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_379s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_380s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_381s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_382s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_383s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_384s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_385s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_386s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_387s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_388s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_389s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_390s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_391s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_392s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_393s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_394s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_395s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_396s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_397s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_398s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_399s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_400s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_401s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_402s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_403s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_404s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_405s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_406s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_407s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_408s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_409s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_410s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_411s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_412s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_413s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_414s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_415s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_416s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_417s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_418s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_419s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_420s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_421s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_422s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_423s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_424s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_425s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_426s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_427s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_428s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_429s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_430s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_431s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_432s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_433s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_434s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_435s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_436s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_437s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_438s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_439s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_440s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_441s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_442s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_443s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_444s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_445s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_446s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_447s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_448s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_449s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_450s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_451s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_452s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_453s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_454s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_455s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_456s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_457s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_458s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_459s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_460s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_461s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_462s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_463s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_464s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_465s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_466s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_467s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_468s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_469s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_470s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_471s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_472s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_473s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_474s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_475s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_476s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_477s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_478s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_479s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_480s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_481s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_482s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_483s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_484s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_485s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_486s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_487s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_488s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_489s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_490s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_491s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_492s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_493s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_494s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_495s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_496s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_497s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_498s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_499s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_500s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_501s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_502s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_503s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_504s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_505s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_506s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_507s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_508s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_509s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_510s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_511s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_512s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_513s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_514s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_515s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_516s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_517s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_518s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_519s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_520s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_521s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_522s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_523s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_524s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_525s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_526s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_527s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_528s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_529s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_530s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_531s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_532s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_533s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_534s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_535s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_536s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_537s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_538s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_539s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_540s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_541s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_542s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_543s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_544s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_545s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_546s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_547s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_548s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_549s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_550s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_551s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_552s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_553s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_554s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_555s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_556s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_557s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_558s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_559s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_560s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_561s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_562s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_563s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_564s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_565s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_566s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_567s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_568s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_569s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_570s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_571s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_572s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_573s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_574s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_575s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_576s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_577s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_578s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_579s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_580s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_581s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_582s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_583s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_584s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_585s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_586s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_587s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_588s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_589s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_590s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_591s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_592s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_593s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_594s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_595s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_596s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_597s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_598s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_599s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_600s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_601s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_602s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_603s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_604s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_605s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_606s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_607s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_608s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_609s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_610s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_611s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_612s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_613s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_614s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_615s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_616s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_617s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_618s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_619s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_620s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_621s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_622s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_623s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_624s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_625s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_626s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_627s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_628s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_629s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_630s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_631s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_632s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_633s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_634s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_635s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_636s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_637s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_638s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_639s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_640s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_641s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_642s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_643s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_644s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_645s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_646s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_647s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_648s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_649s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_650s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_651s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_652s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_653s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_654s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_655s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_656s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_657s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_658s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_659s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_660s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_661s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_662s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_663s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_664s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_665s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_666s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_667s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_668s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_669s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_670s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_671s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_672s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_673s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_674s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_675s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_676s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_677s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_678s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_679s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_680s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_681s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_682s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_683s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_684s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_685s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_686s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_687s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_688s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_689s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_690s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_691s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_692s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_693s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_694s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_695s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_696s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_697s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_698s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_699s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_700s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_701s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_702s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_703s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_704s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_705s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_706s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_707s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_708s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_709s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_710s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_711s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_712s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_713s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_714s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_715s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_716s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_717s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_718s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_719s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_720s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_721s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_722s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_723s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_724s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_725s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_726s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_727s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_728s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_729s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_730s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_731s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_732s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_733s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_734s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_735s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_736s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_737s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_738s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_739s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_740s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_741s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_742s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_743s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_744s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_745s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_746s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_747s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_748s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_749s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_750s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_751s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_752s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_753s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_754s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_755s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_756s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_757s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_758s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_759s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_760s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_761s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_762s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_763s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_764s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_765s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_766s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_767s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_768s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_769s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_770s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_771s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_772s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_773s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_774s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_775s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_776s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_777s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_778s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_779s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_780s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_781s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_782s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_783s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_784s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_785s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_786s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_787s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_788s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_789s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_790s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_791s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_792s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_793s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_794s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_795s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_796s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_797s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_798s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_799s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_800s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_801s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_802s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_803s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_804s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_805s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_806s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_807s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_808s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_809s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_810s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_811s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_812s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_813s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_814s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_815s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_816s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_817s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_818s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_819s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_820s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_821s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_822s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_823s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_824s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_825s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_826s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_827s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_828s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_829s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_830s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_831s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_832s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_833s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_834s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_835s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_836s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_837s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_838s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_839s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_840s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_841s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_842s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_843s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_844s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_845s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_846s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_847s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_848s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_849s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_850s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_851s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_852s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_853s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_854s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_855s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_856s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_857s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_858s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_859s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_860s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_861s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_862s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_863s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_864s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_865s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_866s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_867s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_868s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_869s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_870s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_871s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_872s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_873s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_874s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_875s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_876s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_877s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_878s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_879s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_880s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_881s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_882s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_883s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_884s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_885s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_886s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_887s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_888s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_889s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_890s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_891s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_892s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_893s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_894s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_895s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_896s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_897s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_898s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_899s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_900s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_901s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_902s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_903s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_904s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_905s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_906s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_907s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_908s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_909s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_910s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_911s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_912s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_913s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_914s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_915s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_916s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_917s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_918s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_919s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_920s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_921s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_922s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_923s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_924s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_925s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_926s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_927s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_928s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_929s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_930s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_931s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_932s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_933s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_934s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_935s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_936s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_937s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_938s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_939s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_940s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_941s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_942s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_943s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_944s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_945s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_946s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_947s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_948s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_949s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_950s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_951s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_952s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_953s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_954s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_955s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_956s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_957s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_958s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_959s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_960s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_961s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_962s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_963s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_964s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_965s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_966s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_967s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_968s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_969s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_970s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_971s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_972s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_973s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_974s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_975s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_976s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_977s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_978s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_979s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_980s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_981s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_982s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_983s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_984s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_985s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_986s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_987s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_988s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_989s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_990s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_991s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_992s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_993s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_994s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_995s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_996s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_997s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_998s(ctx, &mut state).await?; //depth: 0
    load_root_custom_entity_999s(ctx, &mut state).await?; //depth: 0
    load_root_customers(ctx, &mut state).await?; //depth: 0
    load_root_customer_consents(ctx, &mut state).await?; //depth: 0
    load_root_customer_contacts(ctx, &mut state).await?; //depth: 0
    load_root_customer_histories(ctx, &mut state).await?; //depth: 0
    load_root_customer_notes(ctx, &mut state).await?; //depth: 0
    load_root_customer_preferences(ctx, &mut state).await?; //depth: 0
    load_root_customer_signatures(ctx, &mut state).await?; //depth: 0
    load_root_damage_reports(ctx, &mut state).await?; //depth: 0
    load_root_dashcam_footages(ctx, &mut state).await?; //depth: 0
    load_root_data_retention_policies(ctx, &mut state).await?; //depth: 0
    load_root_debit_notes(ctx, &mut state).await?; //depth: 0
    load_root_departments(ctx, &mut state).await?; //depth: 0
    load_root_depreciation_schedules(ctx, &mut state).await?; //depth: 0
    load_root_detour_logs(ctx, &mut state).await?; //depth: 0
    load_root_direct_deposit_info(ctx, &mut state).await?; //depth: 0
    load_root_discount_codes(ctx, &mut state).await?; //depth: 0
    load_root_dispatch_assignments(ctx, &mut state).await?; //depth: 0
    load_root_do_not_contact_lists(ctx, &mut state).await?; //depth: 0
    load_root_documents(ctx, &mut state).await?; //depth: 0
    load_root_document_versions(ctx, &mut state).await?; //depth: 0
    load_root_email_blasts(ctx, &mut state).await?; //depth: 0
    load_root_email_bounce_logs(ctx, &mut state).await?; //depth: 0
    load_root_emergency_contacts(ctx, &mut state).await?; //depth: 0
    load_root_employees(ctx, &mut state).await?; //depth: 0
    load_root_employee_certifications(ctx, &mut state).await?; //depth: 0
    load_root_entity_changes(ctx, &mut state).await?; //depth: 0
    load_root_equipment(ctx, &mut state).await?; //depth: 0
    load_root_expenses(ctx, &mut state).await?; //depth: 0
    load_root_expense_reimbursements(ctx, &mut state).await?; //depth: 0
    load_root_failed_auth_logs(ctx, &mut state).await?; //depth: 0
    load_root_financial_summaries(ctx, &mut state).await?; //depth: 0
    load_root_fiscal_years(ctx, &mut state).await?; //depth: 0
    load_root_franchises(ctx, &mut state).await?; //depth: 0
    load_root_fuel_records(ctx, &mut state).await?; //depth: 0
    load_root_fuel_stops(ctx, &mut state).await?; //depth: 0
    load_root_fulfillment_events(ctx, &mut state).await?; //depth: 0
    load_root_gdpr_requests(ctx, &mut state).await?; //depth: 0
    load_root_gps_trackers(ctx, &mut state).await?; //depth: 0
    load_root_hoisting_services(ctx, &mut state).await?; //depth: 0
    load_root_insurance_addons(ctx, &mut state).await?; //depth: 0
    load_root_insurance_cards(ctx, &mut state).await?; //depth: 0
    load_root_insurance_claims(ctx, &mut state).await?; //depth: 0
    load_root_insurance_policies(ctx, &mut state).await?; //depth: 0
    load_root_integration_mappings(ctx, &mut state).await?; //depth: 0
    load_root_inventory_items(ctx, &mut state).await?; //depth: 0
    load_root_invoices(ctx, &mut state).await?; //depth: 0
    load_root_invoice_lines(ctx, &mut state).await?; //depth: 0
    load_root_job_assignments(ctx, &mut state).await?; //depth: 0
    load_root_journal_entries(ctx, &mut state).await?; //depth: 0
    load_root_leads(ctx, &mut state).await?; //depth: 0
    load_root_lead_activities(ctx, &mut state).await?; //depth: 0
    load_root_leave_requests(ctx, &mut state).await?; //depth: 0
    load_root_login_attempts(ctx, &mut state).await?; //depth: 0
    load_root_long_carry_fees(ctx, &mut state).await?; //depth: 0
    load_root_loyalty_tiers(ctx, &mut state).await?; //depth: 0
    load_root_magic_links(ctx, &mut state).await?; //depth: 0
    load_root_maintenance_events(ctx, &mut state).await?; //depth: 0
    load_root_maintenance_schedules(ctx, &mut state).await?; //depth: 0
    load_root_merchants(ctx, &mut state).await?; //depth: 0
    load_root_merchant_fees(ctx, &mut state).await?; //depth: 0
    load_root_move_orders(ctx, &mut state).await?; //depth: 0
    load_root_move_quotes(ctx, &mut state).await?; //depth: 0
    load_root_moving_services(ctx, &mut state).await?; //depth: 0
    load_root_nda_agreements(ctx, &mut state).await?; //depth: 0
    load_root_notifications(ctx, &mut state).await?; //depth: 0
    load_root_notification_templates(ctx, &mut state).await?; //depth: 0
    load_root_objection_handling_guides(ctx, &mut state).await?; //depth: 0
    load_root_oil_change_logs(ctx, &mut state).await?; //depth: 0
    load_root_operations_manager_overrides(ctx, &mut state).await?; //depth: 0
    load_root_osha_incidents(ctx, &mut state).await?; //depth: 0
    load_root_overtime_approvals(ctx, &mut state).await?; //depth: 0
    load_root_packing_lists(ctx, &mut state).await?; //depth: 0
    load_root_packing_materials(ctx, &mut state).await?; //depth: 0
    load_root_parking_permits(ctx, &mut state).await?; //depth: 0
    load_root_password_resets(ctx, &mut state).await?; //depth: 0
    load_root_payments(ctx, &mut state).await?; //depth: 0
    load_root_payroll_calculations(ctx, &mut state).await?; //depth: 0
    load_root_payroll_periods(ctx, &mut state).await?; //depth: 0
    load_root_payslips(ctx, &mut state).await?; //depth: 0
    load_root_performance_reviews(ctx, &mut state).await?; //depth: 0
    load_root_permissions(ctx, &mut state).await?; //depth: 0
    load_root_pet_relocation_services(ctx, &mut state).await?; //depth: 0
    load_root_piano_handlings(ctx, &mut state).await?; //depth: 0
    load_root_platforms(ctx, &mut state).await?; //depth: 0
    load_root_platform_configs(ctx, &mut state).await?; //depth: 0
    load_root_post_move_surveys(ctx, &mut state).await?; //depth: 0
    load_root_price_lists(ctx, &mut state).await?; //depth: 0
    load_root_privacy_policies(ctx, &mut state).await?; //depth: 0
    load_root_private_customer_profiles(ctx, &mut state).await?; //depth: 0
    load_root_products(ctx, &mut state).await?; //depth: 0
    load_root_proof_of_deliveries(ctx, &mut state).await?; //depth: 0
    load_root_recovery_requests(ctx, &mut state).await?; //depth: 0
    load_root_referral_codes(ctx, &mut state).await?; //depth: 0
    load_root_refunds(ctx, &mut state).await?; //depth: 0
    load_root_registration_renewals(ctx, &mut state).await?; //depth: 0
    load_root_resolution_offers(ctx, &mut state).await?; //depth: 0
    load_root_roles(ctx, &mut state).await?; //depth: 0
    load_root_role_permissions(ctx, &mut state).await?; //depth: 0
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
    load_root_sms_delivery_receipts(ctx, &mut state).await?; //depth: 0
    load_root_social_media_posts(ctx, &mut state).await?; //depth: 0
    load_root_stair_fees(ctx, &mut state).await?; //depth: 0
    load_root_storage_units(ctx, &mut state).await?; //depth: 0
    load_root_suppliers(ctx, &mut state).await?; //depth: 0
    load_root_sync_jobs(ctx, &mut state).await?; //depth: 0
    load_root_tax_documents(ctx, &mut state).await?; //depth: 0
    load_root_tax_withholdings(ctx, &mut state).await?; //depth: 0
    load_root_tenant_registries(ctx, &mut state).await?; //depth: 0
    load_root_termination_records(ctx, &mut state).await?; //depth: 0
    load_root_terms_of_services(ctx, &mut state).await?; //depth: 0
    load_root_time_slots(ctx, &mut state).await?; //depth: 0
    load_root_tire_replacements(ctx, &mut state).await?; //depth: 0
    load_root_toll_receipts(ctx, &mut state).await?; //depth: 0
    load_root_traffic_violations(ctx, &mut state).await?; //depth: 0
    load_root_two_factor_auths(ctx, &mut state).await?; //depth: 0
    load_root_uniform_assignments(ctx, &mut state).await?; //depth: 0
    load_root_union_dueses(ctx, &mut state).await?; //depth: 0
    load_root_user_accounts(ctx, &mut state).await?; //depth: 0
    load_root_user_role_assignments(ctx, &mut state).await?; //depth: 0
    load_root_user_sessions(ctx, &mut state).await?; //depth: 0
    load_root_vat_rates(ctx, &mut state).await?; //depth: 0
    load_root_vehicles(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_load_plans(ctx, &mut state).await?; //depth: 0
    load_root_vehicle_transports(ctx, &mut state).await?; //depth: 0
    load_root_vip_statuses(ctx, &mut state).await?; //depth: 0
    load_root_walkthrough_checklists(ctx, &mut state).await?; //depth: 0
    load_root_warning_letters(ctx, &mut state).await?; //depth: 0
    load_root_weather_delays(ctx, &mut state).await?; //depth: 0
    load_root_webhooks(ctx, &mut state).await?; //depth: 0
    load_root_webhook_deliveries(ctx, &mut state).await?; //depth: 0
    load_root_weigh_station_tickets(ctx, &mut state).await?; //depth: 0
    load_root_work_shifts(ctx, &mut state).await?; //depth: 0
    load_root_worked_hourses(ctx, &mut state).await?; //depth: 0



    let report = state.into_report();
    log::info!("Sample data generation completed successfully. Generated: {} tables, Skipped: {} tables.", report.generated.len(), report.skipped.len());
    Ok(report)
}

async fn load_root_access_tokens<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::access_tokens().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("access_token", item.id().into_u64());
    }
    Ok(())
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
        state.add_reference("account", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_activity_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::activity_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("activity_log", item.id().into_u64());
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
        state.add_reference("ad_spend", item.id().into_u64());
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
        state.add_reference("address", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_api_clients<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::api_clients().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("api_client", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_api_endpoints<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::api_endpoints().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("api_endpoint", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_api_rate_limits<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::api_rate_limits().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("api_rate_limit", item.id().into_u64());
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
        state.add_reference("asset_assignment", item.id().into_u64());
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
        state.add_reference("asset_inspection", item.id().into_u64());
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
        state.add_reference("audit_adjustment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_audit_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::audit_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("audit_log", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_automation_actions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::automation_actions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("automation_action", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_automation_rules<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::automation_rules().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("automation_rule", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_automation_triggers<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::automation_triggers().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("automation_trigger", item.id().into_u64());
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
        state.add_reference("background_check", item.id().into_u64());
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
        state.add_reference("bank_transaction", item.id().into_u64());
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
        state.add_reference("billing_profile", item.id().into_u64());
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
        state.add_reference("bonus", item.id().into_u64());
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
        state.add_reference("box_rental", item.id().into_u64());
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
        state.add_reference("branch", item.id().into_u64());
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
        state.add_reference("campaign", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_change_sets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::change_sets().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("change_set", item.id().into_u64());
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
        state.add_reference("chargeback_record", item.id().into_u64());
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
        state.add_reference("cleaning_service", item.id().into_u64());
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
        state.add_reference("communication_log", item.id().into_u64());
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
        state.add_reference("competitor_analysis", item.id().into_u64());
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
        state.add_reference("complaint_ticket", item.id().into_u64());
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
        state.add_reference("compliance_check", item.id().into_u64());
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
        state.add_reference("consumable", item.id().into_u64());
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
        state.add_reference("contract", item.id().into_u64());
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
        state.add_reference("conversion_event", item.id().into_u64());
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
        state.add_reference("conversion_metric", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_cookie_consents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::cookie_consents().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("cookie_consent", item.id().into_u64());
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
        state.add_reference("corporate_customer_profile", item.id().into_u64());
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
        state.add_reference("credit_note", item.id().into_u64());
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
        state.add_reference("crew", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_180s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_180s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_180", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_181s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_181s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_181", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_182s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_182s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_182", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_183s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_183s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_183", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_184s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_184s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_184", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_185s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_185s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_185", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_186s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_186s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_186", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_187s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_187s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_187", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_188s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_188s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_188", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_189s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_189s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_189", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_190s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_190s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_190", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_191s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_191s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_191", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_192s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_192s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_192", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_193s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_193s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_193", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_194s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_194s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_194", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_195s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_195s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_195", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_196s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_196s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_196", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_197s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_197s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_197", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_198s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_198s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_198", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_199s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_199s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_199", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_200s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_200s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_200", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_201s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_201s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_201", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_202s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_202s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_202", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_203s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_203s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_203", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_204s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_204s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_204", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_205s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_205s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_205", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_206s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_206s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_206", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_207s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_207s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_207", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_208s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_208s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_208", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_209s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_209s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_209", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_210s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_210s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_210", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_211s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_211s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_211", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_212s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_212s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_212", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_213s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_213s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_213", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_214s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_214s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_214", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_215s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_215s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_215", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_216s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_216s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_216", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_217s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_217s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_217", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_218s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_218s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_218", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_219s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_219s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_219", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_220s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_220s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_220", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_221s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_221s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_221", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_222s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_222s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_222", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_223s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_223s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_223", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_224s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_224s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_224", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_225s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_225s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_225", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_226s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_226s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_226", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_227s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_227s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_227", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_228s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_228s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_228", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_229s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_229s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_229", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_230s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_230s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_230", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_231s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_231s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_231", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_232s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_232s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_232", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_233s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_233s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_233", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_234s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_234s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_234", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_235s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_235s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_235", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_236s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_236s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_236", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_237s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_237s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_237", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_238s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_238s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_238", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_239s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_239s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_239", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_240s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_240s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_240", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_241s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_241s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_241", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_242s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_242s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_242", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_243s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_243s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_243", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_244s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_244s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_244", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_245s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_245s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_245", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_246s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_246s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_246", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_247s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_247s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_247", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_248s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_248s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_248", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_249s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_249s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_249", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_250s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_250s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_250", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_251s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_251s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_251", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_252s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_252s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_252", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_253s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_253s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_253", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_254s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_254s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_254", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_255s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_255s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_255", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_256s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_256s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_256", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_257s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_257s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_257", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_258s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_258s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_258", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_259s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_259s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_259", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_260s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_260s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_260", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_261s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_261s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_261", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_262s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_262s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_262", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_263s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_263s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_263", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_264s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_264s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_264", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_265s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_265s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_265", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_266s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_266s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_266", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_267s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_267s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_267", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_268s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_268s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_268", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_269s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_269s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_269", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_270s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_270s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_270", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_271s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_271s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_271", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_272s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_272s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_272", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_273s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_273s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_273", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_274s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_274s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_274", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_275s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_275s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_275", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_276s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_276s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_276", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_277s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_277s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_277", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_278s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_278s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_278", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_279s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_279s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_279", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_280s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_280s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_280", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_281s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_281s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_281", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_282s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_282s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_282", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_283s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_283s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_283", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_284s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_284s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_284", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_285s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_285s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_285", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_286s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_286s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_286", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_287s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_287s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_287", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_288s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_288s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_288", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_289s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_289s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_289", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_290s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_290s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_290", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_291s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_291s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_291", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_292s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_292s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_292", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_293s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_293s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_293", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_294s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_294s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_294", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_295s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_295s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_295", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_296s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_296s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_296", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_297s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_297s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_297", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_298s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_298s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_298", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_299s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_299s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_299", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_300s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_300s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_300", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_301s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_301s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_301", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_302s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_302s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_302", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_303s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_303s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_303", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_304s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_304s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_304", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_305s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_305s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_305", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_306s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_306s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_306", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_307s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_307s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_307", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_308s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_308s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_308", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_309s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_309s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_309", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_310s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_310s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_310", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_311s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_311s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_311", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_312s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_312s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_312", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_313s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_313s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_313", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_314s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_314s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_314", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_315s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_315s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_315", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_316s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_316s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_316", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_317s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_317s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_317", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_318s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_318s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_318", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_319s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_319s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_319", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_320s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_320s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_320", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_321s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_321s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_321", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_322s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_322s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_322", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_323s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_323s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_323", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_324s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_324s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_324", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_325s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_325s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_325", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_326s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_326s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_326", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_327s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_327s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_327", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_328s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_328s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_328", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_329s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_329s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_329", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_330s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_330s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_330", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_331s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_331s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_331", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_332s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_332s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_332", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_333s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_333s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_333", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_334s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_334s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_334", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_335s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_335s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_335", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_336s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_336s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_336", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_337s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_337s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_337", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_338s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_338s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_338", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_339s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_339s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_339", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_340s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_340s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_340", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_341s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_341s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_341", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_342s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_342s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_342", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_343s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_343s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_343", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_344s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_344s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_344", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_345s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_345s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_345", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_346s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_346s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_346", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_347s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_347s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_347", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_348s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_348s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_348", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_349s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_349s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_349", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_350s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_350s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_350", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_351s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_351s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_351", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_352s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_352s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_352", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_353s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_353s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_353", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_354s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_354s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_354", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_355s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_355s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_355", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_356s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_356s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_356", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_357s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_357s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_357", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_358s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_358s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_358", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_359s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_359s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_359", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_360s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_360s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_360", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_361s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_361s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_361", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_362s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_362s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_362", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_363s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_363s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_363", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_364s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_364s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_364", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_365s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_365s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_365", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_366s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_366s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_366", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_367s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_367s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_367", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_368s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_368s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_368", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_369s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_369s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_369", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_370s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_370s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_370", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_371s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_371s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_371", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_372s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_372s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_372", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_373s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_373s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_373", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_374s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_374s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_374", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_375s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_375s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_375", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_376s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_376s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_376", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_377s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_377s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_377", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_378s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_378s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_378", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_379s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_379s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_379", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_380s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_380s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_380", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_381s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_381s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_381", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_382s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_382s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_382", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_383s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_383s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_383", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_384s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_384s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_384", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_385s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_385s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_385", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_386s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_386s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_386", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_387s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_387s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_387", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_388s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_388s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_388", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_389s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_389s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_389", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_390s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_390s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_390", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_391s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_391s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_391", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_392s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_392s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_392", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_393s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_393s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_393", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_394s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_394s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_394", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_395s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_395s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_395", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_396s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_396s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_396", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_397s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_397s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_397", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_398s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_398s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_398", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_399s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_399s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_399", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_400s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_400s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_400", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_401s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_401s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_401", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_402s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_402s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_402", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_403s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_403s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_403", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_404s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_404s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_404", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_405s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_405s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_405", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_406s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_406s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_406", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_407s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_407s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_407", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_408s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_408s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_408", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_409s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_409s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_409", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_410s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_410s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_410", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_411s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_411s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_411", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_412s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_412s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_412", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_413s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_413s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_413", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_414s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_414s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_414", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_415s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_415s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_415", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_416s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_416s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_416", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_417s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_417s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_417", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_418s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_418s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_418", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_419s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_419s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_419", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_420s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_420s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_420", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_421s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_421s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_421", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_422s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_422s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_422", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_423s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_423s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_423", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_424s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_424s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_424", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_425s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_425s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_425", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_426s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_426s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_426", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_427s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_427s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_427", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_428s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_428s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_428", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_429s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_429s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_429", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_430s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_430s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_430", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_431s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_431s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_431", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_432s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_432s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_432", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_433s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_433s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_433", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_434s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_434s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_434", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_435s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_435s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_435", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_436s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_436s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_436", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_437s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_437s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_437", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_438s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_438s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_438", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_439s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_439s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_439", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_440s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_440s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_440", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_441s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_441s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_441", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_442s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_442s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_442", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_443s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_443s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_443", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_444s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_444s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_444", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_445s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_445s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_445", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_446s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_446s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_446", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_447s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_447s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_447", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_448s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_448s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_448", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_449s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_449s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_449", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_450s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_450s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_450", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_451s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_451s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_451", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_452s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_452s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_452", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_453s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_453s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_453", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_454s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_454s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_454", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_455s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_455s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_455", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_456s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_456s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_456", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_457s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_457s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_457", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_458s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_458s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_458", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_459s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_459s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_459", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_460s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_460s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_460", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_461s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_461s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_461", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_462s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_462s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_462", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_463s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_463s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_463", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_464s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_464s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_464", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_465s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_465s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_465", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_466s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_466s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_466", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_467s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_467s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_467", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_468s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_468s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_468", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_469s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_469s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_469", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_470s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_470s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_470", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_471s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_471s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_471", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_472s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_472s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_472", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_473s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_473s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_473", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_474s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_474s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_474", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_475s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_475s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_475", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_476s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_476s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_476", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_477s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_477s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_477", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_478s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_478s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_478", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_479s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_479s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_479", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_480s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_480s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_480", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_481s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_481s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_481", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_482s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_482s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_482", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_483s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_483s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_483", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_484s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_484s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_484", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_485s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_485s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_485", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_486s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_486s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_486", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_487s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_487s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_487", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_488s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_488s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_488", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_489s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_489s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_489", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_490s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_490s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_490", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_491s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_491s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_491", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_492s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_492s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_492", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_493s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_493s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_493", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_494s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_494s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_494", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_495s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_495s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_495", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_496s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_496s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_496", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_497s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_497s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_497", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_498s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_498s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_498", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_499s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_499s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_499", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_500s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_500s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_500", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_501s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_501s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_501", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_502s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_502s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_502", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_503s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_503s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_503", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_504s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_504s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_504", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_505s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_505s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_505", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_506s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_506s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_506", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_507s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_507s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_507", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_508s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_508s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_508", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_509s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_509s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_509", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_510s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_510s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_510", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_511s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_511s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_511", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_512s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_512s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_512", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_513s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_513s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_513", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_514s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_514s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_514", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_515s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_515s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_515", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_516s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_516s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_516", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_517s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_517s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_517", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_518s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_518s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_518", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_519s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_519s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_519", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_520s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_520s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_520", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_521s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_521s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_521", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_522s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_522s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_522", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_523s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_523s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_523", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_524s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_524s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_524", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_525s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_525s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_525", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_526s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_526s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_526", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_527s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_527s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_527", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_528s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_528s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_528", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_529s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_529s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_529", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_530s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_530s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_530", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_531s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_531s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_531", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_532s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_532s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_532", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_533s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_533s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_533", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_534s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_534s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_534", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_535s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_535s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_535", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_536s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_536s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_536", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_537s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_537s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_537", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_538s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_538s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_538", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_539s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_539s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_539", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_540s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_540s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_540", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_541s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_541s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_541", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_542s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_542s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_542", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_543s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_543s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_543", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_544s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_544s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_544", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_545s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_545s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_545", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_546s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_546s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_546", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_547s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_547s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_547", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_548s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_548s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_548", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_549s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_549s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_549", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_550s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_550s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_550", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_551s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_551s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_551", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_552s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_552s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_552", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_553s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_553s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_553", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_554s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_554s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_554", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_555s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_555s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_555", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_556s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_556s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_556", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_557s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_557s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_557", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_558s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_558s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_558", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_559s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_559s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_559", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_560s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_560s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_560", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_561s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_561s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_561", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_562s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_562s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_562", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_563s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_563s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_563", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_564s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_564s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_564", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_565s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_565s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_565", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_566s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_566s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_566", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_567s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_567s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_567", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_568s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_568s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_568", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_569s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_569s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_569", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_570s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_570s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_570", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_571s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_571s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_571", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_572s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_572s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_572", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_573s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_573s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_573", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_574s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_574s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_574", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_575s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_575s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_575", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_576s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_576s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_576", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_577s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_577s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_577", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_578s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_578s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_578", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_579s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_579s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_579", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_580s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_580s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_580", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_581s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_581s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_581", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_582s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_582s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_582", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_583s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_583s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_583", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_584s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_584s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_584", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_585s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_585s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_585", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_586s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_586s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_586", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_587s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_587s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_587", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_588s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_588s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_588", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_589s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_589s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_589", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_590s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_590s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_590", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_591s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_591s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_591", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_592s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_592s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_592", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_593s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_593s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_593", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_594s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_594s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_594", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_595s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_595s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_595", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_596s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_596s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_596", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_597s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_597s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_597", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_598s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_598s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_598", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_599s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_599s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_599", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_600s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_600s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_600", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_601s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_601s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_601", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_602s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_602s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_602", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_603s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_603s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_603", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_604s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_604s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_604", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_605s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_605s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_605", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_606s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_606s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_606", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_607s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_607s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_607", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_608s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_608s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_608", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_609s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_609s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_609", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_610s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_610s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_610", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_611s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_611s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_611", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_612s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_612s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_612", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_613s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_613s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_613", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_614s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_614s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_614", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_615s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_615s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_615", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_616s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_616s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_616", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_617s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_617s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_617", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_618s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_618s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_618", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_619s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_619s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_619", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_620s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_620s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_620", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_621s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_621s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_621", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_622s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_622s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_622", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_623s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_623s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_623", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_624s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_624s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_624", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_625s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_625s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_625", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_626s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_626s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_626", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_627s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_627s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_627", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_628s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_628s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_628", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_629s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_629s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_629", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_630s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_630s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_630", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_631s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_631s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_631", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_632s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_632s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_632", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_633s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_633s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_633", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_634s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_634s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_634", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_635s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_635s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_635", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_636s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_636s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_636", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_637s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_637s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_637", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_638s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_638s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_638", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_639s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_639s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_639", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_640s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_640s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_640", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_641s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_641s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_641", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_642s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_642s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_642", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_643s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_643s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_643", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_644s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_644s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_644", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_645s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_645s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_645", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_646s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_646s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_646", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_647s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_647s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_647", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_648s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_648s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_648", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_649s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_649s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_649", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_650s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_650s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_650", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_651s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_651s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_651", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_652s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_652s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_652", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_653s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_653s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_653", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_654s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_654s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_654", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_655s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_655s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_655", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_656s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_656s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_656", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_657s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_657s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_657", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_658s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_658s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_658", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_659s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_659s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_659", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_660s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_660s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_660", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_661s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_661s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_661", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_662s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_662s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_662", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_663s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_663s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_663", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_664s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_664s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_664", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_665s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_665s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_665", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_666s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_666s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_666", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_667s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_667s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_667", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_668s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_668s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_668", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_669s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_669s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_669", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_670s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_670s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_670", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_671s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_671s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_671", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_672s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_672s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_672", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_673s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_673s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_673", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_674s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_674s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_674", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_675s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_675s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_675", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_676s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_676s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_676", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_677s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_677s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_677", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_678s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_678s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_678", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_679s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_679s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_679", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_680s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_680s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_680", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_681s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_681s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_681", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_682s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_682s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_682", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_683s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_683s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_683", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_684s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_684s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_684", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_685s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_685s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_685", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_686s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_686s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_686", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_687s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_687s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_687", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_688s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_688s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_688", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_689s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_689s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_689", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_690s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_690s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_690", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_691s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_691s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_691", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_692s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_692s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_692", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_693s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_693s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_693", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_694s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_694s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_694", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_695s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_695s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_695", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_696s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_696s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_696", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_697s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_697s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_697", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_698s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_698s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_698", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_699s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_699s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_699", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_700s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_700s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_700", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_701s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_701s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_701", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_702s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_702s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_702", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_703s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_703s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_703", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_704s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_704s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_704", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_705s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_705s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_705", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_706s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_706s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_706", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_707s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_707s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_707", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_708s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_708s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_708", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_709s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_709s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_709", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_710s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_710s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_710", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_711s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_711s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_711", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_712s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_712s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_712", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_713s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_713s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_713", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_714s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_714s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_714", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_715s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_715s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_715", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_716s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_716s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_716", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_717s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_717s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_717", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_718s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_718s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_718", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_719s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_719s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_719", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_720s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_720s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_720", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_721s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_721s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_721", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_722s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_722s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_722", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_723s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_723s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_723", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_724s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_724s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_724", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_725s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_725s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_725", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_726s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_726s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_726", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_727s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_727s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_727", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_728s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_728s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_728", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_729s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_729s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_729", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_730s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_730s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_730", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_731s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_731s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_731", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_732s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_732s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_732", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_733s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_733s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_733", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_734s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_734s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_734", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_735s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_735s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_735", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_736s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_736s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_736", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_737s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_737s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_737", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_738s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_738s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_738", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_739s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_739s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_739", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_740s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_740s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_740", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_741s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_741s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_741", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_742s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_742s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_742", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_743s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_743s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_743", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_744s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_744s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_744", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_745s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_745s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_745", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_746s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_746s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_746", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_747s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_747s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_747", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_748s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_748s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_748", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_749s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_749s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_749", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_750s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_750s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_750", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_751s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_751s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_751", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_752s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_752s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_752", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_753s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_753s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_753", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_754s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_754s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_754", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_755s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_755s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_755", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_756s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_756s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_756", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_757s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_757s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_757", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_758s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_758s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_758", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_759s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_759s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_759", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_760s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_760s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_760", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_761s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_761s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_761", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_762s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_762s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_762", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_763s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_763s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_763", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_764s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_764s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_764", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_765s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_765s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_765", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_766s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_766s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_766", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_767s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_767s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_767", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_768s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_768s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_768", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_769s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_769s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_769", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_770s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_770s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_770", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_771s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_771s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_771", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_772s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_772s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_772", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_773s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_773s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_773", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_774s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_774s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_774", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_775s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_775s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_775", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_776s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_776s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_776", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_777s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_777s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_777", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_778s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_778s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_778", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_779s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_779s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_779", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_780s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_780s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_780", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_781s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_781s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_781", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_782s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_782s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_782", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_783s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_783s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_783", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_784s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_784s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_784", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_785s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_785s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_785", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_786s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_786s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_786", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_787s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_787s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_787", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_788s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_788s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_788", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_789s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_789s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_789", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_790s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_790s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_790", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_791s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_791s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_791", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_792s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_792s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_792", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_793s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_793s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_793", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_794s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_794s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_794", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_795s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_795s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_795", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_796s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_796s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_796", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_797s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_797s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_797", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_798s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_798s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_798", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_799s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_799s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_799", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_800s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_800s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_800", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_801s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_801s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_801", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_802s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_802s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_802", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_803s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_803s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_803", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_804s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_804s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_804", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_805s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_805s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_805", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_806s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_806s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_806", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_807s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_807s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_807", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_808s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_808s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_808", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_809s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_809s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_809", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_810s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_810s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_810", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_811s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_811s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_811", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_812s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_812s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_812", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_813s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_813s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_813", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_814s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_814s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_814", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_815s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_815s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_815", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_816s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_816s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_816", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_817s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_817s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_817", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_818s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_818s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_818", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_819s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_819s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_819", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_820s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_820s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_820", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_821s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_821s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_821", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_822s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_822s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_822", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_823s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_823s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_823", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_824s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_824s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_824", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_825s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_825s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_825", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_826s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_826s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_826", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_827s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_827s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_827", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_828s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_828s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_828", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_829s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_829s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_829", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_830s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_830s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_830", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_831s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_831s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_831", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_832s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_832s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_832", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_833s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_833s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_833", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_834s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_834s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_834", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_835s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_835s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_835", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_836s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_836s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_836", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_837s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_837s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_837", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_838s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_838s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_838", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_839s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_839s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_839", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_840s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_840s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_840", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_841s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_841s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_841", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_842s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_842s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_842", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_843s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_843s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_843", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_844s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_844s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_844", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_845s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_845s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_845", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_846s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_846s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_846", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_847s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_847s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_847", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_848s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_848s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_848", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_849s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_849s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_849", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_850s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_850s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_850", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_851s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_851s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_851", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_852s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_852s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_852", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_853s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_853s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_853", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_854s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_854s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_854", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_855s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_855s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_855", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_856s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_856s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_856", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_857s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_857s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_857", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_858s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_858s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_858", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_859s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_859s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_859", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_860s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_860s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_860", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_861s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_861s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_861", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_862s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_862s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_862", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_863s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_863s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_863", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_864s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_864s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_864", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_865s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_865s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_865", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_866s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_866s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_866", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_867s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_867s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_867", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_868s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_868s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_868", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_869s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_869s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_869", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_870s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_870s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_870", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_871s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_871s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_871", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_872s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_872s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_872", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_873s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_873s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_873", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_874s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_874s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_874", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_875s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_875s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_875", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_876s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_876s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_876", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_877s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_877s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_877", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_878s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_878s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_878", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_879s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_879s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_879", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_880s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_880s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_880", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_881s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_881s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_881", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_882s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_882s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_882", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_883s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_883s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_883", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_884s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_884s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_884", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_885s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_885s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_885", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_886s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_886s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_886", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_887s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_887s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_887", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_888s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_888s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_888", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_889s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_889s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_889", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_890s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_890s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_890", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_891s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_891s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_891", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_892s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_892s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_892", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_893s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_893s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_893", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_894s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_894s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_894", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_895s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_895s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_895", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_896s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_896s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_896", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_897s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_897s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_897", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_898s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_898s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_898", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_899s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_899s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_899", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_900s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_900s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_900", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_901s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_901s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_901", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_902s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_902s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_902", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_903s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_903s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_903", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_904s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_904s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_904", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_905s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_905s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_905", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_906s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_906s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_906", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_907s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_907s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_907", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_908s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_908s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_908", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_909s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_909s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_909", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_910s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_910s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_910", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_911s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_911s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_911", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_912s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_912s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_912", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_913s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_913s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_913", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_914s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_914s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_914", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_915s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_915s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_915", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_916s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_916s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_916", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_917s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_917s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_917", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_918s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_918s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_918", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_919s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_919s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_919", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_920s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_920s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_920", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_921s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_921s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_921", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_922s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_922s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_922", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_923s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_923s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_923", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_924s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_924s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_924", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_925s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_925s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_925", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_926s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_926s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_926", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_927s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_927s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_927", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_928s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_928s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_928", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_929s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_929s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_929", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_930s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_930s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_930", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_931s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_931s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_931", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_932s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_932s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_932", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_933s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_933s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_933", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_934s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_934s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_934", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_935s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_935s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_935", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_936s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_936s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_936", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_937s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_937s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_937", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_938s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_938s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_938", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_939s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_939s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_939", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_940s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_940s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_940", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_941s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_941s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_941", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_942s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_942s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_942", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_943s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_943s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_943", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_944s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_944s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_944", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_945s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_945s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_945", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_946s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_946s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_946", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_947s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_947s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_947", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_948s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_948s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_948", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_949s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_949s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_949", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_950s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_950s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_950", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_951s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_951s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_951", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_952s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_952s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_952", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_953s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_953s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_953", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_954s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_954s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_954", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_955s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_955s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_955", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_956s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_956s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_956", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_957s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_957s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_957", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_958s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_958s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_958", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_959s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_959s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_959", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_960s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_960s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_960", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_961s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_961s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_961", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_962s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_962s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_962", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_963s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_963s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_963", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_964s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_964s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_964", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_965s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_965s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_965", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_966s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_966s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_966", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_967s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_967s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_967", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_968s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_968s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_968", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_969s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_969s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_969", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_970s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_970s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_970", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_971s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_971s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_971", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_972s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_972s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_972", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_973s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_973s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_973", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_974s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_974s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_974", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_975s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_975s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_975", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_976s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_976s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_976", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_977s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_977s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_977", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_978s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_978s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_978", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_979s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_979s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_979", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_980s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_980s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_980", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_981s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_981s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_981", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_982s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_982s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_982", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_983s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_983s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_983", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_984s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_984s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_984", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_985s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_985s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_985", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_986s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_986s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_986", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_987s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_987s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_987", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_988s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_988s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_988", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_989s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_989s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_989", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_990s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_990s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_990", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_991s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_991s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_991", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_992s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_992s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_992", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_993s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_993s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_993", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_994s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_994s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_994", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_995s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_995s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_995", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_996s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_996s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_996", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_997s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_997s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_997", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_998s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_998s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_998", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_custom_entity_999s<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::custom_entity_999s().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("custom_entity_999", item.id().into_u64());
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
        state.add_reference("customer", item.id().into_u64());
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
        state.add_reference("customer_consent", item.id().into_u64());
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
        state.add_reference("customer_contact", item.id().into_u64());
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
        state.add_reference("customer_history", item.id().into_u64());
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
        state.add_reference("customer_note", item.id().into_u64());
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
        state.add_reference("customer_preference", item.id().into_u64());
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
        state.add_reference("customer_signature", item.id().into_u64());
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
        state.add_reference("damage_report", item.id().into_u64());
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
        state.add_reference("dashcam_footage", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_data_retention_policies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::data_retention_policies().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("data_retention_policy", item.id().into_u64());
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
        state.add_reference("debit_note", item.id().into_u64());
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
        state.add_reference("department", item.id().into_u64());
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
        state.add_reference("depreciation_schedule", item.id().into_u64());
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
        state.add_reference("detour_log", item.id().into_u64());
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
        state.add_reference("direct_deposit_info", item.id().into_u64());
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
        state.add_reference("discount_code", item.id().into_u64());
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
        state.add_reference("dispatch_assignment", item.id().into_u64());
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
        state.add_reference("do_not_contact_list", item.id().into_u64());
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
        state.add_reference("document", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_document_versions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::document_versions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("document_version", item.id().into_u64());
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
        state.add_reference("email_blast", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_email_bounce_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::email_bounce_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("email_bounce_log", item.id().into_u64());
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
        state.add_reference("emergency_contact", item.id().into_u64());
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
        state.add_reference("employee", item.id().into_u64());
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
        state.add_reference("employee_certification", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_entity_changes<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::entity_changes().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("entity_change", item.id().into_u64());
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
        state.add_reference("equipment", item.id().into_u64());
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
        state.add_reference("expense", item.id().into_u64());
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
        state.add_reference("expense_reimbursement", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_failed_auth_logs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::failed_auth_logs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("failed_auth_log", item.id().into_u64());
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
        state.add_reference("financial_summary", item.id().into_u64());
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
        state.add_reference("fiscal_year", item.id().into_u64());
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
        state.add_reference("franchise", item.id().into_u64());
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
        state.add_reference("fuel_record", item.id().into_u64());
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
        state.add_reference("fuel_stop", item.id().into_u64());
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
        state.add_reference("fulfillment_event", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_gdpr_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::gdpr_requests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("gdpr_request", item.id().into_u64());
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
        state.add_reference("gps_tracker", item.id().into_u64());
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
        state.add_reference("hoisting_service", item.id().into_u64());
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
        state.add_reference("insurance_addon", item.id().into_u64());
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
        state.add_reference("insurance_card", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_insurance_claims<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::insurance_claims().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("insurance_claim", item.id().into_u64());
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
        state.add_reference("insurance_policy", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_integration_mappings<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::integration_mappings().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("integration_mapping", item.id().into_u64());
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
        state.add_reference("inventory_item", item.id().into_u64());
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
        state.add_reference("invoice", item.id().into_u64());
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
        state.add_reference("invoice_line", item.id().into_u64());
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
        state.add_reference("job_assignment", item.id().into_u64());
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
        state.add_reference("journal_entry", item.id().into_u64());
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
        state.add_reference("lead", item.id().into_u64());
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
        state.add_reference("lead_activity", item.id().into_u64());
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
        state.add_reference("leave_request", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_login_attempts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::login_attempts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("login_attempt", item.id().into_u64());
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
        state.add_reference("long_carry_fee", item.id().into_u64());
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
        state.add_reference("loyalty_tier", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_magic_links<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::magic_links().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("magic_link", item.id().into_u64());
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
        state.add_reference("maintenance_event", item.id().into_u64());
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
        state.add_reference("maintenance_schedule", item.id().into_u64());
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
        state.add_reference("merchant", item.id().into_u64());
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
        state.add_reference("merchant_fee", item.id().into_u64());
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
        state.add_reference("move_order", item.id().into_u64());
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
        state.add_reference("move_quote", item.id().into_u64());
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
        state.add_reference("moving_service", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_nda_agreements<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::nda_agreements().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("nda_agreement", item.id().into_u64());
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
        state.add_reference("notification", item.id().into_u64());
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
        state.add_reference("notification_template", item.id().into_u64());
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
        state.add_reference("objection_handling_guide", item.id().into_u64());
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
        state.add_reference("oil_change_log", item.id().into_u64());
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
        state.add_reference("operations_manager_override", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_osha_incidents<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::osha_incidents().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("osha_incident", item.id().into_u64());
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
        state.add_reference("overtime_approval", item.id().into_u64());
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
        state.add_reference("packing_list", item.id().into_u64());
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
        state.add_reference("packing_material", item.id().into_u64());
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
        state.add_reference("parking_permit", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_password_resets<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::password_resets().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("password_reset", item.id().into_u64());
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
        state.add_reference("payment", item.id().into_u64());
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
        state.add_reference("payroll_calculation", item.id().into_u64());
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
        state.add_reference("payroll_period", item.id().into_u64());
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
        state.add_reference("payslip", item.id().into_u64());
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
        state.add_reference("performance_review", item.id().into_u64());
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
        state.add_reference("permission", item.id().into_u64());
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
        state.add_reference("pet_relocation_service", item.id().into_u64());
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
        state.add_reference("piano_handling", item.id().into_u64());
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
        state.add_reference("platform", item.id().into_u64());
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
        state.add_reference("platform_config", item.id().into_u64());
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
        state.add_reference("post_move_survey", item.id().into_u64());
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
        state.add_reference("price_list", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_privacy_policies<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::privacy_policies().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("privacy_policy", item.id().into_u64());
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
        state.add_reference("private_customer_profile", item.id().into_u64());
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
        state.add_reference("product", item.id().into_u64());
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
        state.add_reference("proof_of_delivery", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_recovery_requests<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::recovery_requests().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("recovery_request", item.id().into_u64());
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
        state.add_reference("referral_code", item.id().into_u64());
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
        state.add_reference("refund", item.id().into_u64());
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
        state.add_reference("registration_renewal", item.id().into_u64());
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
        state.add_reference("resolution_offer", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_roles<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::roles().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("role", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_role_permissions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::role_permissions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("role_permission", item.id().into_u64());
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
        state.add_reference("route", item.id().into_u64());
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
        state.add_reference("route_stop", item.id().into_u64());
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
        state.add_reference("sales_opportunity", item.id().into_u64());
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
        state.add_reference("sales_script", item.id().into_u64());
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
        state.add_reference("sales_territory", item.id().into_u64());
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
        state.add_reference("scrap_record", item.id().into_u64());
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
        state.add_reference("service", item.id().into_u64());
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
        state.add_reference("service_bundle", item.id().into_u64());
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
        state.add_reference("service_configuration", item.id().into_u64());
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
        state.add_reference("service_price", item.id().into_u64());
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
        state.add_reference("sms_campaign", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_sms_delivery_receipts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::sms_delivery_receipts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("sms_delivery_receipt", item.id().into_u64());
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
        state.add_reference("social_media_post", item.id().into_u64());
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
        state.add_reference("stair_fee", item.id().into_u64());
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
        state.add_reference("storage_unit", item.id().into_u64());
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
        state.add_reference("supplier", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_sync_jobs<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::sync_jobs().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("sync_job", item.id().into_u64());
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
        state.add_reference("tax_document", item.id().into_u64());
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
        state.add_reference("tax_withholding", item.id().into_u64());
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
        state.add_reference("tenant_registry", item.id().into_u64());
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
        state.add_reference("termination_record", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_terms_of_services<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::terms_of_services().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("terms_of_service", item.id().into_u64());
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
        state.add_reference("time_slot", item.id().into_u64());
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
        state.add_reference("tire_replacement", item.id().into_u64());
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
        state.add_reference("toll_receipt", item.id().into_u64());
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
        state.add_reference("traffic_violation", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_two_factor_auths<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::two_factor_auths().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("two_factor_auth", item.id().into_u64());
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
        state.add_reference("uniform_assignment", item.id().into_u64());
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
        state.add_reference("union_dues", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_user_accounts<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::user_accounts().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("user_account", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_user_role_assignments<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::user_role_assignments().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("user_role_assignment", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_user_sessions<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::user_sessions().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("user_session", item.id().into_u64());
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
        state.add_reference("vat_rate", item.id().into_u64());
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
        state.add_reference("vehicle", item.id().into_u64());
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
        state.add_reference("vehicle_load_plan", item.id().into_u64());
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
        state.add_reference("vehicle_transport", item.id().into_u64());
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
        state.add_reference("vip_status", item.id().into_u64());
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
        state.add_reference("walkthrough_checklist", item.id().into_u64());
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
        state.add_reference("warning_letter", item.id().into_u64());
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
        state.add_reference("weather_delay", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_webhooks<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::webhooks().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("webhook", item.id().into_u64());
    }
    Ok(())
}

async fn load_root_webhook_deliveries<C>(
    ctx: &C,
    state: &mut SampleDataState,
) -> Result<(), String>
where
    C: TeaqlRuntime + ?Sized + crate::TeaqlRepositoryProvider,
{
    let list = Q::webhook_deliveries().purpose("Init Sample Data").execute_for_list(ctx).await.unwrap_or_default();
    for item in list {
        state.add_reference("webhook_delivery", item.id().into_u64());
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
        state.add_reference("weigh_station_ticket", item.id().into_u64());
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
        state.add_reference("work_shift", item.id().into_u64());
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
        state.add_reference("worked_hours", item.id().into_u64());
    }
    Ok(())
}


