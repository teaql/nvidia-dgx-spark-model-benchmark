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

// @source model.xml:2
    name: String,

// @source model.xml:2
    tax_id: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "platform_ref")]
    platform_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Platform", local_key = "platform_ref_id", foreign_key = "id"))]
    platform_ref: Option<crate::Platform>,
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
            tax_id: String::new(),
            version: 0_i64,
            platform_ref_id: 0_u64,
            platform_ref: None,
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
        if let Some(entity) = &mut self.platform_ref {
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

    pub fn tax_id(&self) -> String {
        self.changed_tax_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.tax_id.clone())
    }

    pub fn update_tax_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.tax_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.tax_id.clone());
        self.root.set(self.entity_key(), "tax_id", value);
        self
    }

    pub fn changed_tax_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "tax_id")
    }

    pub fn eval_tax_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("tax_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "tax_id".to_string(), attempted_path: "tax_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.tax_id())
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
    pub fn platform_ref_id(&self) -> u64 {
        self.changed_platform_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.platform_ref_id)
    }

    pub fn update_platform_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.platform_ref_id = value.try_u64().unwrap_or(self.platform_ref_id.clone());
        self.root.set(self.entity_key(), "platform_ref_id", value);
        self
    }

    pub fn changed_platform_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "platform_ref_id")
    }

    pub fn eval_platform_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("platform_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_ref_id".to_string(), attempted_path: "platform_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.platform_ref_id())
                }}
    pub fn platform_ref(&self) -> Option<&crate::Platform> {
        self.platform_ref.as_ref()
    }

    pub fn eval_platform_ref(&self) -> teaql_core::eval::EvalResult<&crate::Platform> {
        if !self.is_loaded("platform_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "platform_ref".to_string(), attempted_path: "platform_ref".to_string() }
        } else {
            match &self.platform_ref {
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

    pub fn tenant_configuration_list(&self) -> &SmartList<crate::TenantConfiguration> {
        &self._relations.tenant_configuration_list
    }

    pub fn tenant_configuration_list_mut(&mut self) -> &mut SmartList<crate::TenantConfiguration> {
        &mut self._relations.tenant_configuration_list
    }

    pub fn eval_tenant_configuration_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TenantConfiguration>> {
        if !self.is_loaded("tenant_configuration_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "tenant_configuration_list".to_string(), attempted_path: "tenant_configuration_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.tenant_configuration_list)
        }
    }

    pub fn organization_unit_list(&self) -> &SmartList<crate::OrganizationUnit> {
        &self._relations.organization_unit_list
    }

    pub fn organization_unit_list_mut(&mut self) -> &mut SmartList<crate::OrganizationUnit> {
        &mut self._relations.organization_unit_list
    }

    pub fn eval_organization_unit_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::OrganizationUnit>> {
        if !self.is_loaded("organization_unit_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "organization_unit_list".to_string(), attempted_path: "organization_unit_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.organization_unit_list)
        }
    }

    pub fn department_hierarchy_list(&self) -> &SmartList<crate::DepartmentHierarchy> {
        &self._relations.department_hierarchy_list
    }

    pub fn department_hierarchy_list_mut(&mut self) -> &mut SmartList<crate::DepartmentHierarchy> {
        &mut self._relations.department_hierarchy_list
    }

    pub fn eval_department_hierarchy_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DepartmentHierarchy>> {
        if !self.is_loaded("department_hierarchy_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "department_hierarchy_list".to_string(), attempted_path: "department_hierarchy_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.department_hierarchy_list)
        }
    }

    pub fn branch_office_list(&self) -> &SmartList<crate::BranchOffice> {
        &self._relations.branch_office_list
    }

    pub fn branch_office_list_mut(&mut self) -> &mut SmartList<crate::BranchOffice> {
        &mut self._relations.branch_office_list
    }

    pub fn eval_branch_office_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::BranchOffice>> {
        if !self.is_loaded("branch_office_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "branch_office_list".to_string(), attempted_path: "branch_office_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.branch_office_list)
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

    pub fn packing_material_list(&self) -> &SmartList<crate::PackingMaterial> {
        &self._relations.packing_material_list
    }

    pub fn packing_material_list_mut(&mut self) -> &mut SmartList<crate::PackingMaterial> {
        &mut self._relations.packing_material_list
    }

    pub fn eval_packing_material_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PackingMaterial>> {
        if !self.is_loaded("packing_material_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "packing_material_list".to_string(), attempted_path: "packing_material_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.packing_material_list)
        }
    }

    pub fn loading_zone_list(&self) -> &SmartList<crate::LoadingZone> {
        &self._relations.loading_zone_list
    }

    pub fn loading_zone_list_mut(&mut self) -> &mut SmartList<crate::LoadingZone> {
        &mut self._relations.loading_zone_list
    }

    pub fn eval_loading_zone_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LoadingZone>> {
        if !self.is_loaded("loading_zone_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "loading_zone_list".to_string(), attempted_path: "loading_zone_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.loading_zone_list)
        }
    }

    pub fn unloading_zone_list(&self) -> &SmartList<crate::UnloadingZone> {
        &self._relations.unloading_zone_list
    }

    pub fn unloading_zone_list_mut(&mut self) -> &mut SmartList<crate::UnloadingZone> {
        &mut self._relations.unloading_zone_list
    }

    pub fn eval_unloading_zone_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UnloadingZone>> {
        if !self.is_loaded("unloading_zone_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "unloading_zone_list".to_string(), attempted_path: "unloading_zone_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.unloading_zone_list)
        }
    }

    pub fn route_optimization_rule_list(&self) -> &SmartList<crate::RouteOptimizationRule> {
        &self._relations.route_optimization_rule_list
    }

    pub fn route_optimization_rule_list_mut(&mut self) -> &mut SmartList<crate::RouteOptimizationRule> {
        &mut self._relations.route_optimization_rule_list
    }

    pub fn eval_route_optimization_rule_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RouteOptimizationRule>> {
        if !self.is_loaded("route_optimization_rule_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "route_optimization_rule_list".to_string(), attempted_path: "route_optimization_rule_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.route_optimization_rule_list)
        }
    }

    pub fn move_status_list(&self) -> &SmartList<crate::MoveStatus> {
        &self._relations.move_status_list
    }

    pub fn move_status_list_mut(&mut self) -> &mut SmartList<crate::MoveStatus> {
        &mut self._relations.move_status_list
    }

    pub fn eval_move_status_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MoveStatus>> {
        if !self.is_loaded("move_status_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_status_list".to_string(), attempted_path: "move_status_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.move_status_list)
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

    pub fn customer_segment_list(&self) -> &SmartList<crate::CustomerSegment> {
        &self._relations.customer_segment_list
    }

    pub fn customer_segment_list_mut(&mut self) -> &mut SmartList<crate::CustomerSegment> {
        &mut self._relations.customer_segment_list
    }

    pub fn eval_customer_segment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerSegment>> {
        if !self.is_loaded("customer_segment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_segment_list".to_string(), attempted_path: "customer_segment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_segment_list)
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

    pub fn service_category_list(&self) -> &SmartList<crate::ServiceCategory> {
        &self._relations.service_category_list
    }

    pub fn service_category_list_mut(&mut self) -> &mut SmartList<crate::ServiceCategory> {
        &mut self._relations.service_category_list
    }

    pub fn eval_service_category_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServiceCategory>> {
        if !self.is_loaded("service_category_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_category_list".to_string(), attempted_path: "service_category_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.service_category_list)
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

    pub fn marketing_channel_list(&self) -> &SmartList<crate::MarketingChannel> {
        &self._relations.marketing_channel_list
    }

    pub fn marketing_channel_list_mut(&mut self) -> &mut SmartList<crate::MarketingChannel> {
        &mut self._relations.marketing_channel_list
    }

    pub fn eval_marketing_channel_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MarketingChannel>> {
        if !self.is_loaded("marketing_channel_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "marketing_channel_list".to_string(), attempted_path: "marketing_channel_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.marketing_channel_list)
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

    pub fn budget_list(&self) -> &SmartList<crate::Budget> {
        &self._relations.budget_list
    }

    pub fn budget_list_mut(&mut self) -> &mut SmartList<crate::Budget> {
        &mut self._relations.budget_list
    }

    pub fn eval_budget_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Budget>> {
        if !self.is_loaded("budget_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "budget_list".to_string(), attempted_path: "budget_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.budget_list)
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

    pub fn currency_rate_list(&self) -> &SmartList<crate::CurrencyRate> {
        &self._relations.currency_rate_list
    }

    pub fn currency_rate_list_mut(&mut self) -> &mut SmartList<crate::CurrencyRate> {
        &mut self._relations.currency_rate_list
    }

    pub fn eval_currency_rate_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CurrencyRate>> {
        if !self.is_loaded("currency_rate_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "currency_rate_list".to_string(), attempted_path: "currency_rate_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.currency_rate_list)
        }
    }

    pub fn payment_method_list(&self) -> &SmartList<crate::PaymentMethod> {
        &self._relations.payment_method_list
    }

    pub fn payment_method_list_mut(&mut self) -> &mut SmartList<crate::PaymentMethod> {
        &mut self._relations.payment_method_list
    }

    pub fn eval_payment_method_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PaymentMethod>> {
        if !self.is_loaded("payment_method_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "payment_method_list".to_string(), attempted_path: "payment_method_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.payment_method_list)
        }
    }

    pub fn financial_period_list(&self) -> &SmartList<crate::FinancialPeriod> {
        &self._relations.financial_period_list
    }

    pub fn financial_period_list_mut(&mut self) -> &mut SmartList<crate::FinancialPeriod> {
        &mut self._relations.financial_period_list
    }

    pub fn eval_financial_period_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::FinancialPeriod>> {
        if !self.is_loaded("financial_period_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "financial_period_list".to_string(), attempted_path: "financial_period_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.financial_period_list)
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

    pub fn storage_location_list(&self) -> &SmartList<crate::StorageLocation> {
        &self._relations.storage_location_list
    }

    pub fn storage_location_list_mut(&mut self) -> &mut SmartList<crate::StorageLocation> {
        &mut self._relations.storage_location_list
    }

    pub fn eval_storage_location_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::StorageLocation>> {
        if !self.is_loaded("storage_location_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "storage_location_list".to_string(), attempted_path: "storage_location_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.storage_location_list)
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

    pub fn policy_document_list(&self) -> &SmartList<crate::PolicyDocument> {
        &self._relations.policy_document_list
    }

    pub fn policy_document_list_mut(&mut self) -> &mut SmartList<crate::PolicyDocument> {
        &mut self._relations.policy_document_list
    }

    pub fn eval_policy_document_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PolicyDocument>> {
        if !self.is_loaded("policy_document_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "policy_document_list".to_string(), attempted_path: "policy_document_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.policy_document_list)
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

    pub fn legal_entity_list(&self) -> &SmartList<crate::LegalEntity> {
        &self._relations.legal_entity_list
    }

    pub fn legal_entity_list_mut(&mut self) -> &mut SmartList<crate::LegalEntity> {
        &mut self._relations.legal_entity_list
    }

    pub fn eval_legal_entity_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LegalEntity>> {
        if !self.is_loaded("legal_entity_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "legal_entity_list".to_string(), attempted_path: "legal_entity_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.legal_entity_list)
        }
    }

    pub fn regulatory_requirement_list(&self) -> &SmartList<crate::RegulatoryRequirement> {
        &self._relations.regulatory_requirement_list
    }

    pub fn regulatory_requirement_list_mut(&mut self) -> &mut SmartList<crate::RegulatoryRequirement> {
        &mut self._relations.regulatory_requirement_list
    }

    pub fn eval_regulatory_requirement_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::RegulatoryRequirement>> {
        if !self.is_loaded("regulatory_requirement_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "regulatory_requirement_list".to_string(), attempted_path: "regulatory_requirement_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.regulatory_requirement_list)
        }
    }

    pub fn compliance_certificate_list(&self) -> &SmartList<crate::ComplianceCertificate> {
        &self._relations.compliance_certificate_list
    }

    pub fn compliance_certificate_list_mut(&mut self) -> &mut SmartList<crate::ComplianceCertificate> {
        &mut self._relations.compliance_certificate_list
    }

    pub fn eval_compliance_certificate_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ComplianceCertificate>> {
        if !self.is_loaded("compliance_certificate_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "compliance_certificate_list".to_string(), attempted_path: "compliance_certificate_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.compliance_certificate_list)
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

    pub fn system_event_list(&self) -> &SmartList<crate::SystemEvent> {
        &self._relations.system_event_list
    }

    pub fn system_event_list_mut(&mut self) -> &mut SmartList<crate::SystemEvent> {
        &mut self._relations.system_event_list
    }

    pub fn eval_system_event_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::SystemEvent>> {
        if !self.is_loaded("system_event_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "system_event_list".to_string(), attempted_path: "system_event_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.system_event_list)
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
#[teaql(relation(target = "Employee", local_key = "id", foreign_key = "merchant_ref_id", many))]
    employee_list: SmartList<crate::Employee>,
#[teaql(relation(target = "TenantConfiguration", local_key = "id", foreign_key = "merchant_ref_id", many))]
    tenant_configuration_list: SmartList<crate::TenantConfiguration>,
#[teaql(relation(target = "OrganizationUnit", local_key = "id", foreign_key = "merchant_ref_id", many))]
    organization_unit_list: SmartList<crate::OrganizationUnit>,
#[teaql(relation(target = "DepartmentHierarchy", local_key = "id", foreign_key = "merchant_ref_id", many))]
    department_hierarchy_list: SmartList<crate::DepartmentHierarchy>,
#[teaql(relation(target = "BranchOffice", local_key = "id", foreign_key = "merchant_ref_id", many))]
    branch_office_list: SmartList<crate::BranchOffice>,
#[teaql(relation(target = "MoveOrder", local_key = "id", foreign_key = "merchant_ref_id", many))]
    move_order_list: SmartList<crate::MoveOrder>,
#[teaql(relation(target = "Address", local_key = "id", foreign_key = "merchant_ref_id", many))]
    address_list: SmartList<crate::Address>,
#[teaql(relation(target = "Crew", local_key = "id", foreign_key = "merchant_ref_id", many))]
    crew_list: SmartList<crate::Crew>,
#[teaql(relation(target = "PackingMaterial", local_key = "id", foreign_key = "merchant_ref_id", many))]
    packing_material_list: SmartList<crate::PackingMaterial>,
#[teaql(relation(target = "LoadingZone", local_key = "id", foreign_key = "merchant_ref_id", many))]
    loading_zone_list: SmartList<crate::LoadingZone>,
#[teaql(relation(target = "UnloadingZone", local_key = "id", foreign_key = "merchant_ref_id", many))]
    unloading_zone_list: SmartList<crate::UnloadingZone>,
#[teaql(relation(target = "RouteOptimizationRule", local_key = "id", foreign_key = "merchant_ref_id", many))]
    route_optimization_rule_list: SmartList<crate::RouteOptimizationRule>,
#[teaql(relation(target = "MoveStatus", local_key = "id", foreign_key = "merchant_ref_id", many))]
    move_status_list: SmartList<crate::MoveStatus>,
#[teaql(relation(target = "Department", local_key = "id", foreign_key = "merchant_ref_id", many))]
    department_list: SmartList<crate::Department>,
#[teaql(relation(target = "PayrollPeriod", local_key = "id", foreign_key = "merchant_ref_id", many))]
    payroll_period_list: SmartList<crate::PayrollPeriod>,
#[teaql(relation(target = "Customer", local_key = "id", foreign_key = "merchant_ref_id", many))]
    customer_list: SmartList<crate::Customer>,
#[teaql(relation(target = "CustomerSegment", local_key = "id", foreign_key = "merchant_ref_id", many))]
    customer_segment_list: SmartList<crate::CustomerSegment>,
#[teaql(relation(target = "Product", local_key = "id", foreign_key = "merchant_ref_id", many))]
    product_list: SmartList<crate::Product>,
#[teaql(relation(target = "Service", local_key = "id", foreign_key = "merchant_ref_id", many))]
    service_list: SmartList<crate::Service>,
#[teaql(relation(target = "PriceList", local_key = "id", foreign_key = "merchant_ref_id", many))]
    price_list_list: SmartList<crate::PriceList>,
#[teaql(relation(target = "ServiceBundle", local_key = "id", foreign_key = "merchant_ref_id", many))]
    service_bundle_list: SmartList<crate::ServiceBundle>,
#[teaql(relation(target = "StorageUnit", local_key = "id", foreign_key = "merchant_ref_id", many))]
    storage_unit_list: SmartList<crate::StorageUnit>,
#[teaql(relation(target = "ServiceArea", local_key = "id", foreign_key = "merchant_ref_id", many))]
    service_area_list: SmartList<crate::ServiceArea>,
#[teaql(relation(target = "InventoryItem", local_key = "id", foreign_key = "merchant_ref_id", many))]
    inventory_item_list: SmartList<crate::InventoryItem>,
#[teaql(relation(target = "ServiceCategory", local_key = "id", foreign_key = "merchant_ref_id", many))]
    service_category_list: SmartList<crate::ServiceCategory>,
#[teaql(relation(target = "Campaign", local_key = "id", foreign_key = "merchant_ref_id", many))]
    campaign_list: SmartList<crate::Campaign>,
#[teaql(relation(target = "MarketingChannel", local_key = "id", foreign_key = "merchant_ref_id", many))]
    marketing_channel_list: SmartList<crate::MarketingChannel>,
#[teaql(relation(target = "Payment", local_key = "id", foreign_key = "merchant_ref_id", many))]
    payment_list: SmartList<crate::Payment>,
#[teaql(relation(target = "Expense", local_key = "id", foreign_key = "merchant_ref_id", many))]
    expense_list: SmartList<crate::Expense>,
#[teaql(relation(target = "VatRate", local_key = "id", foreign_key = "merchant_ref_id", many))]
    vat_rate_list: SmartList<crate::VatRate>,
#[teaql(relation(target = "Account", local_key = "id", foreign_key = "merchant_ref_id", many))]
    account_list: SmartList<crate::Account>,
#[teaql(relation(target = "FinancialSummary", local_key = "id", foreign_key = "merchant_ref_id", many))]
    financial_summary_list: SmartList<crate::FinancialSummary>,
#[teaql(relation(target = "Budget", local_key = "id", foreign_key = "merchant_ref_id", many))]
    budget_list: SmartList<crate::Budget>,
#[teaql(relation(target = "Payable", local_key = "id", foreign_key = "merchant_ref_id", many))]
    payable_list: SmartList<crate::Payable>,
#[teaql(relation(target = "CurrencyRate", local_key = "id", foreign_key = "merchant_ref_id", many))]
    currency_rate_list: SmartList<crate::CurrencyRate>,
#[teaql(relation(target = "PaymentMethod", local_key = "id", foreign_key = "merchant_ref_id", many))]
    payment_method_list: SmartList<crate::PaymentMethod>,
#[teaql(relation(target = "FinancialPeriod", local_key = "id", foreign_key = "merchant_ref_id", many))]
    financial_period_list: SmartList<crate::FinancialPeriod>,
#[teaql(relation(target = "Vehicle", local_key = "id", foreign_key = "merchant_ref_id", many))]
    vehicle_list: SmartList<crate::Vehicle>,
#[teaql(relation(target = "Equipment", local_key = "id", foreign_key = "merchant_ref_id", many))]
    equipment_list: SmartList<crate::Equipment>,
#[teaql(relation(target = "Consumable", local_key = "id", foreign_key = "merchant_ref_id", many))]
    consumable_list: SmartList<crate::Consumable>,
#[teaql(relation(target = "Supplier", local_key = "id", foreign_key = "merchant_ref_id", many))]
    supplier_list: SmartList<crate::Supplier>,
#[teaql(relation(target = "StorageLocation", local_key = "id", foreign_key = "merchant_ref_id", many))]
    storage_location_list: SmartList<crate::StorageLocation>,
#[teaql(relation(target = "Contract", local_key = "id", foreign_key = "merchant_ref_id", many))]
    contract_list: SmartList<crate::Contract>,
#[teaql(relation(target = "InsurancePolicy", local_key = "id", foreign_key = "merchant_ref_id", many))]
    insurance_policy_list: SmartList<crate::InsurancePolicy>,
#[teaql(relation(target = "ComplianceCheck", local_key = "id", foreign_key = "merchant_ref_id", many))]
    compliance_check_list: SmartList<crate::ComplianceCheck>,
#[teaql(relation(target = "DataRetentionPolicy", local_key = "id", foreign_key = "merchant_ref_id", many))]
    data_retention_policy_list: SmartList<crate::DataRetentionPolicy>,
#[teaql(relation(target = "PolicyDocument", local_key = "id", foreign_key = "merchant_ref_id", many))]
    policy_document_list: SmartList<crate::PolicyDocument>,
#[teaql(relation(target = "IncidentReport", local_key = "id", foreign_key = "merchant_ref_id", many))]
    incident_report_list: SmartList<crate::IncidentReport>,
#[teaql(relation(target = "LegalEntity", local_key = "id", foreign_key = "merchant_ref_id", many))]
    legal_entity_list: SmartList<crate::LegalEntity>,
#[teaql(relation(target = "RegulatoryRequirement", local_key = "id", foreign_key = "merchant_ref_id", many))]
    regulatory_requirement_list: SmartList<crate::RegulatoryRequirement>,
#[teaql(relation(target = "ComplianceCertificate", local_key = "id", foreign_key = "merchant_ref_id", many))]
    compliance_certificate_list: SmartList<crate::ComplianceCertificate>,
#[teaql(relation(target = "Role", local_key = "id", foreign_key = "merchant_ref_id", many))]
    role_list: SmartList<crate::Role>,
#[teaql(relation(target = "Permission", local_key = "id", foreign_key = "merchant_ref_id", many))]
    permission_list: SmartList<crate::Permission>,
#[teaql(relation(target = "AuditLog", local_key = "id", foreign_key = "merchant_ref_id", many))]
    audit_log_list: SmartList<crate::AuditLog>,
#[teaql(relation(target = "SystemEvent", local_key = "id", foreign_key = "merchant_ref_id", many))]
    system_event_list: SmartList<crate::SystemEvent>,
#[teaql(relation(target = "NotificationTemplate", local_key = "id", foreign_key = "merchant_ref_id", many))]
    notification_template_list: SmartList<crate::NotificationTemplate>,
#[teaql(relation(target = "AutomationRule", local_key = "id", foreign_key = "merchant_ref_id", many))]
    automation_rule_list: SmartList<crate::AutomationRule>,
#[teaql(relation(target = "ApiClient", local_key = "id", foreign_key = "merchant_ref_id", many))]
    api_client_list: SmartList<crate::ApiClient>,
}

impl MerchantReverseRelations {
    pub fn new() -> Self {
        Self {
            employee_list: Default::default(),
            tenant_configuration_list: Default::default(),
            organization_unit_list: Default::default(),
            department_hierarchy_list: Default::default(),
            branch_office_list: Default::default(),
            move_order_list: Default::default(),
            address_list: Default::default(),
            crew_list: Default::default(),
            packing_material_list: Default::default(),
            loading_zone_list: Default::default(),
            unloading_zone_list: Default::default(),
            route_optimization_rule_list: Default::default(),
            move_status_list: Default::default(),
            department_list: Default::default(),
            payroll_period_list: Default::default(),
            customer_list: Default::default(),
            customer_segment_list: Default::default(),
            product_list: Default::default(),
            service_list: Default::default(),
            price_list_list: Default::default(),
            service_bundle_list: Default::default(),
            storage_unit_list: Default::default(),
            service_area_list: Default::default(),
            inventory_item_list: Default::default(),
            service_category_list: Default::default(),
            campaign_list: Default::default(),
            marketing_channel_list: Default::default(),
            payment_list: Default::default(),
            expense_list: Default::default(),
            vat_rate_list: Default::default(),
            account_list: Default::default(),
            financial_summary_list: Default::default(),
            budget_list: Default::default(),
            payable_list: Default::default(),
            currency_rate_list: Default::default(),
            payment_method_list: Default::default(),
            financial_period_list: Default::default(),
            vehicle_list: Default::default(),
            equipment_list: Default::default(),
            consumable_list: Default::default(),
            supplier_list: Default::default(),
            storage_location_list: Default::default(),
            contract_list: Default::default(),
            insurance_policy_list: Default::default(),
            compliance_check_list: Default::default(),
            data_retention_policy_list: Default::default(),
            policy_document_list: Default::default(),
            incident_report_list: Default::default(),
            legal_entity_list: Default::default(),
            regulatory_requirement_list: Default::default(),
            compliance_certificate_list: Default::default(),
            role_list: Default::default(),
            permission_list: Default::default(),
            audit_log_list: Default::default(),
            system_event_list: Default::default(),
            notification_template_list: Default::default(),
            automation_rule_list: Default::default(),
            api_client_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.employee_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.tenant_configuration_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.organization_unit_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.department_hierarchy_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.branch_office_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_order_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.address_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.crew_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.packing_material_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.loading_zone_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.unloading_zone_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.route_optimization_rule_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.move_status_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.department_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payroll_period_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_segment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.product_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.price_list_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_bundle_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.storage_unit_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_area_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.inventory_item_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_category_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.campaign_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.marketing_channel_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.expense_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vat_rate_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.account_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.financial_summary_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.budget_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payable_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.currency_rate_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.payment_method_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.financial_period_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.equipment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.consumable_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.supplier_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.storage_location_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.contract_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.insurance_policy_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.compliance_check_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.data_retention_policy_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.policy_document_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.incident_report_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.legal_entity_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.regulatory_requirement_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.compliance_certificate_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.role_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.permission_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.audit_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.system_event_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.notification_template_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.automation_rule_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.api_client_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
