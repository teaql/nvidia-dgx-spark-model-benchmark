// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/attribution_model
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "AttributionModel", table = "attribution_model_data", data_service = "sqlite")]
pub struct AttributionModel {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    model_id: String,

// @source model.xml:2
    record_type: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "campaign_ref")]
    campaign_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Campaign", local_key = "campaign_ref_id", foreign_key = "id"))]
    campaign_ref: Option<crate::Campaign>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl AttributionModel {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            model_id: String::new(),
            record_type: String::new(),
            version: 0_i64,
            campaign_ref_id: 0_u64,
            campaign_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("AttributionModel", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.campaign_ref {
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

    pub fn model_id(&self) -> String {
        self.changed_model_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.model_id.clone())
    }

    pub fn update_model_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.model_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.model_id.clone());
        self.root.set(self.entity_key(), "model_id", value);
        self
    }

    pub fn changed_model_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "model_id")
    }

    pub fn eval_model_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("model_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "model_id".to_string(), attempted_path: "model_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.model_id())
                }}

    pub fn record_type(&self) -> String {
        self.changed_record_type().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.record_type.clone())
    }

    pub fn update_record_type(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.record_type = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.record_type.clone());
        self.root.set(self.entity_key(), "record_type", value);
        self
    }

    pub fn changed_record_type(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "record_type")
    }

    pub fn eval_record_type(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("record_type") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "record_type".to_string(), attempted_path: "record_type".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.record_type())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::AttributionModelRepository<'a>>>
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
            .attribution_model_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("AttributionModel"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

