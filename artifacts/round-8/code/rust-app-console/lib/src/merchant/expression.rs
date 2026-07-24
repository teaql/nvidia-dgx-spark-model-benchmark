#[derive(Clone)]
pub struct MerchantExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Merchant>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MerchantExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Merchant>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Merchant> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Merchant> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Merchant {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("tax_id", |entity| entity.eval_tax_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_platform_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("platform_ref_id", |entity| entity.eval_platform_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_platform_ref(self) -> crate::PlatformExpression<'a> {
        let next = self.result.and_then("platform_ref", |entity| entity.eval_platform_ref());
        crate::PlatformExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_list(self) -> crate::EmployeeListExpression<'a> {
        let next = self.result.and_then("employee_list", |entity| entity.eval_employee_list());
        crate::EmployeeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tenant_configuration_list(self) -> crate::TenantConfigurationListExpression<'a> {
        let next = self.result.and_then("tenant_configuration_list", |entity| entity.eval_tenant_configuration_list());
        crate::TenantConfigurationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_organization_unit_list(self) -> crate::OrganizationUnitListExpression<'a> {
        let next = self.result.and_then("organization_unit_list", |entity| entity.eval_organization_unit_list());
        crate::OrganizationUnitListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_department_hierarchy_list(self) -> crate::DepartmentHierarchyListExpression<'a> {
        let next = self.result.and_then("department_hierarchy_list", |entity| entity.eval_department_hierarchy_list());
        crate::DepartmentHierarchyListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_branch_office_list(self) -> crate::BranchOfficeListExpression<'a> {
        let next = self.result.and_then("branch_office_list", |entity| entity.eval_branch_office_list());
        crate::BranchOfficeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_list(self) -> crate::MoveOrderListExpression<'a> {
        let next = self.result.and_then("move_order_list", |entity| entity.eval_move_order_list());
        crate::MoveOrderListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_address_list(self) -> crate::AddressListExpression<'a> {
        let next = self.result.and_then("address_list", |entity| entity.eval_address_list());
        crate::AddressListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_list(self) -> crate::CrewListExpression<'a> {
        let next = self.result.and_then("crew_list", |entity| entity.eval_crew_list());
        crate::CrewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_packing_material_list(self) -> crate::PackingMaterialListExpression<'a> {
        let next = self.result.and_then("packing_material_list", |entity| entity.eval_packing_material_list());
        crate::PackingMaterialListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_loading_zone_list(self) -> crate::LoadingZoneListExpression<'a> {
        let next = self.result.and_then("loading_zone_list", |entity| entity.eval_loading_zone_list());
        crate::LoadingZoneListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_unloading_zone_list(self) -> crate::UnloadingZoneListExpression<'a> {
        let next = self.result.and_then("unloading_zone_list", |entity| entity.eval_unloading_zone_list());
        crate::UnloadingZoneListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_route_optimization_rule_list(self) -> crate::RouteOptimizationRuleListExpression<'a> {
        let next = self.result.and_then("route_optimization_rule_list", |entity| entity.eval_route_optimization_rule_list());
        crate::RouteOptimizationRuleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_status_list(self) -> crate::MoveStatusListExpression<'a> {
        let next = self.result.and_then("move_status_list", |entity| entity.eval_move_status_list());
        crate::MoveStatusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_department_list(self) -> crate::DepartmentListExpression<'a> {
        let next = self.result.and_then("department_list", |entity| entity.eval_department_list());
        crate::DepartmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payroll_period_list(self) -> crate::PayrollPeriodListExpression<'a> {
        let next = self.result.and_then("payroll_period_list", |entity| entity.eval_payroll_period_list());
        crate::PayrollPeriodListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_list(self) -> crate::CustomerListExpression<'a> {
        let next = self.result.and_then("customer_list", |entity| entity.eval_customer_list());
        crate::CustomerListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_segment_list(self) -> crate::CustomerSegmentListExpression<'a> {
        let next = self.result.and_then("customer_segment_list", |entity| entity.eval_customer_segment_list());
        crate::CustomerSegmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_product_list(self) -> crate::ProductListExpression<'a> {
        let next = self.result.and_then("product_list", |entity| entity.eval_product_list());
        crate::ProductListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_list(self) -> crate::ServiceListExpression<'a> {
        let next = self.result.and_then("service_list", |entity| entity.eval_service_list());
        crate::ServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_price_list_list(self) -> crate::PriceListListExpression<'a> {
        let next = self.result.and_then("price_list_list", |entity| entity.eval_price_list_list());
        crate::PriceListListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_bundle_list(self) -> crate::ServiceBundleListExpression<'a> {
        let next = self.result.and_then("service_bundle_list", |entity| entity.eval_service_bundle_list());
        crate::ServiceBundleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_storage_unit_list(self) -> crate::StorageUnitListExpression<'a> {
        let next = self.result.and_then("storage_unit_list", |entity| entity.eval_storage_unit_list());
        crate::StorageUnitListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_area_list(self) -> crate::ServiceAreaListExpression<'a> {
        let next = self.result.and_then("service_area_list", |entity| entity.eval_service_area_list());
        crate::ServiceAreaListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_inventory_item_list(self) -> crate::InventoryItemListExpression<'a> {
        let next = self.result.and_then("inventory_item_list", |entity| entity.eval_inventory_item_list());
        crate::InventoryItemListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_category_list(self) -> crate::ServiceCategoryListExpression<'a> {
        let next = self.result.and_then("service_category_list", |entity| entity.eval_service_category_list());
        crate::ServiceCategoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_campaign_list(self) -> crate::CampaignListExpression<'a> {
        let next = self.result.and_then("campaign_list", |entity| entity.eval_campaign_list());
        crate::CampaignListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_marketing_channel_list(self) -> crate::MarketingChannelListExpression<'a> {
        let next = self.result.and_then("marketing_channel_list", |entity| entity.eval_marketing_channel_list());
        crate::MarketingChannelListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payment_list(self) -> crate::PaymentListExpression<'a> {
        let next = self.result.and_then("payment_list", |entity| entity.eval_payment_list());
        crate::PaymentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_expense_list(self) -> crate::ExpenseListExpression<'a> {
        let next = self.result.and_then("expense_list", |entity| entity.eval_expense_list());
        crate::ExpenseListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vat_rate_list(self) -> crate::VatRateListExpression<'a> {
        let next = self.result.and_then("vat_rate_list", |entity| entity.eval_vat_rate_list());
        crate::VatRateListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_account_list(self) -> crate::AccountListExpression<'a> {
        let next = self.result.and_then("account_list", |entity| entity.eval_account_list());
        crate::AccountListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_financial_summary_list(self) -> crate::FinancialSummaryListExpression<'a> {
        let next = self.result.and_then("financial_summary_list", |entity| entity.eval_financial_summary_list());
        crate::FinancialSummaryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_budget_list(self) -> crate::BudgetListExpression<'a> {
        let next = self.result.and_then("budget_list", |entity| entity.eval_budget_list());
        crate::BudgetListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payable_list(self) -> crate::PayableListExpression<'a> {
        let next = self.result.and_then("payable_list", |entity| entity.eval_payable_list());
        crate::PayableListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_currency_rate_list(self) -> crate::CurrencyRateListExpression<'a> {
        let next = self.result.and_then("currency_rate_list", |entity| entity.eval_currency_rate_list());
        crate::CurrencyRateListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payment_method_list(self) -> crate::PaymentMethodListExpression<'a> {
        let next = self.result.and_then("payment_method_list", |entity| entity.eval_payment_method_list());
        crate::PaymentMethodListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_financial_period_list(self) -> crate::FinancialPeriodListExpression<'a> {
        let next = self.result.and_then("financial_period_list", |entity| entity.eval_financial_period_list());
        crate::FinancialPeriodListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_list(self) -> crate::VehicleListExpression<'a> {
        let next = self.result.and_then("vehicle_list", |entity| entity.eval_vehicle_list());
        crate::VehicleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_equipment_list(self) -> crate::EquipmentListExpression<'a> {
        let next = self.result.and_then("equipment_list", |entity| entity.eval_equipment_list());
        crate::EquipmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_consumable_list(self) -> crate::ConsumableListExpression<'a> {
        let next = self.result.and_then("consumable_list", |entity| entity.eval_consumable_list());
        crate::ConsumableListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_supplier_list(self) -> crate::SupplierListExpression<'a> {
        let next = self.result.and_then("supplier_list", |entity| entity.eval_supplier_list());
        crate::SupplierListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_storage_location_list(self) -> crate::StorageLocationListExpression<'a> {
        let next = self.result.and_then("storage_location_list", |entity| entity.eval_storage_location_list());
        crate::StorageLocationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contract_list(self) -> crate::ContractListExpression<'a> {
        let next = self.result.and_then("contract_list", |entity| entity.eval_contract_list());
        crate::ContractListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_insurance_policy_list(self) -> crate::InsurancePolicyListExpression<'a> {
        let next = self.result.and_then("insurance_policy_list", |entity| entity.eval_insurance_policy_list());
        crate::InsurancePolicyListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_compliance_check_list(self) -> crate::ComplianceCheckListExpression<'a> {
        let next = self.result.and_then("compliance_check_list", |entity| entity.eval_compliance_check_list());
        crate::ComplianceCheckListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_data_retention_policy_list(self) -> crate::DataRetentionPolicyListExpression<'a> {
        let next = self.result.and_then("data_retention_policy_list", |entity| entity.eval_data_retention_policy_list());
        crate::DataRetentionPolicyListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_policy_document_list(self) -> crate::PolicyDocumentListExpression<'a> {
        let next = self.result.and_then("policy_document_list", |entity| entity.eval_policy_document_list());
        crate::PolicyDocumentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_incident_report_list(self) -> crate::IncidentReportListExpression<'a> {
        let next = self.result.and_then("incident_report_list", |entity| entity.eval_incident_report_list());
        crate::IncidentReportListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_legal_entity_list(self) -> crate::LegalEntityListExpression<'a> {
        let next = self.result.and_then("legal_entity_list", |entity| entity.eval_legal_entity_list());
        crate::LegalEntityListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_regulatory_requirement_list(self) -> crate::RegulatoryRequirementListExpression<'a> {
        let next = self.result.and_then("regulatory_requirement_list", |entity| entity.eval_regulatory_requirement_list());
        crate::RegulatoryRequirementListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_compliance_certificate_list(self) -> crate::ComplianceCertificateListExpression<'a> {
        let next = self.result.and_then("compliance_certificate_list", |entity| entity.eval_compliance_certificate_list());
        crate::ComplianceCertificateListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_role_list(self) -> crate::RoleListExpression<'a> {
        let next = self.result.and_then("role_list", |entity| entity.eval_role_list());
        crate::RoleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_permission_list(self) -> crate::PermissionListExpression<'a> {
        let next = self.result.and_then("permission_list", |entity| entity.eval_permission_list());
        crate::PermissionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_audit_log_list(self) -> crate::AuditLogListExpression<'a> {
        let next = self.result.and_then("audit_log_list", |entity| entity.eval_audit_log_list());
        crate::AuditLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_system_event_list(self) -> crate::SystemEventListExpression<'a> {
        let next = self.result.and_then("system_event_list", |entity| entity.eval_system_event_list());
        crate::SystemEventListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_notification_template_list(self) -> crate::NotificationTemplateListExpression<'a> {
        let next = self.result.and_then("notification_template_list", |entity| entity.eval_notification_template_list());
        crate::NotificationTemplateListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_automation_rule_list(self) -> crate::AutomationRuleListExpression<'a> {
        let next = self.result.and_then("automation_rule_list", |entity| entity.eval_automation_rule_list());
        crate::AutomationRuleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_api_client_list(self) -> crate::ApiClientListExpression<'a> {
        let next = self.result.and_then("api_client_list", |entity| entity.eval_api_client_list());
        crate::ApiClientListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct MerchantListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Merchant>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> MerchantListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Merchant>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Merchant>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Merchant>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Merchant> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::MerchantExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::MerchantExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
}