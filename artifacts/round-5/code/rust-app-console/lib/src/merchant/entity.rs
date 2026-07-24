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

// @source prepared.xml:4
    name: String,

// @source prepared.xml:4
    tax_number: String,

// @source prepared.xml:4
    address: String,

// @source prepared.xml:4
    external_id: String,

// @source prepared.xml:4
    create_time: chrono::DateTime<chrono::Utc>,

// @source prepared.xml:4
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source prepared.xml:4
#[teaql(column = "platform")]
    platform_id: u64,
// @source prepared.xml:4
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
    pub fn employee_list(&self) -> &SmartList<crate::Employee> {
        &self._relations.employee_list
    }

    pub fn employee_list_mut(&mut self) -> &mut SmartList<crate::Employee> {
        &mut self._relations.employee_list
    }

    pub fn eval_employee_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Employee>> {
        if !self.is_loaded("employee_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_list".to_string(), attempted_path: "employee_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.employee_list)
        }
    }

    pub fn platform_setting_list(&self) -> &SmartList<crate::PlatformSetting> {
        &self._relations.platform_setting_list
    }

    pub fn platform_setting_list_mut(&mut self) -> &mut SmartList<crate::PlatformSetting> {
        &mut self._relations.platform_setting_list
    }

    pub fn eval_platform_setting_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PlatformSetting>> {
        if !self.is_loaded("platform_setting_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_setting_list".to_string(), attempted_path: "platform_setting_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.platform_setting_list)
        }
    }

    pub fn platform_user_list(&self) -> &SmartList<crate::PlatformUser> {
        &self._relations.platform_user_list
    }

    pub fn platform_user_list_mut(&mut self) -> &mut SmartList<crate::PlatformUser> {
        &mut self._relations.platform_user_list
    }

    pub fn eval_platform_user_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PlatformUser>> {
        if !self.is_loaded("platform_user_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_user_list".to_string(), attempted_path: "platform_user_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.platform_user_list)
        }
    }

    pub fn platform_audit_log_list(&self) -> &SmartList<crate::PlatformAuditLog> {
        &self._relations.platform_audit_log_list
    }

    pub fn platform_audit_log_list_mut(&mut self) -> &mut SmartList<crate::PlatformAuditLog> {
        &mut self._relations.platform_audit_log_list
    }

    pub fn eval_platform_audit_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PlatformAuditLog>> {
        if !self.is_loaded("platform_audit_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_audit_log_list".to_string(), attempted_path: "platform_audit_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.platform_audit_log_list)
        }
    }

    pub fn organization_list(&self) -> &SmartList<crate::Organization> {
        &self._relations.organization_list
    }

    pub fn organization_list_mut(&mut self) -> &mut SmartList<crate::Organization> {
        &mut self._relations.organization_list
    }

    pub fn eval_organization_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Organization>> {
        if !self.is_loaded("organization_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "organization_list".to_string(), attempted_path: "organization_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.organization_list)
        }
    }

    pub fn organization_setting_list(&self) -> &SmartList<crate::OrganizationSetting> {
        &self._relations.organization_setting_list
    }

    pub fn organization_setting_list_mut(&mut self) -> &mut SmartList<crate::OrganizationSetting> {
        &mut self._relations.organization_setting_list
    }

    pub fn eval_organization_setting_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OrganizationSetting>> {
        if !self.is_loaded("organization_setting_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "organization_setting_list".to_string(), attempted_path: "organization_setting_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.organization_setting_list)
        }
    }

    pub fn organization_member_list(&self) -> &SmartList<crate::OrganizationMember> {
        &self._relations.organization_member_list
    }

    pub fn organization_member_list_mut(&mut self) -> &mut SmartList<crate::OrganizationMember> {
        &mut self._relations.organization_member_list
    }

    pub fn eval_organization_member_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OrganizationMember>> {
        if !self.is_loaded("organization_member_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "organization_member_list".to_string(), attempted_path: "organization_member_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.organization_member_list)
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

    pub fn address_list(&self) -> &SmartList<crate::Address> {
        &self._relations.address_list
    }

    pub fn address_list_mut(&mut self) -> &mut SmartList<crate::Address> {
        &mut self._relations.address_list
    }

    pub fn eval_address_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Address>> {
        if !self.is_loaded("address_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "address_list".to_string(), attempted_path: "address_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.address_list)
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

    pub fn inventory_item_list(&self) -> &SmartList<crate::InventoryItem> {
        &self._relations.inventory_item_list
    }

    pub fn inventory_item_list_mut(&mut self) -> &mut SmartList<crate::InventoryItem> {
        &mut self._relations.inventory_item_list
    }

    pub fn eval_inventory_item_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::InventoryItem>> {
        if !self.is_loaded("inventory_item_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "inventory_item_list".to_string(), attempted_path: "inventory_item_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.inventory_item_list)
        }
    }

    pub fn packing_list_list(&self) -> &SmartList<crate::PackingList> {
        &self._relations.packing_list_list
    }

    pub fn packing_list_list_mut(&mut self) -> &mut SmartList<crate::PackingList> {
        &mut self._relations.packing_list_list
    }

    pub fn eval_packing_list_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PackingList>> {
        if !self.is_loaded("packing_list_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "packing_list_list".to_string(), attempted_path: "packing_list_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.packing_list_list)
        }
    }

    pub fn packing_item_list(&self) -> &SmartList<crate::PackingItem> {
        &self._relations.packing_item_list
    }

    pub fn packing_item_list_mut(&mut self) -> &mut SmartList<crate::PackingItem> {
        &mut self._relations.packing_item_list
    }

    pub fn eval_packing_item_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PackingItem>> {
        if !self.is_loaded("packing_item_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "packing_item_list".to_string(), attempted_path: "packing_item_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.packing_item_list)
        }
    }

    pub fn loading_plan_list(&self) -> &SmartList<crate::LoadingPlan> {
        &self._relations.loading_plan_list
    }

    pub fn loading_plan_list_mut(&mut self) -> &mut SmartList<crate::LoadingPlan> {
        &mut self._relations.loading_plan_list
    }

    pub fn eval_loading_plan_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LoadingPlan>> {
        if !self.is_loaded("loading_plan_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "loading_plan_list".to_string(), attempted_path: "loading_plan_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.loading_plan_list)
        }
    }

    pub fn unloading_plan_list(&self) -> &SmartList<crate::UnloadingPlan> {
        &self._relations.unloading_plan_list
    }

    pub fn unloading_plan_list_mut(&mut self) -> &mut SmartList<crate::UnloadingPlan> {
        &mut self._relations.unloading_plan_list
    }

    pub fn eval_unloading_plan_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UnloadingPlan>> {
        if !self.is_loaded("unloading_plan_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "unloading_plan_list".to_string(), attempted_path: "unloading_plan_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.unloading_plan_list)
        }
    }

    pub fn storage_facility_list(&self) -> &SmartList<crate::StorageFacility> {
        &self._relations.storage_facility_list
    }

    pub fn storage_facility_list_mut(&mut self) -> &mut SmartList<crate::StorageFacility> {
        &mut self._relations.storage_facility_list
    }

    pub fn eval_storage_facility_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::StorageFacility>> {
        if !self.is_loaded("storage_facility_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "storage_facility_list".to_string(), attempted_path: "storage_facility_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.storage_facility_list)
        }
    }

    pub fn storage_unit_list(&self) -> &SmartList<crate::StorageUnit> {
        &self._relations.storage_unit_list
    }

    pub fn storage_unit_list_mut(&mut self) -> &mut SmartList<crate::StorageUnit> {
        &mut self._relations.storage_unit_list
    }

    pub fn eval_storage_unit_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::StorageUnit>> {
        if !self.is_loaded("storage_unit_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "storage_unit_list".to_string(), attempted_path: "storage_unit_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.storage_unit_list)
        }
    }

    pub fn storage_inventory_list(&self) -> &SmartList<crate::StorageInventory> {
        &self._relations.storage_inventory_list
    }

    pub fn storage_inventory_list_mut(&mut self) -> &mut SmartList<crate::StorageInventory> {
        &mut self._relations.storage_inventory_list
    }

    pub fn eval_storage_inventory_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::StorageInventory>> {
        if !self.is_loaded("storage_inventory_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "storage_inventory_list".to_string(), attempted_path: "storage_inventory_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.storage_inventory_list)
        }
    }

    pub fn transport_manifest_list(&self) -> &SmartList<crate::TransportManifest> {
        &self._relations.transport_manifest_list
    }

    pub fn transport_manifest_list_mut(&mut self) -> &mut SmartList<crate::TransportManifest> {
        &mut self._relations.transport_manifest_list
    }

    pub fn eval_transport_manifest_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TransportManifest>> {
        if !self.is_loaded("transport_manifest_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "transport_manifest_list".to_string(), attempted_path: "transport_manifest_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.transport_manifest_list)
        }
    }

    pub fn customs_declaration_list(&self) -> &SmartList<crate::CustomsDeclaration> {
        &self._relations.customs_declaration_list
    }

    pub fn customs_declaration_list_mut(&mut self) -> &mut SmartList<crate::CustomsDeclaration> {
        &mut self._relations.customs_declaration_list
    }

    pub fn eval_customs_declaration_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomsDeclaration>> {
        if !self.is_loaded("customs_declaration_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customs_declaration_list".to_string(), attempted_path: "customs_declaration_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customs_declaration_list)
        }
    }

    pub fn equipment_checklist_list(&self) -> &SmartList<crate::EquipmentChecklist> {
        &self._relations.equipment_checklist_list
    }

    pub fn equipment_checklist_list_mut(&mut self) -> &mut SmartList<crate::EquipmentChecklist> {
        &mut self._relations.equipment_checklist_list
    }

    pub fn eval_equipment_checklist_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EquipmentChecklist>> {
        if !self.is_loaded("equipment_checklist_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "equipment_checklist_list".to_string(), attempted_path: "equipment_checklist_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.equipment_checklist_list)
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

    pub fn maintenance_request_list(&self) -> &SmartList<crate::MaintenanceRequest> {
        &self._relations.maintenance_request_list
    }

    pub fn maintenance_request_list_mut(&mut self) -> &mut SmartList<crate::MaintenanceRequest> {
        &mut self._relations.maintenance_request_list
    }

    pub fn eval_maintenance_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MaintenanceRequest>> {
        if !self.is_loaded("maintenance_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "maintenance_request_list".to_string(), attempted_path: "maintenance_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.maintenance_request_list)
        }
    }

    pub fn department_list(&self) -> &SmartList<crate::Department> {
        &self._relations.department_list
    }

    pub fn department_list_mut(&mut self) -> &mut SmartList<crate::Department> {
        &mut self._relations.department_list
    }

    pub fn eval_department_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Department>> {
        if !self.is_loaded("department_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "department_list".to_string(), attempted_path: "department_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.department_list)
        }
    }

    pub fn job_assignment_list(&self) -> &SmartList<crate::JobAssignment> {
        &self._relations.job_assignment_list
    }

    pub fn job_assignment_list_mut(&mut self) -> &mut SmartList<crate::JobAssignment> {
        &mut self._relations.job_assignment_list
    }

    pub fn eval_job_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::JobAssignment>> {
        if !self.is_loaded("job_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_assignment_list".to_string(), attempted_path: "job_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.job_assignment_list)
        }
    }

    pub fn work_shift_list(&self) -> &SmartList<crate::WorkShift> {
        &self._relations.work_shift_list
    }

    pub fn work_shift_list_mut(&mut self) -> &mut SmartList<crate::WorkShift> {
        &mut self._relations.work_shift_list
    }

    pub fn eval_work_shift_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::WorkShift>> {
        if !self.is_loaded("work_shift_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "work_shift_list".to_string(), attempted_path: "work_shift_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.work_shift_list)
        }
    }

    pub fn worked_hours_list(&self) -> &SmartList<crate::WorkedHours> {
        &self._relations.worked_hours_list
    }

    pub fn worked_hours_list_mut(&mut self) -> &mut SmartList<crate::WorkedHours> {
        &mut self._relations.worked_hours_list
    }

    pub fn eval_worked_hours_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::WorkedHours>> {
        if !self.is_loaded("worked_hours_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "worked_hours_list".to_string(), attempted_path: "worked_hours_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.worked_hours_list)
        }
    }

    pub fn payroll_period_list(&self) -> &SmartList<crate::PayrollPeriod> {
        &self._relations.payroll_period_list
    }

    pub fn payroll_period_list_mut(&mut self) -> &mut SmartList<crate::PayrollPeriod> {
        &mut self._relations.payroll_period_list
    }

    pub fn eval_payroll_period_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PayrollPeriod>> {
        if !self.is_loaded("payroll_period_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_period_list".to_string(), attempted_path: "payroll_period_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payroll_period_list)
        }
    }

    pub fn payroll_calculation_list(&self) -> &SmartList<crate::PayrollCalculation> {
        &self._relations.payroll_calculation_list
    }

    pub fn payroll_calculation_list_mut(&mut self) -> &mut SmartList<crate::PayrollCalculation> {
        &mut self._relations.payroll_calculation_list
    }

    pub fn eval_payroll_calculation_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PayrollCalculation>> {
        if !self.is_loaded("payroll_calculation_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_calculation_list".to_string(), attempted_path: "payroll_calculation_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payroll_calculation_list)
        }
    }

    pub fn payslip_list(&self) -> &SmartList<crate::Payslip> {
        &self._relations.payslip_list
    }

    pub fn payslip_list_mut(&mut self) -> &mut SmartList<crate::Payslip> {
        &mut self._relations.payslip_list
    }

    pub fn eval_payslip_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Payslip>> {
        if !self.is_loaded("payslip_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payslip_list".to_string(), attempted_path: "payslip_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payslip_list)
        }
    }

    pub fn bonus_list(&self) -> &SmartList<crate::Bonus> {
        &self._relations.bonus_list
    }

    pub fn bonus_list_mut(&mut self) -> &mut SmartList<crate::Bonus> {
        &mut self._relations.bonus_list
    }

    pub fn eval_bonus_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Bonus>> {
        if !self.is_loaded("bonus_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "bonus_list".to_string(), attempted_path: "bonus_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.bonus_list)
        }
    }

    pub fn employee_certification_list(&self) -> &SmartList<crate::EmployeeCertification> {
        &self._relations.employee_certification_list
    }

    pub fn employee_certification_list_mut(&mut self) -> &mut SmartList<crate::EmployeeCertification> {
        &mut self._relations.employee_certification_list
    }

    pub fn eval_employee_certification_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EmployeeCertification>> {
        if !self.is_loaded("employee_certification_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_certification_list".to_string(), attempted_path: "employee_certification_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.employee_certification_list)
        }
    }

    pub fn leave_request_list(&self) -> &SmartList<crate::LeaveRequest> {
        &self._relations.leave_request_list
    }

    pub fn leave_request_list_mut(&mut self) -> &mut SmartList<crate::LeaveRequest> {
        &mut self._relations.leave_request_list
    }

    pub fn eval_leave_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeaveRequest>> {
        if !self.is_loaded("leave_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "leave_request_list".to_string(), attempted_path: "leave_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.leave_request_list)
        }
    }

    pub fn billing_profile_list(&self) -> &SmartList<crate::BillingProfile> {
        &self._relations.billing_profile_list
    }

    pub fn billing_profile_list_mut(&mut self) -> &mut SmartList<crate::BillingProfile> {
        &mut self._relations.billing_profile_list
    }

    pub fn eval_billing_profile_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::BillingProfile>> {
        if !self.is_loaded("billing_profile_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "billing_profile_list".to_string(), attempted_path: "billing_profile_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.billing_profile_list)
        }
    }

    pub fn corporate_customer_profile_list(&self) -> &SmartList<crate::CorporateCustomerProfile> {
        &self._relations.corporate_customer_profile_list
    }

    pub fn corporate_customer_profile_list_mut(&mut self) -> &mut SmartList<crate::CorporateCustomerProfile> {
        &mut self._relations.corporate_customer_profile_list
    }

    pub fn eval_corporate_customer_profile_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CorporateCustomerProfile>> {
        if !self.is_loaded("corporate_customer_profile_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "corporate_customer_profile_list".to_string(), attempted_path: "corporate_customer_profile_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.corporate_customer_profile_list)
        }
    }

    pub fn customer_list(&self) -> &SmartList<crate::Customer> {
        &self._relations.customer_list
    }

    pub fn customer_list_mut(&mut self) -> &mut SmartList<crate::Customer> {
        &mut self._relations.customer_list
    }

    pub fn eval_customer_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Customer>> {
        if !self.is_loaded("customer_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_list".to_string(), attempted_path: "customer_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_list)
        }
    }

    pub fn customer_consent_list(&self) -> &SmartList<crate::CustomerConsent> {
        &self._relations.customer_consent_list
    }

    pub fn customer_consent_list_mut(&mut self) -> &mut SmartList<crate::CustomerConsent> {
        &mut self._relations.customer_consent_list
    }

    pub fn eval_customer_consent_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerConsent>> {
        if !self.is_loaded("customer_consent_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_consent_list".to_string(), attempted_path: "customer_consent_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_consent_list)
        }
    }

    pub fn customer_contact_list(&self) -> &SmartList<crate::CustomerContact> {
        &self._relations.customer_contact_list
    }

    pub fn customer_contact_list_mut(&mut self) -> &mut SmartList<crate::CustomerContact> {
        &mut self._relations.customer_contact_list
    }

    pub fn eval_customer_contact_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerContact>> {
        if !self.is_loaded("customer_contact_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_contact_list".to_string(), attempted_path: "customer_contact_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_contact_list)
        }
    }

    pub fn customer_history_list(&self) -> &SmartList<crate::CustomerHistory> {
        &self._relations.customer_history_list
    }

    pub fn customer_history_list_mut(&mut self) -> &mut SmartList<crate::CustomerHistory> {
        &mut self._relations.customer_history_list
    }

    pub fn eval_customer_history_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerHistory>> {
        if !self.is_loaded("customer_history_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_history_list".to_string(), attempted_path: "customer_history_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_history_list)
        }
    }

    pub fn customer_preference_list(&self) -> &SmartList<crate::CustomerPreference> {
        &self._relations.customer_preference_list
    }

    pub fn customer_preference_list_mut(&mut self) -> &mut SmartList<crate::CustomerPreference> {
        &mut self._relations.customer_preference_list
    }

    pub fn eval_customer_preference_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerPreference>> {
        if !self.is_loaded("customer_preference_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_preference_list".to_string(), attempted_path: "customer_preference_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_preference_list)
        }
    }

    pub fn private_customer_profile_list(&self) -> &SmartList<crate::PrivateCustomerProfile> {
        &self._relations.private_customer_profile_list
    }

    pub fn private_customer_profile_list_mut(&mut self) -> &mut SmartList<crate::PrivateCustomerProfile> {
        &mut self._relations.private_customer_profile_list
    }

    pub fn eval_private_customer_profile_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PrivateCustomerProfile>> {
        if !self.is_loaded("private_customer_profile_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "private_customer_profile_list".to_string(), attempted_path: "private_customer_profile_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.private_customer_profile_list)
        }
    }

    pub fn box_rental_list(&self) -> &SmartList<crate::BoxRental> {
        &self._relations.box_rental_list
    }

    pub fn box_rental_list_mut(&mut self) -> &mut SmartList<crate::BoxRental> {
        &mut self._relations.box_rental_list
    }

    pub fn eval_box_rental_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::BoxRental>> {
        if !self.is_loaded("box_rental_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "box_rental_list".to_string(), attempted_path: "box_rental_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.box_rental_list)
        }
    }

    pub fn cleaning_service_list(&self) -> &SmartList<crate::CleaningService> {
        &self._relations.cleaning_service_list
    }

    pub fn cleaning_service_list_mut(&mut self) -> &mut SmartList<crate::CleaningService> {
        &mut self._relations.cleaning_service_list
    }

    pub fn eval_cleaning_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CleaningService>> {
        if !self.is_loaded("cleaning_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "cleaning_service_list".to_string(), attempted_path: "cleaning_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.cleaning_service_list)
        }
    }

    pub fn moving_service_list(&self) -> &SmartList<crate::MovingService> {
        &self._relations.moving_service_list
    }

    pub fn moving_service_list_mut(&mut self) -> &mut SmartList<crate::MovingService> {
        &mut self._relations.moving_service_list
    }

    pub fn eval_moving_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MovingService>> {
        if !self.is_loaded("moving_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "moving_service_list".to_string(), attempted_path: "moving_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.moving_service_list)
        }
    }

    pub fn price_list_list(&self) -> &SmartList<crate::PriceList> {
        &self._relations.price_list_list
    }

    pub fn price_list_list_mut(&mut self) -> &mut SmartList<crate::PriceList> {
        &mut self._relations.price_list_list
    }

    pub fn eval_price_list_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PriceList>> {
        if !self.is_loaded("price_list_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "price_list_list".to_string(), attempted_path: "price_list_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.price_list_list)
        }
    }

    pub fn product_list(&self) -> &SmartList<crate::Product> {
        &self._relations.product_list
    }

    pub fn product_list_mut(&mut self) -> &mut SmartList<crate::Product> {
        &mut self._relations.product_list
    }

    pub fn eval_product_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Product>> {
        if !self.is_loaded("product_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "product_list".to_string(), attempted_path: "product_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.product_list)
        }
    }

    pub fn service_list(&self) -> &SmartList<crate::Service> {
        &self._relations.service_list
    }

    pub fn service_list_mut(&mut self) -> &mut SmartList<crate::Service> {
        &mut self._relations.service_list
    }

    pub fn eval_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Service>> {
        if !self.is_loaded("service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_list".to_string(), attempted_path: "service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.service_list)
        }
    }

    pub fn service_bundle_list(&self) -> &SmartList<crate::ServiceBundle> {
        &self._relations.service_bundle_list
    }

    pub fn service_bundle_list_mut(&mut self) -> &mut SmartList<crate::ServiceBundle> {
        &mut self._relations.service_bundle_list
    }

    pub fn eval_service_bundle_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServiceBundle>> {
        if !self.is_loaded("service_bundle_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_bundle_list".to_string(), attempted_path: "service_bundle_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.service_bundle_list)
        }
    }

    pub fn service_configuration_list(&self) -> &SmartList<crate::ServiceConfiguration> {
        &self._relations.service_configuration_list
    }

    pub fn service_configuration_list_mut(&mut self) -> &mut SmartList<crate::ServiceConfiguration> {
        &mut self._relations.service_configuration_list
    }

    pub fn eval_service_configuration_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServiceConfiguration>> {
        if !self.is_loaded("service_configuration_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_configuration_list".to_string(), attempted_path: "service_configuration_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.service_configuration_list)
        }
    }

    pub fn service_price_list(&self) -> &SmartList<crate::ServicePrice> {
        &self._relations.service_price_list
    }

    pub fn service_price_list_mut(&mut self) -> &mut SmartList<crate::ServicePrice> {
        &mut self._relations.service_price_list
    }

    pub fn eval_service_price_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServicePrice>> {
        if !self.is_loaded("service_price_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_price_list".to_string(), attempted_path: "service_price_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.service_price_list)
        }
    }

    pub fn campaign_list(&self) -> &SmartList<crate::Campaign> {
        &self._relations.campaign_list
    }

    pub fn campaign_list_mut(&mut self) -> &mut SmartList<crate::Campaign> {
        &mut self._relations.campaign_list
    }

    pub fn eval_campaign_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Campaign>> {
        if !self.is_loaded("campaign_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "campaign_list".to_string(), attempted_path: "campaign_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.campaign_list)
        }
    }

    pub fn conversion_event_list(&self) -> &SmartList<crate::ConversionEvent> {
        &self._relations.conversion_event_list
    }

    pub fn conversion_event_list_mut(&mut self) -> &mut SmartList<crate::ConversionEvent> {
        &mut self._relations.conversion_event_list
    }

    pub fn eval_conversion_event_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ConversionEvent>> {
        if !self.is_loaded("conversion_event_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "conversion_event_list".to_string(), attempted_path: "conversion_event_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.conversion_event_list)
        }
    }

    pub fn conversion_metric_list(&self) -> &SmartList<crate::ConversionMetric> {
        &self._relations.conversion_metric_list
    }

    pub fn conversion_metric_list_mut(&mut self) -> &mut SmartList<crate::ConversionMetric> {
        &mut self._relations.conversion_metric_list
    }

    pub fn eval_conversion_metric_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ConversionMetric>> {
        if !self.is_loaded("conversion_metric_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "conversion_metric_list".to_string(), attempted_path: "conversion_metric_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.conversion_metric_list)
        }
    }

    pub fn discount_code_list(&self) -> &SmartList<crate::DiscountCode> {
        &self._relations.discount_code_list
    }

    pub fn discount_code_list_mut(&mut self) -> &mut SmartList<crate::DiscountCode> {
        &mut self._relations.discount_code_list
    }

    pub fn eval_discount_code_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DiscountCode>> {
        if !self.is_loaded("discount_code_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "discount_code_list".to_string(), attempted_path: "discount_code_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.discount_code_list)
        }
    }

    pub fn lead_list(&self) -> &SmartList<crate::Lead> {
        &self._relations.lead_list
    }

    pub fn lead_list_mut(&mut self) -> &mut SmartList<crate::Lead> {
        &mut self._relations.lead_list
    }

    pub fn eval_lead_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Lead>> {
        if !self.is_loaded("lead_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "lead_list".to_string(), attempted_path: "lead_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.lead_list)
        }
    }

    pub fn lead_activity_list(&self) -> &SmartList<crate::LeadActivity> {
        &self._relations.lead_activity_list
    }

    pub fn lead_activity_list_mut(&mut self) -> &mut SmartList<crate::LeadActivity> {
        &mut self._relations.lead_activity_list
    }

    pub fn eval_lead_activity_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeadActivity>> {
        if !self.is_loaded("lead_activity_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "lead_activity_list".to_string(), attempted_path: "lead_activity_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.lead_activity_list)
        }
    }

    pub fn sales_opportunity_list(&self) -> &SmartList<crate::SalesOpportunity> {
        &self._relations.sales_opportunity_list
    }

    pub fn sales_opportunity_list_mut(&mut self) -> &mut SmartList<crate::SalesOpportunity> {
        &mut self._relations.sales_opportunity_list
    }

    pub fn eval_sales_opportunity_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SalesOpportunity>> {
        if !self.is_loaded("sales_opportunity_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "sales_opportunity_list".to_string(), attempted_path: "sales_opportunity_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.sales_opportunity_list)
        }
    }

    pub fn account_list(&self) -> &SmartList<crate::Account> {
        &self._relations.account_list
    }

    pub fn account_list_mut(&mut self) -> &mut SmartList<crate::Account> {
        &mut self._relations.account_list
    }

    pub fn eval_account_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Account>> {
        if !self.is_loaded("account_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "account_list".to_string(), attempted_path: "account_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.account_list)
        }
    }

    pub fn expense_list(&self) -> &SmartList<crate::Expense> {
        &self._relations.expense_list
    }

    pub fn expense_list_mut(&mut self) -> &mut SmartList<crate::Expense> {
        &mut self._relations.expense_list
    }

    pub fn eval_expense_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Expense>> {
        if !self.is_loaded("expense_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "expense_list".to_string(), attempted_path: "expense_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.expense_list)
        }
    }

    pub fn financial_summary_list(&self) -> &SmartList<crate::FinancialSummary> {
        &self._relations.financial_summary_list
    }

    pub fn financial_summary_list_mut(&mut self) -> &mut SmartList<crate::FinancialSummary> {
        &mut self._relations.financial_summary_list
    }

    pub fn eval_financial_summary_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::FinancialSummary>> {
        if !self.is_loaded("financial_summary_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "financial_summary_list".to_string(), attempted_path: "financial_summary_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.financial_summary_list)
        }
    }

    pub fn invoice_list(&self) -> &SmartList<crate::Invoice> {
        &self._relations.invoice_list
    }

    pub fn invoice_list_mut(&mut self) -> &mut SmartList<crate::Invoice> {
        &mut self._relations.invoice_list
    }

    pub fn eval_invoice_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Invoice>> {
        if !self.is_loaded("invoice_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "invoice_list".to_string(), attempted_path: "invoice_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.invoice_list)
        }
    }

    pub fn invoice_line_list(&self) -> &SmartList<crate::InvoiceLine> {
        &self._relations.invoice_line_list
    }

    pub fn invoice_line_list_mut(&mut self) -> &mut SmartList<crate::InvoiceLine> {
        &mut self._relations.invoice_line_list
    }

    pub fn eval_invoice_line_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::InvoiceLine>> {
        if !self.is_loaded("invoice_line_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "invoice_line_list".to_string(), attempted_path: "invoice_line_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.invoice_line_list)
        }
    }

    pub fn journal_entry_list(&self) -> &SmartList<crate::JournalEntry> {
        &self._relations.journal_entry_list
    }

    pub fn journal_entry_list_mut(&mut self) -> &mut SmartList<crate::JournalEntry> {
        &mut self._relations.journal_entry_list
    }

    pub fn eval_journal_entry_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::JournalEntry>> {
        if !self.is_loaded("journal_entry_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "journal_entry_list".to_string(), attempted_path: "journal_entry_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.journal_entry_list)
        }
    }

    pub fn payment_list(&self) -> &SmartList<crate::Payment> {
        &self._relations.payment_list
    }

    pub fn payment_list_mut(&mut self) -> &mut SmartList<crate::Payment> {
        &mut self._relations.payment_list
    }

    pub fn eval_payment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Payment>> {
        if !self.is_loaded("payment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payment_list".to_string(), attempted_path: "payment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payment_list)
        }
    }

    pub fn refund_list(&self) -> &SmartList<crate::Refund> {
        &self._relations.refund_list
    }

    pub fn refund_list_mut(&mut self) -> &mut SmartList<crate::Refund> {
        &mut self._relations.refund_list
    }

    pub fn eval_refund_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Refund>> {
        if !self.is_loaded("refund_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "refund_list".to_string(), attempted_path: "refund_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.refund_list)
        }
    }

    pub fn vat_rate_list(&self) -> &SmartList<crate::VatRate> {
        &self._relations.vat_rate_list
    }

    pub fn vat_rate_list_mut(&mut self) -> &mut SmartList<crate::VatRate> {
        &mut self._relations.vat_rate_list
    }

    pub fn eval_vat_rate_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::VatRate>> {
        if !self.is_loaded("vat_rate_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vat_rate_list".to_string(), attempted_path: "vat_rate_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.vat_rate_list)
        }
    }

    pub fn asset_assignment_list(&self) -> &SmartList<crate::AssetAssignment> {
        &self._relations.asset_assignment_list
    }

    pub fn asset_assignment_list_mut(&mut self) -> &mut SmartList<crate::AssetAssignment> {
        &mut self._relations.asset_assignment_list
    }

    pub fn eval_asset_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AssetAssignment>> {
        if !self.is_loaded("asset_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "asset_assignment_list".to_string(), attempted_path: "asset_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.asset_assignment_list)
        }
    }

    pub fn asset_inspection_list(&self) -> &SmartList<crate::AssetInspection> {
        &self._relations.asset_inspection_list
    }

    pub fn asset_inspection_list_mut(&mut self) -> &mut SmartList<crate::AssetInspection> {
        &mut self._relations.asset_inspection_list
    }

    pub fn eval_asset_inspection_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AssetInspection>> {
        if !self.is_loaded("asset_inspection_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "asset_inspection_list".to_string(), attempted_path: "asset_inspection_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.asset_inspection_list)
        }
    }

    pub fn consumable_list(&self) -> &SmartList<crate::Consumable> {
        &self._relations.consumable_list
    }

    pub fn consumable_list_mut(&mut self) -> &mut SmartList<crate::Consumable> {
        &mut self._relations.consumable_list
    }

    pub fn eval_consumable_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Consumable>> {
        if !self.is_loaded("consumable_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "consumable_list".to_string(), attempted_path: "consumable_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.consumable_list)
        }
    }

    pub fn equipment_list(&self) -> &SmartList<crate::Equipment> {
        &self._relations.equipment_list
    }

    pub fn equipment_list_mut(&mut self) -> &mut SmartList<crate::Equipment> {
        &mut self._relations.equipment_list
    }

    pub fn eval_equipment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Equipment>> {
        if !self.is_loaded("equipment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "equipment_list".to_string(), attempted_path: "equipment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.equipment_list)
        }
    }

    pub fn fuel_record_list(&self) -> &SmartList<crate::FuelRecord> {
        &self._relations.fuel_record_list
    }

    pub fn fuel_record_list_mut(&mut self) -> &mut SmartList<crate::FuelRecord> {
        &mut self._relations.fuel_record_list
    }

    pub fn eval_fuel_record_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::FuelRecord>> {
        if !self.is_loaded("fuel_record_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "fuel_record_list".to_string(), attempted_path: "fuel_record_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.fuel_record_list)
        }
    }

    pub fn maintenance_event_list(&self) -> &SmartList<crate::MaintenanceEvent> {
        &self._relations.maintenance_event_list
    }

    pub fn maintenance_event_list_mut(&mut self) -> &mut SmartList<crate::MaintenanceEvent> {
        &mut self._relations.maintenance_event_list
    }

    pub fn eval_maintenance_event_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MaintenanceEvent>> {
        if !self.is_loaded("maintenance_event_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "maintenance_event_list".to_string(), attempted_path: "maintenance_event_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.maintenance_event_list)
        }
    }

    pub fn maintenance_schedule_list(&self) -> &SmartList<crate::MaintenanceSchedule> {
        &self._relations.maintenance_schedule_list
    }

    pub fn maintenance_schedule_list_mut(&mut self) -> &mut SmartList<crate::MaintenanceSchedule> {
        &mut self._relations.maintenance_schedule_list
    }

    pub fn eval_maintenance_schedule_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MaintenanceSchedule>> {
        if !self.is_loaded("maintenance_schedule_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "maintenance_schedule_list".to_string(), attempted_path: "maintenance_schedule_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.maintenance_schedule_list)
        }
    }

    pub fn supplier_list(&self) -> &SmartList<crate::Supplier> {
        &self._relations.supplier_list
    }

    pub fn supplier_list_mut(&mut self) -> &mut SmartList<crate::Supplier> {
        &mut self._relations.supplier_list
    }

    pub fn eval_supplier_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Supplier>> {
        if !self.is_loaded("supplier_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "supplier_list".to_string(), attempted_path: "supplier_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.supplier_list)
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

    pub fn compliance_check_list(&self) -> &SmartList<crate::ComplianceCheck> {
        &self._relations.compliance_check_list
    }

    pub fn compliance_check_list_mut(&mut self) -> &mut SmartList<crate::ComplianceCheck> {
        &mut self._relations.compliance_check_list
    }

    pub fn eval_compliance_check_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ComplianceCheck>> {
        if !self.is_loaded("compliance_check_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "compliance_check_list".to_string(), attempted_path: "compliance_check_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.compliance_check_list)
        }
    }

    pub fn contract_list(&self) -> &SmartList<crate::Contract> {
        &self._relations.contract_list
    }

    pub fn contract_list_mut(&mut self) -> &mut SmartList<crate::Contract> {
        &mut self._relations.contract_list
    }

    pub fn eval_contract_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Contract>> {
        if !self.is_loaded("contract_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "contract_list".to_string(), attempted_path: "contract_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.contract_list)
        }
    }

    pub fn data_retention_policy_list(&self) -> &SmartList<crate::DataRetentionPolicy> {
        &self._relations.data_retention_policy_list
    }

    pub fn data_retention_policy_list_mut(&mut self) -> &mut SmartList<crate::DataRetentionPolicy> {
        &mut self._relations.data_retention_policy_list
    }

    pub fn eval_data_retention_policy_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DataRetentionPolicy>> {
        if !self.is_loaded("data_retention_policy_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "data_retention_policy_list".to_string(), attempted_path: "data_retention_policy_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.data_retention_policy_list)
        }
    }

    pub fn document_list(&self) -> &SmartList<crate::Document> {
        &self._relations.document_list
    }

    pub fn document_list_mut(&mut self) -> &mut SmartList<crate::Document> {
        &mut self._relations.document_list
    }

    pub fn eval_document_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Document>> {
        if !self.is_loaded("document_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "document_list".to_string(), attempted_path: "document_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.document_list)
        }
    }

    pub fn document_version_list(&self) -> &SmartList<crate::DocumentVersion> {
        &self._relations.document_version_list
    }

    pub fn document_version_list_mut(&mut self) -> &mut SmartList<crate::DocumentVersion> {
        &mut self._relations.document_version_list
    }

    pub fn eval_document_version_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DocumentVersion>> {
        if !self.is_loaded("document_version_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "document_version_list".to_string(), attempted_path: "document_version_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.document_version_list)
        }
    }

    pub fn insurance_claim_list(&self) -> &SmartList<crate::InsuranceClaim> {
        &self._relations.insurance_claim_list
    }

    pub fn insurance_claim_list_mut(&mut self) -> &mut SmartList<crate::InsuranceClaim> {
        &mut self._relations.insurance_claim_list
    }

    pub fn eval_insurance_claim_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::InsuranceClaim>> {
        if !self.is_loaded("insurance_claim_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "insurance_claim_list".to_string(), attempted_path: "insurance_claim_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.insurance_claim_list)
        }
    }

    pub fn insurance_policy_list(&self) -> &SmartList<crate::InsurancePolicy> {
        &self._relations.insurance_policy_list
    }

    pub fn insurance_policy_list_mut(&mut self) -> &mut SmartList<crate::InsurancePolicy> {
        &mut self._relations.insurance_policy_list
    }

    pub fn eval_insurance_policy_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::InsurancePolicy>> {
        if !self.is_loaded("insurance_policy_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "insurance_policy_list".to_string(), attempted_path: "insurance_policy_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.insurance_policy_list)
        }
    }

    pub fn recovery_request_list(&self) -> &SmartList<crate::RecoveryRequest> {
        &self._relations.recovery_request_list
    }

    pub fn recovery_request_list_mut(&mut self) -> &mut SmartList<crate::RecoveryRequest> {
        &mut self._relations.recovery_request_list
    }

    pub fn eval_recovery_request_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RecoveryRequest>> {
        if !self.is_loaded("recovery_request_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "recovery_request_list".to_string(), attempted_path: "recovery_request_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.recovery_request_list)
        }
    }

    pub fn magic_link_list(&self) -> &SmartList<crate::MagicLink> {
        &self._relations.magic_link_list
    }

    pub fn magic_link_list_mut(&mut self) -> &mut SmartList<crate::MagicLink> {
        &mut self._relations.magic_link_list
    }

    pub fn eval_magic_link_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MagicLink>> {
        if !self.is_loaded("magic_link_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "magic_link_list".to_string(), attempted_path: "magic_link_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.magic_link_list)
        }
    }

    pub fn permission_list(&self) -> &SmartList<crate::Permission> {
        &self._relations.permission_list
    }

    pub fn permission_list_mut(&mut self) -> &mut SmartList<crate::Permission> {
        &mut self._relations.permission_list
    }

    pub fn eval_permission_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Permission>> {
        if !self.is_loaded("permission_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "permission_list".to_string(), attempted_path: "permission_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.permission_list)
        }
    }

    pub fn role_list(&self) -> &SmartList<crate::Role> {
        &self._relations.role_list
    }

    pub fn role_list_mut(&mut self) -> &mut SmartList<crate::Role> {
        &mut self._relations.role_list
    }

    pub fn eval_role_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Role>> {
        if !self.is_loaded("role_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "role_list".to_string(), attempted_path: "role_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.role_list)
        }
    }

    pub fn role_permission_list(&self) -> &SmartList<crate::RolePermission> {
        &self._relations.role_permission_list
    }

    pub fn role_permission_list_mut(&mut self) -> &mut SmartList<crate::RolePermission> {
        &mut self._relations.role_permission_list
    }

    pub fn eval_role_permission_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RolePermission>> {
        if !self.is_loaded("role_permission_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "role_permission_list".to_string(), attempted_path: "role_permission_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.role_permission_list)
        }
    }

    pub fn user_account_list(&self) -> &SmartList<crate::UserAccount> {
        &self._relations.user_account_list
    }

    pub fn user_account_list_mut(&mut self) -> &mut SmartList<crate::UserAccount> {
        &mut self._relations.user_account_list
    }

    pub fn eval_user_account_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UserAccount>> {
        if !self.is_loaded("user_account_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_account_list".to_string(), attempted_path: "user_account_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.user_account_list)
        }
    }

    pub fn user_role_assignment_list(&self) -> &SmartList<crate::UserRoleAssignment> {
        &self._relations.user_role_assignment_list
    }

    pub fn user_role_assignment_list_mut(&mut self) -> &mut SmartList<crate::UserRoleAssignment> {
        &mut self._relations.user_role_assignment_list
    }

    pub fn eval_user_role_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UserRoleAssignment>> {
        if !self.is_loaded("user_role_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_role_assignment_list".to_string(), attempted_path: "user_role_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.user_role_assignment_list)
        }
    }

    pub fn user_session_list(&self) -> &SmartList<crate::UserSession> {
        &self._relations.user_session_list
    }

    pub fn user_session_list_mut(&mut self) -> &mut SmartList<crate::UserSession> {
        &mut self._relations.user_session_list
    }

    pub fn eval_user_session_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UserSession>> {
        if !self.is_loaded("user_session_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_session_list".to_string(), attempted_path: "user_session_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.user_session_list)
        }
    }

    pub fn activity_log_list(&self) -> &SmartList<crate::ActivityLog> {
        &self._relations.activity_log_list
    }

    pub fn activity_log_list_mut(&mut self) -> &mut SmartList<crate::ActivityLog> {
        &mut self._relations.activity_log_list
    }

    pub fn eval_activity_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ActivityLog>> {
        if !self.is_loaded("activity_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "activity_log_list".to_string(), attempted_path: "activity_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.activity_log_list)
        }
    }

    pub fn audit_log_list(&self) -> &SmartList<crate::AuditLog> {
        &self._relations.audit_log_list
    }

    pub fn audit_log_list_mut(&mut self) -> &mut SmartList<crate::AuditLog> {
        &mut self._relations.audit_log_list
    }

    pub fn eval_audit_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AuditLog>> {
        if !self.is_loaded("audit_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "audit_log_list".to_string(), attempted_path: "audit_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.audit_log_list)
        }
    }

    pub fn change_set_list(&self) -> &SmartList<crate::ChangeSet> {
        &self._relations.change_set_list
    }

    pub fn change_set_list_mut(&mut self) -> &mut SmartList<crate::ChangeSet> {
        &mut self._relations.change_set_list
    }

    pub fn eval_change_set_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ChangeSet>> {
        if !self.is_loaded("change_set_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "change_set_list".to_string(), attempted_path: "change_set_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.change_set_list)
        }
    }

    pub fn entity_change_list(&self) -> &SmartList<crate::EntityChange> {
        &self._relations.entity_change_list
    }

    pub fn entity_change_list_mut(&mut self) -> &mut SmartList<crate::EntityChange> {
        &mut self._relations.entity_change_list
    }

    pub fn eval_entity_change_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EntityChange>> {
        if !self.is_loaded("entity_change_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "entity_change_list".to_string(), attempted_path: "entity_change_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.entity_change_list)
        }
    }

    pub fn automation_action_list(&self) -> &SmartList<crate::AutomationAction> {
        &self._relations.automation_action_list
    }

    pub fn automation_action_list_mut(&mut self) -> &mut SmartList<crate::AutomationAction> {
        &mut self._relations.automation_action_list
    }

    pub fn eval_automation_action_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AutomationAction>> {
        if !self.is_loaded("automation_action_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "automation_action_list".to_string(), attempted_path: "automation_action_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.automation_action_list)
        }
    }

    pub fn automation_rule_list(&self) -> &SmartList<crate::AutomationRule> {
        &self._relations.automation_rule_list
    }

    pub fn automation_rule_list_mut(&mut self) -> &mut SmartList<crate::AutomationRule> {
        &mut self._relations.automation_rule_list
    }

    pub fn eval_automation_rule_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AutomationRule>> {
        if !self.is_loaded("automation_rule_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "automation_rule_list".to_string(), attempted_path: "automation_rule_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.automation_rule_list)
        }
    }

    pub fn automation_trigger_list(&self) -> &SmartList<crate::AutomationTrigger> {
        &self._relations.automation_trigger_list
    }

    pub fn automation_trigger_list_mut(&mut self) -> &mut SmartList<crate::AutomationTrigger> {
        &mut self._relations.automation_trigger_list
    }

    pub fn eval_automation_trigger_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AutomationTrigger>> {
        if !self.is_loaded("automation_trigger_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "automation_trigger_list".to_string(), attempted_path: "automation_trigger_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.automation_trigger_list)
        }
    }

    pub fn notification_list(&self) -> &SmartList<crate::Notification> {
        &self._relations.notification_list
    }

    pub fn notification_list_mut(&mut self) -> &mut SmartList<crate::Notification> {
        &mut self._relations.notification_list
    }

    pub fn eval_notification_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Notification>> {
        if !self.is_loaded("notification_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "notification_list".to_string(), attempted_path: "notification_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.notification_list)
        }
    }

    pub fn notification_template_list(&self) -> &SmartList<crate::NotificationTemplate> {
        &self._relations.notification_template_list
    }

    pub fn notification_template_list_mut(&mut self) -> &mut SmartList<crate::NotificationTemplate> {
        &mut self._relations.notification_template_list
    }

    pub fn eval_notification_template_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::NotificationTemplate>> {
        if !self.is_loaded("notification_template_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "notification_template_list".to_string(), attempted_path: "notification_template_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.notification_template_list)
        }
    }

    pub fn api_client_list(&self) -> &SmartList<crate::ApiClient> {
        &self._relations.api_client_list
    }

    pub fn api_client_list_mut(&mut self) -> &mut SmartList<crate::ApiClient> {
        &mut self._relations.api_client_list
    }

    pub fn eval_api_client_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ApiClient>> {
        if !self.is_loaded("api_client_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "api_client_list".to_string(), attempted_path: "api_client_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.api_client_list)
        }
    }

    pub fn api_endpoint_list(&self) -> &SmartList<crate::ApiEndpoint> {
        &self._relations.api_endpoint_list
    }

    pub fn api_endpoint_list_mut(&mut self) -> &mut SmartList<crate::ApiEndpoint> {
        &mut self._relations.api_endpoint_list
    }

    pub fn eval_api_endpoint_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ApiEndpoint>> {
        if !self.is_loaded("api_endpoint_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "api_endpoint_list".to_string(), attempted_path: "api_endpoint_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.api_endpoint_list)
        }
    }

    pub fn integration_mapping_list(&self) -> &SmartList<crate::IntegrationMapping> {
        &self._relations.integration_mapping_list
    }

    pub fn integration_mapping_list_mut(&mut self) -> &mut SmartList<crate::IntegrationMapping> {
        &mut self._relations.integration_mapping_list
    }

    pub fn eval_integration_mapping_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::IntegrationMapping>> {
        if !self.is_loaded("integration_mapping_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "integration_mapping_list".to_string(), attempted_path: "integration_mapping_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.integration_mapping_list)
        }
    }

    pub fn webhook_list(&self) -> &SmartList<crate::Webhook> {
        &self._relations.webhook_list
    }

    pub fn webhook_list_mut(&mut self) -> &mut SmartList<crate::Webhook> {
        &mut self._relations.webhook_list
    }

    pub fn eval_webhook_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Webhook>> {
        if !self.is_loaded("webhook_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "webhook_list".to_string(), attempted_path: "webhook_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.webhook_list)
        }
    }

    pub fn webhook_delivery_list(&self) -> &SmartList<crate::WebhookDelivery> {
        &self._relations.webhook_delivery_list
    }

    pub fn webhook_delivery_list_mut(&mut self) -> &mut SmartList<crate::WebhookDelivery> {
        &mut self._relations.webhook_delivery_list
    }

    pub fn eval_webhook_delivery_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::WebhookDelivery>> {
        if !self.is_loaded("webhook_delivery_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "webhook_delivery_list".to_string(), attempted_path: "webhook_delivery_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.webhook_delivery_list)
        }
    }

    pub fn platform_configuration_list(&self) -> &SmartList<crate::PlatformConfiguration> {
        &self._relations.platform_configuration_list
    }

    pub fn platform_configuration_list_mut(&mut self) -> &mut SmartList<crate::PlatformConfiguration> {
        &mut self._relations.platform_configuration_list
    }

    pub fn eval_platform_configuration_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PlatformConfiguration>> {
        if !self.is_loaded("platform_configuration_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_configuration_list".to_string(), attempted_path: "platform_configuration_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.platform_configuration_list)
        }
    }

    pub fn platform_locale_list(&self) -> &SmartList<crate::PlatformLocale> {
        &self._relations.platform_locale_list
    }

    pub fn platform_locale_list_mut(&mut self) -> &mut SmartList<crate::PlatformLocale> {
        &mut self._relations.platform_locale_list
    }

    pub fn eval_platform_locale_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PlatformLocale>> {
        if !self.is_loaded("platform_locale_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_locale_list".to_string(), attempted_path: "platform_locale_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.platform_locale_list)
        }
    }

    pub fn merchant_branch_list(&self) -> &SmartList<crate::MerchantBranch> {
        &self._relations.merchant_branch_list
    }

    pub fn merchant_branch_list_mut(&mut self) -> &mut SmartList<crate::MerchantBranch> {
        &mut self._relations.merchant_branch_list
    }

    pub fn eval_merchant_branch_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MerchantBranch>> {
        if !self.is_loaded("merchant_branch_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_branch_list".to_string(), attempted_path: "merchant_branch_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.merchant_branch_list)
        }
    }

    pub fn merchant_setting_list(&self) -> &SmartList<crate::MerchantSetting> {
        &self._relations.merchant_setting_list
    }

    pub fn merchant_setting_list_mut(&mut self) -> &mut SmartList<crate::MerchantSetting> {
        &mut self._relations.merchant_setting_list
    }

    pub fn eval_merchant_setting_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MerchantSetting>> {
        if !self.is_loaded("merchant_setting_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_setting_list".to_string(), attempted_path: "merchant_setting_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.merchant_setting_list)
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

    pub fn extra_operations_logistics1_list(&self) -> &SmartList<crate::ExtraOperationsLogistics1> {
        &self._relations.extra_operations_logistics1_list
    }

    pub fn extra_operations_logistics1_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics1> {
        &mut self._relations.extra_operations_logistics1_list
    }

    pub fn eval_extra_operations_logistics1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics1>> {
        if !self.is_loaded("extra_operations_logistics1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics1_list".to_string(), attempted_path: "extra_operations_logistics1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics1_list)
        }
    }

    pub fn extra_operations_logistics2_list(&self) -> &SmartList<crate::ExtraOperationsLogistics2> {
        &self._relations.extra_operations_logistics2_list
    }

    pub fn extra_operations_logistics2_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics2> {
        &mut self._relations.extra_operations_logistics2_list
    }

    pub fn eval_extra_operations_logistics2_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics2>> {
        if !self.is_loaded("extra_operations_logistics2_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics2_list".to_string(), attempted_path: "extra_operations_logistics2_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics2_list)
        }
    }

    pub fn extra_operations_logistics3_list(&self) -> &SmartList<crate::ExtraOperationsLogistics3> {
        &self._relations.extra_operations_logistics3_list
    }

    pub fn extra_operations_logistics3_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics3> {
        &mut self._relations.extra_operations_logistics3_list
    }

    pub fn eval_extra_operations_logistics3_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics3>> {
        if !self.is_loaded("extra_operations_logistics3_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics3_list".to_string(), attempted_path: "extra_operations_logistics3_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics3_list)
        }
    }

    pub fn extra_operations_logistics4_list(&self) -> &SmartList<crate::ExtraOperationsLogistics4> {
        &self._relations.extra_operations_logistics4_list
    }

    pub fn extra_operations_logistics4_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics4> {
        &mut self._relations.extra_operations_logistics4_list
    }

    pub fn eval_extra_operations_logistics4_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics4>> {
        if !self.is_loaded("extra_operations_logistics4_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics4_list".to_string(), attempted_path: "extra_operations_logistics4_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics4_list)
        }
    }

    pub fn extra_operations_logistics5_list(&self) -> &SmartList<crate::ExtraOperationsLogistics5> {
        &self._relations.extra_operations_logistics5_list
    }

    pub fn extra_operations_logistics5_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics5> {
        &mut self._relations.extra_operations_logistics5_list
    }

    pub fn eval_extra_operations_logistics5_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics5>> {
        if !self.is_loaded("extra_operations_logistics5_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics5_list".to_string(), attempted_path: "extra_operations_logistics5_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics5_list)
        }
    }

    pub fn extra_operations_logistics6_list(&self) -> &SmartList<crate::ExtraOperationsLogistics6> {
        &self._relations.extra_operations_logistics6_list
    }

    pub fn extra_operations_logistics6_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics6> {
        &mut self._relations.extra_operations_logistics6_list
    }

    pub fn eval_extra_operations_logistics6_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics6>> {
        if !self.is_loaded("extra_operations_logistics6_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics6_list".to_string(), attempted_path: "extra_operations_logistics6_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics6_list)
        }
    }

    pub fn extra_operations_logistics7_list(&self) -> &SmartList<crate::ExtraOperationsLogistics7> {
        &self._relations.extra_operations_logistics7_list
    }

    pub fn extra_operations_logistics7_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics7> {
        &mut self._relations.extra_operations_logistics7_list
    }

    pub fn eval_extra_operations_logistics7_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics7>> {
        if !self.is_loaded("extra_operations_logistics7_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics7_list".to_string(), attempted_path: "extra_operations_logistics7_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics7_list)
        }
    }

    pub fn extra_operations_logistics8_list(&self) -> &SmartList<crate::ExtraOperationsLogistics8> {
        &self._relations.extra_operations_logistics8_list
    }

    pub fn extra_operations_logistics8_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics8> {
        &mut self._relations.extra_operations_logistics8_list
    }

    pub fn eval_extra_operations_logistics8_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics8>> {
        if !self.is_loaded("extra_operations_logistics8_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics8_list".to_string(), attempted_path: "extra_operations_logistics8_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics8_list)
        }
    }

    pub fn extra_operations_logistics9_list(&self) -> &SmartList<crate::ExtraOperationsLogistics9> {
        &self._relations.extra_operations_logistics9_list
    }

    pub fn extra_operations_logistics9_list_mut(&mut self) -> &mut SmartList<crate::ExtraOperationsLogistics9> {
        &mut self._relations.extra_operations_logistics9_list
    }

    pub fn eval_extra_operations_logistics9_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraOperationsLogistics9>> {
        if !self.is_loaded("extra_operations_logistics9_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_operations_logistics9_list".to_string(), attempted_path: "extra_operations_logistics9_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_operations_logistics9_list)
        }
    }

    pub fn employee_availability_list(&self) -> &SmartList<crate::EmployeeAvailability> {
        &self._relations.employee_availability_list
    }

    pub fn employee_availability_list_mut(&mut self) -> &mut SmartList<crate::EmployeeAvailability> {
        &mut self._relations.employee_availability_list
    }

    pub fn eval_employee_availability_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EmployeeAvailability>> {
        if !self.is_loaded("employee_availability_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_availability_list".to_string(), attempted_path: "employee_availability_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.employee_availability_list)
        }
    }

    pub fn payroll_deduction_list(&self) -> &SmartList<crate::PayrollDeduction> {
        &self._relations.payroll_deduction_list
    }

    pub fn payroll_deduction_list_mut(&mut self) -> &mut SmartList<crate::PayrollDeduction> {
        &mut self._relations.payroll_deduction_list
    }

    pub fn eval_payroll_deduction_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PayrollDeduction>> {
        if !self.is_loaded("payroll_deduction_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payroll_deduction_list".to_string(), attempted_path: "payroll_deduction_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payroll_deduction_list)
        }
    }

    pub fn training_session_list(&self) -> &SmartList<crate::TrainingSession> {
        &self._relations.training_session_list
    }

    pub fn training_session_list_mut(&mut self) -> &mut SmartList<crate::TrainingSession> {
        &mut self._relations.training_session_list
    }

    pub fn eval_training_session_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TrainingSession>> {
        if !self.is_loaded("training_session_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "training_session_list".to_string(), attempted_path: "training_session_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.training_session_list)
        }
    }

    pub fn shift_assignment_list(&self) -> &SmartList<crate::ShiftAssignment> {
        &self._relations.shift_assignment_list
    }

    pub fn shift_assignment_list_mut(&mut self) -> &mut SmartList<crate::ShiftAssignment> {
        &mut self._relations.shift_assignment_list
    }

    pub fn eval_shift_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ShiftAssignment>> {
        if !self.is_loaded("shift_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "shift_assignment_list".to_string(), attempted_path: "shift_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.shift_assignment_list)
        }
    }

    pub fn extra_employees_payroll1_list(&self) -> &SmartList<crate::ExtraEmployeesPayroll1> {
        &self._relations.extra_employees_payroll1_list
    }

    pub fn extra_employees_payroll1_list_mut(&mut self) -> &mut SmartList<crate::ExtraEmployeesPayroll1> {
        &mut self._relations.extra_employees_payroll1_list
    }

    pub fn eval_extra_employees_payroll1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraEmployeesPayroll1>> {
        if !self.is_loaded("extra_employees_payroll1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_employees_payroll1_list".to_string(), attempted_path: "extra_employees_payroll1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_employees_payroll1_list)
        }
    }

    pub fn extra_employees_payroll2_list(&self) -> &SmartList<crate::ExtraEmployeesPayroll2> {
        &self._relations.extra_employees_payroll2_list
    }

    pub fn extra_employees_payroll2_list_mut(&mut self) -> &mut SmartList<crate::ExtraEmployeesPayroll2> {
        &mut self._relations.extra_employees_payroll2_list
    }

    pub fn eval_extra_employees_payroll2_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraEmployeesPayroll2>> {
        if !self.is_loaded("extra_employees_payroll2_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_employees_payroll2_list".to_string(), attempted_path: "extra_employees_payroll2_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_employees_payroll2_list)
        }
    }

    pub fn extra_employees_payroll3_list(&self) -> &SmartList<crate::ExtraEmployeesPayroll3> {
        &self._relations.extra_employees_payroll3_list
    }

    pub fn extra_employees_payroll3_list_mut(&mut self) -> &mut SmartList<crate::ExtraEmployeesPayroll3> {
        &mut self._relations.extra_employees_payroll3_list
    }

    pub fn eval_extra_employees_payroll3_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraEmployeesPayroll3>> {
        if !self.is_loaded("extra_employees_payroll3_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_employees_payroll3_list".to_string(), attempted_path: "extra_employees_payroll3_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_employees_payroll3_list)
        }
    }

    pub fn extra_employees_payroll4_list(&self) -> &SmartList<crate::ExtraEmployeesPayroll4> {
        &self._relations.extra_employees_payroll4_list
    }

    pub fn extra_employees_payroll4_list_mut(&mut self) -> &mut SmartList<crate::ExtraEmployeesPayroll4> {
        &mut self._relations.extra_employees_payroll4_list
    }

    pub fn eval_extra_employees_payroll4_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraEmployeesPayroll4>> {
        if !self.is_loaded("extra_employees_payroll4_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_employees_payroll4_list".to_string(), attempted_path: "extra_employees_payroll4_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_employees_payroll4_list)
        }
    }

    pub fn extra_employees_payroll5_list(&self) -> &SmartList<crate::ExtraEmployeesPayroll5> {
        &self._relations.extra_employees_payroll5_list
    }

    pub fn extra_employees_payroll5_list_mut(&mut self) -> &mut SmartList<crate::ExtraEmployeesPayroll5> {
        &mut self._relations.extra_employees_payroll5_list
    }

    pub fn eval_extra_employees_payroll5_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraEmployeesPayroll5>> {
        if !self.is_loaded("extra_employees_payroll5_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_employees_payroll5_list".to_string(), attempted_path: "extra_employees_payroll5_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_employees_payroll5_list)
        }
    }

    pub fn extra_employees_payroll6_list(&self) -> &SmartList<crate::ExtraEmployeesPayroll6> {
        &self._relations.extra_employees_payroll6_list
    }

    pub fn extra_employees_payroll6_list_mut(&mut self) -> &mut SmartList<crate::ExtraEmployeesPayroll6> {
        &mut self._relations.extra_employees_payroll6_list
    }

    pub fn eval_extra_employees_payroll6_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraEmployeesPayroll6>> {
        if !self.is_loaded("extra_employees_payroll6_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_employees_payroll6_list".to_string(), attempted_path: "extra_employees_payroll6_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_employees_payroll6_list)
        }
    }

    pub fn extra_employees_payroll7_list(&self) -> &SmartList<crate::ExtraEmployeesPayroll7> {
        &self._relations.extra_employees_payroll7_list
    }

    pub fn extra_employees_payroll7_list_mut(&mut self) -> &mut SmartList<crate::ExtraEmployeesPayroll7> {
        &mut self._relations.extra_employees_payroll7_list
    }

    pub fn eval_extra_employees_payroll7_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraEmployeesPayroll7>> {
        if !self.is_loaded("extra_employees_payroll7_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_employees_payroll7_list".to_string(), attempted_path: "extra_employees_payroll7_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_employees_payroll7_list)
        }
    }

    pub fn customer_complaint_list(&self) -> &SmartList<crate::CustomerComplaint> {
        &self._relations.customer_complaint_list
    }

    pub fn customer_complaint_list_mut(&mut self) -> &mut SmartList<crate::CustomerComplaint> {
        &mut self._relations.customer_complaint_list
    }

    pub fn eval_customer_complaint_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerComplaint>> {
        if !self.is_loaded("customer_complaint_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_complaint_list".to_string(), attempted_path: "customer_complaint_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_complaint_list)
        }
    }

    pub fn customer_note_list(&self) -> &SmartList<crate::CustomerNote> {
        &self._relations.customer_note_list
    }

    pub fn customer_note_list_mut(&mut self) -> &mut SmartList<crate::CustomerNote> {
        &mut self._relations.customer_note_list
    }

    pub fn eval_customer_note_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerNote>> {
        if !self.is_loaded("customer_note_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_note_list".to_string(), attempted_path: "customer_note_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_note_list)
        }
    }

    pub fn extra_customer_management1_list(&self) -> &SmartList<crate::ExtraCustomerManagement1> {
        &self._relations.extra_customer_management1_list
    }

    pub fn extra_customer_management1_list_mut(&mut self) -> &mut SmartList<crate::ExtraCustomerManagement1> {
        &mut self._relations.extra_customer_management1_list
    }

    pub fn eval_extra_customer_management1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraCustomerManagement1>> {
        if !self.is_loaded("extra_customer_management1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_customer_management1_list".to_string(), attempted_path: "extra_customer_management1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_customer_management1_list)
        }
    }

    pub fn extra_customer_management2_list(&self) -> &SmartList<crate::ExtraCustomerManagement2> {
        &self._relations.extra_customer_management2_list
    }

    pub fn extra_customer_management2_list_mut(&mut self) -> &mut SmartList<crate::ExtraCustomerManagement2> {
        &mut self._relations.extra_customer_management2_list
    }

    pub fn eval_extra_customer_management2_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraCustomerManagement2>> {
        if !self.is_loaded("extra_customer_management2_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_customer_management2_list".to_string(), attempted_path: "extra_customer_management2_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_customer_management2_list)
        }
    }

    pub fn extra_customer_management3_list(&self) -> &SmartList<crate::ExtraCustomerManagement3> {
        &self._relations.extra_customer_management3_list
    }

    pub fn extra_customer_management3_list_mut(&mut self) -> &mut SmartList<crate::ExtraCustomerManagement3> {
        &mut self._relations.extra_customer_management3_list
    }

    pub fn eval_extra_customer_management3_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraCustomerManagement3>> {
        if !self.is_loaded("extra_customer_management3_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_customer_management3_list".to_string(), attempted_path: "extra_customer_management3_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_customer_management3_list)
        }
    }

    pub fn extra_customer_management4_list(&self) -> &SmartList<crate::ExtraCustomerManagement4> {
        &self._relations.extra_customer_management4_list
    }

    pub fn extra_customer_management4_list_mut(&mut self) -> &mut SmartList<crate::ExtraCustomerManagement4> {
        &mut self._relations.extra_customer_management4_list
    }

    pub fn eval_extra_customer_management4_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraCustomerManagement4>> {
        if !self.is_loaded("extra_customer_management4_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_customer_management4_list".to_string(), attempted_path: "extra_customer_management4_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_customer_management4_list)
        }
    }

    pub fn extra_customer_management5_list(&self) -> &SmartList<crate::ExtraCustomerManagement5> {
        &self._relations.extra_customer_management5_list
    }

    pub fn extra_customer_management5_list_mut(&mut self) -> &mut SmartList<crate::ExtraCustomerManagement5> {
        &mut self._relations.extra_customer_management5_list
    }

    pub fn eval_extra_customer_management5_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraCustomerManagement5>> {
        if !self.is_loaded("extra_customer_management5_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_customer_management5_list".to_string(), attempted_path: "extra_customer_management5_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_customer_management5_list)
        }
    }

    pub fn extra_customer_management6_list(&self) -> &SmartList<crate::ExtraCustomerManagement6> {
        &self._relations.extra_customer_management6_list
    }

    pub fn extra_customer_management6_list_mut(&mut self) -> &mut SmartList<crate::ExtraCustomerManagement6> {
        &mut self._relations.extra_customer_management6_list
    }

    pub fn eval_extra_customer_management6_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraCustomerManagement6>> {
        if !self.is_loaded("extra_customer_management6_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_customer_management6_list".to_string(), attempted_path: "extra_customer_management6_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_customer_management6_list)
        }
    }

    pub fn storage_service_list(&self) -> &SmartList<crate::StorageService> {
        &self._relations.storage_service_list
    }

    pub fn storage_service_list_mut(&mut self) -> &mut SmartList<crate::StorageService> {
        &mut self._relations.storage_service_list
    }

    pub fn eval_storage_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::StorageService>> {
        if !self.is_loaded("storage_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "storage_service_list".to_string(), attempted_path: "storage_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.storage_service_list)
        }
    }

    pub fn packing_service_list(&self) -> &SmartList<crate::PackingService> {
        &self._relations.packing_service_list
    }

    pub fn packing_service_list_mut(&mut self) -> &mut SmartList<crate::PackingService> {
        &mut self._relations.packing_service_list
    }

    pub fn eval_packing_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PackingService>> {
        if !self.is_loaded("packing_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "packing_service_list".to_string(), attempted_path: "packing_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.packing_service_list)
        }
    }

    pub fn disposal_service_list(&self) -> &SmartList<crate::DisposalService> {
        &self._relations.disposal_service_list
    }

    pub fn disposal_service_list_mut(&mut self) -> &mut SmartList<crate::DisposalService> {
        &mut self._relations.disposal_service_list
    }

    pub fn eval_disposal_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DisposalService>> {
        if !self.is_loaded("disposal_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "disposal_service_list".to_string(), attempted_path: "disposal_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.disposal_service_list)
        }
    }

    pub fn rental_period_list(&self) -> &SmartList<crate::RentalPeriod> {
        &self._relations.rental_period_list
    }

    pub fn rental_period_list_mut(&mut self) -> &mut SmartList<crate::RentalPeriod> {
        &mut self._relations.rental_period_list
    }

    pub fn eval_rental_period_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RentalPeriod>> {
        if !self.is_loaded("rental_period_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "rental_period_list".to_string(), attempted_path: "rental_period_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.rental_period_list)
        }
    }

    pub fn service_area_list(&self) -> &SmartList<crate::ServiceArea> {
        &self._relations.service_area_list
    }

    pub fn service_area_list_mut(&mut self) -> &mut SmartList<crate::ServiceArea> {
        &mut self._relations.service_area_list
    }

    pub fn eval_service_area_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServiceArea>> {
        if !self.is_loaded("service_area_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_area_list".to_string(), attempted_path: "service_area_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.service_area_list)
        }
    }

    pub fn extra_products_services1_list(&self) -> &SmartList<crate::ExtraProductsServices1> {
        &self._relations.extra_products_services1_list
    }

    pub fn extra_products_services1_list_mut(&mut self) -> &mut SmartList<crate::ExtraProductsServices1> {
        &mut self._relations.extra_products_services1_list
    }

    pub fn eval_extra_products_services1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraProductsServices1>> {
        if !self.is_loaded("extra_products_services1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_products_services1_list".to_string(), attempted_path: "extra_products_services1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_products_services1_list)
        }
    }

    pub fn extra_products_services2_list(&self) -> &SmartList<crate::ExtraProductsServices2> {
        &self._relations.extra_products_services2_list
    }

    pub fn extra_products_services2_list_mut(&mut self) -> &mut SmartList<crate::ExtraProductsServices2> {
        &mut self._relations.extra_products_services2_list
    }

    pub fn eval_extra_products_services2_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraProductsServices2>> {
        if !self.is_loaded("extra_products_services2_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_products_services2_list".to_string(), attempted_path: "extra_products_services2_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_products_services2_list)
        }
    }

    pub fn extra_products_services3_list(&self) -> &SmartList<crate::ExtraProductsServices3> {
        &self._relations.extra_products_services3_list
    }

    pub fn extra_products_services3_list_mut(&mut self) -> &mut SmartList<crate::ExtraProductsServices3> {
        &mut self._relations.extra_products_services3_list
    }

    pub fn eval_extra_products_services3_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraProductsServices3>> {
        if !self.is_loaded("extra_products_services3_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_products_services3_list".to_string(), attempted_path: "extra_products_services3_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_products_services3_list)
        }
    }

    pub fn extra_products_services4_list(&self) -> &SmartList<crate::ExtraProductsServices4> {
        &self._relations.extra_products_services4_list
    }

    pub fn extra_products_services4_list_mut(&mut self) -> &mut SmartList<crate::ExtraProductsServices4> {
        &mut self._relations.extra_products_services4_list
    }

    pub fn eval_extra_products_services4_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraProductsServices4>> {
        if !self.is_loaded("extra_products_services4_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_products_services4_list".to_string(), attempted_path: "extra_products_services4_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_products_services4_list)
        }
    }

    pub fn campaign_audience_list(&self) -> &SmartList<crate::CampaignAudience> {
        &self._relations.campaign_audience_list
    }

    pub fn campaign_audience_list_mut(&mut self) -> &mut SmartList<crate::CampaignAudience> {
        &mut self._relations.campaign_audience_list
    }

    pub fn eval_campaign_audience_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CampaignAudience>> {
        if !self.is_loaded("campaign_audience_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "campaign_audience_list".to_string(), attempted_path: "campaign_audience_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.campaign_audience_list)
        }
    }

    pub fn campaign_channel_list(&self) -> &SmartList<crate::CampaignChannel> {
        &self._relations.campaign_channel_list
    }

    pub fn campaign_channel_list_mut(&mut self) -> &mut SmartList<crate::CampaignChannel> {
        &mut self._relations.campaign_channel_list
    }

    pub fn eval_campaign_channel_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CampaignChannel>> {
        if !self.is_loaded("campaign_channel_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "campaign_channel_list".to_string(), attempted_path: "campaign_channel_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.campaign_channel_list)
        }
    }

    pub fn lead_attribution_list(&self) -> &SmartList<crate::LeadAttribution> {
        &self._relations.lead_attribution_list
    }

    pub fn lead_attribution_list_mut(&mut self) -> &mut SmartList<crate::LeadAttribution> {
        &mut self._relations.lead_attribution_list
    }

    pub fn eval_lead_attribution_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LeadAttribution>> {
        if !self.is_loaded("lead_attribution_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "lead_attribution_list".to_string(), attempted_path: "lead_attribution_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.lead_attribution_list)
        }
    }

    pub fn sales_funnel_list(&self) -> &SmartList<crate::SalesFunnel> {
        &self._relations.sales_funnel_list
    }

    pub fn sales_funnel_list_mut(&mut self) -> &mut SmartList<crate::SalesFunnel> {
        &mut self._relations.sales_funnel_list
    }

    pub fn eval_sales_funnel_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SalesFunnel>> {
        if !self.is_loaded("sales_funnel_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "sales_funnel_list".to_string(), attempted_path: "sales_funnel_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.sales_funnel_list)
        }
    }

    pub fn extra_marketing_sales1_list(&self) -> &SmartList<crate::ExtraMarketingSales1> {
        &self._relations.extra_marketing_sales1_list
    }

    pub fn extra_marketing_sales1_list_mut(&mut self) -> &mut SmartList<crate::ExtraMarketingSales1> {
        &mut self._relations.extra_marketing_sales1_list
    }

    pub fn eval_extra_marketing_sales1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraMarketingSales1>> {
        if !self.is_loaded("extra_marketing_sales1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_marketing_sales1_list".to_string(), attempted_path: "extra_marketing_sales1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_marketing_sales1_list)
        }
    }

    pub fn extra_marketing_sales2_list(&self) -> &SmartList<crate::ExtraMarketingSales2> {
        &self._relations.extra_marketing_sales2_list
    }

    pub fn extra_marketing_sales2_list_mut(&mut self) -> &mut SmartList<crate::ExtraMarketingSales2> {
        &mut self._relations.extra_marketing_sales2_list
    }

    pub fn eval_extra_marketing_sales2_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraMarketingSales2>> {
        if !self.is_loaded("extra_marketing_sales2_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_marketing_sales2_list".to_string(), attempted_path: "extra_marketing_sales2_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_marketing_sales2_list)
        }
    }

    pub fn extra_marketing_sales3_list(&self) -> &SmartList<crate::ExtraMarketingSales3> {
        &self._relations.extra_marketing_sales3_list
    }

    pub fn extra_marketing_sales3_list_mut(&mut self) -> &mut SmartList<crate::ExtraMarketingSales3> {
        &mut self._relations.extra_marketing_sales3_list
    }

    pub fn eval_extra_marketing_sales3_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraMarketingSales3>> {
        if !self.is_loaded("extra_marketing_sales3_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_marketing_sales3_list".to_string(), attempted_path: "extra_marketing_sales3_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_marketing_sales3_list)
        }
    }

    pub fn extra_marketing_sales4_list(&self) -> &SmartList<crate::ExtraMarketingSales4> {
        &self._relations.extra_marketing_sales4_list
    }

    pub fn extra_marketing_sales4_list_mut(&mut self) -> &mut SmartList<crate::ExtraMarketingSales4> {
        &mut self._relations.extra_marketing_sales4_list
    }

    pub fn eval_extra_marketing_sales4_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraMarketingSales4>> {
        if !self.is_loaded("extra_marketing_sales4_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_marketing_sales4_list".to_string(), attempted_path: "extra_marketing_sales4_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_marketing_sales4_list)
        }
    }

    pub fn expense_claim_list(&self) -> &SmartList<crate::ExpenseClaim> {
        &self._relations.expense_claim_list
    }

    pub fn expense_claim_list_mut(&mut self) -> &mut SmartList<crate::ExpenseClaim> {
        &mut self._relations.expense_claim_list
    }

    pub fn eval_expense_claim_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExpenseClaim>> {
        if !self.is_loaded("expense_claim_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "expense_claim_list".to_string(), attempted_path: "expense_claim_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.expense_claim_list)
        }
    }

    pub fn settlement_list(&self) -> &SmartList<crate::Settlement> {
        &self._relations.settlement_list
    }

    pub fn settlement_list_mut(&mut self) -> &mut SmartList<crate::Settlement> {
        &mut self._relations.settlement_list
    }

    pub fn eval_settlement_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Settlement>> {
        if !self.is_loaded("settlement_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "settlement_list".to_string(), attempted_path: "settlement_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.settlement_list)
        }
    }

    pub fn receivable_list(&self) -> &SmartList<crate::Receivable> {
        &self._relations.receivable_list
    }

    pub fn receivable_list_mut(&mut self) -> &mut SmartList<crate::Receivable> {
        &mut self._relations.receivable_list
    }

    pub fn eval_receivable_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Receivable>> {
        if !self.is_loaded("receivable_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "receivable_list".to_string(), attempted_path: "receivable_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.receivable_list)
        }
    }

    pub fn payable_list(&self) -> &SmartList<crate::Payable> {
        &self._relations.payable_list
    }

    pub fn payable_list_mut(&mut self) -> &mut SmartList<crate::Payable> {
        &mut self._relations.payable_list
    }

    pub fn eval_payable_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Payable>> {
        if !self.is_loaded("payable_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payable_list".to_string(), attempted_path: "payable_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payable_list)
        }
    }

    pub fn extra_finance_accounting1_list(&self) -> &SmartList<crate::ExtraFinanceAccounting1> {
        &self._relations.extra_finance_accounting1_list
    }

    pub fn extra_finance_accounting1_list_mut(&mut self) -> &mut SmartList<crate::ExtraFinanceAccounting1> {
        &mut self._relations.extra_finance_accounting1_list
    }

    pub fn eval_extra_finance_accounting1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraFinanceAccounting1>> {
        if !self.is_loaded("extra_finance_accounting1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_finance_accounting1_list".to_string(), attempted_path: "extra_finance_accounting1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_finance_accounting1_list)
        }
    }

    pub fn extra_finance_accounting2_list(&self) -> &SmartList<crate::ExtraFinanceAccounting2> {
        &self._relations.extra_finance_accounting2_list
    }

    pub fn extra_finance_accounting2_list_mut(&mut self) -> &mut SmartList<crate::ExtraFinanceAccounting2> {
        &mut self._relations.extra_finance_accounting2_list
    }

    pub fn eval_extra_finance_accounting2_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraFinanceAccounting2>> {
        if !self.is_loaded("extra_finance_accounting2_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_finance_accounting2_list".to_string(), attempted_path: "extra_finance_accounting2_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_finance_accounting2_list)
        }
    }

    pub fn extra_finance_accounting3_list(&self) -> &SmartList<crate::ExtraFinanceAccounting3> {
        &self._relations.extra_finance_accounting3_list
    }

    pub fn extra_finance_accounting3_list_mut(&mut self) -> &mut SmartList<crate::ExtraFinanceAccounting3> {
        &mut self._relations.extra_finance_accounting3_list
    }

    pub fn eval_extra_finance_accounting3_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraFinanceAccounting3>> {
        if !self.is_loaded("extra_finance_accounting3_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_finance_accounting3_list".to_string(), attempted_path: "extra_finance_accounting3_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_finance_accounting3_list)
        }
    }

    pub fn extra_finance_accounting4_list(&self) -> &SmartList<crate::ExtraFinanceAccounting4> {
        &self._relations.extra_finance_accounting4_list
    }

    pub fn extra_finance_accounting4_list_mut(&mut self) -> &mut SmartList<crate::ExtraFinanceAccounting4> {
        &mut self._relations.extra_finance_accounting4_list
    }

    pub fn eval_extra_finance_accounting4_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraFinanceAccounting4>> {
        if !self.is_loaded("extra_finance_accounting4_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_finance_accounting4_list".to_string(), attempted_path: "extra_finance_accounting4_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_finance_accounting4_list)
        }
    }

    pub fn vehicle_inspection_list(&self) -> &SmartList<crate::VehicleInspection> {
        &self._relations.vehicle_inspection_list
    }

    pub fn vehicle_inspection_list_mut(&mut self) -> &mut SmartList<crate::VehicleInspection> {
        &mut self._relations.vehicle_inspection_list
    }

    pub fn eval_vehicle_inspection_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::VehicleInspection>> {
        if !self.is_loaded("vehicle_inspection_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_inspection_list".to_string(), attempted_path: "vehicle_inspection_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.vehicle_inspection_list)
        }
    }

    pub fn equipment_checkout_list(&self) -> &SmartList<crate::EquipmentCheckout> {
        &self._relations.equipment_checkout_list
    }

    pub fn equipment_checkout_list_mut(&mut self) -> &mut SmartList<crate::EquipmentCheckout> {
        &mut self._relations.equipment_checkout_list
    }

    pub fn eval_equipment_checkout_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::EquipmentCheckout>> {
        if !self.is_loaded("equipment_checkout_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "equipment_checkout_list".to_string(), attempted_path: "equipment_checkout_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.equipment_checkout_list)
        }
    }

    pub fn consumable_reorder_list(&self) -> &SmartList<crate::ConsumableReorder> {
        &self._relations.consumable_reorder_list
    }

    pub fn consumable_reorder_list_mut(&mut self) -> &mut SmartList<crate::ConsumableReorder> {
        &mut self._relations.consumable_reorder_list
    }

    pub fn eval_consumable_reorder_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ConsumableReorder>> {
        if !self.is_loaded("consumable_reorder_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "consumable_reorder_list".to_string(), attempted_path: "consumable_reorder_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.consumable_reorder_list)
        }
    }

    pub fn extra_asset_management1_list(&self) -> &SmartList<crate::ExtraAssetManagement1> {
        &self._relations.extra_asset_management1_list
    }

    pub fn extra_asset_management1_list_mut(&mut self) -> &mut SmartList<crate::ExtraAssetManagement1> {
        &mut self._relations.extra_asset_management1_list
    }

    pub fn eval_extra_asset_management1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraAssetManagement1>> {
        if !self.is_loaded("extra_asset_management1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_asset_management1_list".to_string(), attempted_path: "extra_asset_management1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_asset_management1_list)
        }
    }

    pub fn extra_asset_management2_list(&self) -> &SmartList<crate::ExtraAssetManagement2> {
        &self._relations.extra_asset_management2_list
    }

    pub fn extra_asset_management2_list_mut(&mut self) -> &mut SmartList<crate::ExtraAssetManagement2> {
        &mut self._relations.extra_asset_management2_list
    }

    pub fn eval_extra_asset_management2_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraAssetManagement2>> {
        if !self.is_loaded("extra_asset_management2_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_asset_management2_list".to_string(), attempted_path: "extra_asset_management2_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_asset_management2_list)
        }
    }

    pub fn extra_asset_management3_list(&self) -> &SmartList<crate::ExtraAssetManagement3> {
        &self._relations.extra_asset_management3_list
    }

    pub fn extra_asset_management3_list_mut(&mut self) -> &mut SmartList<crate::ExtraAssetManagement3> {
        &mut self._relations.extra_asset_management3_list
    }

    pub fn eval_extra_asset_management3_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraAssetManagement3>> {
        if !self.is_loaded("extra_asset_management3_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_asset_management3_list".to_string(), attempted_path: "extra_asset_management3_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_asset_management3_list)
        }
    }

    pub fn extra_asset_management4_list(&self) -> &SmartList<crate::ExtraAssetManagement4> {
        &self._relations.extra_asset_management4_list
    }

    pub fn extra_asset_management4_list_mut(&mut self) -> &mut SmartList<crate::ExtraAssetManagement4> {
        &mut self._relations.extra_asset_management4_list
    }

    pub fn eval_extra_asset_management4_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraAssetManagement4>> {
        if !self.is_loaded("extra_asset_management4_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_asset_management4_list".to_string(), attempted_path: "extra_asset_management4_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_asset_management4_list)
        }
    }

    pub fn extra_asset_management5_list(&self) -> &SmartList<crate::ExtraAssetManagement5> {
        &self._relations.extra_asset_management5_list
    }

    pub fn extra_asset_management5_list_mut(&mut self) -> &mut SmartList<crate::ExtraAssetManagement5> {
        &mut self._relations.extra_asset_management5_list
    }

    pub fn eval_extra_asset_management5_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraAssetManagement5>> {
        if !self.is_loaded("extra_asset_management5_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_asset_management5_list".to_string(), attempted_path: "extra_asset_management5_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_asset_management5_list)
        }
    }

    pub fn authentication_attempt_list(&self) -> &SmartList<crate::AuthenticationAttempt> {
        &self._relations.authentication_attempt_list
    }

    pub fn authentication_attempt_list_mut(&mut self) -> &mut SmartList<crate::AuthenticationAttempt> {
        &mut self._relations.authentication_attempt_list
    }

    pub fn eval_authentication_attempt_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AuthenticationAttempt>> {
        if !self.is_loaded("authentication_attempt_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "authentication_attempt_list".to_string(), attempted_path: "authentication_attempt_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.authentication_attempt_list)
        }
    }

    pub fn access_policy_list(&self) -> &SmartList<crate::AccessPolicy> {
        &self._relations.access_policy_list
    }

    pub fn access_policy_list_mut(&mut self) -> &mut SmartList<crate::AccessPolicy> {
        &mut self._relations.access_policy_list
    }

    pub fn eval_access_policy_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AccessPolicy>> {
        if !self.is_loaded("access_policy_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "access_policy_list".to_string(), attempted_path: "access_policy_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.access_policy_list)
        }
    }

    pub fn extra_identity_access1_list(&self) -> &SmartList<crate::ExtraIdentityAccess1> {
        &self._relations.extra_identity_access1_list
    }

    pub fn extra_identity_access1_list_mut(&mut self) -> &mut SmartList<crate::ExtraIdentityAccess1> {
        &mut self._relations.extra_identity_access1_list
    }

    pub fn eval_extra_identity_access1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraIdentityAccess1>> {
        if !self.is_loaded("extra_identity_access1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_identity_access1_list".to_string(), attempted_path: "extra_identity_access1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_identity_access1_list)
        }
    }

    pub fn audit_export_list(&self) -> &SmartList<crate::AuditExport> {
        &self._relations.audit_export_list
    }

    pub fn audit_export_list_mut(&mut self) -> &mut SmartList<crate::AuditExport> {
        &mut self._relations.audit_export_list
    }

    pub fn eval_audit_export_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AuditExport>> {
        if !self.is_loaded("audit_export_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "audit_export_list".to_string(), attempted_path: "audit_export_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.audit_export_list)
        }
    }

    pub fn extra_activity_audit1_list(&self) -> &SmartList<crate::ExtraActivityAudit1> {
        &self._relations.extra_activity_audit1_list
    }

    pub fn extra_activity_audit1_list_mut(&mut self) -> &mut SmartList<crate::ExtraActivityAudit1> {
        &mut self._relations.extra_activity_audit1_list
    }

    pub fn eval_extra_activity_audit1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraActivityAudit1>> {
        if !self.is_loaded("extra_activity_audit1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_activity_audit1_list".to_string(), attempted_path: "extra_activity_audit1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_activity_audit1_list)
        }
    }

    pub fn notification_preference_list(&self) -> &SmartList<crate::NotificationPreference> {
        &self._relations.notification_preference_list
    }

    pub fn notification_preference_list_mut(&mut self) -> &mut SmartList<crate::NotificationPreference> {
        &mut self._relations.notification_preference_list
    }

    pub fn eval_notification_preference_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::NotificationPreference>> {
        if !self.is_loaded("notification_preference_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "notification_preference_list".to_string(), attempted_path: "notification_preference_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.notification_preference_list)
        }
    }

    pub fn notification_delivery_list(&self) -> &SmartList<crate::NotificationDelivery> {
        &self._relations.notification_delivery_list
    }

    pub fn notification_delivery_list_mut(&mut self) -> &mut SmartList<crate::NotificationDelivery> {
        &mut self._relations.notification_delivery_list
    }

    pub fn eval_notification_delivery_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::NotificationDelivery>> {
        if !self.is_loaded("notification_delivery_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "notification_delivery_list".to_string(), attempted_path: "notification_delivery_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.notification_delivery_list)
        }
    }

    pub fn synchronization_run_list(&self) -> &SmartList<crate::SynchronizationRun> {
        &self._relations.synchronization_run_list
    }

    pub fn synchronization_run_list_mut(&mut self) -> &mut SmartList<crate::SynchronizationRun> {
        &mut self._relations.synchronization_run_list
    }

    pub fn eval_synchronization_run_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SynchronizationRun>> {
        if !self.is_loaded("synchronization_run_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "synchronization_run_list".to_string(), attempted_path: "synchronization_run_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.synchronization_run_list)
        }
    }

    pub fn extra_api_integrations1_list(&self) -> &SmartList<crate::ExtraApiIntegrations1> {
        &self._relations.extra_api_integrations1_list
    }

    pub fn extra_api_integrations1_list_mut(&mut self) -> &mut SmartList<crate::ExtraApiIntegrations1> {
        &mut self._relations.extra_api_integrations1_list
    }

    pub fn eval_extra_api_integrations1_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ExtraApiIntegrations1>> {
        if !self.is_loaded("extra_api_integrations1_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "extra_api_integrations1_list".to_string(), attempted_path: "extra_api_integrations1_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.extra_api_integrations1_list)
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
#[teaql(relation(target = "Employee", local_key = "id", foreign_key = "merchant_id", many))]
    employee_list: SmartList<crate::Employee>,
#[teaql(relation(target = "PlatformSetting", local_key = "id", foreign_key = "merchant_id", many))]
    platform_setting_list: SmartList<crate::PlatformSetting>,
#[teaql(relation(target = "PlatformUser", local_key = "id", foreign_key = "merchant_id", many))]
    platform_user_list: SmartList<crate::PlatformUser>,
#[teaql(relation(target = "PlatformAuditLog", local_key = "id", foreign_key = "merchant_id", many))]
    platform_audit_log_list: SmartList<crate::PlatformAuditLog>,
#[teaql(relation(target = "Organization", local_key = "id", foreign_key = "merchant_id", many))]
    organization_list: SmartList<crate::Organization>,
#[teaql(relation(target = "OrganizationSetting", local_key = "id", foreign_key = "merchant_id", many))]
    organization_setting_list: SmartList<crate::OrganizationSetting>,
#[teaql(relation(target = "OrganizationMember", local_key = "id", foreign_key = "merchant_id", many))]
    organization_member_list: SmartList<crate::OrganizationMember>,
#[teaql(relation(target = "MoveOrder", local_key = "id", foreign_key = "merchant_id", many))]
    move_order_list: SmartList<crate::MoveOrder>,
#[teaql(relation(target = "MoveQuote", local_key = "id", foreign_key = "merchant_id", many))]
    move_quote_list: SmartList<crate::MoveQuote>,
#[teaql(relation(target = "Route", local_key = "id", foreign_key = "merchant_id", many))]
    route_list: SmartList<crate::Route>,
#[teaql(relation(target = "RouteStop", local_key = "id", foreign_key = "merchant_id", many))]
    route_stop_list: SmartList<crate::RouteStop>,
#[teaql(relation(target = "TimeSlot", local_key = "id", foreign_key = "merchant_id", many))]
    time_slot_list: SmartList<crate::TimeSlot>,
#[teaql(relation(target = "FulfillmentEvent", local_key = "id", foreign_key = "merchant_id", many))]
    fulfillment_event_list: SmartList<crate::FulfillmentEvent>,
#[teaql(relation(target = "Address", local_key = "id", foreign_key = "merchant_id", many))]
    address_list: SmartList<crate::Address>,
#[teaql(relation(target = "Crew", local_key = "id", foreign_key = "merchant_id", many))]
    crew_list: SmartList<crate::Crew>,
#[teaql(relation(target = "DispatchAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    dispatch_assignment_list: SmartList<crate::DispatchAssignment>,
#[teaql(relation(target = "DamageReport", local_key = "id", foreign_key = "merchant_id", many))]
    damage_report_list: SmartList<crate::DamageReport>,
#[teaql(relation(target = "ProofOfDelivery", local_key = "id", foreign_key = "merchant_id", many))]
    proof_of_delivery_list: SmartList<crate::ProofOfDelivery>,
#[teaql(relation(target = "InventoryItem", local_key = "id", foreign_key = "merchant_id", many))]
    inventory_item_list: SmartList<crate::InventoryItem>,
#[teaql(relation(target = "PackingList", local_key = "id", foreign_key = "merchant_id", many))]
    packing_list_list: SmartList<crate::PackingList>,
#[teaql(relation(target = "PackingItem", local_key = "id", foreign_key = "merchant_id", many))]
    packing_item_list: SmartList<crate::PackingItem>,
#[teaql(relation(target = "LoadingPlan", local_key = "id", foreign_key = "merchant_id", many))]
    loading_plan_list: SmartList<crate::LoadingPlan>,
#[teaql(relation(target = "UnloadingPlan", local_key = "id", foreign_key = "merchant_id", many))]
    unloading_plan_list: SmartList<crate::UnloadingPlan>,
#[teaql(relation(target = "StorageFacility", local_key = "id", foreign_key = "merchant_id", many))]
    storage_facility_list: SmartList<crate::StorageFacility>,
#[teaql(relation(target = "StorageUnit", local_key = "id", foreign_key = "merchant_id", many))]
    storage_unit_list: SmartList<crate::StorageUnit>,
#[teaql(relation(target = "StorageInventory", local_key = "id", foreign_key = "merchant_id", many))]
    storage_inventory_list: SmartList<crate::StorageInventory>,
#[teaql(relation(target = "TransportManifest", local_key = "id", foreign_key = "merchant_id", many))]
    transport_manifest_list: SmartList<crate::TransportManifest>,
#[teaql(relation(target = "CustomsDeclaration", local_key = "id", foreign_key = "merchant_id", many))]
    customs_declaration_list: SmartList<crate::CustomsDeclaration>,
#[teaql(relation(target = "EquipmentChecklist", local_key = "id", foreign_key = "merchant_id", many))]
    equipment_checklist_list: SmartList<crate::EquipmentChecklist>,
#[teaql(relation(target = "FuelLog", local_key = "id", foreign_key = "merchant_id", many))]
    fuel_log_list: SmartList<crate::FuelLog>,
#[teaql(relation(target = "MaintenanceRequest", local_key = "id", foreign_key = "merchant_id", many))]
    maintenance_request_list: SmartList<crate::MaintenanceRequest>,
#[teaql(relation(target = "Department", local_key = "id", foreign_key = "merchant_id", many))]
    department_list: SmartList<crate::Department>,
#[teaql(relation(target = "JobAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    job_assignment_list: SmartList<crate::JobAssignment>,
#[teaql(relation(target = "WorkShift", local_key = "id", foreign_key = "merchant_id", many))]
    work_shift_list: SmartList<crate::WorkShift>,
#[teaql(relation(target = "WorkedHours", local_key = "id", foreign_key = "merchant_id", many))]
    worked_hours_list: SmartList<crate::WorkedHours>,
#[teaql(relation(target = "PayrollPeriod", local_key = "id", foreign_key = "merchant_id", many))]
    payroll_period_list: SmartList<crate::PayrollPeriod>,
#[teaql(relation(target = "PayrollCalculation", local_key = "id", foreign_key = "merchant_id", many))]
    payroll_calculation_list: SmartList<crate::PayrollCalculation>,
#[teaql(relation(target = "Payslip", local_key = "id", foreign_key = "merchant_id", many))]
    payslip_list: SmartList<crate::Payslip>,
#[teaql(relation(target = "Bonus", local_key = "id", foreign_key = "merchant_id", many))]
    bonus_list: SmartList<crate::Bonus>,
#[teaql(relation(target = "EmployeeCertification", local_key = "id", foreign_key = "merchant_id", many))]
    employee_certification_list: SmartList<crate::EmployeeCertification>,
#[teaql(relation(target = "LeaveRequest", local_key = "id", foreign_key = "merchant_id", many))]
    leave_request_list: SmartList<crate::LeaveRequest>,
#[teaql(relation(target = "BillingProfile", local_key = "id", foreign_key = "merchant_id", many))]
    billing_profile_list: SmartList<crate::BillingProfile>,
#[teaql(relation(target = "CorporateCustomerProfile", local_key = "id", foreign_key = "merchant_id", many))]
    corporate_customer_profile_list: SmartList<crate::CorporateCustomerProfile>,
#[teaql(relation(target = "Customer", local_key = "id", foreign_key = "merchant_id", many))]
    customer_list: SmartList<crate::Customer>,
#[teaql(relation(target = "CustomerConsent", local_key = "id", foreign_key = "merchant_id", many))]
    customer_consent_list: SmartList<crate::CustomerConsent>,
#[teaql(relation(target = "CustomerContact", local_key = "id", foreign_key = "merchant_id", many))]
    customer_contact_list: SmartList<crate::CustomerContact>,
#[teaql(relation(target = "CustomerHistory", local_key = "id", foreign_key = "merchant_id", many))]
    customer_history_list: SmartList<crate::CustomerHistory>,
#[teaql(relation(target = "CustomerPreference", local_key = "id", foreign_key = "merchant_id", many))]
    customer_preference_list: SmartList<crate::CustomerPreference>,
#[teaql(relation(target = "PrivateCustomerProfile", local_key = "id", foreign_key = "merchant_id", many))]
    private_customer_profile_list: SmartList<crate::PrivateCustomerProfile>,
#[teaql(relation(target = "BoxRental", local_key = "id", foreign_key = "merchant_id", many))]
    box_rental_list: SmartList<crate::BoxRental>,
#[teaql(relation(target = "CleaningService", local_key = "id", foreign_key = "merchant_id", many))]
    cleaning_service_list: SmartList<crate::CleaningService>,
#[teaql(relation(target = "MovingService", local_key = "id", foreign_key = "merchant_id", many))]
    moving_service_list: SmartList<crate::MovingService>,
#[teaql(relation(target = "PriceList", local_key = "id", foreign_key = "merchant_id", many))]
    price_list_list: SmartList<crate::PriceList>,
#[teaql(relation(target = "Product", local_key = "id", foreign_key = "merchant_id", many))]
    product_list: SmartList<crate::Product>,
#[teaql(relation(target = "Service", local_key = "id", foreign_key = "merchant_id", many))]
    service_list: SmartList<crate::Service>,
#[teaql(relation(target = "ServiceBundle", local_key = "id", foreign_key = "merchant_id", many))]
    service_bundle_list: SmartList<crate::ServiceBundle>,
#[teaql(relation(target = "ServiceConfiguration", local_key = "id", foreign_key = "merchant_id", many))]
    service_configuration_list: SmartList<crate::ServiceConfiguration>,
#[teaql(relation(target = "ServicePrice", local_key = "id", foreign_key = "merchant_id", many))]
    service_price_list: SmartList<crate::ServicePrice>,
#[teaql(relation(target = "Campaign", local_key = "id", foreign_key = "merchant_id", many))]
    campaign_list: SmartList<crate::Campaign>,
#[teaql(relation(target = "ConversionEvent", local_key = "id", foreign_key = "merchant_id", many))]
    conversion_event_list: SmartList<crate::ConversionEvent>,
#[teaql(relation(target = "ConversionMetric", local_key = "id", foreign_key = "merchant_id", many))]
    conversion_metric_list: SmartList<crate::ConversionMetric>,
#[teaql(relation(target = "DiscountCode", local_key = "id", foreign_key = "merchant_id", many))]
    discount_code_list: SmartList<crate::DiscountCode>,
#[teaql(relation(target = "Lead", local_key = "id", foreign_key = "merchant_id", many))]
    lead_list: SmartList<crate::Lead>,
#[teaql(relation(target = "LeadActivity", local_key = "id", foreign_key = "merchant_id", many))]
    lead_activity_list: SmartList<crate::LeadActivity>,
#[teaql(relation(target = "SalesOpportunity", local_key = "id", foreign_key = "merchant_id", many))]
    sales_opportunity_list: SmartList<crate::SalesOpportunity>,
#[teaql(relation(target = "Account", local_key = "id", foreign_key = "merchant_id", many))]
    account_list: SmartList<crate::Account>,
#[teaql(relation(target = "Expense", local_key = "id", foreign_key = "merchant_id", many))]
    expense_list: SmartList<crate::Expense>,
#[teaql(relation(target = "FinancialSummary", local_key = "id", foreign_key = "merchant_id", many))]
    financial_summary_list: SmartList<crate::FinancialSummary>,
#[teaql(relation(target = "Invoice", local_key = "id", foreign_key = "merchant_id", many))]
    invoice_list: SmartList<crate::Invoice>,
#[teaql(relation(target = "InvoiceLine", local_key = "id", foreign_key = "merchant_id", many))]
    invoice_line_list: SmartList<crate::InvoiceLine>,
#[teaql(relation(target = "JournalEntry", local_key = "id", foreign_key = "merchant_id", many))]
    journal_entry_list: SmartList<crate::JournalEntry>,
#[teaql(relation(target = "Payment", local_key = "id", foreign_key = "merchant_id", many))]
    payment_list: SmartList<crate::Payment>,
#[teaql(relation(target = "Refund", local_key = "id", foreign_key = "merchant_id", many))]
    refund_list: SmartList<crate::Refund>,
#[teaql(relation(target = "VatRate", local_key = "id", foreign_key = "merchant_id", many))]
    vat_rate_list: SmartList<crate::VatRate>,
#[teaql(relation(target = "AssetAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    asset_assignment_list: SmartList<crate::AssetAssignment>,
#[teaql(relation(target = "AssetInspection", local_key = "id", foreign_key = "merchant_id", many))]
    asset_inspection_list: SmartList<crate::AssetInspection>,
#[teaql(relation(target = "Consumable", local_key = "id", foreign_key = "merchant_id", many))]
    consumable_list: SmartList<crate::Consumable>,
#[teaql(relation(target = "Equipment", local_key = "id", foreign_key = "merchant_id", many))]
    equipment_list: SmartList<crate::Equipment>,
#[teaql(relation(target = "FuelRecord", local_key = "id", foreign_key = "merchant_id", many))]
    fuel_record_list: SmartList<crate::FuelRecord>,
#[teaql(relation(target = "MaintenanceEvent", local_key = "id", foreign_key = "merchant_id", many))]
    maintenance_event_list: SmartList<crate::MaintenanceEvent>,
#[teaql(relation(target = "MaintenanceSchedule", local_key = "id", foreign_key = "merchant_id", many))]
    maintenance_schedule_list: SmartList<crate::MaintenanceSchedule>,
#[teaql(relation(target = "Supplier", local_key = "id", foreign_key = "merchant_id", many))]
    supplier_list: SmartList<crate::Supplier>,
#[teaql(relation(target = "Vehicle", local_key = "id", foreign_key = "merchant_id", many))]
    vehicle_list: SmartList<crate::Vehicle>,
#[teaql(relation(target = "ComplianceCheck", local_key = "id", foreign_key = "merchant_id", many))]
    compliance_check_list: SmartList<crate::ComplianceCheck>,
#[teaql(relation(target = "Contract", local_key = "id", foreign_key = "merchant_id", many))]
    contract_list: SmartList<crate::Contract>,
#[teaql(relation(target = "DataRetentionPolicy", local_key = "id", foreign_key = "merchant_id", many))]
    data_retention_policy_list: SmartList<crate::DataRetentionPolicy>,
#[teaql(relation(target = "Document", local_key = "id", foreign_key = "merchant_id", many))]
    document_list: SmartList<crate::Document>,
#[teaql(relation(target = "DocumentVersion", local_key = "id", foreign_key = "merchant_id", many))]
    document_version_list: SmartList<crate::DocumentVersion>,
#[teaql(relation(target = "InsuranceClaim", local_key = "id", foreign_key = "merchant_id", many))]
    insurance_claim_list: SmartList<crate::InsuranceClaim>,
#[teaql(relation(target = "InsurancePolicy", local_key = "id", foreign_key = "merchant_id", many))]
    insurance_policy_list: SmartList<crate::InsurancePolicy>,
#[teaql(relation(target = "RecoveryRequest", local_key = "id", foreign_key = "merchant_id", many))]
    recovery_request_list: SmartList<crate::RecoveryRequest>,
#[teaql(relation(target = "MagicLink", local_key = "id", foreign_key = "merchant_id", many))]
    magic_link_list: SmartList<crate::MagicLink>,
#[teaql(relation(target = "Permission", local_key = "id", foreign_key = "merchant_id", many))]
    permission_list: SmartList<crate::Permission>,
#[teaql(relation(target = "Role", local_key = "id", foreign_key = "merchant_id", many))]
    role_list: SmartList<crate::Role>,
#[teaql(relation(target = "RolePermission", local_key = "id", foreign_key = "merchant_id", many))]
    role_permission_list: SmartList<crate::RolePermission>,
#[teaql(relation(target = "UserAccount", local_key = "id", foreign_key = "merchant_id", many))]
    user_account_list: SmartList<crate::UserAccount>,
#[teaql(relation(target = "UserRoleAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    user_role_assignment_list: SmartList<crate::UserRoleAssignment>,
#[teaql(relation(target = "UserSession", local_key = "id", foreign_key = "merchant_id", many))]
    user_session_list: SmartList<crate::UserSession>,
#[teaql(relation(target = "ActivityLog", local_key = "id", foreign_key = "merchant_id", many))]
    activity_log_list: SmartList<crate::ActivityLog>,
#[teaql(relation(target = "AuditLog", local_key = "id", foreign_key = "merchant_id", many))]
    audit_log_list: SmartList<crate::AuditLog>,
#[teaql(relation(target = "ChangeSet", local_key = "id", foreign_key = "merchant_id", many))]
    change_set_list: SmartList<crate::ChangeSet>,
#[teaql(relation(target = "EntityChange", local_key = "id", foreign_key = "merchant_id", many))]
    entity_change_list: SmartList<crate::EntityChange>,
#[teaql(relation(target = "AutomationAction", local_key = "id", foreign_key = "merchant_id", many))]
    automation_action_list: SmartList<crate::AutomationAction>,
#[teaql(relation(target = "AutomationRule", local_key = "id", foreign_key = "merchant_id", many))]
    automation_rule_list: SmartList<crate::AutomationRule>,
#[teaql(relation(target = "AutomationTrigger", local_key = "id", foreign_key = "merchant_id", many))]
    automation_trigger_list: SmartList<crate::AutomationTrigger>,
#[teaql(relation(target = "Notification", local_key = "id", foreign_key = "merchant_id", many))]
    notification_list: SmartList<crate::Notification>,
#[teaql(relation(target = "NotificationTemplate", local_key = "id", foreign_key = "merchant_id", many))]
    notification_template_list: SmartList<crate::NotificationTemplate>,
#[teaql(relation(target = "ApiClient", local_key = "id", foreign_key = "merchant_id", many))]
    api_client_list: SmartList<crate::ApiClient>,
#[teaql(relation(target = "ApiEndpoint", local_key = "id", foreign_key = "merchant_id", many))]
    api_endpoint_list: SmartList<crate::ApiEndpoint>,
#[teaql(relation(target = "IntegrationMapping", local_key = "id", foreign_key = "merchant_id", many))]
    integration_mapping_list: SmartList<crate::IntegrationMapping>,
#[teaql(relation(target = "Webhook", local_key = "id", foreign_key = "merchant_id", many))]
    webhook_list: SmartList<crate::Webhook>,
#[teaql(relation(target = "WebhookDelivery", local_key = "id", foreign_key = "merchant_id", many))]
    webhook_delivery_list: SmartList<crate::WebhookDelivery>,
#[teaql(relation(target = "PlatformConfiguration", local_key = "id", foreign_key = "merchant_id", many))]
    platform_configuration_list: SmartList<crate::PlatformConfiguration>,
#[teaql(relation(target = "PlatformLocale", local_key = "id", foreign_key = "merchant_id", many))]
    platform_locale_list: SmartList<crate::PlatformLocale>,
#[teaql(relation(target = "MerchantBranch", local_key = "id", foreign_key = "merchant_id", many))]
    merchant_branch_list: SmartList<crate::MerchantBranch>,
#[teaql(relation(target = "MerchantSetting", local_key = "id", foreign_key = "merchant_id", many))]
    merchant_setting_list: SmartList<crate::MerchantSetting>,
#[teaql(relation(target = "OperationalException", local_key = "id", foreign_key = "merchant_id", many))]
    operational_exception_list: SmartList<crate::OperationalException>,
#[teaql(relation(target = "CrewMemberAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    crew_member_assignment_list: SmartList<crate::CrewMemberAssignment>,
#[teaql(relation(target = "PickupInstruction", local_key = "id", foreign_key = "merchant_id", many))]
    pickup_instruction_list: SmartList<crate::PickupInstruction>,
#[teaql(relation(target = "DeliveryInstruction", local_key = "id", foreign_key = "merchant_id", many))]
    delivery_instruction_list: SmartList<crate::DeliveryInstruction>,
#[teaql(relation(target = "MoveInventory", local_key = "id", foreign_key = "merchant_id", many))]
    move_inventory_list: SmartList<crate::MoveInventory>,
#[teaql(relation(target = "ExtraOperationsLogistics1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics1_list: SmartList<crate::ExtraOperationsLogistics1>,
#[teaql(relation(target = "ExtraOperationsLogistics2", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics2_list: SmartList<crate::ExtraOperationsLogistics2>,
#[teaql(relation(target = "ExtraOperationsLogistics3", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics3_list: SmartList<crate::ExtraOperationsLogistics3>,
#[teaql(relation(target = "ExtraOperationsLogistics4", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics4_list: SmartList<crate::ExtraOperationsLogistics4>,
#[teaql(relation(target = "ExtraOperationsLogistics5", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics5_list: SmartList<crate::ExtraOperationsLogistics5>,
#[teaql(relation(target = "ExtraOperationsLogistics6", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics6_list: SmartList<crate::ExtraOperationsLogistics6>,
#[teaql(relation(target = "ExtraOperationsLogistics7", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics7_list: SmartList<crate::ExtraOperationsLogistics7>,
#[teaql(relation(target = "ExtraOperationsLogistics8", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics8_list: SmartList<crate::ExtraOperationsLogistics8>,
#[teaql(relation(target = "ExtraOperationsLogistics9", local_key = "id", foreign_key = "merchant_id", many))]
    extra_operations_logistics9_list: SmartList<crate::ExtraOperationsLogistics9>,
#[teaql(relation(target = "EmployeeAvailability", local_key = "id", foreign_key = "merchant_id", many))]
    employee_availability_list: SmartList<crate::EmployeeAvailability>,
#[teaql(relation(target = "PayrollDeduction", local_key = "id", foreign_key = "merchant_id", many))]
    payroll_deduction_list: SmartList<crate::PayrollDeduction>,
#[teaql(relation(target = "TrainingSession", local_key = "id", foreign_key = "merchant_id", many))]
    training_session_list: SmartList<crate::TrainingSession>,
#[teaql(relation(target = "ShiftAssignment", local_key = "id", foreign_key = "merchant_id", many))]
    shift_assignment_list: SmartList<crate::ShiftAssignment>,
#[teaql(relation(target = "ExtraEmployeesPayroll1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_employees_payroll1_list: SmartList<crate::ExtraEmployeesPayroll1>,
#[teaql(relation(target = "ExtraEmployeesPayroll2", local_key = "id", foreign_key = "merchant_id", many))]
    extra_employees_payroll2_list: SmartList<crate::ExtraEmployeesPayroll2>,
#[teaql(relation(target = "ExtraEmployeesPayroll3", local_key = "id", foreign_key = "merchant_id", many))]
    extra_employees_payroll3_list: SmartList<crate::ExtraEmployeesPayroll3>,
#[teaql(relation(target = "ExtraEmployeesPayroll4", local_key = "id", foreign_key = "merchant_id", many))]
    extra_employees_payroll4_list: SmartList<crate::ExtraEmployeesPayroll4>,
#[teaql(relation(target = "ExtraEmployeesPayroll5", local_key = "id", foreign_key = "merchant_id", many))]
    extra_employees_payroll5_list: SmartList<crate::ExtraEmployeesPayroll5>,
#[teaql(relation(target = "ExtraEmployeesPayroll6", local_key = "id", foreign_key = "merchant_id", many))]
    extra_employees_payroll6_list: SmartList<crate::ExtraEmployeesPayroll6>,
#[teaql(relation(target = "ExtraEmployeesPayroll7", local_key = "id", foreign_key = "merchant_id", many))]
    extra_employees_payroll7_list: SmartList<crate::ExtraEmployeesPayroll7>,
#[teaql(relation(target = "CustomerComplaint", local_key = "id", foreign_key = "merchant_id", many))]
    customer_complaint_list: SmartList<crate::CustomerComplaint>,
#[teaql(relation(target = "CustomerNote", local_key = "id", foreign_key = "merchant_id", many))]
    customer_note_list: SmartList<crate::CustomerNote>,
#[teaql(relation(target = "ExtraCustomerManagement1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_customer_management1_list: SmartList<crate::ExtraCustomerManagement1>,
#[teaql(relation(target = "ExtraCustomerManagement2", local_key = "id", foreign_key = "merchant_id", many))]
    extra_customer_management2_list: SmartList<crate::ExtraCustomerManagement2>,
#[teaql(relation(target = "ExtraCustomerManagement3", local_key = "id", foreign_key = "merchant_id", many))]
    extra_customer_management3_list: SmartList<crate::ExtraCustomerManagement3>,
#[teaql(relation(target = "ExtraCustomerManagement4", local_key = "id", foreign_key = "merchant_id", many))]
    extra_customer_management4_list: SmartList<crate::ExtraCustomerManagement4>,
#[teaql(relation(target = "ExtraCustomerManagement5", local_key = "id", foreign_key = "merchant_id", many))]
    extra_customer_management5_list: SmartList<crate::ExtraCustomerManagement5>,
#[teaql(relation(target = "ExtraCustomerManagement6", local_key = "id", foreign_key = "merchant_id", many))]
    extra_customer_management6_list: SmartList<crate::ExtraCustomerManagement6>,
#[teaql(relation(target = "StorageService", local_key = "id", foreign_key = "merchant_id", many))]
    storage_service_list: SmartList<crate::StorageService>,
#[teaql(relation(target = "PackingService", local_key = "id", foreign_key = "merchant_id", many))]
    packing_service_list: SmartList<crate::PackingService>,
#[teaql(relation(target = "DisposalService", local_key = "id", foreign_key = "merchant_id", many))]
    disposal_service_list: SmartList<crate::DisposalService>,
#[teaql(relation(target = "RentalPeriod", local_key = "id", foreign_key = "merchant_id", many))]
    rental_period_list: SmartList<crate::RentalPeriod>,
#[teaql(relation(target = "ServiceArea", local_key = "id", foreign_key = "merchant_id", many))]
    service_area_list: SmartList<crate::ServiceArea>,
#[teaql(relation(target = "ExtraProductsServices1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_products_services1_list: SmartList<crate::ExtraProductsServices1>,
#[teaql(relation(target = "ExtraProductsServices2", local_key = "id", foreign_key = "merchant_id", many))]
    extra_products_services2_list: SmartList<crate::ExtraProductsServices2>,
#[teaql(relation(target = "ExtraProductsServices3", local_key = "id", foreign_key = "merchant_id", many))]
    extra_products_services3_list: SmartList<crate::ExtraProductsServices3>,
#[teaql(relation(target = "ExtraProductsServices4", local_key = "id", foreign_key = "merchant_id", many))]
    extra_products_services4_list: SmartList<crate::ExtraProductsServices4>,
#[teaql(relation(target = "CampaignAudience", local_key = "id", foreign_key = "merchant_id", many))]
    campaign_audience_list: SmartList<crate::CampaignAudience>,
#[teaql(relation(target = "CampaignChannel", local_key = "id", foreign_key = "merchant_id", many))]
    campaign_channel_list: SmartList<crate::CampaignChannel>,
#[teaql(relation(target = "LeadAttribution", local_key = "id", foreign_key = "merchant_id", many))]
    lead_attribution_list: SmartList<crate::LeadAttribution>,
#[teaql(relation(target = "SalesFunnel", local_key = "id", foreign_key = "merchant_id", many))]
    sales_funnel_list: SmartList<crate::SalesFunnel>,
#[teaql(relation(target = "ExtraMarketingSales1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_marketing_sales1_list: SmartList<crate::ExtraMarketingSales1>,
#[teaql(relation(target = "ExtraMarketingSales2", local_key = "id", foreign_key = "merchant_id", many))]
    extra_marketing_sales2_list: SmartList<crate::ExtraMarketingSales2>,
#[teaql(relation(target = "ExtraMarketingSales3", local_key = "id", foreign_key = "merchant_id", many))]
    extra_marketing_sales3_list: SmartList<crate::ExtraMarketingSales3>,
#[teaql(relation(target = "ExtraMarketingSales4", local_key = "id", foreign_key = "merchant_id", many))]
    extra_marketing_sales4_list: SmartList<crate::ExtraMarketingSales4>,
#[teaql(relation(target = "ExpenseClaim", local_key = "id", foreign_key = "merchant_id", many))]
    expense_claim_list: SmartList<crate::ExpenseClaim>,
#[teaql(relation(target = "Settlement", local_key = "id", foreign_key = "merchant_id", many))]
    settlement_list: SmartList<crate::Settlement>,
#[teaql(relation(target = "Receivable", local_key = "id", foreign_key = "merchant_id", many))]
    receivable_list: SmartList<crate::Receivable>,
#[teaql(relation(target = "Payable", local_key = "id", foreign_key = "merchant_id", many))]
    payable_list: SmartList<crate::Payable>,
#[teaql(relation(target = "ExtraFinanceAccounting1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_finance_accounting1_list: SmartList<crate::ExtraFinanceAccounting1>,
#[teaql(relation(target = "ExtraFinanceAccounting2", local_key = "id", foreign_key = "merchant_id", many))]
    extra_finance_accounting2_list: SmartList<crate::ExtraFinanceAccounting2>,
#[teaql(relation(target = "ExtraFinanceAccounting3", local_key = "id", foreign_key = "merchant_id", many))]
    extra_finance_accounting3_list: SmartList<crate::ExtraFinanceAccounting3>,
#[teaql(relation(target = "ExtraFinanceAccounting4", local_key = "id", foreign_key = "merchant_id", many))]
    extra_finance_accounting4_list: SmartList<crate::ExtraFinanceAccounting4>,
#[teaql(relation(target = "VehicleInspection", local_key = "id", foreign_key = "merchant_id", many))]
    vehicle_inspection_list: SmartList<crate::VehicleInspection>,
#[teaql(relation(target = "EquipmentCheckout", local_key = "id", foreign_key = "merchant_id", many))]
    equipment_checkout_list: SmartList<crate::EquipmentCheckout>,
#[teaql(relation(target = "ConsumableReorder", local_key = "id", foreign_key = "merchant_id", many))]
    consumable_reorder_list: SmartList<crate::ConsumableReorder>,
#[teaql(relation(target = "ExtraAssetManagement1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_asset_management1_list: SmartList<crate::ExtraAssetManagement1>,
#[teaql(relation(target = "ExtraAssetManagement2", local_key = "id", foreign_key = "merchant_id", many))]
    extra_asset_management2_list: SmartList<crate::ExtraAssetManagement2>,
#[teaql(relation(target = "ExtraAssetManagement3", local_key = "id", foreign_key = "merchant_id", many))]
    extra_asset_management3_list: SmartList<crate::ExtraAssetManagement3>,
#[teaql(relation(target = "ExtraAssetManagement4", local_key = "id", foreign_key = "merchant_id", many))]
    extra_asset_management4_list: SmartList<crate::ExtraAssetManagement4>,
#[teaql(relation(target = "ExtraAssetManagement5", local_key = "id", foreign_key = "merchant_id", many))]
    extra_asset_management5_list: SmartList<crate::ExtraAssetManagement5>,
#[teaql(relation(target = "AuthenticationAttempt", local_key = "id", foreign_key = "merchant_id", many))]
    authentication_attempt_list: SmartList<crate::AuthenticationAttempt>,
#[teaql(relation(target = "AccessPolicy", local_key = "id", foreign_key = "merchant_id", many))]
    access_policy_list: SmartList<crate::AccessPolicy>,
#[teaql(relation(target = "ExtraIdentityAccess1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_identity_access1_list: SmartList<crate::ExtraIdentityAccess1>,
#[teaql(relation(target = "AuditExport", local_key = "id", foreign_key = "merchant_id", many))]
    audit_export_list: SmartList<crate::AuditExport>,
#[teaql(relation(target = "ExtraActivityAudit1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_activity_audit1_list: SmartList<crate::ExtraActivityAudit1>,
#[teaql(relation(target = "NotificationPreference", local_key = "id", foreign_key = "merchant_id", many))]
    notification_preference_list: SmartList<crate::NotificationPreference>,
#[teaql(relation(target = "NotificationDelivery", local_key = "id", foreign_key = "merchant_id", many))]
    notification_delivery_list: SmartList<crate::NotificationDelivery>,
#[teaql(relation(target = "SynchronizationRun", local_key = "id", foreign_key = "merchant_id", many))]
    synchronization_run_list: SmartList<crate::SynchronizationRun>,
#[teaql(relation(target = "ExtraApiIntegrations1", local_key = "id", foreign_key = "merchant_id", many))]
    extra_api_integrations1_list: SmartList<crate::ExtraApiIntegrations1>,
}

