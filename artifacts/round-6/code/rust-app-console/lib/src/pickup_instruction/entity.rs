// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/pickup_instruction
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "PickupInstruction", table = "pickup_instruction_data", data_service = "sqlite")]
pub struct PickupInstruction {
#[teaql(id)]
    id: u64,

// @source model.xml:150
    instruction_text: String,

// @source model.xml:150
    gate_code: String,

// @source model.xml:150
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:150
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:150
#[teaql(column = "route_stop")]
    route_stop_id: u64,

// @source model.xml:150
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:150
#[teaql(relation(target = "RouteStop", local_key = "route_stop_id", foreign_key = "id"))]
    route_stop: Option<crate::RouteStop>,

// @source model.xml:150
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl PickupInstruction {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            instruction_text: String::new(),
            gate_code: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            route_stop_id: 0_u64,
            merchant_id: 0_u64,
            route_stop: None,
            merchant: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("PickupInstruction", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.route_stop {
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

    pub fn instruction_text(&self) -> String {
        self.changed_instruction_text().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.instruction_text.clone())
    }

    pub fn update_instruction_text(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.instruction_text = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.instruction_text.clone());
        self.root.set(self.entity_key(), "instruction_text", value);
        self
    }

    pub fn changed_instruction_text(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "instruction_text")
    }

    pub fn eval_instruction_text(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("instruction_text") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "instruction_text".to_string(), attempted_path: "instruction_text".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.instruction_text())
                }}

    pub fn gate_code(&self) -> String {
        self.changed_gate_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.gate_code.clone())
    }

    pub fn update_gate_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.gate_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.gate_code.clone());
        self.root.set(self.entity_key(), "gate_code", value);
        self
    }

    pub fn changed_gate_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "gate_code")
    }

    pub fn eval_gate_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("gate_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "gate_code".to_string(), attempted_path: "gate_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.gate_code())
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
    pub fn route_stop_id(&self) -> u64 {
        self.changed_route_stop_id().and_then(|value| value.try_u64()).unwrap_or(self.route_stop_id)
    }

    pub fn update_route_stop_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.route_stop_id = value.try_u64().unwrap_or(self.route_stop_id.clone());
        self.root.set(self.entity_key(), "route_stop_id", value);
        self
    }

    pub fn changed_route_stop_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "route_stop_id")
    }

    pub fn eval_route_stop_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("route_stop_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "route_stop_id".to_string(), attempted_path: "route_stop_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.route_stop_id())
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
    pub fn route_stop(&self) -> Option<&crate::RouteStop> {
        self.route_stop.as_ref()
    }

    pub fn eval_route_stop(&self) -> teaql_core::eval::EvalResult<&crate::RouteStop> {
        if !self.is_loaded("route_stop") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "route_stop".to_string(), attempted_path: "route_stop".to_string() }
        } else {
            match &self.route_stop {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::PickupInstructionRepository<'a>>>
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
            .pickup_instruction_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("PickupInstruction"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

