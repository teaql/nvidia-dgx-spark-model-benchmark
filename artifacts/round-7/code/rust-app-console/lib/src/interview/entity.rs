// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/interview
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Interview", table = "interview_data", data_service = "sqlite")]
pub struct Interview {
#[teaql(id)]
    id: u64,

// @source model.xml:229
    interview_date: String,

// @source model.xml:229
    feedback: String,

// @source model.xml:229
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:229
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:229
#[teaql(column = "job_application")]
    job_application_id: u64,

// @source model.xml:229
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:229
#[teaql(relation(target = "JobApplication", local_key = "job_application_id", foreign_key = "id"))]
    job_application: Option<crate::JobApplication>,

// @source model.xml:229
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Interview {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            interview_date: String::new(),
            feedback: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            job_application_id: 0_u64,
            merchant_id: 0_u64,
            job_application: None,
            merchant: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Interview", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.job_application {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
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

    pub fn interview_date(&self) -> String {
        self.changed_interview_date().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.interview_date.clone())
    }

    pub fn update_interview_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.interview_date = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.interview_date.clone());
        self.root.set(self.entity_key(), "interview_date", value);
        self
    }

    pub fn changed_interview_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "interview_date")
    }

    pub fn eval_interview_date(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("interview_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "interview_date".to_string(), attempted_path: "interview_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.interview_date())
                }}

    pub fn feedback(&self) -> String {
        self.changed_feedback().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.feedback.clone())
    }

    pub fn update_feedback(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.feedback = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.feedback.clone());
        self.root.set(self.entity_key(), "feedback", value);
        self
    }

    pub fn changed_feedback(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "feedback")
    }

    pub fn eval_feedback(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("feedback") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "feedback".to_string(), attempted_path: "feedback".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.feedback())
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
    pub fn job_application_id(&self) -> u64 {
        self.changed_job_application_id().and_then(|value| value.try_u64()).unwrap_or(self.job_application_id)
    }

    pub fn update_job_application_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.job_application_id = value.try_u64().unwrap_or(self.job_application_id.clone());
        self.root.set(self.entity_key(), "job_application_id", value);
        self
    }

    pub fn changed_job_application_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "job_application_id")
    }

    pub fn eval_job_application_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("job_application_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_application_id".to_string(), attempted_path: "job_application_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.job_application_id())
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
    pub fn job_application(&self) -> Option<&crate::JobApplication> {
        self.job_application.as_ref()
    }

    pub fn eval_job_application(&self) -> teaql_core::eval::EvalResult<&crate::JobApplication> {
        if !self.is_loaded("job_application") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_application".to_string(), attempted_path: "job_application".to_string() }
        } else {
            match &self.job_application {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::InterviewRepository<'a>>>
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
            .interview_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Interview"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

