// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/competitor_analysis
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CompetitorAnalysis", table = "competitor_analysis_data", data_service = "sqlite")]
pub struct CompetitorAnalysis {
#[teaql(id)]
    id: u64,

// @source module_6.xml:12
    competitor: String,

// @source module_6.xml:12
    strengths: String,

// @source module_6.xml:12
    weaknesses: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CompetitorAnalysis {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            competitor: String::new(),
            strengths: String::new(),
            weaknesses: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CompetitorAnalysis", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
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

    pub fn competitor(&self) -> String {
        self.changed_competitor().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.competitor.clone())
    }

    pub fn update_competitor(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.competitor = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.competitor.clone());
        self.root.set(self.entity_key(), "competitor", value);
        self
    }

    pub fn changed_competitor(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "competitor")
    }

    pub fn eval_competitor(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("competitor") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "competitor".to_string(), attempted_path: "competitor".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.competitor())
                }}

    pub fn strengths(&self) -> String {
        self.changed_strengths().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.strengths.clone())
    }

    pub fn update_strengths(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.strengths = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.strengths.clone());
        self.root.set(self.entity_key(), "strengths", value);
        self
    }

    pub fn changed_strengths(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "strengths")
    }

    pub fn eval_strengths(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("strengths") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "strengths".to_string(), attempted_path: "strengths".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.strengths())
                }}

    pub fn weaknesses(&self) -> String {
        self.changed_weaknesses().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.weaknesses.clone())
    }

    pub fn update_weaknesses(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.weaknesses = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.weaknesses.clone());
        self.root.set(self.entity_key(), "weaknesses", value);
        self
    }

    pub fn changed_weaknesses(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "weaknesses")
    }

    pub fn eval_weaknesses(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("weaknesses") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "weaknesses".to_string(), attempted_path: "weaknesses".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.weaknesses())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CompetitorAnalysisRepository<'a>>>
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
            .competitor_analysis_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CompetitorAnalysis"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

