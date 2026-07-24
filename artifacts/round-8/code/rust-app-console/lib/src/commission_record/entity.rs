// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/commission_record
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "CommissionRecord", table = "commission_record_data", data_service = "sqlite")]
pub struct CommissionRecord {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    commission_id: String,

// @source model.xml:2
    percentage: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "employee_ref")]
    employee_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Employee", local_key = "employee_ref_id", foreign_key = "id"))]
    employee_ref: Option<crate::Employee>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl CommissionRecord {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            commission_id: String::new(),
            percentage: String::new(),
            version: 0_i64,
            employee_ref_id: 0_u64,
            employee_ref: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("CommissionRecord", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.employee_ref {
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

    pub fn commission_id(&self) -> String {
        self.changed_commission_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.commission_id.clone())
    }

    pub fn update_commission_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.commission_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.commission_id.clone());
        self.root.set(self.entity_key(), "commission_id", value);
        self
    }

    pub fn changed_commission_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "commission_id")
    }

    pub fn eval_commission_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("commission_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "commission_id".to_string(), attempted_path: "commission_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.commission_id())
                }}

    pub fn percentage(&self) -> String {
        self.changed_percentage().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.percentage.clone())
    }

    pub fn update_percentage(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.percentage = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.percentage.clone());
        self.root.set(self.entity_key(), "percentage", value);
        self
    }

    pub fn changed_percentage(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "percentage")
    }

    pub fn eval_percentage(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("percentage") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "percentage".to_string(), attempted_path: "percentage".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.percentage())
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
    pub fn employee_ref_id(&self) -> u64 {
        self.changed_employee_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.employee_ref_id)
    }

    pub fn update_employee_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employee_ref_id = value.try_u64().unwrap_or(self.employee_ref_id.clone());
        self.root.set(self.entity_key(), "employee_ref_id", value);
        self
    }

    pub fn changed_employee_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employee_ref_id")
    }

    pub fn eval_employee_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("employee_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_ref_id".to_string(), attempted_path: "employee_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employee_ref_id())
                }}
    pub fn employee_ref(&self) -> Option<&crate::Employee> {
        self.employee_ref.as_ref()
    }

    pub fn eval_employee_ref(&self) -> teaql_core::eval::EvalResult<&crate::Employee> {
        if !self.is_loaded("employee_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_ref".to_string(), attempted_path: "employee_ref".to_string() }
        } else {
            match &self.employee_ref {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CommissionRecordRepository<'a>>>
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
            .commission_record_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("CommissionRecord"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

