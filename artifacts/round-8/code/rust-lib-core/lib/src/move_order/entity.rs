// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/move_order
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
#[teaql(entity = "MoveOrder", table = "move_order_data", data_service = "sqlite")]
pub struct MoveOrder {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    order_id: String,

// @source model.xml:2
    status: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "merchant_ref")]
    merchant_ref_id: u64,

// @source model.xml:2
#[teaql(column = "customer_ref")]
    customer_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Merchant", local_key = "merchant_ref_id", foreign_key = "id"))]
    merchant_ref: Option<crate::Merchant>,

// @source model.xml:2
#[teaql(relation(target = "Customer", local_key = "customer_ref_id", foreign_key = "id"))]
    customer_ref: Option<crate::Customer>,
    #[teaql(boxed_relations)]
    pub _relations: Box<MoveOrderReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl MoveOrder {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            order_id: String::new(),
            status: String::new(),
            version: 0_i64,
            merchant_ref_id: 0_u64,
            customer_ref_id: 0_u64,
            merchant_ref: None,
            customer_ref: None,
            _relations: Box::new(MoveOrderReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("MoveOrder", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.merchant_ref {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.customer_ref {
            entity.attach_root_recursive(root.clone());
        }
        self._relations.attach_root_recursive(root.clone());
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

    pub fn order_id(&self) -> String {
        self.changed_order_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.order_id.clone())
    }

    pub fn update_order_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.order_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.order_id.clone());
        self.root.set(self.entity_key(), "order_id", value);
        self
    }

    pub fn changed_order_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "order_id")
    }

    pub fn eval_order_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("order_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "order_id".to_string(), attempted_path: "order_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.order_id())
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
    pub fn merchant_ref_id(&self) -> u64 {
        self.changed_merchant_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_ref_id)
    }

    pub fn update_merchant_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_ref_id = value.try_u64().unwrap_or(self.merchant_ref_id.clone());
        self.root.set(self.entity_key(), "merchant_ref_id", value);
        self
    }

    pub fn changed_merchant_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_ref_id")
    }

    pub fn eval_merchant_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref_id".to_string(), attempted_path: "merchant_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_ref_id())
                }}

    pub fn customer_ref_id(&self) -> u64 {
        self.changed_customer_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.customer_ref_id)
    }

    pub fn update_customer_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.customer_ref_id = value.try_u64().unwrap_or(self.customer_ref_id.clone());
        self.root.set(self.entity_key(), "customer_ref_id", value);
        self
    }

    pub fn changed_customer_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "customer_ref_id")
    }

    pub fn eval_customer_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("customer_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_ref_id".to_string(), attempted_path: "customer_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.customer_ref_id())
                }}
    pub fn merchant_ref(&self) -> Option<&crate::Merchant> {
        self.merchant_ref.as_ref()
    }

    pub fn eval_merchant_ref(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref".to_string(), attempted_path: "merchant_ref".to_string() }
        } else {
            match &self.merchant_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn customer_ref(&self) -> Option<&crate::Customer> {
        self.customer_ref.as_ref()
    }

    pub fn eval_customer_ref(&self) -> teaql_core::eval::EvalResult<&crate::Customer> {
        if !self.is_loaded("customer_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_ref".to_string(), attempted_path: "customer_ref".to_string() }
        } else {
            match &self.customer_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn move_quote_list(&self) -> &SmartList<crate::MoveQuote> {
        &self._relations.move_quote_list
    }

    pub fn move_quote_list_mut(&mut self) -> &mut SmartList<crate::MoveQuote> {
        &mut self._relations.move_quote_list
    }

    pub fn eval_move_quote_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MoveQuote>> {
        if !self.is_loaded("move_quote_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_quote_list".to_string(), attempted_path: "move_quote_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.move_quote_list)
        }
    }

    pub fn route_list(&self) -> &SmartList<crate::Route> {
        &self._relations.route_list
    }

    pub fn route_list_mut(&mut self) -> &mut SmartList<crate::Route> {
        &mut self._relations.route_list
    }

    pub fn eval_route_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Route>> {
        if !self.is_loaded("route_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "route_list".to_string(), attempted_path: "route_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.route_list)
        }
    }

    pub fn time_slot_list(&self) -> &SmartList<crate::TimeSlot> {
        &self._relations.time_slot_list
    }

    pub fn time_slot_list_mut(&mut self) -> &mut SmartList<crate::TimeSlot> {
        &mut self._relations.time_slot_list
    }

    pub fn eval_time_slot_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TimeSlot>> {
        if !self.is_loaded("time_slot_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "time_slot_list".to_string(), attempted_path: "time_slot_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.time_slot_list)
        }
    }

    pub fn fulfillment_event_list(&self) -> &SmartList<crate::FulfillmentEvent> {
        &self._relations.fulfillment_event_list
    }

    pub fn fulfillment_event_list_mut(&mut self) -> &mut SmartList<crate::FulfillmentEvent> {
        &mut self._relations.fulfillment_event_list
    }

    pub fn eval_fulfillment_event_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::FulfillmentEvent>> {
        if !self.is_loaded("fulfillment_event_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "fulfillment_event_list".to_string(), attempted_path: "fulfillment_event_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.fulfillment_event_list)
        }
    }

    pub fn dispatch_assignment_list(&self) -> &SmartList<crate::DispatchAssignment> {
        &self._relations.dispatch_assignment_list
    }

    pub fn dispatch_assignment_list_mut(&mut self) -> &mut SmartList<crate::DispatchAssignment> {
        &mut self._relations.dispatch_assignment_list
    }

    pub fn eval_dispatch_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DispatchAssignment>> {
        if !self.is_loaded("dispatch_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "dispatch_assignment_list".to_string(), attempted_path: "dispatch_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.dispatch_assignment_list)
        }
    }

    pub fn damage_report_list(&self) -> &SmartList<crate::DamageReport> {
        &self._relations.damage_report_list
    }

    pub fn damage_report_list_mut(&mut self) -> &mut SmartList<crate::DamageReport> {
        &mut self._relations.damage_report_list
    }

    pub fn eval_damage_report_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DamageReport>> {
        if !self.is_loaded("damage_report_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "damage_report_list".to_string(), attempted_path: "damage_report_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.damage_report_list)
        }
    }

    pub fn proof_of_delivery_list(&self) -> &SmartList<crate::ProofOfDelivery> {
        &self._relations.proof_of_delivery_list
    }

    pub fn proof_of_delivery_list_mut(&mut self) -> &mut SmartList<crate::ProofOfDelivery> {
        &mut self._relations.proof_of_delivery_list
    }

    pub fn eval_proof_of_delivery_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ProofOfDelivery>> {
        if !self.is_loaded("proof_of_delivery_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "proof_of_delivery_list".to_string(), attempted_path: "proof_of_delivery_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.proof_of_delivery_list)
        }
    }

    pub fn move_item_list(&self) -> &SmartList<crate::MoveItem> {
        &self._relations.move_item_list
    }

    pub fn move_item_list_mut(&mut self) -> &mut SmartList<crate::MoveItem> {
        &mut self._relations.move_item_list
    }

    pub fn eval_move_item_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MoveItem>> {
        if !self.is_loaded("move_item_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_item_list".to_string(), attempted_path: "move_item_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.move_item_list)
        }
    }

    pub fn inventory_list_list(&self) -> &SmartList<crate::InventoryList> {
        &self._relations.inventory_list_list
    }

    pub fn inventory_list_list_mut(&mut self) -> &mut SmartList<crate::InventoryList> {
        &mut self._relations.inventory_list_list
    }

    pub fn eval_inventory_list_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::InventoryList>> {
        if !self.is_loaded("inventory_list_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "inventory_list_list".to_string(), attempted_path: "inventory_list_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.inventory_list_list)
        }
    }

    pub fn transit_log_list(&self) -> &SmartList<crate::TransitLog> {
        &self._relations.transit_log_list
    }

    pub fn transit_log_list_mut(&mut self) -> &mut SmartList<crate::TransitLog> {
        &mut self._relations.transit_log_list
    }

    pub fn eval_transit_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TransitLog>> {
        if !self.is_loaded("transit_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "transit_log_list".to_string(), attempted_path: "transit_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.transit_log_list)
        }
    }

    pub fn delay_record_list(&self) -> &SmartList<crate::DelayRecord> {
        &self._relations.delay_record_list
    }

    pub fn delay_record_list_mut(&mut self) -> &mut SmartList<crate::DelayRecord> {
        &mut self._relations.delay_record_list
    }

    pub fn eval_delay_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DelayRecord>> {
        if !self.is_loaded("delay_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "delay_record_list".to_string(), attempted_path: "delay_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.delay_record_list)
        }
    }

    pub fn vehicle_assignment_list(&self) -> &SmartList<crate::VehicleAssignment> {
        &self._relations.vehicle_assignment_list
    }

    pub fn vehicle_assignment_list_mut(&mut self) -> &mut SmartList<crate::VehicleAssignment> {
        &mut self._relations.vehicle_assignment_list
    }

    pub fn eval_vehicle_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::VehicleAssignment>> {
        if !self.is_loaded("vehicle_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_assignment_list".to_string(), attempted_path: "vehicle_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.vehicle_assignment_list)
        }
    }

    pub fn cargo_weight_record_list(&self) -> &SmartList<crate::CargoWeightRecord> {
        &self._relations.cargo_weight_record_list
    }

    pub fn cargo_weight_record_list_mut(&mut self) -> &mut SmartList<crate::CargoWeightRecord> {
        &mut self._relations.cargo_weight_record_list
    }

    pub fn eval_cargo_weight_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CargoWeightRecord>> {
        if !self.is_loaded("cargo_weight_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "cargo_weight_record_list".to_string(), attempted_path: "cargo_weight_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.cargo_weight_record_list)
        }
    }

    pub fn special_handling_instruction_list(&self) -> &SmartList<crate::SpecialHandlingInstruction> {
        &self._relations.special_handling_instruction_list
    }

    pub fn special_handling_instruction_list_mut(&mut self) -> &mut SmartList<crate::SpecialHandlingInstruction> {
        &mut self._relations.special_handling_instruction_list
    }

    pub fn eval_special_handling_instruction_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SpecialHandlingInstruction>> {
        if !self.is_loaded("special_handling_instruction_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "special_handling_instruction_list".to_string(), attempted_path: "special_handling_instruction_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.special_handling_instruction_list)
        }
    }

    pub fn delivery_window_list(&self) -> &SmartList<crate::DeliveryWindow> {
        &self._relations.delivery_window_list
    }

    pub fn delivery_window_list_mut(&mut self) -> &mut SmartList<crate::DeliveryWindow> {
        &mut self._relations.delivery_window_list
    }

    pub fn eval_delivery_window_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DeliveryWindow>> {
        if !self.is_loaded("delivery_window_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "delivery_window_list".to_string(), attempted_path: "delivery_window_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.delivery_window_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
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
            .move_order_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("MoveOrder"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct MoveOrderReverseRelations {
#[teaql(relation(target = "MoveQuote", local_key = "id", foreign_key = "move_order_ref_id", many))]
    move_quote_list: SmartList<crate::MoveQuote>,
#[teaql(relation(target = "Route", local_key = "id", foreign_key = "move_order_ref_id", many))]
    route_list: SmartList<crate::Route>,
#[teaql(relation(target = "TimeSlot", local_key = "id", foreign_key = "move_order_ref_id", many))]
    time_slot_list: SmartList<crate::TimeSlot>,
#[teaql(relation(target = "FulfillmentEvent", local_key = "id", foreign_key = "move_order_ref_id", many))]
    fulfillment_event_list: SmartList<crate::FulfillmentEvent>,
#[teaql(relation(target = "DispatchAssignment", local_key = "id", foreign_key = "move_order_ref_id", many))]
    dispatch_assignment_list: SmartList<crate::DispatchAssignment>,
#[teaql(relation(target = "DamageReport", local_key = "id", foreign_key = "move_order_ref_id", many))]
    damage_report_list: SmartList<crate::DamageReport>,
#[teaql(relation(target = "ProofOfDelivery", local_key = "id", foreign_key = "move_order_ref_id", many))]
    proof_of_delivery_list: SmartList<crate::ProofOfDelivery>,
#[teaql(relation(target = "MoveItem", local_key = "id", foreign_key = "move_order_ref_id", many))]
    move_item_list: SmartList<crate::MoveItem>,
#[teaql(relation(target = "InventoryList", local_key = "id", foreign_key = "move_order_ref_id", many))]
    inventory_list_list: SmartList<crate::InventoryList>,
#[teaql(relation(target = "TransitLog", local_key = "id", foreign_key = "move_order_ref_id", many))]
    transit_log_list: SmartList<crate::TransitLog>,
#[teaql(relation(target = "DelayRecord", local_key = "id", foreign_key = "move_order_ref_id", many))]
    delay_record_list: SmartList<crate::DelayRecord>,
#[teaql(relation(target = "VehicleAssignment", local_key = "id", foreign_key = "move_order_ref_id", many))]
    vehicle_assignment_list: SmartList<crate::VehicleAssignment>,
#[teaql(relation(target = "CargoWeightRecord", local_key = "id", foreign_key = "move_order_ref_id", many))]
    cargo_weight_record_list: SmartList<crate::CargoWeightRecord>,
#[teaql(relation(target = "SpecialHandlingInstruction", local_key = "id", foreign_key = "move_order_ref_id", many))]
    special_handling_instruction_list: SmartList<crate::SpecialHandlingInstruction>,
#[teaql(relation(target = "DeliveryWindow", local_key = "id", foreign_key = "move_order_ref_id", many))]
    delivery_window_list: SmartList<crate::DeliveryWindow>,
}

impl MoveOrderReverseRelations {
    pub fn new() -> Self {
        Self {
            move_quote_list: Default::default(),
            route_list: Default::default(),
            time_slot_list: Default::default(),
            fulfillment_event_list: Default::default(),
            dispatch_assignment_list: Default::default(),
            damage_report_list: Default::default(),
            proof_of_delivery_list: Default::default(),
            move_item_list: Default::default(),
            inventory_list_list: Default::default(),
            transit_log_list: Default::default(),
            delay_record_list: Default::default(),
            vehicle_assignment_list: Default::default(),
            cargo_weight_record_list: Default::default(),
            special_handling_instruction_list: Default::default(),
            delivery_window_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.move_quote_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.route_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.time_slot_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.fulfillment_event_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.dispatch_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.damage_report_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.proof_of_delivery_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_item_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.inventory_list_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.transit_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.delay_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.cargo_weight_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.special_handling_instruction_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.delivery_window_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
