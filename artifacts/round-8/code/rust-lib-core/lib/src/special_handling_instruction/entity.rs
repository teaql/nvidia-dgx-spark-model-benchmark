// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/special_handling_instruction
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "SpecialHandlingInstruction", table = "special_handling_instruction_data", data_service = "sqlite")]
pub struct SpecialHandlingInstruction {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    instruction: String,

// @source model.xml:2
    applies_to: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "move_order_ref")]
    move_order_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "MoveOrder", local_key = "move_order_ref_id", foreign_key = "id"))]
    move_order_ref: Option<crate::MoveOrder>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl SpecialHandlingInstruction {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            instruction: String::new(),
            applies_to: String::new(),
            version: 0_i64,
            move_order_ref_id: 0_u64,
            move_order_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("SpecialHandlingInstruction", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.move_order_ref {
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

    pub fn instruction(&self) -> String {
        self.changed_instruction().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.instruction.clone())
    }

    pub fn update_instruction(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.instruction = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.instruction.clone());
        self.root.set(self.entity_key(), "instruction", value);
        self
    }

    pub fn changed_instruction(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "instruction")
    }

    pub fn eval_instruction(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("instruction") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "instruction".to_string(), attempted_path: "instruction".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.instruction())
                }}

    pub fn applies_to(&self) -> String {
        self.changed_applies_to().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.applies_to.clone())
    }

    pub fn update_applies_to(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.applies_to = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.applies_to.clone());
        self.root.set(self.entity_key(), "applies_to", value);
        self
    }

    pub fn changed_applies_to(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "applies_to")
    }

    pub fn eval_applies_to(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("applies_to") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "applies_to".to_string(), attempted_path: "applies_to".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.applies_to())
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
    pub fn move_order_ref_id(&self) -> u64 {
        self.changed_move_order_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.move_order_ref_id)
    }

    pub fn update_move_order_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.move_order_ref_id = value.try_u64().unwrap_or(self.move_order_ref_id.clone());
        self.root.set(self.entity_key(), "move_order_ref_id", value);
        self
    }

    pub fn changed_move_order_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "move_order_ref_id")
    }

    pub fn eval_move_order_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("move_order_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_order_ref_id".to_string(), attempted_path: "move_order_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.move_order_ref_id())
                }}
    pub fn move_order_ref(&self) -> Option<&crate::MoveOrder> {
        self.move_order_ref.as_ref()
    }

    pub fn eval_move_order_ref(&self) -> teaql_core::eval::EvalResult<&crate::MoveOrder> {
        if !self.is_loaded("move_order_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_order_ref".to_string(), attempted_path: "move_order_ref".to_string() }
        } else {
            match &self.move_order_ref {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::SpecialHandlingInstructionRepository<'a>>>
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
            .special_handling_instruction_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("SpecialHandlingInstruction"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