impl MerchantReverseRelations {
    pub fn new() -> Self {
        Self {
            employee_list: Default::default(),
            platform_setting_list: Default::default(),
            platform_user_list: Default::default(),
            platform_audit_log_list: Default::default(),
            organization_list: Default::default(),
            organization_setting_list: Default::default(),
            organization_member_list: Default::default(),
            move_order_list: Default::default(),
            move_quote_list: Default::default(),
            route_list: Default::default(),
            route_stop_list: Default::default(),
            time_slot_list: Default::default(),
            fulfillment_event_list: Default::default(),
            address_list: Default::default(),
            crew_list: Default::default(),
            dispatch_assignment_list: Default::default(),
            damage_report_list: Default::default(),
            proof_of_delivery_list: Default::default(),
            inventory_item_list: Default::default(),
            packing_list_list: Default::default(),
            packing_item_list: Default::default(),
            loading_plan_list: Default::default(),
            unloading_plan_list: Default::default(),
            storage_facility_list: Default::default(),
            storage_unit_list: Default::default(),
            storage_inventory_list: Default::default(),
            transport_manifest_list: Default::default(),
            customs_declaration_list: Default::default(),
            equipment_checklist_list: Default::default(),
            fuel_log_list: Default::default(),
            maintenance_request_list: Default::default(),
            department_list: Default::default(),
            job_assignment_list: Default::default(),
            work_shift_list: Default::default(),
            worked_hours_list: Default::default(),
            payroll_period_list: Default::default(),
            payroll_calculation_list: Default::default(),
            payslip_list: Default::default(),
            bonus_list: Default::default(),
            employee_certification_list: Default::default(),
            leave_request_list: Default::default(),
            billing_profile_list: Default::default(),
            corporate_customer_profile_list: Default::default(),
            customer_list: Default::default(),
            customer_consent_list: Default::default(),
            customer_contact_list: Default::default(),
            customer_history_list: Default::default(),
            customer_preference_list: Default::default(),
            private_customer_profile_list: Default::default(),
            box_rental_list: Default::default(),
            cleaning_service_list: Default::default(),
            moving_service_list: Default::default(),
            price_list_list: Default::default(),
            product_list: Default::default(),
            service_list: Default::default(),
            service_bundle_list: Default::default(),
            service_configuration_list: Default::default(),
            service_price_list: Default::default(),
            campaign_list: Default::default(),
            conversion_event_list: Default::default(),
            conversion_metric_list: Default::default(),
            discount_code_list: Default::default(),
            lead_list: Default::default(),
            lead_activity_list: Default::default(),
            sales_opportunity_list: Default::default(),
            account_list: Default::default(),
            expense_list: Default::default(),
            financial_summary_list: Default::default(),
            invoice_list: Default::default(),
            invoice_line_list: Default::default(),
            journal_entry_list: Default::default(),
            payment_list: Default::default(),
            refund_list: Default::default(),
            vat_rate_list: Default::default(),
            asset_assignment_list: Default::default(),
            asset_inspection_list: Default::default(),
            consumable_list: Default::default(),
            equipment_list: Default::default(),
            fuel_record_list: Default::default(),
            maintenance_event_list: Default::default(),
            maintenance_schedule_list: Default::default(),
            supplier_list: Default::default(),
            vehicle_list: Default::default(),
            compliance_check_list: Default::default(),
            contract_list: Default::default(),
            data_retention_policy_list: Default::default(),
            document_list: Default::default(),
            document_version_list: Default::default(),
            insurance_claim_list: Default::default(),
            insurance_policy_list: Default::default(),
            recovery_request_list: Default::default(),
            magic_link_list: Default::default(),
            permission_list: Default::default(),
            role_list: Default::default(),
            role_permission_list: Default::default(),
            user_account_list: Default::default(),
            user_role_assignment_list: Default::default(),
            user_session_list: Default::default(),
            activity_log_list: Default::default(),
            audit_log_list: Default::default(),
            change_set_list: Default::default(),
            entity_change_list: Default::default(),
            automation_action_list: Default::default(),
            automation_rule_list: Default::default(),
            automation_trigger_list: Default::default(),
            notification_list: Default::default(),
            notification_template_list: Default::default(),
            api_client_list: Default::default(),
            api_endpoint_list: Default::default(),
            integration_mapping_list: Default::default(),
            webhook_list: Default::default(),
            webhook_delivery_list: Default::default(),
            platform_configuration_list: Default::default(),
            platform_locale_list: Default::default(),
            merchant_branch_list: Default::default(),
            merchant_setting_list: Default::default(),
            operational_exception_list: Default::default(),
            crew_member_assignment_list: Default::default(),
            pickup_instruction_list: Default::default(),
            delivery_instruction_list: Default::default(),
            move_inventory_list: Default::default(),
            extra_operations_logistics1_list: Default::default(),
            extra_operations_logistics2_list: Default::default(),
            extra_operations_logistics3_list: Default::default(),
            extra_operations_logistics4_list: Default::default(),
            extra_operations_logistics5_list: Default::default(),
            extra_operations_logistics6_list: Default::default(),
            extra_operations_logistics7_list: Default::default(),
            extra_operations_logistics8_list: Default::default(),
            extra_operations_logistics9_list: Default::default(),
            employee_availability_list: Default::default(),
            payroll_deduction_list: Default::default(),
            training_session_list: Default::default(),
            shift_assignment_list: Default::default(),
            extra_employees_payroll1_list: Default::default(),
            extra_employees_payroll2_list: Default::default(),
            extra_employees_payroll3_list: Default::default(),
            extra_employees_payroll4_list: Default::default(),
            extra_employees_payroll5_list: Default::default(),
            extra_employees_payroll6_list: Default::default(),
            extra_employees_payroll7_list: Default::default(),
            customer_complaint_list: Default::default(),
            customer_note_list: Default::default(),
            extra_customer_management1_list: Default::default(),
            extra_customer_management2_list: Default::default(),
            extra_customer_management3_list: Default::default(),
            extra_customer_management4_list: Default::default(),
            extra_customer_management5_list: Default::default(),
            extra_customer_management6_list: Default::default(),
            storage_service_list: Default::default(),
            packing_service_list: Default::default(),
            disposal_service_list: Default::default(),
            rental_period_list: Default::default(),
            service_area_list: Default::default(),
            extra_products_services1_list: Default::default(),
            extra_products_services2_list: Default::default(),
            extra_products_services3_list: Default::default(),
            extra_products_services4_list: Default::default(),
            campaign_audience_list: Default::default(),
            campaign_channel_list: Default::default(),
            lead_attribution_list: Default::default(),
            sales_funnel_list: Default::default(),
            extra_marketing_sales1_list: Default::default(),
            extra_marketing_sales2_list: Default::default(),
            extra_marketing_sales3_list: Default::default(),
            extra_marketing_sales4_list: Default::default(),
            expense_claim_list: Default::default(),
            settlement_list: Default::default(),
            receivable_list: Default::default(),
            payable_list: Default::default(),
            extra_finance_accounting1_list: Default::default(),
            extra_finance_accounting2_list: Default::default(),
            extra_finance_accounting3_list: Default::default(),
            extra_finance_accounting4_list: Default::default(),
            vehicle_inspection_list: Default::default(),
            equipment_checkout_list: Default::default(),
            consumable_reorder_list: Default::default(),
            extra_asset_management1_list: Default::default(),
            extra_asset_management2_list: Default::default(),
            extra_asset_management3_list: Default::default(),
            extra_asset_management4_list: Default::default(),
            extra_asset_management5_list: Default::default(),
            authentication_attempt_list: Default::default(),
            access_policy_list: Default::default(),
            extra_identity_access1_list: Default::default(),
            audit_export_list: Default::default(),
            extra_activity_audit1_list: Default::default(),
            notification_preference_list: Default::default(),
            notification_delivery_list: Default::default(),
            synchronization_run_list: Default::default(),
            extra_api_integrations1_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.employee_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.platform_setting_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.platform_user_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.platform_audit_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.organization_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.organization_setting_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.organization_member_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_order_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_quote_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.route_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.route_stop_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.time_slot_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.fulfillment_event_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.address_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.crew_list {
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
        for entity in &mut self.inventory_item_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.packing_list_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.packing_item_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.loading_plan_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.unloading_plan_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.storage_facility_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.storage_unit_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.storage_inventory_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.transport_manifest_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customs_declaration_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.equipment_checklist_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.fuel_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.maintenance_request_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.department_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.job_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.work_shift_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.worked_hours_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payroll_period_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payroll_calculation_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payslip_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.bonus_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.employee_certification_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.leave_request_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.billing_profile_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.corporate_customer_profile_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_consent_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_contact_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_history_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_preference_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.private_customer_profile_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.box_rental_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.cleaning_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.moving_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.price_list_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.product_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_bundle_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_configuration_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_price_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.campaign_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.conversion_event_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.conversion_metric_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.discount_code_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.lead_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.lead_activity_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.sales_opportunity_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.account_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.expense_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.financial_summary_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.invoice_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.invoice_line_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.journal_entry_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.refund_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vat_rate_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.asset_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.asset_inspection_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.consumable_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.equipment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.fuel_record_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.maintenance_event_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.maintenance_schedule_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.supplier_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.compliance_check_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.contract_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.data_retention_policy_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.document_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.document_version_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.insurance_claim_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.insurance_policy_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.recovery_request_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.magic_link_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.permission_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.role_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.role_permission_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.user_account_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.user_role_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.user_session_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.activity_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.audit_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.change_set_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.entity_change_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.automation_action_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.automation_rule_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.automation_trigger_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.notification_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.notification_template_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.api_client_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.api_endpoint_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.integration_mapping_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.webhook_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.webhook_delivery_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.platform_configuration_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.platform_locale_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.merchant_branch_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.merchant_setting_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.operational_exception_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.crew_member_assignment_list {
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
        for entity in &mut self.extra_operations_logistics1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics2_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics3_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics4_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics5_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics6_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics7_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics8_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_operations_logistics9_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.employee_availability_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payroll_deduction_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.training_session_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.shift_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_employees_payroll1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_employees_payroll2_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_employees_payroll3_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_employees_payroll4_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_employees_payroll5_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_employees_payroll6_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_employees_payroll7_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_complaint_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_note_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_customer_management1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_customer_management2_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_customer_management3_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_customer_management4_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_customer_management5_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_customer_management6_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.storage_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.packing_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.disposal_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.rental_period_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_area_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_products_services1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_products_services2_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_products_services3_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_products_services4_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.campaign_audience_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.campaign_channel_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.lead_attribution_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.sales_funnel_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_marketing_sales1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_marketing_sales2_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_marketing_sales3_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_marketing_sales4_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.expense_claim_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.settlement_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.receivable_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payable_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_finance_accounting1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_finance_accounting2_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_finance_accounting3_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_finance_accounting4_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_inspection_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.equipment_checkout_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.consumable_reorder_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_asset_management1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_asset_management2_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_asset_management3_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_asset_management4_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_asset_management5_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.authentication_attempt_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.access_policy_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_identity_access1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.audit_export_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_activity_audit1_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.notification_preference_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.notification_delivery_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.synchronization_run_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.extra_api_integrations1_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
