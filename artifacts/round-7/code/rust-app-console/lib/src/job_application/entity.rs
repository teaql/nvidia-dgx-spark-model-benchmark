// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/job_application
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
#[teaql(entity = "JobApplication", table = "job_application_data", data_service = "sqlite")]
pub struct JobApplication {
#[teaql(id)]
    id: u64,

// @source model.xml:221
    candidate_name: String,

// @source model.xml:221
    resume_url: String,

// @source model.xml:221
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:221
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:221
#[teaql(column = "status")]
    status_id: u64,

// @source model.xml:221
#[teaql(column = "recruitment_post")]
    recruitment_post_id: u64,

// @source model.xml:221
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:221
#[teaql(relation(target = "ApplicationStatus", local_key = "status_id", foreign_key = "id"))]
    status: Option<crate::ApplicationStatus>,

// @source model.xml:221
#[teaql(relation(target = "RecruitmentPost", local_key = "recruitment_post_id", foreign_key = "id"))]
    recruitment_post: Option<crate::RecruitmentPost>,

// @source model.xml:221
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
#[teaql(relation(target = "Interview", local_key = "id", foreign_key = "job_application_id", many))]
    interview_list: SmartList<crate::Interview>,
#[teaql(relation(target = "OfferLetter", local_key = "id", foreign_key = "job_application_id", many))]
    offer_letter_list: SmartList<crate::OfferLetter>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl JobApplication {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            candidate_name: String::new(),
            resume_url: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            status_id: 0_u64,
            recruitment_post_id: 0_u64,
            merchant_id: 0_u64,
            status: None,
            recruitment_post: None,
            merchant: None,
            interview_list: Default::default(),
            offer_letter_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("JobApplication", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.status {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.recruitment_post {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.interview_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.offer_letter_list {
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

    pub fn candidate_name(&self) -> String {
        self.changed_candidate_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.candidate_name.clone())
    }

    pub fn update_candidate_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.candidate_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.candidate_name.clone());
        self.root.set(self.entity_key(), "candidate_name", value);
        self
    }

    pub fn changed_candidate_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "candidate_name")
    }

    pub fn eval_candidate_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("candidate_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "candidate_name".to_string(), attempted_path: "candidate_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.candidate_name())
                }}

    pub fn resume_url(&self) -> String {
        self.changed_resume_url().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.resume_url.clone())
    }

    pub fn update_resume_url(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.resume_url = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.resume_url.clone());
        self.root.set(self.entity_key(), "resume_url", value);
        self
    }

    pub fn changed_resume_url(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "resume_url")
    }

    pub fn eval_resume_url(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("resume_url") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "resume_url".to_string(), attempted_path: "resume_url".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.resume_url())
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
    pub fn status_id(&self) -> u64 {
        self.changed_status_id().and_then(|value| value.try_u64()).unwrap_or(self.status_id)
    }

    pub(crate) fn update_status_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status_id = value.try_u64().unwrap_or(self.status_id.clone());
        self.root.set(self.entity_key(), "status_id", value);
        self
    }

    pub fn changed_status_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status_id")
    }

    pub fn eval_status_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("status_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status_id())
                }}

    pub fn recruitment_post_id(&self) -> u64 {
        self.changed_recruitment_post_id().and_then(|value| value.try_u64()).unwrap_or(self.recruitment_post_id)
    }

    pub fn update_recruitment_post_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.recruitment_post_id = value.try_u64().unwrap_or(self.recruitment_post_id.clone());
        self.root.set(self.entity_key(), "recruitment_post_id", value);
        self
    }

    pub fn changed_recruitment_post_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "recruitment_post_id")
    }

    pub fn eval_recruitment_post_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("recruitment_post_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "recruitment_post_id".to_string(), attempted_path: "recruitment_post_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.recruitment_post_id())
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
    pub fn update_status_to_applied(&mut self) -> &mut Self {
        self.update_status_id(1001_u64)
    }

    pub fn status_is_applied(&self) -> bool {
        self.status_id() == 1001_u64
    }
    pub fn update_status_to_interviewing(&mut self) -> &mut Self {
        self.update_status_id(1002_u64)
    }

    pub fn status_is_interviewing(&self) -> bool {
        self.status_id() == 1002_u64
    }
    pub fn update_status_to_offered(&mut self) -> &mut Self {
        self.update_status_id(1003_u64)
    }

    pub fn status_is_offered(&self) -> bool {
        self.status_id() == 1003_u64
    }
    pub fn status(&self) -> Option<&crate::ApplicationStatus> {
        self.status.as_ref()
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<&crate::ApplicationStatus> {
        if !self.is_loaded("status") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
        } else {
            match &self.status {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn recruitment_post(&self) -> Option<&crate::RecruitmentPost> {
        self.recruitment_post.as_ref()
    }

    pub fn eval_recruitment_post(&self) -> teaql_core::eval::EvalResult<&crate::RecruitmentPost> {
        if !self.is_loaded("recruitment_post") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "recruitment_post".to_string(), attempted_path: "recruitment_post".to_string() }
        } else {
            match &self.recruitment_post {
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
    pub fn interview_list(&self) -> &SmartList<crate::Interview> {
        &self.interview_list
    }

    pub fn interview_list_mut(&mut self) -> &mut SmartList<crate::Interview> {
        &mut self.interview_list
    }

    pub fn eval_interview_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Interview>> {
        if !self.is_loaded("interview_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "interview_list".to_string(), attempted_path: "interview_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.interview_list)
        }
    }

    pub fn offer_letter_list(&self) -> &SmartList<crate::OfferLetter> {
        &self.offer_letter_list
    }

    pub fn offer_letter_list_mut(&mut self) -> &mut SmartList<crate::OfferLetter> {
        &mut self.offer_letter_list
    }

    pub fn eval_offer_letter_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OfferLetter>> {
        if !self.is_loaded("offer_letter_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "offer_letter_list".to_string(), attempted_path: "offer_letter_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.offer_letter_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::JobApplicationRepository<'a>>>
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
            .job_application_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("JobApplication"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

