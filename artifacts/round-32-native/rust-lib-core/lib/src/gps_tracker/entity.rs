// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/gps_tracker
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "GpsTracker", table = "gps_tracker_data", data_service = "sqlite")]
pub struct GpsTracker {
#[teaql(id)]
    id: u64,

// @source module_8.xml:10
    device_imei: String,

// @source module_8.xml:10
    serial_number: String,

// @source module_8.xml:10
    installed_at: chrono::DateTime<chrono::Utc>,

// @source module_8.xml:10
    status: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl GpsTracker {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            device_imei: String::new(),
            serial_number: String::new(),
            installed_at: chrono::Utc::now(),
            status: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("GpsTracker", self.id)
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

    pub fn device_imei(&self) -> String {
        self.changed_device_imei().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.device_imei.clone())
    }

    pub fn update_device_imei(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.device_imei = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.device_imei.clone());
        self.root.set(self.entity_key(), "device_imei", value);
        self
    }

    pub fn changed_device_imei(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "device_imei")
    }

    pub fn eval_device_imei(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("device_imei") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "device_imei".to_string(), attempted_path: "device_imei".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.device_imei())
                }}

    pub fn serial_number(&self) -> String {
        self.changed_serial_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.serial_number.clone())
    }

    pub fn update_serial_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.serial_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.serial_number.clone());
        self.root.set(self.entity_key(), "serial_number", value);
        self
    }

    pub fn changed_serial_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "serial_number")
    }

    pub fn eval_serial_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("serial_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "serial_number".to_string(), attempted_path: "serial_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.serial_number())
                }}

    pub fn installed_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_installed_at().and_then(|value| value.try_timestamp()).unwrap_or(self.installed_at)
    }

    pub fn update_installed_at(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.installed_at = value.try_timestamp().unwrap_or(self.installed_at.clone());
        self.root.set(self.entity_key(), "installed_at", value);
        self
    }

    pub fn changed_installed_at(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "installed_at")
    }

    pub fn eval_installed_at(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("installed_at") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "installed_at".to_string(), attempted_path: "installed_at".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.installed_at())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::GpsTrackerRepository<'a>>>
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
            .gps_tracker_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("GpsTracker"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

