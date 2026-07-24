// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/route_stop
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
#[teaql(entity = "RouteStop", table = "route_stop_data", data_service = "sqlite")]
pub struct RouteStop {
#[teaql(id)]
    id: u64,

// @source model.xml:79
    stop_sequence: String,

// @source model.xml:79
    address: String,

// @source model.xml:79
    arrival_time: String,

// @source model.xml:79
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:79
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:79
#[teaql(column = "status")]
    status_id: u64,

// @source model.xml:79
#[teaql(column = "move_order")]
    move_order_id: u64,

// @source model.xml:79
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:79
#[teaql(relation(target = "RouteStatusType", local_key = "status_id", foreign_key = "id"))]
    status: Option<crate::RouteStatusType>,

// @source model.xml:79
#[teaql(relation(target = "MoveOrder", local_key = "move_order_id", foreign_key = "id"))]
    move_order: Option<crate::MoveOrder>,

// @source model.xml:79
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
#[teaql(relation(target = "PickupInstruction", local_key = "id", foreign_key = "route_stop_id", many))]
    pickup_instruction_list: SmartList<crate::PickupInstruction>,
#[teaql(relation(target = "DeliveryInstruction", local_key = "id", foreign_key = "route_stop_id", many))]
    delivery_instruction_list: SmartList<crate::DeliveryInstruction>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl RouteStop {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            stop_sequence: String::new(),
            address: String::new(),
            arrival_time: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            status_id: 0_u64,
            move_order_id: 0_u64,
            merchant_id: 0_u64,
            status: None,
            move_order: None,
            merchant: None,
            pickup_instruction_list: Default::default(),
            delivery_instruction_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("RouteStop", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.status {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.move_order {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.pickup_instruction_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.delivery_instruction_list {
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

    pub fn stop_sequence(&self) -> String {
        self.changed_stop_sequence().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.stop_sequence.clone())
    }

    pub fn update_stop_sequence(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.stop_sequence = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.stop_sequence.clone());
        self.root.set(self.entity_key(), "stop_sequence", value);
        self
    }

    pub fn changed_stop_sequence(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "stop_sequence")
    }

    pub fn eval_stop_sequence(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("stop_sequence") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "stop_sequence".to_string(), attempted_path: "stop_sequence".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.stop_sequence())
                }}

    pub fn address(&self) -> String {
        self.changed_address().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.address.clone())
    }

    pub fn update_address(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.address = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.address.clone());
        self.root.set(self.entity_key(), "address", value);
        self
    }

    pub fn changed_address(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "address")
    }

    pub fn eval_address(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("address") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "address".to_string(), attempted_path: "address".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.address())
                }}

    pub fn arrival_time(&self) -> String {
        self.changed_arrival_time().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.arrival_time.clone())
    }

    pub fn update_arrival_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.arrival_time = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.arrival_time.clone());
        self.root.set(self.entity_key(), "arrival_time", value);
        self
    }

    pub fn changed_arrival_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "arrival_time")
    }

    pub fn eval_arrival_time(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("arrival_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "arrival_time".to_string(), attempted_path: "arrival_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.arrival_time())
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
    pub fn status_id(&self) -> u64 {
        self.changed_status_id().and_then(|value| value.try_u64()).unwrap_or(self.status_id)
    }

    pub(crate) fn update_status_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status_id = value.try_u64().unwrap_or(self.status_id.clone());
        self.root.set(self.entity_key(), "status_id", value);
        self
    }

    pub fn changed_status_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status_id")
    }

    pub fn eval_status_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("status_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status_id".to_string(), attempted_path: "status_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status_id())
                }}

    pub fn move_order_id(&self) -> u64 {
        self.changed_move_order_id().and_then(|value| value.try_u64()).unwrap_or(self.move_order_id)
    }

    pub fn update_move_order_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.move_order_id = value.try_u64().unwrap_or(self.move_order_id.clone());
        self.root.set(self.entity_key(), "move_order_id", value);
        self
    }

    pub fn changed_move_order_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "move_order_id")
    }

    pub fn eval_move_order_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("move_order_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_order_id".to_string(), attempted_path: "move_order_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.move_order_id())
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
    pub fn update_status_to_pending(&mut self) -> &mut Self {
        self.update_status_id(1001_u64)
    }

    pub fn status_is_pending(&self) -> bool {
        self.status_id() == 1001_u64
    }
    pub fn update_status_to_in_progress(&mut self) -> &mut Self {
        self.update_status_id(1002_u64)
    }

    pub fn status_is_in_progress(&self) -> bool {
        self.status_id() == 1002_u64
    }
    pub fn update_status_to_completed(&mut self) -> &mut Self {
        self.update_status_id(1003_u64)
    }

    pub fn status_is_completed(&self) -> bool {
        self.status_id() == 1003_u64
    }
    pub fn status(&self) -> Option<&crate::RouteStatusType> {
        self.status.as_ref()
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<&crate::RouteStatusType> {
        if !self.is_loaded("status") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
        } else {
            match &self.status {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn move_order(&self) -> Option<&crate::MoveOrder> {
        self.move_order.as_ref()
    }

    pub fn eval_move_order(&self) -> teaql_core::eval::EvalResult<&crate::MoveOrder> {
        if !self.is_loaded("move_order") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_order".to_string(), attempted_path: "move_order".to_string() }
        } else {
            match &self.move_order {
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
    pub fn pickup_instruction_list(&self) -> &SmartList<crate::PickupInstruction> {
        &self.pickup_instruction_list
    }

    pub fn pickup_instruction_list_mut(&mut self) -> &mut SmartList<crate::PickupInstruction> {
        &mut self.pickup_instruction_list
    }

    pub fn eval_pickup_instruction_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PickupInstruction>> {
        if !self.is_loaded("pickup_instruction_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "pickup_instruction_list".to_string(), attempted_path: "pickup_instruction_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.pickup_instruction_list)
        }
    }

    pub fn delivery_instruction_list(&self) -> &SmartList<crate::DeliveryInstruction> {
        &self.delivery_instruction_list
    }

    pub fn delivery_instruction_list_mut(&mut self) -> &mut SmartList<crate::DeliveryInstruction> {
        &mut self.delivery_instruction_list
    }

    pub fn eval_delivery_instruction_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DeliveryInstruction>> {
        if !self.is_loaded("delivery_instruction_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "delivery_instruction_list".to_string(), attempted_path: "delivery_instruction_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.delivery_instruction_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::RouteStopRepository<'a>>>
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
            .route_stop_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("RouteStop"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

