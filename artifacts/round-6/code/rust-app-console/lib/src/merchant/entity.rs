// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/merchant
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
#[teaql(entity = "Merchant", table = "merchant_data", data_service = "sqlite")]
pub struct Merchant {
#[teaql(id)]
    id: u64,

// @source model.xml:53
    name: String,

// @source model.xml:53
    tax_number: String,

// @source model.xml:53
    address: String,

// @source model.xml:53
    external_id: String,

// @source model.xml:53
    create_time: chrono::DateTime<chrono::Utc>,

// @source model.xml:53
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source model.xml:53
#[teaql(column = "platform")]
    platform_id: u64,
// @source model.xml:53
#[teaql(relation(target = "Platform", local_key = "platform_id", foreign_key = "id"))]
    platform: Option<crate::Platform>,
    #[teaql(boxed_relations)]
    pub _relations: Box<MerchantReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Merchant {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            name: String::new(),
            tax_number: String::new(),
            address: String::new(),
            external_id: String::new(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            platform_id: 0_u64,
            platform: None,
            _relations: Box::new(MerchantReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Merchant", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.platform {
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

    pub fn tax_number(&self) -> String {
        self.changed_tax_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.tax_number.clone())
    }

    pub fn update_tax_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tax_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.tax_number.clone());
        self.root.set(self.entity_key(), "tax_number", value);
        self
    }

