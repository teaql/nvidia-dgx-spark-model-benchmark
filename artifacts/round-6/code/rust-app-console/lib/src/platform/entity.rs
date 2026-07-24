// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/platform
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
#[teaql(entity = "Platform", table = "platform_data", data_service = "sqlite")]
pub struct Platform {
#[teaql(id)]
    id: u64,

// @source model.xml:41
    name: String,

// @source model.xml:41
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:41
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
#[teaql(relation(target = "RouteStatusType", local_key = "id", foreign_key = "platform_id", many))]
    route_status_type_list: SmartList<crate::RouteStatusType>,
#[teaql(relation(target = "InventoryConditionType", local_key = "id", foreign_key = "platform_id", many))]
    inventory_condition_type_list: SmartList<crate::InventoryConditionType>,
#[teaql(relation(target = "ExceptionSeverity", local_key = "id", foreign_key = "platform_id", many))]
    exception_severity_list: SmartList<crate::ExceptionSeverity>,
#[teaql(relation(target = "OrderStatus", local_key = "id", foreign_key = "platform_id", many))]
    order_status_list: SmartList<crate::OrderStatus>,
#[teaql(relation(target = "CrewRole", local_key = "id", foreign_key = "platform_id", many))]
    crew_role_list: SmartList<crate::CrewRole>,
#[teaql(relation(target = "Merchant", local_key = "id", foreign_key = "platform_id", many))]
    merchant_list: SmartList<crate::Merchant>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Platform {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            name: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            route_status_type_list: Default::default(),
            inventory_condition_type_list: Default::default(),
            exception_severity_list: Default::default(),
            order_status_list: Default::default(),
            crew_role_list: Default::default(),
            merchant_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Platform", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        for entity in &mut self.route_status_type_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.inventory_condition_type_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.exception_severity_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.order_status_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.crew_role_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.merchant_list {
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
    pub fn route_status_type_list(&self) -> &SmartList<crate::RouteStatusType> {
        &self.route_status_type_list
    }

    pub fn route_status_type_list_mut(&mut self) -> &mut SmartList<crate::RouteStatusType> {
        &mut self.route_status_type_list
    }

    pub fn eval_route_status_type_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RouteStatusType>> {
        if !self.is_loaded("route_status_type_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "route_status_type_list".to_string(), attempted_path: "route_status_type_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.route_status_type_list)
        }
    }

    pub fn inventory_condition_type_list(&self) -> &SmartList<crate::InventoryConditionType> {
        &self.inventory_condition_type_list
    }

    pub fn inventory_condition_type_list_mut(&mut self) -> &mut SmartList<crate::InventoryConditionType> {
        &mut self.inventory_condition_type_list
    }

    pub fn eval_inventory_condition_type_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::InventoryConditionType>> {
        if !self.is_loaded("inventory_condition_type_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "inventory_condition_type_list".to_string(), attempted_path: "inventory_condition_type_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.inventory_condition_type_list)
        }
    }

    pub fn exception_severity_list(&self) -> &SmartList<crate::ExceptionSeverity> {
        &self.exception_severity_list
    }

    pub fn exception_severity_list_mut(&mut self) -> &mut SmartList<crate::ExceptionSeverity> {
        &mut self.exception_severity_list
    }

    pub fn eval_exception_severity_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExceptionSeverity>> {
        if !self.is_loaded("exception_severity_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "exception_severity_list".to_string(), attempted_path: "exception_severity_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.exception_severity_list)
        }
    }

    pub fn order_status_list(&self) -> &SmartList<crate::OrderStatus> {
        &self.order_status_list
    }

    pub fn order_status_list_mut(&mut self) -> &mut SmartList<crate::OrderStatus> {
        &mut self.order_status_list
    }

    pub fn eval_order_status_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OrderStatus>> {
        if !self.is_loaded("order_status_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "order_status_list".to_string(), attempted_path: "order_status_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.order_status_list)
        }
    }

    pub fn crew_role_list(&self) -> &SmartList<crate::CrewRole> {
        &self.crew_role_list
    }

    pub fn crew_role_list_mut(&mut self) -> &mut SmartList<crate::CrewRole> {
        &mut self.crew_role_list
    }

    pub fn eval_crew_role_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CrewRole>> {
        if !self.is_loaded("crew_role_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "crew_role_list".to_string(), attempted_path: "crew_role_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.crew_role_list)
        }
    }

    pub fn merchant_list(&self) -> &SmartList<crate::Merchant> {
        &self.merchant_list
    }

    pub fn merchant_list_mut(&mut self) -> &mut SmartList<crate::Merchant> {
        &mut self.merchant_list
    }

    pub fn eval_merchant_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Merchant>> {
        if !self.is_loaded("merchant_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_list".to_string(), attempted_path: "merchant_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.merchant_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::PlatformRepository<'a>>>
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
            .platform_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Platform"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

