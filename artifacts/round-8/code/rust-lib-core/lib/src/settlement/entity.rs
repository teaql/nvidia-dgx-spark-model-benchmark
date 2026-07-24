// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/settlement
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Settlement", table = "settlement_data", data_service = "sqlite")]
pub struct Settlement {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    settlement_id: String,

// @source model.xml:2
    status: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "payment_ref")]
    payment_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Payment", local_key = "payment_ref_id", foreign_key = "id"))]
    payment_ref: Option<crate::Payment>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Settlement {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            settlement_id: String::new(),
            status: String::new(),
            version: 0_i64,
            payment_ref_id: 0_u64,
            payment_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Settlement", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.payment_ref {
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

    pub fn settlement_id(&self) -> String {
        self.changed_settlement_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.settlement_id.clone())
    }

    pub fn update_settlement_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.settlement_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.settlement_id.clone());
        self.root.set(self.entity_key(), "settlement_id", value);
        self
    }

    pub fn changed_settlement_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "settlement_id")
    }

    pub fn eval_settlement_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("settlement_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "settlement_id".to_string(), attempted_path: "settlement_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.settlement_id())
                }}

    pub fn status(&self) -> String {
        self.changed_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.status.clone())
    }

    pub fn update_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.status.clone());
        self.root.set(self.entity_key(), "status", value);
        self
    }

    pub fn changed_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status")
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status())
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
    pub fn payment_ref_id(&self) -> u64 {
        self.changed_payment_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.payment_ref_id)
    }

    pub fn update_payment_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.payment_ref_id = value.try_u64().unwrap_or(self.payment_ref_id.clone());
        self.root.set(self.entity_key(), "payment_ref_id", value);
        self
    }

    pub fn changed_payment_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "payment_ref_id")
    }

    pub fn eval_payment_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("payment_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "payment_ref_id".to_string(), attempted_path: "payment_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.payment_ref_id())
                }}
    pub fn payment_ref(&self) -> Option<&crate::Payment> {
        self.payment_ref.as_ref()
    }

    pub fn eval_payment_ref(&self) -> teaql_core::eval::EvalResult<&crate::Payment> {
        if !self.is_loaded("payment_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payment_ref".to_string(), attempted_path: "payment_ref".to_string() }
        } else {
            match &self.payment_ref {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::SettlementRepository<'a>>>
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
            .settlement_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Settlement"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

