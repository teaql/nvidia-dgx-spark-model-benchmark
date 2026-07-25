// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/customer
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
#[teaql(entity = "Customer", table = "customer_data", data_service = "sqlite")]
pub struct Customer {
#[teaql(id)]
    id: u64,
#[teaql(version)]
    version: i64,
#[teaql(relation(target = "Employee", local_key = "id", foreign_key = "customer_id", many))]
    employee_list: SmartList<crate::Employee>,
#[teaql(relation(target = "Truck", local_key = "id", foreign_key = "customer_id", many))]
    truck_list: SmartList<crate::Truck>,
#[teaql(relation(target = "InventoryItem", local_key = "id", foreign_key = "customer_id", many))]
    inventory_item_list: SmartList<crate::InventoryItem>,
#[teaql(relation(target = "MoveOrder", local_key = "id", foreign_key = "customer_id", many))]
    move_order_list: SmartList<crate::MoveOrder>,
#[teaql(relation(target = "Route", local_key = "id", foreign_key = "customer_id", many))]
    route_list: SmartList<crate::Route>,
#[teaql(relation(target = "Payment", local_key = "id", foreign_key = "customer_id", many))]
    payment_list: SmartList<crate::Payment>,
#[teaql(relation(target = "Invoice", local_key = "id", foreign_key = "customer_id", many))]
    invoice_list: SmartList<crate::Invoice>,
#[teaql(relation(target = "Feedback", local_key = "id", foreign_key = "customer_id", many))]
    feedback_list: SmartList<crate::Feedback>,
#[teaql(relation(target = "Schedule", local_key = "id", foreign_key = "customer_id", many))]
    schedule_list: SmartList<crate::Schedule>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Customer {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            version: 0_i64,
            employee_list: Default::default(),
            truck_list: Default::default(),
            inventory_item_list: Default::default(),
            move_order_list: Default::default(),
            route_list: Default::default(),
            payment_list: Default::default(),
            invoice_list: Default::default(),
            feedback_list: Default::default(),
            schedule_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Customer", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        for entity in &mut self.employee_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.truck_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.inventory_item_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_order_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.route_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.invoice_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.feedback_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.schedule_list {
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
    pub fn employee_list(&self) -> &SmartList<crate::Employee> {
        &self.employee_list
    }

    pub fn employee_list_mut(&mut self) -> &mut SmartList<crate::Employee> {
        &mut self.employee_list
    }

    pub fn eval_employee_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Employee>> {
        if !self.is_loaded("employee_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_list".to_string(), attempted_path: "employee_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.employee_list)
        }
    }

    pub fn truck_list(&self) -> &SmartList<crate::Truck> {
        &self.truck_list
    }

    pub fn truck_list_mut(&mut self) -> &mut SmartList<crate::Truck> {
        &mut self.truck_list
    }

    pub fn eval_truck_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Truck>> {
        if !self.is_loaded("truck_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "truck_list".to_string(), attempted_path: "truck_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.truck_list)
        }
    }

    pub fn inventory_item_list(&self) -> &SmartList<crate::InventoryItem> {
        &self.inventory_item_list
    }

    pub fn inventory_item_list_mut(&mut self) -> &mut SmartList<crate::InventoryItem> {
        &mut self.inventory_item_list
    }

    pub fn eval_inventory_item_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::InventoryItem>> {
        if !self.is_loaded("inventory_item_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "inventory_item_list".to_string(), attempted_path: "inventory_item_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.inventory_item_list)
        }
    }

    pub fn move_order_list(&self) -> &SmartList<crate::MoveOrder> {
        &self.move_order_list
    }

    pub fn move_order_list_mut(&mut self) -> &mut SmartList<crate::MoveOrder> {
        &mut self.move_order_list
    }

    pub fn eval_move_order_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MoveOrder>> {
        if !self.is_loaded("move_order_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_order_list".to_string(), attempted_path: "move_order_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.move_order_list)
        }
    }

    pub fn route_list(&self) -> &SmartList<crate::Route> {
        &self.route_list
    }

    pub fn route_list_mut(&mut self) -> &mut SmartList<crate::Route> {
        &mut self.route_list
    }

    pub fn eval_route_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Route>> {
        if !self.is_loaded("route_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "route_list".to_string(), attempted_path: "route_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.route_list)
        }
    }

    pub fn payment_list(&self) -> &SmartList<crate::Payment> {
        &self.payment_list
    }

    pub fn payment_list_mut(&mut self) -> &mut SmartList<crate::Payment> {
        &mut self.payment_list
    }

    pub fn eval_payment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Payment>> {
        if !self.is_loaded("payment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payment_list".to_string(), attempted_path: "payment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.payment_list)
        }
    }

    pub fn invoice_list(&self) -> &SmartList<crate::Invoice> {
        &self.invoice_list
    }

    pub fn invoice_list_mut(&mut self) -> &mut SmartList<crate::Invoice> {
        &mut self.invoice_list
    }

    pub fn eval_invoice_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Invoice>> {
        if !self.is_loaded("invoice_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "invoice_list".to_string(), attempted_path: "invoice_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.invoice_list)
        }
    }

    pub fn feedback_list(&self) -> &SmartList<crate::Feedback> {
        &self.feedback_list
    }

    pub fn feedback_list_mut(&mut self) -> &mut SmartList<crate::Feedback> {
        &mut self.feedback_list
    }

    pub fn eval_feedback_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Feedback>> {
        if !self.is_loaded("feedback_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "feedback_list".to_string(), attempted_path: "feedback_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.feedback_list)
        }
    }

    pub fn schedule_list(&self) -> &SmartList<crate::Schedule> {
        &self.schedule_list
    }

    pub fn schedule_list_mut(&mut self) -> &mut SmartList<crate::Schedule> {
        &mut self.schedule_list
    }

    pub fn eval_schedule_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Schedule>> {
        if !self.is_loaded("schedule_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "schedule_list".to_string(), attempted_path: "schedule_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.schedule_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
            .customer_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Customer"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

