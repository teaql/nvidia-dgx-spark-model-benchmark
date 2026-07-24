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

// @source model.xml:69
    order_number: String,

// @source model.xml:69
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:69
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:69
#[teaql(column = "status")]
    status_id: u64,

// @source model.xml:69
#[teaql(column = "quote")]
    quote_id: u64,

// @source model.xml:69
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source model.xml:69
#[teaql(relation(target = "OrderStatus", local_key = "status_id", foreign_key = "id"))]
    status: Option<crate::OrderStatus>,

// @source model.xml:69
#[teaql(relation(target = "MoveQuote", local_key = "quote_id", foreign_key = "id"))]
    quote: Option<crate::MoveQuote>,

// @source model.xml:69
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
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
            order_number: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            status_id: 0_u64,
            quote_id: 0_u64,
            merchant_id: 0_u64,
            status: None,
            quote: None,
            merchant: None,
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
        if let Some(entity) = &mut self.status {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.quote {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
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

    pub fn order_number(&self) -> String {
        self.changed_order_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.order_number.clone())
    }

    pub fn update_order_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.order_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.order_number.clone());
        self.root.set(self.entity_key(), "order_number", value);
        self
    }

    pub fn changed_order_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "order_number")
    }

    pub fn eval_order_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("order_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "order_number".to_string(), attempted_path: "order_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.order_number())
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

    pub fn quote_id(&self) -> u64 {
        self.changed_quote_id().and_then(|value| value.try_u64()).unwrap_or(self.quote_id)
    }

    pub fn update_quote_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.quote_id = value.try_u64().unwrap_or(self.quote_id.clone());
        self.root.set(self.entity_key(), "quote_id", value);
        self
    }

    pub fn changed_quote_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "quote_id")
    }

    pub fn eval_quote_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("quote_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "quote_id".to_string(), attempted_path: "quote_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.quote_id())
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
    pub fn update_status_to_draft(&mut self) -> &mut Self {
        self.update_status_id(1001_u64)
    }

    pub fn status_is_draft(&self) -> bool {
        self.status_id() == 1001_u64
    }
    pub fn update_status_to_confirmed(&mut self) -> &mut Self {
        self.update_status_id(1002_u64)
    }

    pub fn status_is_confirmed(&self) -> bool {
        self.status_id() == 1002_u64
    }
    pub fn update_status_to_completed(&mut self) -> &mut Self {
        self.update_status_id(1003_u64)
    }

    pub fn status_is_completed(&self) -> bool {
        self.status_id() == 1003_u64
    }
    pub fn status(&self) -> Option<&crate::OrderStatus> {
        self.status.as_ref()
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<&crate::OrderStatus> {
        if !self.is_loaded("status") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
        } else {
            match &self.status {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn quote(&self) -> Option<&crate::MoveQuote> {
        self.quote.as_ref()
    }

    pub fn eval_quote(&self) -> teaql_core::eval::EvalResult<&crate::MoveQuote> {
        if !self.is_loaded("quote") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "quote".to_string(), attempted_path: "quote".to_string() }
        } else {
            match &self.quote {
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
    pub fn route_stop_list(&self) -> &SmartList<crate::RouteStop> {
        &self._relations.route_stop_list
    }

    pub fn route_stop_list_mut(&mut self) -> &mut SmartList<crate::RouteStop> {
        &mut self._relations.route_stop_list
    }

    pub fn eval_route_stop_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RouteStop>> {
        if !self.is_loaded("route_stop_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "route_stop_list".to_string(), attempted_path: "route_stop_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.route_stop_list)
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

    pub fn operational_exception_list(&self) -> &SmartList<crate::OperationalException> {
        &self._relations.operational_exception_list
    }

    pub fn operational_exception_list_mut(&mut self) -> &mut SmartList<crate::OperationalException> {
        &mut self._relations.operational_exception_list
    }

    pub fn eval_operational_exception_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OperationalException>> {
        if !self.is_loaded("operational_exception_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "operational_exception_list".to_string(), attempted_path: "operational_exception_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.operational_exception_list)
        }
    }

    pub fn move_inventory_list(&self) -> &SmartList<crate::MoveInventory> {
        &self._relations.move_inventory_list
    }

    pub fn move_inventory_list_mut(&mut self) -> &mut SmartList<crate::MoveInventory> {
        &mut self._relations.move_inventory_list
    }

    pub fn eval_move_inventory_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MoveInventory>> {
        if !self.is_loaded("move_inventory_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_inventory_list".to_string(), attempted_path: "move_inventory_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.move_inventory_list)
        }
    }

    pub fn packaging_item_list(&self) -> &SmartList<crate::PackagingItem> {
        &self._relations.packaging_item_list
    }

    pub fn packaging_item_list_mut(&mut self) -> &mut SmartList<crate::PackagingItem> {
        &mut self._relations.packaging_item_list
    }

    pub fn eval_packaging_item_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PackagingItem>> {
        if !self.is_loaded("packaging_item_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "packaging_item_list".to_string(), attempted_path: "packaging_item_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.packaging_item_list)
        }
    }

    pub fn third_party_dispatch_list(&self) -> &SmartList<crate::ThirdPartyDispatch> {
        &self._relations.third_party_dispatch_list
    }

    pub fn third_party_dispatch_list_mut(&mut self) -> &mut SmartList<crate::ThirdPartyDispatch> {
        &mut self._relations.third_party_dispatch_list
    }

    pub fn eval_third_party_dispatch_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ThirdPartyDispatch>> {
        if !self.is_loaded("third_party_dispatch_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "third_party_dispatch_list".to_string(), attempted_path: "third_party_dispatch_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.third_party_dispatch_list)
        }
    }

    pub fn customer_feedback_list(&self) -> &SmartList<crate::CustomerFeedback> {
        &self._relations.customer_feedback_list
    }

    pub fn customer_feedback_list_mut(&mut self) -> &mut SmartList<crate::CustomerFeedback> {
        &mut self._relations.customer_feedback_list
    }

    pub fn eval_customer_feedback_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerFeedback>> {
        if !self.is_loaded("customer_feedback_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_feedback_list".to_string(), attempted_path: "customer_feedback_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_feedback_list)
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
#[teaql(relation(target = "RouteStop", local_key = "id", foreign_key = "move_order_id", many))]
    route_stop_list: SmartList<crate::RouteStop>,
#[teaql(relation(target = "DispatchAssignment", local_key = "id", foreign_key = "move_order_id", many))]
    dispatch_assignment_list: SmartList<crate::DispatchAssignment>,
#[teaql(relation(target = "DamageReport", local_key = "id", foreign_key = "move_order_id", many))]
    damage_report_list: SmartList<crate::DamageReport>,
#[teaql(relation(target = "ProofOfDelivery", local_key = "id", foreign_key = "move_order_id", many))]
    proof_of_delivery_list: SmartList<crate::ProofOfDelivery>,
#[teaql(relation(target = "OperationalException", local_key = "id", foreign_key = "move_order_id", many))]
    operational_exception_list: SmartList<crate::OperationalException>,
#[teaql(relation(target = "MoveInventory", local_key = "id", foreign_key = "move_order_id", many))]
    move_inventory_list: SmartList<crate::MoveInventory>,
#[teaql(relation(target = "PackagingItem", local_key = "id", foreign_key = "move_order_id", many))]
    packaging_item_list: SmartList<crate::PackagingItem>,
#[teaql(relation(target = "ThirdPartyDispatch", local_key = "id", foreign_key = "move_order_id", many))]
    third_party_dispatch_list: SmartList<crate::ThirdPartyDispatch>,
#[teaql(relation(target = "CustomerFeedback", local_key = "id", foreign_key = "move_order_id", many))]
    customer_feedback_list: SmartList<crate::CustomerFeedback>,
}

impl MoveOrderReverseRelations {
    pub fn new() -> Self {
        Self {
            route_stop_list: Default::default(),
            dispatch_assignment_list: Default::default(),
            damage_report_list: Default::default(),
            proof_of_delivery_list: Default::default(),
            operational_exception_list: Default::default(),
            move_inventory_list: Default::default(),
            packaging_item_list: Default::default(),
            third_party_dispatch_list: Default::default(),
            customer_feedback_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.route_stop_list {
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
        for entity in &mut self.operational_exception_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_inventory_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.packaging_item_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.third_party_dispatch_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_feedback_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
