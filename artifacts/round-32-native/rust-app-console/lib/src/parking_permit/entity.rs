// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/parking_permit
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "ParkingPermit", table = "parking_permit_data", data_service = "sqlite")]
pub struct ParkingPermit {
#[teaql(id)]
    id: u64,

// @source module_1.xml:9
    location: String,

// @source module_1.xml:9
    valid_from: chrono::DateTime<chrono::Utc>,

// @source module_1.xml:9
    valid_to: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl ParkingPermit {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            location: String::new(),
            valid_from: chrono::Utc::now(),
            valid_to: chrono::Utc::now(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("ParkingPermit", self.id)
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

    pub fn location(&self) -> String {
        self.changed_location().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.location.clone())
    }

    pub fn update_location(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.location = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.location.clone());
        self.root.set(self.entity_key(), "location", value);
        self
    }

    pub fn changed_location(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "location")
    }

    pub fn eval_location(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("location") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "location".to_string(), attempted_path: "location".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.location())
                }}

    pub fn valid_from(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_valid_from().and_then(|value| value.try_timestamp()).unwrap_or(self.valid_from)
    }

    pub fn update_valid_from(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.valid_from = value.try_timestamp().unwrap_or(self.valid_from.clone());
        self.root.set(self.entity_key(), "valid_from", value);
        self
    }

    pub fn changed_valid_from(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "valid_from")
    }

    pub fn eval_valid_from(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("valid_from") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "valid_from".to_string(), attempted_path: "valid_from".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.valid_from())
                }}

    pub fn valid_to(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_valid_to().and_then(|value| value.try_timestamp()).unwrap_or(self.valid_to)
    }

    pub fn update_valid_to(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.valid_to = value.try_timestamp().unwrap_or(self.valid_to.clone());
        self.root.set(self.entity_key(), "valid_to", value);
        self
    }

    pub fn changed_valid_to(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "valid_to")
    }

    pub fn eval_valid_to(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("valid_to") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "valid_to".to_string(), attempted_path: "valid_to".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.valid_to())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ParkingPermitRepository<'a>>>
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
            .parking_permit_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("ParkingPermit"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