    pub fn changed_tax_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tax_number")
    }

    pub fn eval_tax_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("tax_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_number".to_string(), attempted_path: "tax_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tax_number())
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

    pub fn external_id(&self) -> String {
        self.changed_external_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.external_id.clone())
    }

    pub fn update_external_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.external_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.external_id.clone());
        self.root.set(self.entity_key(), "external_id", value);
        self
    }

    pub fn changed_external_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "external_id")
    }

    pub fn eval_external_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("external_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "external_id".to_string(), attempted_path: "external_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.external_id())
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
    pub fn platform_id(&self) -> u64 {
        self.changed_platform_id().and_then(|value| value.try_u64()).unwrap_or(self.platform_id)
    }

    pub fn update_platform_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.platform_id = value.try_u64().unwrap_or(self.platform_id.clone());
        self.root.set(self.entity_key(), "platform_id", value);
        self
    }

    pub fn changed_platform_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "platform_id")
    }

    pub fn eval_platform_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("platform_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_id".to_string(), attempted_path: "platform_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.platform_id())
                }}
    pub fn platform(&self) -> Option<&crate::Platform> {
        self.platform.as_ref()
    }

    pub fn eval_platform(&self) -> teaql_core::eval::EvalResult<&crate::Platform> {
        if !self.is_loaded("platform") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform".to_string(), attempted_path: "platform".to_string() }
        } else {
            match &self.platform {
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

    pub fn move_order_list(&self) -> &SmartList<crate::MoveOrder> {
        &self._relations.move_order_list
    }

    pub fn move_order_list_mut(&mut self) -> &mut SmartList<crate::MoveOrder> {
        &mut self._relations.move_order_list
    }

    pub fn eval_move_order_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MoveOrder>> {
        if !self.is_loaded("move_order_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_order_list".to_string(), attempted_path: "move_order_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.move_order_list)
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

    pub fn crew_list(&self) -> &SmartList<crate::Crew> {
        &self._relations.crew_list
    }

    pub fn crew_list_mut(&mut self) -> &mut SmartList<crate::Crew> {
        &mut self._relations.crew_list
    }

    pub fn eval_crew_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Crew>> {
        if !self.is_loaded("crew_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "crew_list".to_string(), attempted_path: "crew_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.crew_list)
        }
    }

    pub fn crew_member_assignment_list(&self) -> &SmartList<crate::CrewMemberAssignment> {
        &self._relations.crew_member_assignment_list
    }

    pub fn crew_member_assignment_list_mut(&mut self) -> &mut SmartList<crate::CrewMemberAssignment> {
        &mut self._relations.crew_member_assignment_list
    }

    pub fn eval_crew_member_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CrewMemberAssignment>> {
        if !self.is_loaded("crew_member_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "crew_member_assignment_list".to_string(), attempted_path: "crew_member_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.crew_member_assignment_list)
        }
    }

    pub fn vehicle_list(&self) -> &SmartList<crate::Vehicle> {
        &self._relations.vehicle_list
    }

    pub fn vehicle_list_mut(&mut self) -> &mut SmartList<crate::Vehicle> {
        &mut self._relations.vehicle_list
    }

    pub fn eval_vehicle_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Vehicle>> {
        if !self.is_loaded("vehicle_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_list".to_string(), attempted_path: "vehicle_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.vehicle_list)
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

    pub fn pickup_instruction_list(&self) -> &SmartList<crate::PickupInstruction> {
        &self._relations.pickup_instruction_list
    }

    pub fn pickup_instruction_list_mut(&mut self) -> &mut SmartList<crate::PickupInstruction> {
        &mut self._relations.pickup_instruction_list
    }

    pub fn eval_pickup_instruction_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PickupInstruction>> {
        if !self.is_loaded("pickup_instruction_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "pickup_instruction_list".to_string(), attempted_path: "pickup_instruction_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.pickup_instruction_list)
        }
    }

    pub fn delivery_instruction_list(&self) -> &SmartList<crate::DeliveryInstruction> {
        &self._relations.delivery_instruction_list
    }

    pub fn delivery_instruction_list_mut(&mut self) -> &mut SmartList<crate::DeliveryInstruction> {
        &mut self._relations.delivery_instruction_list
    }

    pub fn eval_delivery_instruction_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DeliveryInstruction>> {
        if !self.is_loaded("delivery_instruction_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "delivery_instruction_list".to_string(), attempted_path: "delivery_instruction_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.delivery_instruction_list)
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

    pub fn logistics_provider_list(&self) -> &SmartList<crate::LogisticsProvider> {
        &self._relations.logistics_provider_list
    }

    pub fn logistics_provider_list_mut(&mut self) -> &mut SmartList<crate::LogisticsProvider> {
        &mut self._relations.logistics_provider_list
    }

    pub fn eval_logistics_provider_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LogisticsProvider>> {
        if !self.is_loaded("logistics_provider_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "logistics_provider_list".to_string(), attempted_path: "logistics_provider_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.logistics_provider_list)
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

    pub fn fuel_log_list(&self) -> &SmartList<crate::FuelLog> {
        &self._relations.fuel_log_list
    }

    pub fn fuel_log_list_mut(&mut self) -> &mut SmartList<crate::FuelLog> {
        &mut self._relations.fuel_log_list
    }

    pub fn eval_fuel_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::FuelLog>> {
        if !self.is_loaded("fuel_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "fuel_log_list".to_string(), attempted_path: "fuel_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.fuel_log_list)
        }
    }

    pub fn maintenance_record_list(&self) -> &SmartList<crate::MaintenanceRecord> {
        &self._relations.maintenance_record_list
    }

    pub fn maintenance_record_list_mut(&mut self) -> &mut SmartList<crate::MaintenanceRecord> {
        &mut self._relations.maintenance_record_list
    }

    pub fn eval_maintenance_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MaintenanceRecord>> {
        if !self.is_loaded("maintenance_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "maintenance_record_list".to_string(), attempted_path: "maintenance_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.maintenance_record_list)
        }
    }

    pub fn toll_receipt_list(&self) -> &SmartList<crate::TollReceipt> {
        &self._relations.toll_receipt_list
    }

    pub fn toll_receipt_list_mut(&mut self) -> &mut SmartList<crate::TollReceipt> {
        &mut self._relations.toll_receipt_list
    }

    pub fn eval_toll_receipt_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TollReceipt>> {
        if !self.is_loaded("toll_receipt_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "toll_receipt_list".to_string(), attempted_path: "toll_receipt_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.toll_receipt_list)
        }
    }

    pub fn shift_log_list(&self) -> &SmartList<crate::ShiftLog> {
        &self._relations.shift_log_list
    }

    pub fn shift_log_list_mut(&mut self) -> &mut SmartList<crate::ShiftLog> {
        &mut self._relations.shift_log_list
    }

    pub fn eval_shift_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ShiftLog>> {
        if !self.is_loaded("shift_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "shift_log_list".to_string(), attempted_path: "shift_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.shift_log_list)
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

    pub fn incident_report_list(&self) -> &SmartList<crate::IncidentReport> {
        &self._relations.incident_report_list
    }

    pub fn incident_report_list_mut(&mut self) -> &mut SmartList<crate::IncidentReport> {
        &mut self._relations.incident_report_list
    }

    pub fn eval_incident_report_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::IncidentReport>> {
        if !self.is_loaded("incident_report_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "incident_report_list".to_string(), attempted_path: "incident_report_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.incident_report_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::MerchantRepository<'a>>>
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
            .merchant_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Merchant"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct MerchantReverseRelations {
#[teaql(relation(target = "MoveQuote", local_key = "id", foreign_key = "merchant_id", many))]
    move_quote_list: SmartList<crate::MoveQuote>,
#[teaql(relation(target = "MoveOrder", local_key = "id", foreign_key = "merchant_id", many))]
    move_order_list: SmartList<crate::MoveOrder>,
#[teaql(relation(target = "RouteStop", local_key = "id", foreign_key = "merchant_id", many))]
    route_stop_list: SmartList<crate::RouteStop>,
#[teaql(relation(target = "Crew", local_key = "id", foreign_key = "merchant_id", many))]
    crew_list: SmartList<crate::Crew>,
#[teaql(relation(target = "CrewMemberAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    crew_member_assignment_list: SmartList<crate::CrewMemberAssignment>,
#[teaql(relation(target = "Vehicle", local_key = "id", foreign_key = "merchant_id", many))]
    vehicle_list: SmartList<crate::Vehicle>,
#[teaql(relation(target = "VehicleAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    vehicle_assignment_list: SmartList<crate::VehicleAssignment>,
#[teaql(relation(target = "DispatchAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    dispatch_assignment_list: SmartList<crate::DispatchAssignment>,
#[teaql(relation(target = "DamageReport", local_key = "id", foreign_key = "merchant_id", many))]
    damage_report_list: SmartList<crate::DamageReport>,
#[teaql(relation(target = "ProofOfDelivery", local_key = "id", foreign_key = "merchant_id", many))]
    proof_of_delivery_list: SmartList<crate::ProofOfDelivery>,
#[teaql(relation(target = "OperationalException", local_key = "id", foreign_key = "merchant_id", many))]
    operational_exception_list: SmartList<crate::OperationalException>,
#[teaql(relation(target = "PickupInstruction", local_key = "id", foreign_key = "merchant_id", many))]
    pickup_instruction_list: SmartList<crate::PickupInstruction>,
#[teaql(relation(target = "DeliveryInstruction", local_key = "id", foreign_key = "merchant_id", many))]
    delivery_instruction_list: SmartList<crate::DeliveryInstruction>,
#[teaql(relation(target = "MoveInventory", local_key = "id", foreign_key = "merchant_id", many))]
    move_inventory_list: SmartList<crate::MoveInventory>,
#[teaql(relation(target = "PackagingItem", local_key = "id", foreign_key = "merchant_id", many))]
    packaging_item_list: SmartList<crate::PackagingItem>,
#[teaql(relation(target = "LogisticsProvider", local_key = "id", foreign_key = "merchant_id", many))]
    logistics_provider_list: SmartList<crate::LogisticsProvider>,
#[teaql(relation(target = "ThirdPartyDispatch", local_key = "id", foreign_key = "merchant_id", many))]
    third_party_dispatch_list: SmartList<crate::ThirdPartyDispatch>,
#[teaql(relation(target = "FuelLog", local_key = "id", foreign_key = "merchant_id", many))]
    fuel_log_list: SmartList<crate::FuelLog>,
#[teaql(relation(target = "MaintenanceRecord", local_key = "id", foreign_key = "merchant_id", many))]
    maintenance_record_list: SmartList<crate::MaintenanceRecord>,
#[teaql(relation(target = "TollReceipt", local_key = "id", foreign_key = "merchant_id", many))]
    toll_receipt_list: SmartList<crate::TollReceipt>,
#[teaql(relation(target = "ShiftLog", local_key = "id", foreign_key = "merchant_id", many))]
    shift_log_list: SmartList<crate::ShiftLog>,
#[teaql(relation(target = "CustomerFeedback", local_key = "id", foreign_key = "merchant_id", many))]
    customer_feedback_list: SmartList<crate::CustomerFeedback>,
#[teaql(relation(target = "IncidentReport", local_key = "id", foreign_key = "merchant_id", many))]
    incident_report_list: SmartList<crate::IncidentReport>,
}

impl MerchantReverseRelations {
    pub fn new() -> Self {
        Self {
            move_quote_list: Default::default(),
            move_order_list: Default::default(),
            route_stop_list: Default::default(),
            crew_list: Default::default(),
            crew_member_assignment_list: Default::default(),
            vehicle_list: Default::default(),
            vehicle_assignment_list: Default::default(),
            dispatch_assignment_list: Default::default(),
            damage_report_list: Default::default(),
            proof_of_delivery_list: Default::default(),
            operational_exception_list: Default::default(),
            pickup_instruction_list: Default::default(),
            delivery_instruction_list: Default::default(),
            move_inventory_list: Default::default(),
            packaging_item_list: Default::default(),
            logistics_provider_list: Default::default(),
            third_party_dispatch_list: Default::default(),
            fuel_log_list: Default::default(),
            maintenance_record_list: Default::default(),
            toll_receipt_list: Default::default(),
            shift_log_list: Default::default(),
            customer_feedback_list: Default::default(),
            incident_report_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.move_quote_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_order_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.route_stop_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.crew_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.crew_member_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_assignment_list {
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
        for entity in &mut self.pickup_instruction_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.delivery_instruction_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_inventory_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.packaging_item_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.logistics_provider_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.third_party_dispatch_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.fuel_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.maintenance_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.toll_receipt_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.shift_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_feedback_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.incident_report_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
