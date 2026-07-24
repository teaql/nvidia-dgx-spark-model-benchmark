// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/recruitment_post
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
#[teaql(entity = "RecruitmentPost", table = "recruitment_post_data", data_service = "sqlite")]
pub struct RecruitmentPost {
#[teaql(id)]
    id: u64,

// @source model.xml:212
    job_description: String,

// @source model.xml:212
    posting_date: String,

// @source model.xml:212
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:212
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:212
#[teaql(column = "position")]
    position_id: u64,

// @source model.xml:212
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:212
#[teaql(relation(target = "Position", local_key = "position_id", foreign_key = "id"))]
    position: Option<crate::Position>,

// @source model.xml:212
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
#[teaql(relation(target = "JobApplication", local_key = "id", foreign_key = "recruitment_post_id", many))]
    job_application_list: SmartList<crate::JobApplication>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl RecruitmentPost {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            job_description: String::new(),
            posting_date: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            position_id: 0_u64,
            merchant_id: 0_u64,
            position: None,
            merchant: None,
            job_application_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("RecruitmentPost", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.position {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.job_application_list {
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

    pub fn job_description(&self) -> String {
        self.changed_job_description().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.job_description.clone())
    }

    pub fn update_job_description(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.job_description = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.job_description.clone());
        self.root.set(self.entity_key(), "job_description", value);
        self
    }

    pub fn changed_job_description(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "job_description")
    }

    pub fn eval_job_description(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("job_description") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_description".to_string(), attempted_path: "job_description".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.job_description())
                }}

    pub fn posting_date(&self) -> String {
        self.changed_posting_date().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.posting_date.clone())
    }

    pub fn update_posting_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.posting_date = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.posting_date.clone());
        self.root.set(self.entity_key(), "posting_date", value);
        self
    }

    pub fn changed_posting_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "posting_date")
    }

    pub fn eval_posting_date(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("posting_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "posting_date".to_string(), attempted_path: "posting_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.posting_date())
                }}

    pub fn create_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.create_time = value.try_timestamp().unwrap_or(self.create_time.clone());
        self.root.set(self.entity_key(), "create_time", value);
        self
    }

    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}

    pub fn update_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.update_time = value.try_timestamp().unwrap_or(self.update_time.clone());
        self.root.set(self.entity_key(), "update_time", value);
        self
    }

    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("update_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "update_time".to_string(), attempted_path: "update_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.update_time())
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
    pub fn position_id(&self) -> u64 {
        self.changed_position_id().and_then(|value| value.try_u64()).unwrap_or(self.position_id)
    }

    pub fn update_position_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.position_id = value.try_u64().unwrap_or(self.position_id.clone());
        self.root.set(self.entity_key(), "position_id", value);
        self
    }

    pub fn changed_position_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "position_id")
    }

    pub fn eval_position_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("position_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "position_id".to_string(), attempted_path: "position_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.position_id())
                }}

    pub fn merchant_id(&self) -> u64 {
        self.changed_merchant_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_id)
    }

    pub fn update_merchant_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_id = value.try_u64().unwrap_or(self.merchant_id.clone());
        self.root.set(self.entity_key(), "merchant_id", value);
        self
    }

    pub fn changed_merchant_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_id")
    }

    pub fn eval_merchant_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_id".to_string(), attempted_path: "merchant_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_id())
                }}
    pub fn position(&self) -> Option<&crate::Position> {
        self.position.as_ref()
    }

    pub fn eval_position(&self) -> teaql_core::eval::EvalResult<&crate::Position> {
        if !self.is_loaded("position") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "position".to_string(), attempted_path: "position".to_string() }
        } else {
            match &self.position {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn merchant(&self) -> Option<&crate::Merchant> {
        self.merchant.as_ref()
    }

    pub fn eval_merchant(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant".to_string(), attempted_path: "merchant".to_string() }
        } else {
            match &self.merchant {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn job_application_list(&self) -> &SmartList<crate::JobApplication> {
        &self.job_application_list
    }

    pub fn job_application_list_mut(&mut self) -> &mut SmartList<crate::JobApplication> {
        &mut self.job_application_list
    }

    pub fn eval_job_application_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::JobApplication>> {
        if !self.is_loaded("job_application_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_application_list".to_string(), attempted_path: "job_application_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.job_application_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::RecruitmentPostRepository<'a>>>
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
            .recruitment_post_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("RecruitmentPost"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

