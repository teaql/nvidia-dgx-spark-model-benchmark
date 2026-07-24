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

    pub fn get_tax_number(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("tax_number", |entity| entity.eval_tax_number());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_address(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("address", |entity| entity.eval_address());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_external_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("external_id", |entity| entity.eval_external_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_create_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("create_time", |entity| entity.eval_create_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_update_time(self) -> crate::ValueExpression<'a, chrono::DateTime<chrono::Utc>> {
        let next = self.result.and_then("update_time", |entity| entity.eval_update_time());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_platform_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("platform_id", |entity| entity.eval_platform_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_platform(self) -> crate::PlatformExpression<'a> {
        let next = self.result.and_then("platform", |entity| entity.eval_platform());
        crate::PlatformExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_list(self) -> crate::EmployeeListExpression<'a> {
        let next = self.result.and_then("employee_list", |entity| entity.eval_employee_list());
        crate::EmployeeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_platform_setting_list(self) -> crate::PlatformSettingListExpression<'a> {
        let next = self.result.and_then("platform_setting_list", |entity| entity.eval_platform_setting_list());
        crate::PlatformSettingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_platform_user_list(self) -> crate::PlatformUserListExpression<'a> {
        let next = self.result.and_then("platform_user_list", |entity| entity.eval_platform_user_list());
        crate::PlatformUserListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_platform_audit_log_list(self) -> crate::PlatformAuditLogListExpression<'a> {
        let next = self.result.and_then("platform_audit_log_list", |entity| entity.eval_platform_audit_log_list());
        crate::PlatformAuditLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_organization_list(self) -> crate::OrganizationListExpression<'a> {
        let next = self.result.and_then("organization_list", |entity| entity.eval_organization_list());
        crate::OrganizationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_organization_setting_list(self) -> crate::OrganizationSettingListExpression<'a> {
        let next = self.result.and_then("organization_setting_list", |entity| entity.eval_organization_setting_list());
        crate::OrganizationSettingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_organization_member_list(self) -> crate::OrganizationMemberListExpression<'a> {
        let next = self.result.and_then("organization_member_list", |entity| entity.eval_organization_member_list());
        crate::OrganizationMemberListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_list(self) -> crate::MoveOrderListExpression<'a> {
        let next = self.result.and_then("move_order_list", |entity| entity.eval_move_order_list());
        crate::MoveOrderListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_quote_list(self) -> crate::MoveQuoteListExpression<'a> {
        let next = self.result.and_then("move_quote_list", |entity| entity.eval_move_quote_list());
        crate::MoveQuoteListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_route_list(self) -> crate::RouteListExpression<'a> {
        let next = self.result.and_then("route_list", |entity| entity.eval_route_list());
        crate::RouteListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_route_stop_list(self) -> crate::RouteStopListExpression<'a> {
        let next = self.result.and_then("route_stop_list", |entity| entity.eval_route_stop_list());
        crate::RouteStopListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_time_slot_list(self) -> crate::TimeSlotListExpression<'a> {
        let next = self.result.and_then("time_slot_list", |entity| entity.eval_time_slot_list());
        crate::TimeSlotListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fulfillment_event_list(self) -> crate::FulfillmentEventListExpression<'a> {
        let next = self.result.and_then("fulfillment_event_list", |entity| entity.eval_fulfillment_event_list());
        crate::FulfillmentEventListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_address_list(self) -> crate::AddressListExpression<'a> {
        let next = self.result.and_then("address_list", |entity| entity.eval_address_list());
        crate::AddressListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_list(self) -> crate::CrewListExpression<'a> {
        let next = self.result.and_then("crew_list", |entity| entity.eval_crew_list());
        crate::CrewListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_dispatch_assignment_list(self) -> crate::DispatchAssignmentListExpression<'a> {
        let next = self.result.and_then("dispatch_assignment_list", |entity| entity.eval_dispatch_assignment_list());
        crate::DispatchAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_damage_report_list(self) -> crate::DamageReportListExpression<'a> {
        let next = self.result.and_then("damage_report_list", |entity| entity.eval_damage_report_list());
        crate::DamageReportListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_proof_of_delivery_list(self) -> crate::ProofOfDeliveryListExpression<'a> {
        let next = self.result.and_then("proof_of_delivery_list", |entity| entity.eval_proof_of_delivery_list());
        crate::ProofOfDeliveryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_inventory_item_list(self) -> crate::InventoryItemListExpression<'a> {
        let next = self.result.and_then("inventory_item_list", |entity| entity.eval_inventory_item_list());
        crate::InventoryItemListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_packing_list_list(self) -> crate::PackingListListExpression<'a> {
        let next = self.result.and_then("packing_list_list", |entity| entity.eval_packing_list_list());
        crate::PackingListListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_packing_item_list(self) -> crate::PackingItemListExpression<'a> {
        let next = self.result.and_then("packing_item_list", |entity| entity.eval_packing_item_list());
        crate::PackingItemListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_loading_plan_list(self) -> crate::LoadingPlanListExpression<'a> {
        let next = self.result.and_then("loading_plan_list", |entity| entity.eval_loading_plan_list());
        crate::LoadingPlanListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_unloading_plan_list(self) -> crate::UnloadingPlanListExpression<'a> {
        let next = self.result.and_then("unloading_plan_list", |entity| entity.eval_unloading_plan_list());
        crate::UnloadingPlanListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_storage_facility_list(self) -> crate::StorageFacilityListExpression<'a> {
        let next = self.result.and_then("storage_facility_list", |entity| entity.eval_storage_facility_list());
        crate::StorageFacilityListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_storage_unit_list(self) -> crate::StorageUnitListExpression<'a> {
        let next = self.result.and_then("storage_unit_list", |entity| entity.eval_storage_unit_list());
        crate::StorageUnitListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_storage_inventory_list(self) -> crate::StorageInventoryListExpression<'a> {
        let next = self.result.and_then("storage_inventory_list", |entity| entity.eval_storage_inventory_list());
        crate::StorageInventoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_transport_manifest_list(self) -> crate::TransportManifestListExpression<'a> {
        let next = self.result.and_then("transport_manifest_list", |entity| entity.eval_transport_manifest_list());
        crate::TransportManifestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customs_declaration_list(self) -> crate::CustomsDeclarationListExpression<'a> {
        let next = self.result.and_then("customs_declaration_list", |entity| entity.eval_customs_declaration_list());
        crate::CustomsDeclarationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_equipment_checklist_list(self) -> crate::EquipmentChecklistListExpression<'a> {
        let next = self.result.and_then("equipment_checklist_list", |entity| entity.eval_equipment_checklist_list());
        crate::EquipmentChecklistListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fuel_log_list(self) -> crate::FuelLogListExpression<'a> {
        let next = self.result.and_then("fuel_log_list", |entity| entity.eval_fuel_log_list());
        crate::FuelLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_maintenance_request_list(self) -> crate::MaintenanceRequestListExpression<'a> {
        let next = self.result.and_then("maintenance_request_list", |entity| entity.eval_maintenance_request_list());
        crate::MaintenanceRequestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_department_list(self) -> crate::DepartmentListExpression<'a> {
        let next = self.result.and_then("department_list", |entity| entity.eval_department_list());
        crate::DepartmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_job_assignment_list(self) -> crate::JobAssignmentListExpression<'a> {
        let next = self.result.and_then("job_assignment_list", |entity| entity.eval_job_assignment_list());
        crate::JobAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_work_shift_list(self) -> crate::WorkShiftListExpression<'a> {
        let next = self.result.and_then("work_shift_list", |entity| entity.eval_work_shift_list());
        crate::WorkShiftListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_worked_hours_list(self) -> crate::WorkedHoursListExpression<'a> {
        let next = self.result.and_then("worked_hours_list", |entity| entity.eval_worked_hours_list());
        crate::WorkedHoursListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payroll_period_list(self) -> crate::PayrollPeriodListExpression<'a> {
        let next = self.result.and_then("payroll_period_list", |entity| entity.eval_payroll_period_list());
        crate::PayrollPeriodListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payroll_calculation_list(self) -> crate::PayrollCalculationListExpression<'a> {
        let next = self.result.and_then("payroll_calculation_list", |entity| entity.eval_payroll_calculation_list());
        crate::PayrollCalculationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payslip_list(self) -> crate::PayslipListExpression<'a> {
        let next = self.result.and_then("payslip_list", |entity| entity.eval_payslip_list());
        crate::PayslipListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_bonus_list(self) -> crate::BonusListExpression<'a> {
        let next = self.result.and_then("bonus_list", |entity| entity.eval_bonus_list());
        crate::BonusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_certification_list(self) -> crate::EmployeeCertificationListExpression<'a> {
        let next = self.result.and_then("employee_certification_list", |entity| entity.eval_employee_certification_list());
        crate::EmployeeCertificationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_leave_request_list(self) -> crate::LeaveRequestListExpression<'a> {
        let next = self.result.and_then("leave_request_list", |entity| entity.eval_leave_request_list());
        crate::LeaveRequestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_billing_profile_list(self) -> crate::BillingProfileListExpression<'a> {
        let next = self.result.and_then("billing_profile_list", |entity| entity.eval_billing_profile_list());
        crate::BillingProfileListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_corporate_customer_profile_list(self) -> crate::CorporateCustomerProfileListExpression<'a> {
        let next = self.result.and_then("corporate_customer_profile_list", |entity| entity.eval_corporate_customer_profile_list());
        crate::CorporateCustomerProfileListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_list(self) -> crate::CustomerListExpression<'a> {
        let next = self.result.and_then("customer_list", |entity| entity.eval_customer_list());
        crate::CustomerListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_consent_list(self) -> crate::CustomerConsentListExpression<'a> {
        let next = self.result.and_then("customer_consent_list", |entity| entity.eval_customer_consent_list());
        crate::CustomerConsentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_contact_list(self) -> crate::CustomerContactListExpression<'a> {
        let next = self.result.and_then("customer_contact_list", |entity| entity.eval_customer_contact_list());
        crate::CustomerContactListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_history_list(self) -> crate::CustomerHistoryListExpression<'a> {
        let next = self.result.and_then("customer_history_list", |entity| entity.eval_customer_history_list());
        crate::CustomerHistoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_preference_list(self) -> crate::CustomerPreferenceListExpression<'a> {
        let next = self.result.and_then("customer_preference_list", |entity| entity.eval_customer_preference_list());
        crate::CustomerPreferenceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_private_customer_profile_list(self) -> crate::PrivateCustomerProfileListExpression<'a> {
        let next = self.result.and_then("private_customer_profile_list", |entity| entity.eval_private_customer_profile_list());
        crate::PrivateCustomerProfileListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_box_rental_list(self) -> crate::BoxRentalListExpression<'a> {
        let next = self.result.and_then("box_rental_list", |entity| entity.eval_box_rental_list());
        crate::BoxRentalListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_cleaning_service_list(self) -> crate::CleaningServiceListExpression<'a> {
        let next = self.result.and_then("cleaning_service_list", |entity| entity.eval_cleaning_service_list());
        crate::CleaningServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_moving_service_list(self) -> crate::MovingServiceListExpression<'a> {
        let next = self.result.and_then("moving_service_list", |entity| entity.eval_moving_service_list());
        crate::MovingServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_price_list_list(self) -> crate::PriceListListExpression<'a> {
        let next = self.result.and_then("price_list_list", |entity| entity.eval_price_list_list());
        crate::PriceListListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_product_list(self) -> crate::ProductListExpression<'a> {
        let next = self.result.and_then("product_list", |entity| entity.eval_product_list());
        crate::ProductListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_list(self) -> crate::ServiceListExpression<'a> {
        let next = self.result.and_then("service_list", |entity| entity.eval_service_list());
        crate::ServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_bundle_list(self) -> crate::ServiceBundleListExpression<'a> {
        let next = self.result.and_then("service_bundle_list", |entity| entity.eval_service_bundle_list());
        crate::ServiceBundleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_configuration_list(self) -> crate::ServiceConfigurationListExpression<'a> {
        let next = self.result.and_then("service_configuration_list", |entity| entity.eval_service_configuration_list());
        crate::ServiceConfigurationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_price_list(self) -> crate::ServicePriceListExpression<'a> {
        let next = self.result.and_then("service_price_list", |entity| entity.eval_service_price_list());
        crate::ServicePriceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_campaign_list(self) -> crate::CampaignListExpression<'a> {
        let next = self.result.and_then("campaign_list", |entity| entity.eval_campaign_list());
        crate::CampaignListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_conversion_event_list(self) -> crate::ConversionEventListExpression<'a> {
        let next = self.result.and_then("conversion_event_list", |entity| entity.eval_conversion_event_list());
        crate::ConversionEventListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_conversion_metric_list(self) -> crate::ConversionMetricListExpression<'a> {
        let next = self.result.and_then("conversion_metric_list", |entity| entity.eval_conversion_metric_list());
        crate::ConversionMetricListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_discount_code_list(self) -> crate::DiscountCodeListExpression<'a> {
        let next = self.result.and_then("discount_code_list", |entity| entity.eval_discount_code_list());
        crate::DiscountCodeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_lead_list(self) -> crate::LeadListExpression<'a> {
        let next = self.result.and_then("lead_list", |entity| entity.eval_lead_list());
        crate::LeadListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_lead_activity_list(self) -> crate::LeadActivityListExpression<'a> {
        let next = self.result.and_then("lead_activity_list", |entity| entity.eval_lead_activity_list());
        crate::LeadActivityListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_sales_opportunity_list(self) -> crate::SalesOpportunityListExpression<'a> {
        let next = self.result.and_then("sales_opportunity_list", |entity| entity.eval_sales_opportunity_list());
        crate::SalesOpportunityListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_account_list(self) -> crate::AccountListExpression<'a> {
        let next = self.result.and_then("account_list", |entity| entity.eval_account_list());
        crate::AccountListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_expense_list(self) -> crate::ExpenseListExpression<'a> {
        let next = self.result.and_then("expense_list", |entity| entity.eval_expense_list());
        crate::ExpenseListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_financial_summary_list(self) -> crate::FinancialSummaryListExpression<'a> {
        let next = self.result.and_then("financial_summary_list", |entity| entity.eval_financial_summary_list());
        crate::FinancialSummaryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_invoice_list(self) -> crate::InvoiceListExpression<'a> {
        let next = self.result.and_then("invoice_list", |entity| entity.eval_invoice_list());
        crate::InvoiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_invoice_line_list(self) -> crate::InvoiceLineListExpression<'a> {
        let next = self.result.and_then("invoice_line_list", |entity| entity.eval_invoice_line_list());
        crate::InvoiceLineListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_journal_entry_list(self) -> crate::JournalEntryListExpression<'a> {
        let next = self.result.and_then("journal_entry_list", |entity| entity.eval_journal_entry_list());
        crate::JournalEntryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payment_list(self) -> crate::PaymentListExpression<'a> {
        let next = self.result.and_then("payment_list", |entity| entity.eval_payment_list());
        crate::PaymentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_refund_list(self) -> crate::RefundListExpression<'a> {
        let next = self.result.and_then("refund_list", |entity| entity.eval_refund_list());
        crate::RefundListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vat_rate_list(self) -> crate::VatRateListExpression<'a> {
        let next = self.result.and_then("vat_rate_list", |entity| entity.eval_vat_rate_list());
        crate::VatRateListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_asset_assignment_list(self) -> crate::AssetAssignmentListExpression<'a> {
        let next = self.result.and_then("asset_assignment_list", |entity| entity.eval_asset_assignment_list());
        crate::AssetAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_asset_inspection_list(self) -> crate::AssetInspectionListExpression<'a> {
        let next = self.result.and_then("asset_inspection_list", |entity| entity.eval_asset_inspection_list());
        crate::AssetInspectionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_consumable_list(self) -> crate::ConsumableListExpression<'a> {
        let next = self.result.and_then("consumable_list", |entity| entity.eval_consumable_list());
        crate::ConsumableListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_equipment_list(self) -> crate::EquipmentListExpression<'a> {
        let next = self.result.and_then("equipment_list", |entity| entity.eval_equipment_list());
        crate::EquipmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_fuel_record_list(self) -> crate::FuelRecordListExpression<'a> {
        let next = self.result.and_then("fuel_record_list", |entity| entity.eval_fuel_record_list());
        crate::FuelRecordListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_maintenance_event_list(self) -> crate::MaintenanceEventListExpression<'a> {
        let next = self.result.and_then("maintenance_event_list", |entity| entity.eval_maintenance_event_list());
        crate::MaintenanceEventListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_maintenance_schedule_list(self) -> crate::MaintenanceScheduleListExpression<'a> {
        let next = self.result.and_then("maintenance_schedule_list", |entity| entity.eval_maintenance_schedule_list());
        crate::MaintenanceScheduleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_supplier_list(self) -> crate::SupplierListExpression<'a> {
        let next = self.result.and_then("supplier_list", |entity| entity.eval_supplier_list());
        crate::SupplierListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_list(self) -> crate::VehicleListExpression<'a> {
        let next = self.result.and_then("vehicle_list", |entity| entity.eval_vehicle_list());
        crate::VehicleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_compliance_check_list(self) -> crate::ComplianceCheckListExpression<'a> {
        let next = self.result.and_then("compliance_check_list", |entity| entity.eval_compliance_check_list());
        crate::ComplianceCheckListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contract_list(self) -> crate::ContractListExpression<'a> {
        let next = self.result.and_then("contract_list", |entity| entity.eval_contract_list());
        crate::ContractListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_data_retention_policy_list(self) -> crate::DataRetentionPolicyListExpression<'a> {
        let next = self.result.and_then("data_retention_policy_list", |entity| entity.eval_data_retention_policy_list());
        crate::DataRetentionPolicyListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_document_list(self) -> crate::DocumentListExpression<'a> {
        let next = self.result.and_then("document_list", |entity| entity.eval_document_list());
        crate::DocumentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_document_version_list(self) -> crate::DocumentVersionListExpression<'a> {
        let next = self.result.and_then("document_version_list", |entity| entity.eval_document_version_list());
        crate::DocumentVersionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_insurance_claim_list(self) -> crate::InsuranceClaimListExpression<'a> {
        let next = self.result.and_then("insurance_claim_list", |entity| entity.eval_insurance_claim_list());
        crate::InsuranceClaimListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_insurance_policy_list(self) -> crate::InsurancePolicyListExpression<'a> {
        let next = self.result.and_then("insurance_policy_list", |entity| entity.eval_insurance_policy_list());
        crate::InsurancePolicyListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_recovery_request_list(self) -> crate::RecoveryRequestListExpression<'a> {
        let next = self.result.and_then("recovery_request_list", |entity| entity.eval_recovery_request_list());
        crate::RecoveryRequestListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_magic_link_list(self) -> crate::MagicLinkListExpression<'a> {
        let next = self.result.and_then("magic_link_list", |entity| entity.eval_magic_link_list());
        crate::MagicLinkListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_permission_list(self) -> crate::PermissionListExpression<'a> {
        let next = self.result.and_then("permission_list", |entity| entity.eval_permission_list());
        crate::PermissionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_role_list(self) -> crate::RoleListExpression<'a> {
        let next = self.result.and_then("role_list", |entity| entity.eval_role_list());
        crate::RoleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_role_permission_list(self) -> crate::RolePermissionListExpression<'a> {
        let next = self.result.and_then("role_permission_list", |entity| entity.eval_role_permission_list());
        crate::RolePermissionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_user_account_list(self) -> crate::UserAccountListExpression<'a> {
        let next = self.result.and_then("user_account_list", |entity| entity.eval_user_account_list());
        crate::UserAccountListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_user_role_assignment_list(self) -> crate::UserRoleAssignmentListExpression<'a> {
        let next = self.result.and_then("user_role_assignment_list", |entity| entity.eval_user_role_assignment_list());
        crate::UserRoleAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_user_session_list(self) -> crate::UserSessionListExpression<'a> {
        let next = self.result.and_then("user_session_list", |entity| entity.eval_user_session_list());
        crate::UserSessionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_activity_log_list(self) -> crate::ActivityLogListExpression<'a> {
        let next = self.result.and_then("activity_log_list", |entity| entity.eval_activity_log_list());
        crate::ActivityLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_audit_log_list(self) -> crate::AuditLogListExpression<'a> {
        let next = self.result.and_then("audit_log_list", |entity| entity.eval_audit_log_list());
        crate::AuditLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_change_set_list(self) -> crate::ChangeSetListExpression<'a> {
        let next = self.result.and_then("change_set_list", |entity| entity.eval_change_set_list());
        crate::ChangeSetListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_entity_change_list(self) -> crate::EntityChangeListExpression<'a> {
        let next = self.result.and_then("entity_change_list", |entity| entity.eval_entity_change_list());
        crate::EntityChangeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_automation_action_list(self) -> crate::AutomationActionListExpression<'a> {
        let next = self.result.and_then("automation_action_list", |entity| entity.eval_automation_action_list());
        crate::AutomationActionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_automation_rule_list(self) -> crate::AutomationRuleListExpression<'a> {
        let next = self.result.and_then("automation_rule_list", |entity| entity.eval_automation_rule_list());
        crate::AutomationRuleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_automation_trigger_list(self) -> crate::AutomationTriggerListExpression<'a> {
        let next = self.result.and_then("automation_trigger_list", |entity| entity.eval_automation_trigger_list());
        crate::AutomationTriggerListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_notification_list(self) -> crate::NotificationListExpression<'a> {
        let next = self.result.and_then("notification_list", |entity| entity.eval_notification_list());
        crate::NotificationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_notification_template_list(self) -> crate::NotificationTemplateListExpression<'a> {
        let next = self.result.and_then("notification_template_list", |entity| entity.eval_notification_template_list());
        crate::NotificationTemplateListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_api_client_list(self) -> crate::ApiClientListExpression<'a> {
        let next = self.result.and_then("api_client_list", |entity| entity.eval_api_client_list());
        crate::ApiClientListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_api_endpoint_list(self) -> crate::ApiEndpointListExpression<'a> {
        let next = self.result.and_then("api_endpoint_list", |entity| entity.eval_api_endpoint_list());
        crate::ApiEndpointListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_integration_mapping_list(self) -> crate::IntegrationMappingListExpression<'a> {
        let next = self.result.and_then("integration_mapping_list", |entity| entity.eval_integration_mapping_list());
        crate::IntegrationMappingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_webhook_list(self) -> crate::WebhookListExpression<'a> {
        let next = self.result.and_then("webhook_list", |entity| entity.eval_webhook_list());
        crate::WebhookListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_webhook_delivery_list(self) -> crate::WebhookDeliveryListExpression<'a> {
        let next = self.result.and_then("webhook_delivery_list", |entity| entity.eval_webhook_delivery_list());
        crate::WebhookDeliveryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_platform_configuration_list(self) -> crate::PlatformConfigurationListExpression<'a> {
        let next = self.result.and_then("platform_configuration_list", |entity| entity.eval_platform_configuration_list());
        crate::PlatformConfigurationListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_platform_locale_list(self) -> crate::PlatformLocaleListExpression<'a> {
        let next = self.result.and_then("platform_locale_list", |entity| entity.eval_platform_locale_list());
        crate::PlatformLocaleListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_branch_list(self) -> crate::MerchantBranchListExpression<'a> {
        let next = self.result.and_then("merchant_branch_list", |entity| entity.eval_merchant_branch_list());
        crate::MerchantBranchListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_merchant_setting_list(self) -> crate::MerchantSettingListExpression<'a> {
        let next = self.result.and_then("merchant_setting_list", |entity| entity.eval_merchant_setting_list());
        crate::MerchantSettingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_operational_exception_list(self) -> crate::OperationalExceptionListExpression<'a> {
        let next = self.result.and_then("operational_exception_list", |entity| entity.eval_operational_exception_list());
        crate::OperationalExceptionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_crew_member_assignment_list(self) -> crate::CrewMemberAssignmentListExpression<'a> {
        let next = self.result.and_then("crew_member_assignment_list", |entity| entity.eval_crew_member_assignment_list());
        crate::CrewMemberAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_pickup_instruction_list(self) -> crate::PickupInstructionListExpression<'a> {
        let next = self.result.and_then("pickup_instruction_list", |entity| entity.eval_pickup_instruction_list());
        crate::PickupInstructionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_delivery_instruction_list(self) -> crate::DeliveryInstructionListExpression<'a> {
        let next = self.result.and_then("delivery_instruction_list", |entity| entity.eval_delivery_instruction_list());
        crate::DeliveryInstructionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_inventory_list(self) -> crate::MoveInventoryListExpression<'a> {
        let next = self.result.and_then("move_inventory_list", |entity| entity.eval_move_inventory_list());
        crate::MoveInventoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics1_list(self) -> crate::ExtraOperationsLogistics1ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics1_list", |entity| entity.eval_extra_operations_logistics1_list());
        crate::ExtraOperationsLogistics1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics2_list(self) -> crate::ExtraOperationsLogistics2ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics2_list", |entity| entity.eval_extra_operations_logistics2_list());
        crate::ExtraOperationsLogistics2ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics3_list(self) -> crate::ExtraOperationsLogistics3ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics3_list", |entity| entity.eval_extra_operations_logistics3_list());
        crate::ExtraOperationsLogistics3ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics4_list(self) -> crate::ExtraOperationsLogistics4ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics4_list", |entity| entity.eval_extra_operations_logistics4_list());
        crate::ExtraOperationsLogistics4ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics5_list(self) -> crate::ExtraOperationsLogistics5ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics5_list", |entity| entity.eval_extra_operations_logistics5_list());
        crate::ExtraOperationsLogistics5ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics6_list(self) -> crate::ExtraOperationsLogistics6ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics6_list", |entity| entity.eval_extra_operations_logistics6_list());
        crate::ExtraOperationsLogistics6ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics7_list(self) -> crate::ExtraOperationsLogistics7ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics7_list", |entity| entity.eval_extra_operations_logistics7_list());
        crate::ExtraOperationsLogistics7ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics8_list(self) -> crate::ExtraOperationsLogistics8ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics8_list", |entity| entity.eval_extra_operations_logistics8_list());
        crate::ExtraOperationsLogistics8ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_operations_logistics9_list(self) -> crate::ExtraOperationsLogistics9ListExpression<'a> {
        let next = self.result.and_then("extra_operations_logistics9_list", |entity| entity.eval_extra_operations_logistics9_list());
        crate::ExtraOperationsLogistics9ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_employee_availability_list(self) -> crate::EmployeeAvailabilityListExpression<'a> {
        let next = self.result.and_then("employee_availability_list", |entity| entity.eval_employee_availability_list());
        crate::EmployeeAvailabilityListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payroll_deduction_list(self) -> crate::PayrollDeductionListExpression<'a> {
        let next = self.result.and_then("payroll_deduction_list", |entity| entity.eval_payroll_deduction_list());
        crate::PayrollDeductionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_training_session_list(self) -> crate::TrainingSessionListExpression<'a> {
        let next = self.result.and_then("training_session_list", |entity| entity.eval_training_session_list());
        crate::TrainingSessionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_shift_assignment_list(self) -> crate::ShiftAssignmentListExpression<'a> {
        let next = self.result.and_then("shift_assignment_list", |entity| entity.eval_shift_assignment_list());
        crate::ShiftAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_employees_payroll1_list(self) -> crate::ExtraEmployeesPayroll1ListExpression<'a> {
        let next = self.result.and_then("extra_employees_payroll1_list", |entity| entity.eval_extra_employees_payroll1_list());
        crate::ExtraEmployeesPayroll1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_employees_payroll2_list(self) -> crate::ExtraEmployeesPayroll2ListExpression<'a> {
        let next = self.result.and_then("extra_employees_payroll2_list", |entity| entity.eval_extra_employees_payroll2_list());
        crate::ExtraEmployeesPayroll2ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_employees_payroll3_list(self) -> crate::ExtraEmployeesPayroll3ListExpression<'a> {
        let next = self.result.and_then("extra_employees_payroll3_list", |entity| entity.eval_extra_employees_payroll3_list());
        crate::ExtraEmployeesPayroll3ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_employees_payroll4_list(self) -> crate::ExtraEmployeesPayroll4ListExpression<'a> {
        let next = self.result.and_then("extra_employees_payroll4_list", |entity| entity.eval_extra_employees_payroll4_list());
        crate::ExtraEmployeesPayroll4ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_employees_payroll5_list(self) -> crate::ExtraEmployeesPayroll5ListExpression<'a> {
        let next = self.result.and_then("extra_employees_payroll5_list", |entity| entity.eval_extra_employees_payroll5_list());
        crate::ExtraEmployeesPayroll5ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_employees_payroll6_list(self) -> crate::ExtraEmployeesPayroll6ListExpression<'a> {
        let next = self.result.and_then("extra_employees_payroll6_list", |entity| entity.eval_extra_employees_payroll6_list());
        crate::ExtraEmployeesPayroll6ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_employees_payroll7_list(self) -> crate::ExtraEmployeesPayroll7ListExpression<'a> {
        let next = self.result.and_then("extra_employees_payroll7_list", |entity| entity.eval_extra_employees_payroll7_list());
        crate::ExtraEmployeesPayroll7ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_complaint_list(self) -> crate::CustomerComplaintListExpression<'a> {
        let next = self.result.and_then("customer_complaint_list", |entity| entity.eval_customer_complaint_list());
        crate::CustomerComplaintListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_note_list(self) -> crate::CustomerNoteListExpression<'a> {
        let next = self.result.and_then("customer_note_list", |entity| entity.eval_customer_note_list());
        crate::CustomerNoteListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_customer_management1_list(self) -> crate::ExtraCustomerManagement1ListExpression<'a> {
        let next = self.result.and_then("extra_customer_management1_list", |entity| entity.eval_extra_customer_management1_list());
        crate::ExtraCustomerManagement1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_customer_management2_list(self) -> crate::ExtraCustomerManagement2ListExpression<'a> {
        let next = self.result.and_then("extra_customer_management2_list", |entity| entity.eval_extra_customer_management2_list());
        crate::ExtraCustomerManagement2ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_customer_management3_list(self) -> crate::ExtraCustomerManagement3ListExpression<'a> {
        let next = self.result.and_then("extra_customer_management3_list", |entity| entity.eval_extra_customer_management3_list());
        crate::ExtraCustomerManagement3ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_customer_management4_list(self) -> crate::ExtraCustomerManagement4ListExpression<'a> {
        let next = self.result.and_then("extra_customer_management4_list", |entity| entity.eval_extra_customer_management4_list());
        crate::ExtraCustomerManagement4ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_customer_management5_list(self) -> crate::ExtraCustomerManagement5ListExpression<'a> {
        let next = self.result.and_then("extra_customer_management5_list", |entity| entity.eval_extra_customer_management5_list());
        crate::ExtraCustomerManagement5ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_customer_management6_list(self) -> crate::ExtraCustomerManagement6ListExpression<'a> {
        let next = self.result.and_then("extra_customer_management6_list", |entity| entity.eval_extra_customer_management6_list());
        crate::ExtraCustomerManagement6ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_storage_service_list(self) -> crate::StorageServiceListExpression<'a> {
        let next = self.result.and_then("storage_service_list", |entity| entity.eval_storage_service_list());
        crate::StorageServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_packing_service_list(self) -> crate::PackingServiceListExpression<'a> {
        let next = self.result.and_then("packing_service_list", |entity| entity.eval_packing_service_list());
        crate::PackingServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_disposal_service_list(self) -> crate::DisposalServiceListExpression<'a> {
        let next = self.result.and_then("disposal_service_list", |entity| entity.eval_disposal_service_list());
        crate::DisposalServiceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_rental_period_list(self) -> crate::RentalPeriodListExpression<'a> {
        let next = self.result.and_then("rental_period_list", |entity| entity.eval_rental_period_list());
        crate::RentalPeriodListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_area_list(self) -> crate::ServiceAreaListExpression<'a> {
        let next = self.result.and_then("service_area_list", |entity| entity.eval_service_area_list());
        crate::ServiceAreaListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_products_services1_list(self) -> crate::ExtraProductsServices1ListExpression<'a> {
        let next = self.result.and_then("extra_products_services1_list", |entity| entity.eval_extra_products_services1_list());
        crate::ExtraProductsServices1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_products_services2_list(self) -> crate::ExtraProductsServices2ListExpression<'a> {
        let next = self.result.and_then("extra_products_services2_list", |entity| entity.eval_extra_products_services2_list());
        crate::ExtraProductsServices2ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_products_services3_list(self) -> crate::ExtraProductsServices3ListExpression<'a> {
        let next = self.result.and_then("extra_products_services3_list", |entity| entity.eval_extra_products_services3_list());
        crate::ExtraProductsServices3ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_products_services4_list(self) -> crate::ExtraProductsServices4ListExpression<'a> {
        let next = self.result.and_then("extra_products_services4_list", |entity| entity.eval_extra_products_services4_list());
        crate::ExtraProductsServices4ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_campaign_audience_list(self) -> crate::CampaignAudienceListExpression<'a> {
        let next = self.result.and_then("campaign_audience_list", |entity| entity.eval_campaign_audience_list());
        crate::CampaignAudienceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_campaign_channel_list(self) -> crate::CampaignChannelListExpression<'a> {
        let next = self.result.and_then("campaign_channel_list", |entity| entity.eval_campaign_channel_list());
        crate::CampaignChannelListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_lead_attribution_list(self) -> crate::LeadAttributionListExpression<'a> {
        let next = self.result.and_then("lead_attribution_list", |entity| entity.eval_lead_attribution_list());
        crate::LeadAttributionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_sales_funnel_list(self) -> crate::SalesFunnelListExpression<'a> {
        let next = self.result.and_then("sales_funnel_list", |entity| entity.eval_sales_funnel_list());
        crate::SalesFunnelListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_marketing_sales1_list(self) -> crate::ExtraMarketingSales1ListExpression<'a> {
        let next = self.result.and_then("extra_marketing_sales1_list", |entity| entity.eval_extra_marketing_sales1_list());
        crate::ExtraMarketingSales1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_marketing_sales2_list(self) -> crate::ExtraMarketingSales2ListExpression<'a> {
        let next = self.result.and_then("extra_marketing_sales2_list", |entity| entity.eval_extra_marketing_sales2_list());
        crate::ExtraMarketingSales2ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_marketing_sales3_list(self) -> crate::ExtraMarketingSales3ListExpression<'a> {
        let next = self.result.and_then("extra_marketing_sales3_list", |entity| entity.eval_extra_marketing_sales3_list());
        crate::ExtraMarketingSales3ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_marketing_sales4_list(self) -> crate::ExtraMarketingSales4ListExpression<'a> {
        let next = self.result.and_then("extra_marketing_sales4_list", |entity| entity.eval_extra_marketing_sales4_list());
        crate::ExtraMarketingSales4ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_expense_claim_list(self) -> crate::ExpenseClaimListExpression<'a> {
        let next = self.result.and_then("expense_claim_list", |entity| entity.eval_expense_claim_list());
        crate::ExpenseClaimListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_settlement_list(self) -> crate::SettlementListExpression<'a> {
        let next = self.result.and_then("settlement_list", |entity| entity.eval_settlement_list());
        crate::SettlementListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_receivable_list(self) -> crate::ReceivableListExpression<'a> {
        let next = self.result.and_then("receivable_list", |entity| entity.eval_receivable_list());
        crate::ReceivableListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payable_list(self) -> crate::PayableListExpression<'a> {
        let next = self.result.and_then("payable_list", |entity| entity.eval_payable_list());
        crate::PayableListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_finance_accounting1_list(self) -> crate::ExtraFinanceAccounting1ListExpression<'a> {
        let next = self.result.and_then("extra_finance_accounting1_list", |entity| entity.eval_extra_finance_accounting1_list());
        crate::ExtraFinanceAccounting1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_finance_accounting2_list(self) -> crate::ExtraFinanceAccounting2ListExpression<'a> {
        let next = self.result.and_then("extra_finance_accounting2_list", |entity| entity.eval_extra_finance_accounting2_list());
        crate::ExtraFinanceAccounting2ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_finance_accounting3_list(self) -> crate::ExtraFinanceAccounting3ListExpression<'a> {
        let next = self.result.and_then("extra_finance_accounting3_list", |entity| entity.eval_extra_finance_accounting3_list());
        crate::ExtraFinanceAccounting3ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_finance_accounting4_list(self) -> crate::ExtraFinanceAccounting4ListExpression<'a> {
        let next = self.result.and_then("extra_finance_accounting4_list", |entity| entity.eval_extra_finance_accounting4_list());
        crate::ExtraFinanceAccounting4ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_vehicle_inspection_list(self) -> crate::VehicleInspectionListExpression<'a> {
        let next = self.result.and_then("vehicle_inspection_list", |entity| entity.eval_vehicle_inspection_list());
        crate::VehicleInspectionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_equipment_checkout_list(self) -> crate::EquipmentCheckoutListExpression<'a> {
        let next = self.result.and_then("equipment_checkout_list", |entity| entity.eval_equipment_checkout_list());
        crate::EquipmentCheckoutListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_consumable_reorder_list(self) -> crate::ConsumableReorderListExpression<'a> {
        let next = self.result.and_then("consumable_reorder_list", |entity| entity.eval_consumable_reorder_list());
        crate::ConsumableReorderListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_asset_management1_list(self) -> crate::ExtraAssetManagement1ListExpression<'a> {
        let next = self.result.and_then("extra_asset_management1_list", |entity| entity.eval_extra_asset_management1_list());
        crate::ExtraAssetManagement1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_asset_management2_list(self) -> crate::ExtraAssetManagement2ListExpression<'a> {
        let next = self.result.and_then("extra_asset_management2_list", |entity| entity.eval_extra_asset_management2_list());
        crate::ExtraAssetManagement2ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_asset_management3_list(self) -> crate::ExtraAssetManagement3ListExpression<'a> {
        let next = self.result.and_then("extra_asset_management3_list", |entity| entity.eval_extra_asset_management3_list());
        crate::ExtraAssetManagement3ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_asset_management4_list(self) -> crate::ExtraAssetManagement4ListExpression<'a> {
        let next = self.result.and_then("extra_asset_management4_list", |entity| entity.eval_extra_asset_management4_list());
        crate::ExtraAssetManagement4ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_asset_management5_list(self) -> crate::ExtraAssetManagement5ListExpression<'a> {
        let next = self.result.and_then("extra_asset_management5_list", |entity| entity.eval_extra_asset_management5_list());
        crate::ExtraAssetManagement5ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_authentication_attempt_list(self) -> crate::AuthenticationAttemptListExpression<'a> {
        let next = self.result.and_then("authentication_attempt_list", |entity| entity.eval_authentication_attempt_list());
        crate::AuthenticationAttemptListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_access_policy_list(self) -> crate::AccessPolicyListExpression<'a> {
        let next = self.result.and_then("access_policy_list", |entity| entity.eval_access_policy_list());
        crate::AccessPolicyListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_identity_access1_list(self) -> crate::ExtraIdentityAccess1ListExpression<'a> {
        let next = self.result.and_then("extra_identity_access1_list", |entity| entity.eval_extra_identity_access1_list());
        crate::ExtraIdentityAccess1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_audit_export_list(self) -> crate::AuditExportListExpression<'a> {
        let next = self.result.and_then("audit_export_list", |entity| entity.eval_audit_export_list());
        crate::AuditExportListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_activity_audit1_list(self) -> crate::ExtraActivityAudit1ListExpression<'a> {
        let next = self.result.and_then("extra_activity_audit1_list", |entity| entity.eval_extra_activity_audit1_list());
        crate::ExtraActivityAudit1ListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_notification_preference_list(self) -> crate::NotificationPreferenceListExpression<'a> {
        let next = self.result.and_then("notification_preference_list", |entity| entity.eval_notification_preference_list());
        crate::NotificationPreferenceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_notification_delivery_list(self) -> crate::NotificationDeliveryListExpression<'a> {
        let next = self.result.and_then("notification_delivery_list", |entity| entity.eval_notification_delivery_list());
        crate::NotificationDeliveryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_synchronization_run_list(self) -> crate::SynchronizationRunListExpression<'a> {
        let next = self.result.and_then("synchronization_run_list", |entity| entity.eval_synchronization_run_list());
        crate::SynchronizationRunListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_extra_api_integrations1_list(self) -> crate::ExtraApiIntegrations1ListExpression<'a> {
        let next = self.result.and_then("extra_api_integrations1_list", |entity| entity.eval_extra_api_integrations1_list());
        crate::ExtraApiIntegrations1ListExpression::new(next, self.root_desc.clone())
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