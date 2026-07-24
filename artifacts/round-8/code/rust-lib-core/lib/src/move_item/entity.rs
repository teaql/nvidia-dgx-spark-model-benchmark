// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/move_item
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "MoveItem", table = "move_item_data", data_service = "sqlite")]
pub struct MoveItem {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    item_name: String,

// @source model.xml:2
    weight_kg: String,
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

impl MoveItem {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            item_name: String::new(),
            weight_kg: String::new(),
            version: 0_i64,
            move_order_ref_id: 0_u64,
            move_order_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MoveItem", self.id)
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

    pub fn item_name(&self) -> String {
        self.changed_item_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.item_name.clone())
    }

    pub fn update_item_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.item_name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.item_name.clone());
        self.root.set(self.entity_key(), "item_name", value);
        self
    }

    pub fn changed_item_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "item_name")
    }

    pub fn eval_item_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("item_name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "item_name".to_string(), attempted_path: "item_name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.item_name())
                }}

    pub fn weight_kg(&self) -> String {
        self.changed_weight_kg().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.weight_kg.clone())
    }

    pub fn update_weight_kg(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.weight_kg = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.weight_kg.clone());
        self.root.set(self.entity_key(), "weight_kg", value);
        self
    }

    pub fn changed_weight_kg(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "weight_kg")
    }

    pub fn eval_weight_kg(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("weight_kg") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "weight_kg".to_string(), attempted_path: "weight_kg".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.weight_kg())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MoveItemRepository<'a>>>
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
            .move_item_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("MoveItem"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

