// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/leave_request
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "LeaveRequest", table = "leave_request_data", data_service = "sqlite")]
pub struct LeaveRequest {
#[teaql(id)]
    id: u64,

// @source model.xml:106
    start_date: String,

// @source model.xml:106
    end_date: String,

// @source model.xml:106
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:106
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:106
#[teaql(column = "leave_type")]
    leave_type_id: u64,

// @source model.xml:106
#[teaql(column = "employee")]
    employee_id: u64,

// @source model.xml:106
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:106
#[teaql(relation(target = "LeaveType", local_key = "leave_type_id", foreign_key = "id"))]
    leave_type: Option<crate::LeaveType>,

// @source model.xml:106
#[teaql(relation(target = "Employee", local_key = "employee_id", foreign_key = "id"))]
    employee: Option<crate::Employee>,

// @source model.xml:106
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl LeaveRequest {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            start_date: String::new(),
            end_date: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            leave_type_id: 0_u64,
            employee_id: 0_u64,
            merchant_id: 0_u64,
            leave_type: None,
            employee: None,
            merchant: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("LeaveRequest", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.leave_type {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.employee {
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

    pub fn start_date(&self) -> String {
        self.changed_start_date().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.start_date.clone())
    }

    pub fn update_start_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.start_date = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.start_date.clone());
        self.root.set(self.entity_key(), "start_date", value);
        self
    }

    pub fn changed_start_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "start_date")
    }

    pub fn eval_start_date(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("start_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "start_date".to_string(), attempted_path: "start_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.start_date())
                }}

    pub fn end_date(&self) -> String {
        self.changed_end_date().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.end_date.clone())
    }

    pub fn update_end_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.end_date = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.end_date.clone());
        self.root.set(self.entity_key(), "end_date", value);
        self
    }

    pub fn changed_end_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "end_date")
    }

    pub fn eval_end_date(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("end_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "end_date".to_string(), attempted_path: "end_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.end_date())
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
    pub fn leave_type_id(&self) -> u64 {
        self.changed_leave_type_id().and_then(|value| value.try_u64()).unwrap_or(self.leave_type_id)
    }

    pub(crate) fn update_leave_type_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.leave_type_id = value.try_u64().unwrap_or(self.leave_type_id.clone());
        self.root.set(self.entity_key(), "leave_type_id", value);
        self
    }

    pub fn changed_leave_type_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "leave_type_id")
    }

    pub fn eval_leave_type_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("leave_type_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_type_id".to_string(), attempted_path: "leave_type_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.leave_type_id())
                }}

    pub fn employee_id(&self) -> u64 {
        self.changed_employee_id().and_then(|value| value.try_u64()).unwrap_or(self.employee_id)
    }

    pub fn update_employee_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employee_id = value.try_u64().unwrap_or(self.employee_id.clone());
        self.root.set(self.entity_key(), "employee_id", value);
        self
    }

    pub fn changed_employee_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employee_id")
    }

    pub fn eval_employee_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("employee_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_id".to_string(), attempted_path: "employee_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employee_id())
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
    pub fn update_leave_type_to_annual(&mut self) -> &mut Self {
        self.update_leave_type_id(1001_u64)
    }

    pub fn leave_type_is_annual(&self) -> bool {
        self.leave_type_id() == 1001_u64
    }
    pub fn update_leave_type_to_sick(&mut self) -> &mut Self {
        self.update_leave_type_id(1002_u64)
    }

    pub fn leave_type_is_sick(&self) -> bool {
        self.leave_type_id() == 1002_u64
    }
    pub fn update_leave_type_to_unpaid(&mut self) -> &mut Self {
        self.update_leave_type_id(1003_u64)
    }

    pub fn leave_type_is_unpaid(&self) -> bool {
        self.leave_type_id() == 1003_u64
    }
    pub fn leave_type(&self) -> Option<&crate::LeaveType> {
        self.leave_type.as_ref()
    }

    pub fn eval_leave_type(&self) -> teaql_core::eval::EvalResult<&crate::LeaveType> {
        if !self.is_loaded("leave_type") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_type".to_string(), attempted_path: "leave_type".to_string() }
        } else {
            match &self.leave_type {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn employee(&self) -> Option<&crate::Employee> {
        self.employee.as_ref()
    }

    pub fn eval_employee(&self) -> teaql_core::eval::EvalResult<&crate::Employee> {
        if !self.is_loaded("employee") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee".to_string(), attempted_path: "employee".to_string() }
        } else {
            match &self.employee {
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::LeaveRequestRepository<'a>>>
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
            .leave_request_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("LeaveRequest"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

