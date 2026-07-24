// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/lead
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
#[teaql(entity = "Lead", table = "lead_data", data_service = "sqlite")]
pub struct Lead {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    lead_id: String,

// @source model.xml:2
    source: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "campaign_ref")]
    campaign_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Campaign", local_key = "campaign_ref_id", foreign_key = "id"))]
    campaign_ref: Option<crate::Campaign>,
#[teaql(relation(target = "SalesOpportunity", local_key = "id", foreign_key = "lead_ref_id", many))]
    sales_opportunity_list: SmartList<crate::SalesOpportunity>,
#[teaql(relation(target = "LeadActivity", local_key = "id", foreign_key = "lead_ref_id", many))]
    lead_activity_list: SmartList<crate::LeadActivity>,
#[teaql(relation(target = "SalesFunnel", local_key = "id", foreign_key = "lead_ref_id", many))]
    sales_funnel_list: SmartList<crate::SalesFunnel>,
#[teaql(relation(target = "LeadScore", local_key = "id", foreign_key = "lead_ref_id", many))]
    lead_score_list: SmartList<crate::LeadScore>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Lead {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            lead_id: String::new(),
            source: String::new(),
            version: 0_i64,
            campaign_ref_id: 0_u64,
            campaign_ref: None,
            sales_opportunity_list: Default::default(),
            lead_activity_list: Default::default(),
            sales_funnel_list: Default::default(),
            lead_score_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Lead", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.campaign_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.sales_opportunity_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.lead_activity_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.sales_funnel_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.lead_score_list {
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

    pub fn lead_id(&self) -> String {
        self.changed_lead_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.lead_id.clone())
    }

    pub fn update_lead_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.lead_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.lead_id.clone());
        self.root.set(self.entity_key(), "lead_id", value);
        self
    }

    pub fn changed_lead_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "lead_id")
    }

    pub fn eval_lead_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("lead_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "lead_id".to_string(), attempted_path: "lead_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.lead_id())
                }}

    pub fn source(&self) -> String {
        self.changed_source().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.source.clone())
    }

    pub fn update_source(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.source = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.source.clone());
        self.root.set(self.entity_key(), "source", value);
        self
    }

    pub fn changed_source(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "source")
    }

    pub fn eval_source(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("source") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "source".to_string(), attempted_path: "source".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.source())
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
    pub fn campaign_ref_id(&self) -> u64 {
        self.changed_campaign_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.campaign_ref_id)
    }

    pub fn update_campaign_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.campaign_ref_id = value.try_u64().unwrap_or(self.campaign_ref_id.clone());
        self.root.set(self.entity_key(), "campaign_ref_id", value);
        self
    }

    pub fn changed_campaign_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "campaign_ref_id")
    }

    pub fn eval_campaign_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("campaign_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "campaign_ref_id".to_string(), attempted_path: "campaign_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.campaign_ref_id())
                }}
    pub fn campaign_ref(&self) -> Option<&crate::Campaign> {
        self.campaign_ref.as_ref()
    }

    pub fn eval_campaign_ref(&self) -> teaql_core::eval::EvalResult<&crate::Campaign> {
        if !self.is_loaded("campaign_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "campaign_ref".to_string(), attempted_path: "campaign_ref".to_string() }
        } else {
            match &self.campaign_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn sales_opportunity_list(&self) -> &SmartList<crate::SalesOpportunity> {
        &self.sales_opportunity_list
    }

    pub fn sales_opportunity_list_mut(&mut self) -> &mut SmartList<crate::SalesOpportunity> {
        &mut self.sales_opportunity_list
    }

    pub fn eval_sales_opportunity_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SalesOpportunity>> {
        if !self.is_loaded("sales_opportunity_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "sales_opportunity_list".to_string(), attempted_path: "sales_opportunity_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.sales_opportunity_list)
        }
    }

    pub fn lead_activity_list(&self) -> &SmartList<crate::LeadActivity> {
        &self.lead_activity_list
    }

    pub fn lead_activity_list_mut(&mut self) -> &mut SmartList<crate::LeadActivity> {
        &mut self.lead_activity_list
    }

    pub fn eval_lead_activity_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeadActivity>> {
        if !self.is_loaded("lead_activity_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "lead_activity_list".to_string(), attempted_path: "lead_activity_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.lead_activity_list)
        }
    }

    pub fn sales_funnel_list(&self) -> &SmartList<crate::SalesFunnel> {
        &self.sales_funnel_list
    }

    pub fn sales_funnel_list_mut(&mut self) -> &mut SmartList<crate::SalesFunnel> {
        &mut self.sales_funnel_list
    }

    pub fn eval_sales_funnel_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SalesFunnel>> {
        if !self.is_loaded("sales_funnel_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "sales_funnel_list".to_string(), attempted_path: "sales_funnel_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.sales_funnel_list)
        }
    }

    pub fn lead_score_list(&self) -> &SmartList<crate::LeadScore> {
        &self.lead_score_list
    }

    pub fn lead_score_list_mut(&mut self) -> &mut SmartList<crate::LeadScore> {
        &mut self.lead_score_list
    }

    pub fn eval_lead_score_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeadScore>> {
        if !self.is_loaded("lead_score_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "lead_score_list".to_string(), attempted_path: "lead_score_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.lead_score_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::LeadRepository<'a>>>
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
            .lead_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Lead"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

