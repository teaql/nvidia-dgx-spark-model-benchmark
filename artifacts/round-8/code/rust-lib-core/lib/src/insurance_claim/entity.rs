// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/insurance_claim
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "InsuranceClaim", table = "insurance_claim_data", data_service = "sqlite")]
pub struct InsuranceClaim {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    claim_id: String,

// @source model.xml:2
    amount: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "insurance_policy_ref")]
    insurance_policy_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "InsurancePolicy", local_key = "insurance_policy_ref_id", foreign_key = "id"))]
    insurance_policy_ref: Option<crate::InsurancePolicy>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl InsuranceClaim {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            claim_id: String::new(),
            amount: String::new(),
            version: 0_i64,
            insurance_policy_ref_id: 0_u64,
            insurance_policy_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("InsuranceClaim", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.insurance_policy_ref {
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

    pub fn claim_id(&self) -> String {
        self.changed_claim_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.claim_id.clone())
    }

    pub fn update_claim_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.claim_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.claim_id.clone());
        self.root.set(self.entity_key(), "claim_id", value);
        self
    }

    pub fn changed_claim_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "claim_id")
    }

    pub fn eval_claim_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("claim_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "claim_id".to_string(), attempted_path: "claim_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.claim_id())
                }}

    pub fn amount(&self) -> String {
        self.changed_amount().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.amount.clone())
    }

    pub fn update_amount(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.amount = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.amount.clone());
        self.root.set(self.entity_key(), "amount", value);
        self
    }

    pub fn changed_amount(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "amount")
    }

    pub fn eval_amount(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("amount") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "amount".to_string(), attempted_path: "amount".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.amount())
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
    pub fn insurance_policy_ref_id(&self) -> u64 {
        self.changed_insurance_policy_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.insurance_policy_ref_id)
    }

    pub fn update_insurance_policy_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.insurance_policy_ref_id = value.try_u64().unwrap_or(self.insurance_policy_ref_id.clone());
        self.root.set(self.entity_key(), "insurance_policy_ref_id", value);
        self
    }

    pub fn changed_insurance_policy_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "insurance_policy_ref_id")
    }

    pub fn eval_insurance_policy_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("insurance_policy_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "insurance_policy_ref_id".to_string(), attempted_path: "insurance_policy_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.insurance_policy_ref_id())
                }}
    pub fn insurance_policy_ref(&self) -> Option<&crate::InsurancePolicy> {
        self.insurance_policy_ref.as_ref()
    }

    pub fn eval_insurance_policy_ref(&self) -> teaql_core::eval::EvalResult<&crate::InsurancePolicy> {
        if !self.is_loaded("insurance_policy_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "insurance_policy_ref".to_string(), attempted_path: "insurance_policy_ref".to_string() }
        } else {
            match &self.insurance_policy_ref {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::InsuranceClaimRepository<'a>>>
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
            .insurance_claim_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("InsuranceClaim"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

