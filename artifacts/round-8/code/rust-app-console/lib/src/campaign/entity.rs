// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/campaign
use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Campaign", table = "campaign_data", data_service = "sqlite")]
pub struct Campaign {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    campaign_id: String,

// @source model.xml:2
    name: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "merchant_ref")]
    merchant_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Merchant", local_key = "merchant_ref_id", foreign_key = "id"))]
    merchant_ref: Option<crate::Merchant>,
#[teaql(relation(target = "DiscountCode", local_key = "id", foreign_key = "campaign_ref_id", many))]
    discount_code_list: SmartList<crate::DiscountCode>,
#[teaql(relation(target = "Lead", local_key = "id", foreign_key = "campaign_ref_id", many))]
    lead_list: SmartList<crate::Lead>,
#[teaql(relation(target = "ConversionMetric", local_key = "id", foreign_key = "campaign_ref_id", many))]
    conversion_metric_list: SmartList<crate::ConversionMetric>,
#[teaql(relation(target = "AudienceSegment", local_key = "id", foreign_key = "campaign_ref_id", many))]
    audience_segment_list: SmartList<crate::AudienceSegment>,
#[teaql(relation(target = "PromotionalOffer", local_key = "id", foreign_key = "campaign_ref_id", many))]
    promotional_offer_list: SmartList<crate::PromotionalOffer>,
#[teaql(relation(target = "AttributionModel", local_key = "id", foreign_key = "campaign_ref_id", many))]
    attribution_model_list: SmartList<crate::AttributionModel>,
#[teaql(relation(target = "CampaignBudget", local_key = "id", foreign_key = "campaign_ref_id", many))]
    campaign_budget_list: SmartList<crate::CampaignBudget>,
