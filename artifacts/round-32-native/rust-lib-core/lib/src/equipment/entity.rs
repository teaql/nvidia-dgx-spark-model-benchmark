// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/equipment
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Equipment", table = "equipment_data", data_service = "sqlite")]
pub struct Equipment {
#[teaql(id)]
    id: u64,

// @source module_8.xml:2
    serial_number: String,

// @source module_8.xml:2
    model_number: String,

// @source module_8.xml:2
    make: String,

// @source module_8.xml:2
    purchase_date: chrono::NaiveDate,

// @source module_8.xml:2
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

impl Equipment {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            serial_number: String::new(),
            model_number: String::new(),
            make: String::new(),
            purchase_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            status: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Equipment", self.id)
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

    pub fn model_number(&self) -> String {
        self.changed_model_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.model_number.clone())
    }

    pub fn update_model_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.model_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.model_number.clone());
        self.root.set(self.entity_key(), "model_number", value);
        self
    }

    pub fn changed_model_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "model_number")
    }

    pub fn eval_model_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("model_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "model_number".to_string(), attempted_path: "model_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.model_number())
                }}

    pub fn make(&self) -> String {
        self.changed_make().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.make.clone())
    }

    pub fn update_make(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.make = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.make.clone());
        self.root.set(self.entity_key(), "make", value);
        self
    }

    pub fn changed_make(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "make")
    }

    pub fn eval_make(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("make") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "make".to_string(), attempted_path: "make".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.make())
                }}

    pub fn purchase_date(&self) -> chrono::NaiveDate {
        self.changed_purchase_date().and_then(|value| value.try_date()).unwrap_or(self.purchase_date)
    }

    pub fn update_purchase_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.purchase_date = value.try_date().unwrap_or(self.purchase_date.clone());
        self.root.set(self.entity_key(), "purchase_date", value);
        self
    }

    pub fn changed_purchase_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "purchase_date")
    }

    pub fn eval_purchase_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("purchase_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "purchase_date".to_string(), attempted_path: "purchase_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.purchase_date())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EquipmentRepository<'a>>>
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
            .equipment_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Equipment"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

