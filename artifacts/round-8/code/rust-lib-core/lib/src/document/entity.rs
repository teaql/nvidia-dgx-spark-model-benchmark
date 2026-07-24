// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/document
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
#[teaql(entity = "Document", table = "document_data", data_service = "sqlite")]
pub struct Document {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    document_id: String,

// @source model.xml:2
    name: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "contract_ref")]
    contract_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Contract", local_key = "contract_ref_id", foreign_key = "id"))]
    contract_ref: Option<crate::Contract>,
#[teaql(relation(target = "DocumentVersion", local_key = "id", foreign_key = "document_ref_id", many))]
    document_version_list: SmartList<crate::DocumentVersion>,
#[teaql(relation(target = "RecoveryRequest", local_key = "id", foreign_key = "document_ref_id", many))]
    recovery_request_list: SmartList<crate::RecoveryRequest>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Document {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            document_id: String::new(),
            name: String::new(),
            version: 0_i64,
            contract_ref_id: 0_u64,
            contract_ref: None,
            document_version_list: Default::default(),
            recovery_request_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Document", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.contract_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.document_version_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.recovery_request_list {
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

    pub fn document_id(&self) -> String {
        self.changed_document_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.document_id.clone())
    }

    pub fn update_document_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.document_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.document_id.clone());
        self.root.set(self.entity_key(), "document_id", value);
        self
    }

    pub fn changed_document_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "document_id")
    }

    pub fn eval_document_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("document_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "document_id".to_string(), attempted_path: "document_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.document_id())
                }}

    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.name.clone());
        self.root.set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
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
    pub fn contract_ref_id(&self) -> u64 {
        self.changed_contract_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.contract_ref_id)
    }

    pub fn update_contract_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.contract_ref_id = value.try_u64().unwrap_or(self.contract_ref_id.clone());
        self.root.set(self.entity_key(), "contract_ref_id", value);
        self
    }

    pub fn changed_contract_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "contract_ref_id")
    }

    pub fn eval_contract_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("contract_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "contract_ref_id".to_string(), attempted_path: "contract_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.contract_ref_id())
                }}
    pub fn contract_ref(&self) -> Option<&crate::Contract> {
        self.contract_ref.as_ref()
    }

    pub fn eval_contract_ref(&self) -> teaql_core::eval::EvalResult<&crate::Contract> {
        if !self.is_loaded("contract_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "contract_ref".to_string(), attempted_path: "contract_ref".to_string() }
        } else {
            match &self.contract_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn document_version_list(&self) -> &SmartList<crate::DocumentVersion> {
        &self.document_version_list
    }

    pub fn document_version_list_mut(&mut self) -> &mut SmartList<crate::DocumentVersion> {
        &mut self.document_version_list
    }

    pub fn eval_document_version_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DocumentVersion>> {
        if !self.is_loaded("document_version_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "document_version_list".to_string(), attempted_path: "document_version_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.document_version_list)
        }
    }

    pub fn recovery_request_list(&self) -> &SmartList<crate::RecoveryRequest> {
        &self.recovery_request_list
    }

    pub fn recovery_request_list_mut(&mut self) -> &mut SmartList<crate::RecoveryRequest> {
        &mut self.recovery_request_list
    }

    pub fn eval_recovery_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RecoveryRequest>> {
        if !self.is_loaded("recovery_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "recovery_request_list".to_string(), attempted_path: "recovery_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.recovery_request_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::DocumentRepository<'a>>>
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
            .document_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Document"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