#[teaql(relation(target = "ConversionReport", local_key = "id", foreign_key = "campaign_ref_id", many))]
    conversion_report_list: SmartList<crate::ConversionReport>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Campaign {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            campaign_id: String::new(),
            name: String::new(),
            version: 0_i64,
            merchant_ref_id: 0_u64,
            merchant_ref: None,
            discount_code_list: Default::default(),
            lead_list: Default::default(),
            conversion_metric_list: Default::default(),
            audience_segment_list: Default::default(),
            promotional_offer_list: Default::default(),
            attribution_model_list: Default::default(),
            campaign_budget_list: Default::default(),
            conversion_report_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Campaign", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.merchant_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.discount_code_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.lead_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.conversion_metric_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.audience_segment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.promotional_offer_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.attribution_model_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.campaign_budget_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.conversion_report_list {
            entity.attach_root_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn campaign_id(&self) -> String {
        self.changed_campaign_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.campaign_id.clone())
    }

    pub fn update_campaign_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.campaign_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.campaign_id.clone());
        self.root.set(self.entity_key(), "campaign_id", value);
        self
    }

    pub fn changed_campaign_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "campaign_id")
    }

    pub fn eval_campaign_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("campaign_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "campaign_id".to_string(), attempted_path: "campaign_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.campaign_id())
                }}

    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.name.clone());
        self.root.set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
                }}

    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn merchant_ref_id(&self) -> u64 {
        self.changed_merchant_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_ref_id)
    }

    pub fn update_merchant_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_ref_id = value.try_u64().unwrap_or(self.merchant_ref_id.clone());
        self.root.set(self.entity_key(), "merchant_ref_id", value);
        self
    }

    pub fn changed_merchant_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_ref_id")
    }

    pub fn eval_merchant_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref_id".to_string(), attempted_path: "merchant_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_ref_id())
                }}
    pub fn merchant_ref(&self) -> Option<&crate::Merchant> {
        self.merchant_ref.as_ref()
    }

    pub fn eval_merchant_ref(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref".to_string(), attempted_path: "merchant_ref".to_string() }
        } else {
            match &self.merchant_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn discount_code_list(&self) -> &SmartList<crate::DiscountCode> {
        &self.discount_code_list
    }

    pub fn discount_code_list_mut(&mut self) -> &mut SmartList<crate::DiscountCode> {
        &mut self.discount_code_list
    }

    pub fn eval_discount_code_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DiscountCode>> {
        if !self.is_loaded("discount_code_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "discount_code_list".to_string(), attempted_path: "discount_code_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.discount_code_list)
        }
    }

    pub fn lead_list(&self) -> &SmartList<crate::Lead> {
        &self.lead_list
    }

    pub fn lead_list_mut(&mut self) -> &mut SmartList<crate::Lead> {
        &mut self.lead_list
    }

    pub fn eval_lead_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Lead>> {
        if !self.is_loaded("lead_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "lead_list".to_string(), attempted_path: "lead_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.lead_list)
        }
    }

    pub fn conversion_metric_list(&self) -> &SmartList<crate::ConversionMetric> {
        &self.conversion_metric_list
    }

    pub fn conversion_metric_list_mut(&mut self) -> &mut SmartList<crate::ConversionMetric> {
        &mut self.conversion_metric_list
    }

    pub fn eval_conversion_metric_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ConversionMetric>> {
        if !self.is_loaded("conversion_metric_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "conversion_metric_list".to_string(), attempted_path: "conversion_metric_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.conversion_metric_list)
        }
    }

    pub fn audience_segment_list(&self) -> &SmartList<crate::AudienceSegment> {
        &self.audience_segment_list
    }

    pub fn audience_segment_list_mut(&mut self) -> &mut SmartList<crate::AudienceSegment> {
        &mut self.audience_segment_list
    }

    pub fn eval_audience_segment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AudienceSegment>> {
        if !self.is_loaded("audience_segment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "audience_segment_list".to_string(), attempted_path: "audience_segment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.audience_segment_list)
        }
    }

    pub fn promotional_offer_list(&self) -> &SmartList<crate::PromotionalOffer> {
        &self.promotional_offer_list
    }

    pub fn promotional_offer_list_mut(&mut self) -> &mut SmartList<crate::PromotionalOffer> {
        &mut self.promotional_offer_list
    }

    pub fn eval_promotional_offer_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PromotionalOffer>> {
        if !self.is_loaded("promotional_offer_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "promotional_offer_list".to_string(), attempted_path: "promotional_offer_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.promotional_offer_list)
        }
    }

    pub fn attribution_model_list(&self) -> &SmartList<crate::AttributionModel> {
        &self.attribution_model_list
    }

    pub fn attribution_model_list_mut(&mut self) -> &mut SmartList<crate::AttributionModel> {
        &mut self.attribution_model_list
    }

    pub fn eval_attribution_model_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AttributionModel>> {
        if !self.is_loaded("attribution_model_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "attribution_model_list".to_string(), attempted_path: "attribution_model_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.attribution_model_list)
        }
    }

    pub fn campaign_budget_list(&self) -> &SmartList<crate::CampaignBudget> {
        &self.campaign_budget_list
    }

    pub fn campaign_budget_list_mut(&mut self) -> &mut SmartList<crate::CampaignBudget> {
        &mut self.campaign_budget_list
    }

    pub fn eval_campaign_budget_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CampaignBudget>> {
        if !self.is_loaded("campaign_budget_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "campaign_budget_list".to_string(), attempted_path: "campaign_budget_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.campaign_budget_list)
        }
    }

    pub fn conversion_report_list(&self) -> &SmartList<crate::ConversionReport> {
        &self.conversion_report_list
    }

    pub fn conversion_report_list_mut(&mut self) -> &mut SmartList<crate::ConversionReport> {
        &mut self.conversion_report_list
    }

    pub fn eval_conversion_report_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ConversionReport>> {
        if !self.is_loaded("conversion_report_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "conversion_report_list".to_string(), attempted_path: "conversion_report_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.conversion_report_list)
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }

    pub(crate) async fn save<'a, C>(
        &self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let root = ctx.user_context().entity_root();
        let key = self.entity_key();
        let has_ledger_change = (self.id != 0)
            && (root.current_change_set().changes().contains_key(&key)
                || root.is_marked_as_delete(&key)
                || root.is_new(&key));
        let repository = ctx
            .campaign_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Campaign"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

