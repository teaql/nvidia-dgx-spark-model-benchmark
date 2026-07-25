// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/crew
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Crew", table = "crew_data", data_service = "sqlite", audit_mask_fields = "leader_phone")]
pub struct Crew {
#[teaql(id)]
    id: u64,

// @source module_0.xml:16
    crew_code: String,

// @source module_0.xml:16
    leader_phone: String,

// @source module_0.xml:16
    member_count: i64,

// @source module_0.xml:16
    is_active: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Crew {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            crew_code: String::new(),
            leader_phone: String::new(),
            member_count: 0_i64,
            is_active: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Crew", self.id)
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

    pub fn crew_code(&self) -> String {
        self.changed_crew_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.crew_code.clone())
    }

    pub fn update_crew_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.crew_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.crew_code.clone());
        self.root.set(self.entity_key(), "crew_code", value);
        self
    }

    pub fn changed_crew_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "crew_code")
    }

    pub fn eval_crew_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("crew_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "crew_code".to_string(), attempted_path: "crew_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.crew_code())
                }}

    pub fn leader_phone(&self) -> String {
        self.changed_leader_phone().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.leader_phone.clone())
    }

    pub fn update_leader_phone(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.leader_phone = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.leader_phone.clone());
        self.root.set(self.entity_key(), "leader_phone", value);
        self
    }

    pub fn changed_leader_phone(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "leader_phone")
    }

    pub fn eval_leader_phone(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("leader_phone") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "leader_phone".to_string(), attempted_path: "leader_phone".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.leader_phone())
                }}

    pub fn member_count(&self) -> i64 {
        self.changed_member_count().and_then(|value| value.try_i64()).map(|value| value as i64).unwrap_or(self.member_count)
    }

    pub fn update_member_count(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.member_count = value.try_i64().map(|value| value as i64).unwrap_or(self.member_count.clone());
        self.root.set(self.entity_key(), "member_count", value);
        self
    }

    pub fn changed_member_count(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "member_count")
    }

    pub fn eval_member_count(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("member_count") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "member_count".to_string(), attempted_path: "member_count".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.member_count())
                }}

    pub fn is_active(&self) -> String {
        self.changed_is_active().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.is_active.clone())
    }

    pub fn update_is_active(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.is_active = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.is_active.clone());
        self.root.set(self.entity_key(), "is_active", value);
        self
    }

    pub fn changed_is_active(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "is_active")
    }

    pub fn eval_is_active(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("is_active") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "is_active".to_string(), attempted_path: "is_active".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.is_active())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CrewRepository<'a>>>
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
            .crew_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Crew"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

