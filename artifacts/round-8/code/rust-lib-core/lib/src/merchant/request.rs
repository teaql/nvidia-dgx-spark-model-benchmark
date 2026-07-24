use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Merchant {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Merchant {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/merchant
#[derive(Debug)]
pub struct MerchantRequest<R = crate::Merchant> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for MerchantRequest<R> {
    fn clone(&self) -> Self {
        Self {
            query: self.query.clone(),
            relation_selections: self.relation_selections.clone(),
            relation_filters: self.relation_filters.clone(),
            child_enhancements: self.child_enhancements.clone(),
            query_options: self.query_options.clone(),
            marker: PhantomData,
        }
    }
}

impl<R> MerchantRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Merchant")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> MerchantRequest<T> {
        MerchantRequest {
            query: self.query,
            relation_selections: self.relation_selections,
            relation_filters: self.relation_filters,
            child_enhancements: self.child_enhancements,
            query_options: self.query_options,
            marker: PhantomData,
        }
    }

    pub fn query(&self) -> &SelectQuery {
        &self.query
    }

    pub fn relation_selections(&self) -> &[RelationSelection] {
        &self.relation_selections
    }

    pub fn relation_filters(&self) -> &[RelationFilter] {
        &self.relation_filters
    }

    pub fn child_enhancements(&self) -> &[QuerySelection] {
        &self.child_enhancements
    }

    pub fn query_options(&self) -> &QueryOptions {
        &self.query_options
    }

    pub fn into_query(self) -> SelectQuery {
        self.query
    }


    pub fn purpose(self, purpose: impl Into<String>) -> crate::PurposedQuery<Self> {
        crate::PurposedQuery::new(self, purpose)
    }

    pub(crate) async fn _execute_for_list<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .merchant_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_enhanced_entities_with_relation_aggregates::<R>(
            &query,
            &relation_aggregates,
        ).await?;
        let facets = execute_facets(ctx, &query, &query_options)
            .await
            .map_err(DataServiceError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_stream<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let chunks = repository.fetch_stream(&query)
            .await?;
        Ok(chunks)
    }

    pub(crate) async fn _execute_for_first<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let rows = self.limit(1)._execute_for_list(ctx).await?;
        Ok(rows.into_iter().next())
    }

    pub(crate) async fn _execute_for_one<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        self._execute_for_first(ctx).await
    }


    pub(crate) async fn _execute_for_page<'a, C>(
        self,
        ctx: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let total_count = self.clone()._execute_for_count(ctx).await?;
        let mut rows = self.page_offset(offset, limit)._execute_for_list(ctx).await?;
        rows.total_count = Some(total_count);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_count<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<u64, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query;
        query.projection.clear();
        query.expr_projection.clear();
        query.order_by.clear();
        query.slice = None;
        query.relations.clear();
        query = query.count(COUNT_ALIAS);
        let rows = repository.fetch_all(&query).await?;
        rows.first()
            .and_then(|row| row.get(COUNT_ALIAS))
            .and_then(teaql_core::Value::try_u64)
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Merchant is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let query_options = self.query_options.clone();
        let outer_query = self.query.clone();
        let relation_aggregates = runtime_relation_aggregates(&query_options);
        let query = apply_runtime_metadata(self.query, &query_options, &self.child_enhancements);
        let mut rows = repository.fetch_smart_list_with_relation_aggregates(&query, &relation_aggregates).await?;
        let facets = execute_facets(ctx, &outer_query, &query_options)
            .await
            .map_err(DataServiceError::Runtime)?;
        attach_facets(&mut rows, facets);
        Ok(rows)
    }

    pub(crate) async fn _execute_for_record<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let records = self.limit(1)._execute_for_records(ctx).await?;
        Ok(records.into_iter().next())
    }

    pub fn search_with_text(mut self, text: impl Into<String>) -> Self {
        self.query = self.query.search_with_text(text);
        self
    }

    pub fn filter(mut self, filter: Expr) -> Self {
        self.query = self.query.filter(filter);
        self
    }

    pub fn and_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.and_filter(filter);
        self
    }

    pub fn or_filter(mut self, filter: Expr) -> Self {
        self.query = self.query.or_filter(filter);
        self
    }

    pub fn append_search_criteria(self, criteria: Expr) -> Self {
        self.and_filter(criteria)
    }

    pub fn filter_property(
        mut self,
        property1: impl AsRef<str>,
        operator: FieldOperator,
        property2: impl AsRef<str>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_column_expr(
            property1.as_ref(),
            operator,
            property2.as_ref(),
        ));
        self
    }

    pub fn with_deleted_rows(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self
    }

    pub fn deleted_rows_only(mut self) -> Self {
        self.query.filter = remove_default_live_filter(self.query.filter);
        self.query = self.query.and_filter(Expr::lte("version", 0_i64));
        self
    }

    pub fn match_types(
        mut self,
        types: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(TYPE_FIELD, types.into_iter().map(Into::into)));
        self
    }


    pub fn with_type_group(mut self) -> Self {
        self.query = self.query.project(TYPE_GROUP_FIELD);
        self
    }

    pub fn matching_any_of(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        let entity = EntityDescriptor::new(selection.query.entity.clone());
        self.query = self.query.and_filter(Expr::in_subquery("id", entity, selection.query.clone(), "id"));
        self
    }

    pub fn match_any_of(self, request: impl Into<QuerySelection>) -> Self {
        self.matching_any_of(request)
    }

    pub fn enhance_child(mut self, request: impl Into<QuerySelection>) -> Self {
        self.child_enhancements.push(request.into());
        self
    }

    pub fn enhance_children_if_needed(self) -> Self {
        let request = self;
        request
    }


    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.query_options.comment = Some(comment.into());
        self
    }

    pub fn raw_sql(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql = Some(raw_sql.into_sql());
        self
    }

    pub fn raw_sql_filter(self, raw_sql: impl Into<String>) -> Self {
        self.unsafe_raw_sql_filter(UnsafeRawSqlSegment::trusted(raw_sql))
    }

    pub fn unsafe_raw_sql_filter(mut self, raw_sql: UnsafeRawSqlSegment) -> Self {
        self.query_options.raw_sql_search_criteria.push(raw_sql.into_sql());
        self
    }
    pub fn filter_with_json(self, json_expr: impl Into<String>) -> Self {
        self.merge_dynamic_json_expr(json_expr.into())
    }

    fn merge_dynamic_json_expr(self, json_expr: String) -> Self {
        let json = serde_json::from_str::<JsonValue>(&json_expr)
            .unwrap_or_else(|_| panic!("Input JSON format error: {json_expr}"));
        self.merge_dynamic_json(&json)
    }

    fn merge_dynamic_json(mut self, json: &JsonValue) -> Self {
        let Some(object) = json.as_object() else {
            return self;
        };

        for (field, value) in object {
            if field.starts_with('_') {
                continue;
            }
            self = self.apply_dynamic_json_filter(field, value);
        }

        self = self.apply_dynamic_json_order_by(object.get("_orderBy"));

        if let Some(offset) = dynamic_json_u64_field(object, "_start") {
            self = self.skip(offset);
        }
        if let Some(size) = dynamic_json_u64_field(object, "_size") {
            self = self.limit(size);
        }

        if let Some(page_size) = dynamic_json_u64_field(object, "_pageSize") {
            self = self.limit(page_size);
        }
        if let Some(page_number) = dynamic_json_u64_field(object, "_page") {
            if page_number > 0 {
                let size = dynamic_json_u64_field(object, "_pageSize")
                    .or_else(|| self.query.slice.as_ref().and_then(|slice| slice.limit))
                    .unwrap_or(10);
                let offset = page_number.saturating_sub(1).saturating_mul(size);
                self = self.page_offset(offset, size);
            }
        }

        self
    }

    pub(crate) fn apply_dynamic_json_filter(self, field: &str, value: &JsonValue) -> Self {
        if let Some((head, tail)) = field.split_once('.') {
            self.apply_dynamic_json_chain_filter(head, tail, value)
        } else if let Some(storage_field) = Self::dynamic_json_self_field(field) {
            self.and_filter(dynamic_json_filter_expr(storage_field, value))
        } else {
            self
        }
    }

    fn apply_dynamic_json_order_by(mut self, order_by: Option<&JsonValue>) -> Self {
        match order_by {
            Some(JsonValue::String(field)) => {
                if let Some(storage_field) = Self::dynamic_json_self_field(field) {
                    self.query = self.query.order_desc(storage_field);
                }
            }
            Some(JsonValue::Object(order_by)) => {
                self = self.apply_dynamic_json_single_order_by(order_by);
            }
            Some(JsonValue::Array(order_bys)) => {
                for order_by in order_bys {
                    if let Some(order_by) = order_by.as_object() {
                        self = self.apply_dynamic_json_single_order_by(order_by);
                    }
                }
            }
            _ => {}
        }
        self
    }

    fn apply_dynamic_json_single_order_by(
        mut self,
        order_by: &serde_json::Map<String, JsonValue>,
    ) -> Self {
        let Some(field) = order_by.get("field").and_then(JsonValue::as_str) else {
            return self;
        };
        let Some(storage_field) = Self::dynamic_json_self_field(field) else {
            return self;
        };
        if order_by
            .get("useAsc")
            .and_then(JsonValue::as_bool)
            .unwrap_or(false)
        {
            self.query = self.query.order_asc(storage_field);
        } else {
            self.query = self.query.order_desc(storage_field);
        }
        self
    }

    fn dynamic_json_self_field(field: &str) -> Option<&'static str> {
        match field {
            "id" => Some("id"),
            "name" => Some("name"),
            "tax_id" => Some("tax_id"),
            "version" => Some("version"),
            "platform_ref" | "platform_ref_id" => Some("platform_ref_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "platform_ref" => {
                self.with_platform_ref_matching(
                    crate::Q::platforms_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "employee_list" => {
                self.with_employee_list_matching(
                    crate::Q::employees_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tenant_configuration_list" => {
                self.with_tenant_configuration_list_matching(
                    crate::Q::tenant_configurations_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "organization_unit_list" => {
                self.with_organization_unit_list_matching(
                    crate::Q::organization_units_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "department_hierarchy_list" => {
                self.with_department_hierarchy_list_matching(
                    crate::Q::department_hierarchies_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "branch_office_list" => {
                self.with_branch_office_list_matching(
                    crate::Q::branch_offices_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "move_order_list" => {
                self.with_move_order_list_matching(
                    crate::Q::move_orders_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "address_list" => {
                self.with_address_list_matching(
                    crate::Q::addresses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "crew_list" => {
                self.with_crew_list_matching(
                    crate::Q::crews_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "packing_material_list" => {
                self.with_packing_material_list_matching(
                    crate::Q::packing_materials_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "loading_zone_list" => {
                self.with_loading_zone_list_matching(
                    crate::Q::loading_zones_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "unloading_zone_list" => {
                self.with_unloading_zone_list_matching(
                    crate::Q::unloading_zones_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "route_optimization_rule_list" => {
                self.with_route_optimization_rule_list_matching(
                    crate::Q::route_optimization_rules_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "move_status_list" => {
                self.with_move_status_list_matching(
                    crate::Q::move_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "department_list" => {
                self.with_department_list_matching(
                    crate::Q::departments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "payroll_period_list" => {
                self.with_payroll_period_list_matching(
                    crate::Q::payroll_periods_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_list" => {
                self.with_customer_list_matching(
                    crate::Q::customers_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_segment_list" => {
                self.with_customer_segment_list_matching(
                    crate::Q::customer_segments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "product_list" => {
                self.with_product_list_matching(
                    crate::Q::products_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "service_list" => {
                self.with_service_list_matching(
                    crate::Q::services_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "price_list_list" => {
                self.with_price_list_list_matching(
                    crate::Q::price_lists_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "service_bundle_list" => {
                self.with_service_bundle_list_matching(
                    crate::Q::service_bundles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "storage_unit_list" => {
                self.with_storage_unit_list_matching(
                    crate::Q::storage_units_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "service_area_list" => {
                self.with_service_area_list_matching(
                    crate::Q::service_areas_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "inventory_item_list" => {
                self.with_inventory_item_list_matching(
                    crate::Q::inventory_items_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "service_category_list" => {
                self.with_service_category_list_matching(
                    crate::Q::service_categories_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "campaign_list" => {
                self.with_campaign_list_matching(
                    crate::Q::campaigns_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "marketing_channel_list" => {
                self.with_marketing_channel_list_matching(
                    crate::Q::marketing_channels_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "payment_list" => {
                self.with_payment_list_matching(
                    crate::Q::payments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "expense_list" => {
                self.with_expense_list_matching(
                    crate::Q::expenses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "vat_rate_list" => {
                self.with_vat_rate_list_matching(
                    crate::Q::vat_rates_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "account_list" => {
                self.with_account_list_matching(
                    crate::Q::accounts_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "financial_summary_list" => {
                self.with_financial_summary_list_matching(
                    crate::Q::financial_summaries_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "budget_list" => {
                self.with_budget_list_matching(
                    crate::Q::budgets_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "payable_list" => {
                self.with_payable_list_matching(
                    crate::Q::payables_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "currency_rate_list" => {
                self.with_currency_rate_list_matching(
                    crate::Q::currency_rates_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "payment_method_list" => {
                self.with_payment_method_list_matching(
                    crate::Q::payment_methods_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "financial_period_list" => {
                self.with_financial_period_list_matching(
                    crate::Q::financial_periods_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "vehicle_list" => {
                self.with_vehicle_list_matching(
                    crate::Q::vehicles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "equipment_list" => {
                self.with_equipment_list_matching(
                    crate::Q::equipment_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "consumable_list" => {
                self.with_consumable_list_matching(
                    crate::Q::consumables_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "supplier_list" => {
                self.with_supplier_list_matching(
                    crate::Q::suppliers_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "storage_location_list" => {
                self.with_storage_location_list_matching(
                    crate::Q::storage_locations_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "contract_list" => {
                self.with_contract_list_matching(
                    crate::Q::contracts_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "insurance_policy_list" => {
                self.with_insurance_policy_list_matching(
                    crate::Q::insurance_policies_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "compliance_check_list" => {
                self.with_compliance_check_list_matching(
                    crate::Q::compliance_checks_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "data_retention_policy_list" => {
                self.with_data_retention_policy_list_matching(
                    crate::Q::data_retention_policies_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "policy_document_list" => {
                self.with_policy_document_list_matching(
                    crate::Q::policy_documents_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "incident_report_list" => {
                self.with_incident_report_list_matching(
                    crate::Q::incident_reports_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "legal_entity_list" => {
                self.with_legal_entity_list_matching(
                    crate::Q::legal_entities_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "regulatory_requirement_list" => {
                self.with_regulatory_requirement_list_matching(
                    crate::Q::regulatory_requirements_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "compliance_certificate_list" => {
                self.with_compliance_certificate_list_matching(
                    crate::Q::compliance_certificates_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "role_list" => {
                self.with_role_list_matching(
                    crate::Q::roles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "permission_list" => {
                self.with_permission_list_matching(
                    crate::Q::permissions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "audit_log_list" => {
                self.with_audit_log_list_matching(
                    crate::Q::audit_logs_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "system_event_list" => {
                self.with_system_event_list_matching(
                    crate::Q::system_events_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "notification_template_list" => {
                self.with_notification_template_list_matching(
                    crate::Q::notification_templates_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "automation_rule_list" => {
                self.with_automation_rule_list_matching(
                    crate::Q::automation_rules_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "api_client_list" => {
                self.with_api_client_list_matching(
                    crate::Q::api_clients_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            _ => self,
        }
    }

    pub fn create_property_as(
        self,
        property_name: impl Into<String>,
        raw_sql_segment: impl Into<String>,
    ) -> Self {
        self.unsafe_create_property_as(property_name, UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn unsafe_create_property_as(
        mut self,
        property_name: impl Into<String>,
        raw_sql_segment: UnsafeRawSqlSegment,
    ) -> Self {
        self.query_options
            .dynamic_properties
            .push(RawDynamicProperty::new(property_name, raw_sql_segment));
        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.query = self.query.limit(limit);
        self
    }

    pub fn skip(mut self, offset: u64) -> Self {
        self.query = self.query.offset(offset);
        self
    }

    pub fn offset_only(self, offset: u64) -> Self {
        self.skip(offset)
    }

    pub fn offset(self, offset: u64, size: u64) -> Self {
        self.page_offset(offset, size)
    }

    pub fn page_offset(mut self, offset: u64, limit: u64) -> Self {
        self.query = self.query.page(offset, limit);
        self
    }

    pub fn top(self, top_n: u64) -> Self {
        self.limit(top_n)
    }

    pub fn offset_size(self, offset: u64, size: u64) -> Self {
        self.offset(offset, size)
    }

    pub fn unlimited(mut self) -> Self {
        self.query.slice = None;
        self
    }

    pub fn page_number(self, page_number: u64, page_size: u64) -> Self {
        let offset = page_number.saturating_sub(1).saturating_mul(page_size);
        self.page_offset(offset, page_size)
    }

    pub fn page_number_default(self, page_number: u64) -> Self {
        self.page_number(page_number, 10)
    }

    pub fn page(self, page_number: u64, page_size: u64) -> Self {
        self.page_number(page_number, page_size)
    }

    pub fn page_default(self, page_number: u64) -> Self {
        self.page_number_default(page_number)
    }

    pub fn select_self(mut self) -> Self {
        self.query = self.query.project("id");
        self.query = self.query.project("name");
        self.query = self.query.project("tax_id");
        self.query = self.query.project("version");
        self.query = self.query.project("platform_ref_id");
        self
    }

    pub fn select_self_fields(self) -> Self {
        self.select_self()
    }

    pub fn select_self_without_parent(self) -> Self {
        self.select_self_fields()
    }

    pub fn select_all(self) -> Self {
        let mut request = self.select_self();
        request = request.select_platform_ref();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_employee_list();
        request = request.select_tenant_configuration_list();
        request = request.select_organization_unit_list();
        request = request.select_department_hierarchy_list();
        request = request.select_branch_office_list();
        request = request.select_move_order_list();
        request = request.select_address_list();
        request = request.select_crew_list();
        request = request.select_packing_material_list();
        request = request.select_loading_zone_list();
        request = request.select_unloading_zone_list();
        request = request.select_route_optimization_rule_list();
        request = request.select_move_status_list();
        request = request.select_department_list();
        request = request.select_payroll_period_list();
        request = request.select_customer_list();
        request = request.select_customer_segment_list();
        request = request.select_product_list();
        request = request.select_service_list();
        request = request.select_price_list_list();
        request = request.select_service_bundle_list();
        request = request.select_storage_unit_list();
        request = request.select_service_area_list();
        request = request.select_inventory_item_list();
        request = request.select_service_category_list();
        request = request.select_campaign_list();
        request = request.select_marketing_channel_list();
        request = request.select_payment_list();
        request = request.select_expense_list();
        request = request.select_vat_rate_list();
        request = request.select_account_list();
        request = request.select_financial_summary_list();
        request = request.select_budget_list();
        request = request.select_payable_list();
        request = request.select_currency_rate_list();
        request = request.select_payment_method_list();
        request = request.select_financial_period_list();
        request = request.select_vehicle_list();
        request = request.select_equipment_list();
        request = request.select_consumable_list();
        request = request.select_supplier_list();
        request = request.select_storage_location_list();
        request = request.select_contract_list();
        request = request.select_insurance_policy_list();
        request = request.select_compliance_check_list();
        request = request.select_data_retention_policy_list();
        request = request.select_policy_document_list();
        request = request.select_incident_report_list();
        request = request.select_legal_entity_list();
        request = request.select_regulatory_requirement_list();
        request = request.select_compliance_certificate_list();
        request = request.select_role_list();
        request = request.select_permission_list();
        request = request.select_audit_log_list();
        request = request.select_system_event_list();
        request = request.select_notification_template_list();
        request = request.select_automation_rule_list();
        request = request.select_api_client_list();
        request
    }

    pub fn select_any(self) -> Self {
        self.select_children()
    }

    pub fn group_by(mut self, field: impl Into<String>) -> Self {
        self.query = self.query.group_by(field);
        self
    }

    pub fn aggregate_count(mut self, alias: impl Into<String>) -> Self {
        self.query = self.query.count(alias);
        self
    }

    pub fn aggregate_count_field(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.count_field(field, alias);
        self
    }

    pub fn aggregate_with_function(
        mut self,
        field: impl Into<String>,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.query = self.query.aggregate(Aggregate::new(function, field, alias));
        self
    }

    pub fn aggregate_sum(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.sum(field, alias);
        self
    }

    pub fn aggregate_avg(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.avg(field, alias);
        self
    }

    pub fn aggregate_min(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.min(field, alias);
        self
    }

    pub fn aggregate_max(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.max(field, alias);
        self
    }

    pub fn aggregate_stddev(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev(field, alias);
        self
    }

    pub fn aggregate_stddev_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.stddev_pop(field, alias);
        self
    }

    pub fn aggregate_var_samp(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_samp(field, alias);
        self
    }

    pub fn aggregate_var_pop(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.var_pop(field, alias);
        self
    }

    pub fn aggregate_bit_and(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_and(field, alias);
        self
    }

    pub fn aggregate_bit_or(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_or(field, alias);
        self
    }

    pub fn aggregate_bit_xor(mut self, field: impl Into<String>, alias: impl Into<String>) -> Self {
        self.query = self.query.bit_xor(field, alias);
        self
    }

    pub fn enable_aggregation_cache(mut self) -> Self {
        self.query = self.query.enable_aggregation_cache();
        self
    }

    pub fn enable_aggregation_cache_for(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.enable_aggregation_cache_for(cache_expired_millis);
        self
    }

    pub fn propagate_aggregation_cache(mut self, cache_expired_millis: u64) -> Self {
        self.query = self.query.propagate_aggregation_cache(cache_expired_millis);
        self
    }

    pub fn group_by_id(self) -> Self {
        self.group_by("id")
    }

    pub fn group_by_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("id"));
        request
    }

    pub fn group_by_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("id")
            .aggregate_with_function("id", alias, function)
    }

    pub fn count_id(self) -> Self {
        self.count_id_as("id_count")
    }

    pub fn count_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("id", alias)
    }

    pub fn sum_id(self) -> Self {
        self.sum_id_as("sum_id")
    }

    pub fn sum_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("id", alias)
    }

    pub fn avg_id(self) -> Self {
        self.avg_id_as("avg_id")
    }

    pub fn avg_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("id", alias)
    }

    pub fn min_id(self) -> Self {
        self.min_id_as("min_id")
    }

    pub fn min_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("id", alias)
    }

    pub fn max_id(self) -> Self {
        self.max_id_as("max_id")
    }

    pub fn max_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("id", alias)
    }


    pub fn with_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("id", value));
        self
    }



    pub fn with_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("id", value));
        self
    }

    pub fn with_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn order_by_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("id");
        self
    }

    pub fn order_by_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("id");
        self
    }

    pub fn order_by_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("id");
        self
    }

    pub fn order_by_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("id");
        self
    }


    pub fn select_name(mut self) -> Self {
        self.query = self.query.project("name");
        self
    }

    pub fn project_name(self) -> Self {
        self.select_name()
    }

    pub fn select_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("name", raw_sql_segment));
        self
    }

    pub fn group_by_name(self) -> Self {
        self.group_by("name")
    }

    pub fn group_by_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("name"));
        request
    }

    pub fn group_by_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("name")
            .aggregate_with_function("name", alias, function)
    }

    pub fn count_name(self) -> Self {
        self.count_name_as("name_count")
    }

    pub fn count_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("name", alias)
    }

    pub fn sum_name(self) -> Self {
        self.sum_name_as("sum_name")
    }

    pub fn sum_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("name", alias)
    }

    pub fn avg_name(self) -> Self {
        self.avg_name_as("avg_name")
    }

    pub fn avg_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("name", alias)
    }

    pub fn min_name(self) -> Self {
        self.min_name_as("min_name")
    }

    pub fn min_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("name", alias)
    }

    pub fn max_name(self) -> Self {
        self.max_name_as("max_name")
    }

    pub fn max_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("name", alias)
    }

    pub fn unselect_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "name");
        self
    }


    pub fn with_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("name", value));
        self
    }



    pub fn with_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("name", value));
        self
    }

    pub fn with_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("name", value));
        self
    }

    pub fn with_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("name", value));
        self
    }

    pub fn with_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("name", value));
        self
    }

    pub fn with_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("name", value));
        self
    }

    pub fn with_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("name", lower, upper));
        self
    }

    pub fn with_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("name", value));
        self
    }

    pub fn with_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("name", value));
        self
    }

    pub fn with_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("name", value));
        self
    }

    pub fn with_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("name", value));
        self
    }

    pub fn with_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("name", value));
        self
    }

    pub fn with_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("name", value));
        self
    }

    pub fn with_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("name", value));
        self
    }
    pub fn with_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("name", value));
        self
    }

    pub fn with_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("name", value));
        self
    }

    pub fn with_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("name"));
        self
    }



    pub fn with_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("name"));
        self
    }


    pub fn order_by_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("name");
        self
    }

    pub fn order_by_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("name");
        self
    }

    pub fn order_by_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("name");
        self
    }

    pub fn order_by_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("name");
        self
    }


    pub fn select_tax_id(mut self) -> Self {
        self.query = self.query.project("tax_id");
        self
    }

    pub fn project_tax_id(self) -> Self {
        self.select_tax_id()
    }

    pub fn select_tax_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_tax_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_tax_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("tax_id", raw_sql_segment));
        self
    }

    pub fn group_by_tax_id(self) -> Self {
        self.group_by("tax_id")
    }

    pub fn group_by_tax_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tax_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tax_id"));
        request
    }

    pub fn group_by_tax_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tax_id")
            .aggregate_with_function("tax_id", alias, function)
    }

    pub fn count_tax_id(self) -> Self {
        self.count_tax_id_as("tax_id_count")
    }

    pub fn count_tax_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tax_id", alias)
    }

    pub fn sum_tax_id(self) -> Self {
        self.sum_tax_id_as("sum_tax_id")
    }

    pub fn sum_tax_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("tax_id", alias)
    }

    pub fn avg_tax_id(self) -> Self {
        self.avg_tax_id_as("avg_tax_id")
    }

    pub fn avg_tax_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("tax_id", alias)
    }

    pub fn min_tax_id(self) -> Self {
        self.min_tax_id_as("min_tax_id")
    }

    pub fn min_tax_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("tax_id", alias)
    }

    pub fn max_tax_id(self) -> Self {
        self.max_tax_id_as("max_tax_id")
    }

    pub fn max_tax_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("tax_id", alias)
    }

    pub fn unselect_tax_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "tax_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "tax_id");
        self
    }


    pub fn with_tax_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "tax_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_tax_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "tax_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_tax_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("tax_id", value));
        self
    }



    pub fn with_tax_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("tax_id", value));
        self
    }

    pub fn with_tax_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tax_id", value));
        self
    }

    pub fn with_tax_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("tax_id", value));
        self
    }

    pub fn with_tax_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tax_id", value));
        self
    }

    pub fn with_tax_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("tax_id", value));
        self
    }

    pub fn with_tax_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("tax_id", lower, upper));
        self
    }

    pub fn with_tax_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "tax_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_tax_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "tax_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tax_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "tax_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tax_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("tax_id", value));
        self
    }

    pub fn with_tax_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("tax_id", value));
        self
    }

    pub fn with_tax_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("tax_id", value));
        self
    }

    pub fn with_tax_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("tax_id", value));
        self
    }

    pub fn with_tax_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("tax_id", value));
        self
    }

    pub fn with_tax_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("tax_id", value));
        self
    }

    pub fn with_tax_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("tax_id", value));
        self
    }
    pub fn with_tax_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tax_id", value));
        self
    }

    pub fn with_tax_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tax_id", value));
        self
    }

    pub fn with_tax_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tax_id"));
        self
    }



    pub fn with_tax_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tax_id"));
        self
    }


    pub fn order_by_tax_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("tax_id");
        self
    }

    pub fn order_by_tax_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("tax_id");
        self
    }

    pub fn order_by_tax_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("tax_id");
        self
    }

    pub fn order_by_tax_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("tax_id");
        self
    }

    pub fn group_by_version(self) -> Self {
        self.group_by("version")
    }

    pub fn group_by_version_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("version");
        request.query = request
            .query
            .project_expr(alias, Expr::column("version"));
        request
    }

    pub fn group_by_version_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("version")
            .aggregate_with_function("version", alias, function)
    }

    pub fn count_version(self) -> Self {
        self.count_version_as("version_count")
    }

    pub fn count_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("version", alias)
    }

    pub fn sum_version(self) -> Self {
        self.sum_version_as("sum_version")
    }

    pub fn sum_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("version", alias)
    }

    pub fn avg_version(self) -> Self {
        self.avg_version_as("avg_version")
    }

    pub fn avg_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("version", alias)
    }

    pub fn min_version(self) -> Self {
        self.min_version_as("min_version")
    }

    pub fn min_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("version", alias)
    }

    pub fn max_version(self) -> Self {
        self.max_version_as("max_version")
    }

    pub fn max_version_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("version", alias)
    }

    pub fn order_by_version_asc(mut self) -> Self {
        self.query = self.query.order_asc("version");
        self
    }

    pub fn order_by_version_desc(mut self) -> Self {
        self.query = self.query.order_desc("version");
        self
    }

    pub fn order_by_version_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("version");
        self
    }

    pub fn order_by_version_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("version");
        self
    }
    pub fn filter_by_platform_ref(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("platform_ref_id", value.entity_id_value()));
        self
    }

    pub fn with_platform_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "platform_ref_id",
            <crate::Platform as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("platform_ref", selection));
        self
    }


    pub fn without_platform_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "platform_ref_id",
            <crate::Platform as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("platform_ref", selection));
        self
    }


    pub fn have_platform_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("platform_ref_id"));
        self
    }

    pub fn have_no_platform_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("platform_ref_id"));
        self
    }


    pub fn group_by_platform_ref(self) -> Self {
        self.group_by("platform_ref_id")
    }

    pub fn group_by_platform_ref_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("platform_ref_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("platform_ref_id"));
        request
    }

    pub fn group_by_platform_ref_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("platform_ref_id")
            .aggregate_with_function("platform_ref_id", alias, function)
    }

    pub fn group_by_platform_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("platform_ref_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "platform_ref",
            "platform_ref_id",
            request,
        ));
        self
    }

    pub fn group_by_platform_ref_with_details(self) -> Self {
        self.group_by_platform_ref_with_details_from(crate::Q::platforms().unlimited())
    }

    pub fn group_by_platform_ref_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_platform_ref_with(request)
    }


    pub fn roll_up_to_platform_ref(self) -> Self {
        self.roll_up_to_platform_ref_with(crate::Q::platforms().unlimited())
    }

    pub fn roll_up_to_platform_ref_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_platform_ref_matching(selection.clone())
            .group_by_platform_ref_with(selection)
    }

    pub fn count_platform_ref(self) -> Self {
        self.count_platform_ref_as("platform_ref_count")
    }

    pub fn count_platform_ref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("platform_ref_id", alias)
    }

    pub fn unselect_platform_ref(mut self) -> Self {
        self.query.projection.retain(|field| field != "platform_ref_id");
        self.query.relations.retain(|relation| relation.name != "platform_ref");
        self
    }
    pub fn select_platform_ref(mut self) -> Self {
        self.query = self.query.relation("platform_ref");
        self
    }

    pub fn select_platform_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("platform_ref", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("platform_ref", selection));
        self
}

    pub fn facet_by_platform_ref_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_platform_ref_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_platform_ref_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "platform_ref",
            request,
            include_all_facets,
        ));
        self
    }
    pub fn have_employees(self) -> Self {
        self.with_employee_list_matching(SelectQuery::new("Employee"))
    }

    pub fn have_no_employees(self) -> Self {
        self.without_employee_list_matching(SelectQuery::new("Employee"))
    }

    pub fn with_employee_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Employee as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_list", selection));
        self
    }

    pub fn without_employee_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Employee as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_list", selection));
        self
    }

    pub fn select_employee_list(mut self) -> Self {
        self.query = self.query.relation("employee_list");
        self
    }

    pub fn select_employee_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("employee_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("employee_list", selection));
        self
}

    pub fn have_tenant_configurations(self) -> Self {
        self.with_tenant_configuration_list_matching(SelectQuery::new("TenantConfiguration"))
    }

    pub fn have_no_tenant_configurations(self) -> Self {
        self.without_tenant_configuration_list_matching(SelectQuery::new("TenantConfiguration"))
    }

    pub fn with_tenant_configuration_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TenantConfiguration as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("tenant_configuration_list", selection));
        self
    }

    pub fn without_tenant_configuration_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TenantConfiguration as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("tenant_configuration_list", selection));
        self
    }

    pub fn select_tenant_configuration_list(mut self) -> Self {
        self.query = self.query.relation("tenant_configuration_list");
        self
    }

    pub fn select_tenant_configuration_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tenant_configuration_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tenant_configuration_list", selection));
        self
}

    pub fn have_organization_units(self) -> Self {
        self.with_organization_unit_list_matching(SelectQuery::new("OrganizationUnit"))
    }

    pub fn have_no_organization_units(self) -> Self {
        self.without_organization_unit_list_matching(SelectQuery::new("OrganizationUnit"))
    }

    pub fn with_organization_unit_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::OrganizationUnit as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("organization_unit_list", selection));
        self
    }

    pub fn without_organization_unit_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::OrganizationUnit as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("organization_unit_list", selection));
        self
    }

    pub fn select_organization_unit_list(mut self) -> Self {
        self.query = self.query.relation("organization_unit_list");
        self
    }

    pub fn select_organization_unit_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("organization_unit_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("organization_unit_list", selection));
        self
}

    pub fn have_department_hierarchies(self) -> Self {
        self.with_department_hierarchy_list_matching(SelectQuery::new("DepartmentHierarchy"))
    }

    pub fn have_no_department_hierarchies(self) -> Self {
        self.without_department_hierarchy_list_matching(SelectQuery::new("DepartmentHierarchy"))
    }

    pub fn with_department_hierarchy_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DepartmentHierarchy as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("department_hierarchy_list", selection));
        self
    }

    pub fn without_department_hierarchy_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DepartmentHierarchy as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("department_hierarchy_list", selection));
        self
    }

    pub fn select_department_hierarchy_list(mut self) -> Self {
        self.query = self.query.relation("department_hierarchy_list");
        self
    }

    pub fn select_department_hierarchy_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("department_hierarchy_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("department_hierarchy_list", selection));
        self
}

    pub fn have_branch_offices(self) -> Self {
        self.with_branch_office_list_matching(SelectQuery::new("BranchOffice"))
    }

    pub fn have_no_branch_offices(self) -> Self {
        self.without_branch_office_list_matching(SelectQuery::new("BranchOffice"))
    }

    pub fn with_branch_office_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::BranchOffice as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("branch_office_list", selection));
        self
    }

    pub fn without_branch_office_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::BranchOffice as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("branch_office_list", selection));
        self
    }

    pub fn select_branch_office_list(mut self) -> Self {
        self.query = self.query.relation("branch_office_list");
        self
    }

    pub fn select_branch_office_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("branch_office_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("branch_office_list", selection));
        self
}

    pub fn have_move_orders(self) -> Self {
        self.with_move_order_list_matching(SelectQuery::new("MoveOrder"))
    }

    pub fn have_no_move_orders(self) -> Self {
        self.without_move_order_list_matching(SelectQuery::new("MoveOrder"))
    }

    pub fn with_move_order_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MoveOrder as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_order_list", selection));
        self
    }

    pub fn without_move_order_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MoveOrder as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_order_list", selection));
        self
    }

    pub fn select_move_order_list(mut self) -> Self {
        self.query = self.query.relation("move_order_list");
        self
    }

    pub fn select_move_order_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("move_order_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("move_order_list", selection));
        self
}

    pub fn have_addresses(self) -> Self {
        self.with_address_list_matching(SelectQuery::new("Address"))
    }

    pub fn have_no_addresses(self) -> Self {
        self.without_address_list_matching(SelectQuery::new("Address"))
    }

    pub fn with_address_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Address as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("address_list", selection));
        self
    }

    pub fn without_address_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Address as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("address_list", selection));
        self
    }

    pub fn select_address_list(mut self) -> Self {
        self.query = self.query.relation("address_list");
        self
    }

    pub fn select_address_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("address_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("address_list", selection));
        self
}

    pub fn have_crews(self) -> Self {
        self.with_crew_list_matching(SelectQuery::new("Crew"))
    }

    pub fn have_no_crews(self) -> Self {
        self.without_crew_list_matching(SelectQuery::new("Crew"))
    }

    pub fn with_crew_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Crew as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("crew_list", selection));
        self
    }

    pub fn without_crew_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Crew as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("crew_list", selection));
        self
    }

    pub fn select_crew_list(mut self) -> Self {
        self.query = self.query.relation("crew_list");
        self
    }

    pub fn select_crew_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("crew_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("crew_list", selection));
        self
}

    pub fn have_packing_materials(self) -> Self {
        self.with_packing_material_list_matching(SelectQuery::new("PackingMaterial"))
    }

    pub fn have_no_packing_materials(self) -> Self {
        self.without_packing_material_list_matching(SelectQuery::new("PackingMaterial"))
    }

    pub fn with_packing_material_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PackingMaterial as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("packing_material_list", selection));
        self
    }

    pub fn without_packing_material_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PackingMaterial as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("packing_material_list", selection));
        self
    }

    pub fn select_packing_material_list(mut self) -> Self {
        self.query = self.query.relation("packing_material_list");
        self
    }

    pub fn select_packing_material_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("packing_material_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("packing_material_list", selection));
        self
}

    pub fn have_loading_zones(self) -> Self {
        self.with_loading_zone_list_matching(SelectQuery::new("LoadingZone"))
    }

    pub fn have_no_loading_zones(self) -> Self {
        self.without_loading_zone_list_matching(SelectQuery::new("LoadingZone"))
    }

    pub fn with_loading_zone_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::LoadingZone as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("loading_zone_list", selection));
        self
    }

    pub fn without_loading_zone_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::LoadingZone as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("loading_zone_list", selection));
        self
    }

    pub fn select_loading_zone_list(mut self) -> Self {
        self.query = self.query.relation("loading_zone_list");
        self
    }

    pub fn select_loading_zone_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("loading_zone_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("loading_zone_list", selection));
        self
}

    pub fn have_unloading_zones(self) -> Self {
        self.with_unloading_zone_list_matching(SelectQuery::new("UnloadingZone"))
    }

    pub fn have_no_unloading_zones(self) -> Self {
        self.without_unloading_zone_list_matching(SelectQuery::new("UnloadingZone"))
    }

    pub fn with_unloading_zone_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::UnloadingZone as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("unloading_zone_list", selection));
        self
    }

    pub fn without_unloading_zone_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::UnloadingZone as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("unloading_zone_list", selection));
        self
    }

    pub fn select_unloading_zone_list(mut self) -> Self {
        self.query = self.query.relation("unloading_zone_list");
        self
    }

    pub fn select_unloading_zone_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("unloading_zone_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("unloading_zone_list", selection));
        self
}

    pub fn have_route_optimization_rules(self) -> Self {
        self.with_route_optimization_rule_list_matching(SelectQuery::new("RouteOptimizationRule"))
    }

    pub fn have_no_route_optimization_rules(self) -> Self {
        self.without_route_optimization_rule_list_matching(SelectQuery::new("RouteOptimizationRule"))
    }

    pub fn with_route_optimization_rule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::RouteOptimizationRule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("route_optimization_rule_list", selection));
        self
    }

    pub fn without_route_optimization_rule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::RouteOptimizationRule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("route_optimization_rule_list", selection));
        self
    }

    pub fn select_route_optimization_rule_list(mut self) -> Self {
        self.query = self.query.relation("route_optimization_rule_list");
        self
    }

    pub fn select_route_optimization_rule_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("route_optimization_rule_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("route_optimization_rule_list", selection));
        self
}

    pub fn have_move_statuses(self) -> Self {
        self.with_move_status_list_matching(SelectQuery::new("MoveStatus"))
    }

    pub fn have_no_move_statuses(self) -> Self {
        self.without_move_status_list_matching(SelectQuery::new("MoveStatus"))
    }

    pub fn with_move_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MoveStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_status_list", selection));
        self
    }

    pub fn without_move_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MoveStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_status_list", selection));
        self
    }

    pub fn select_move_status_list(mut self) -> Self {
        self.query = self.query.relation("move_status_list");
        self
    }

    pub fn select_move_status_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("move_status_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("move_status_list", selection));
        self
}

    pub fn have_departments(self) -> Self {
        self.with_department_list_matching(SelectQuery::new("Department"))
    }

    pub fn have_no_departments(self) -> Self {
        self.without_department_list_matching(SelectQuery::new("Department"))
    }

    pub fn with_department_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Department as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("department_list", selection));
        self
    }

    pub fn without_department_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Department as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("department_list", selection));
        self
    }

    pub fn select_department_list(mut self) -> Self {
        self.query = self.query.relation("department_list");
        self
    }

    pub fn select_department_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("department_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("department_list", selection));
        self
}

    pub fn have_payroll_periods(self) -> Self {
        self.with_payroll_period_list_matching(SelectQuery::new("PayrollPeriod"))
    }

    pub fn have_no_payroll_periods(self) -> Self {
        self.without_payroll_period_list_matching(SelectQuery::new("PayrollPeriod"))
    }

    pub fn with_payroll_period_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PayrollPeriod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_period_list", selection));
        self
    }

    pub fn without_payroll_period_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PayrollPeriod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_period_list", selection));
        self
    }

    pub fn select_payroll_period_list(mut self) -> Self {
        self.query = self.query.relation("payroll_period_list");
        self
    }

    pub fn select_payroll_period_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payroll_period_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payroll_period_list", selection));
        self
}

    pub fn have_customers(self) -> Self {
        self.with_customer_list_matching(SelectQuery::new("Customer"))
    }

    pub fn have_no_customers(self) -> Self {
        self.without_customer_list_matching(SelectQuery::new("Customer"))
    }

    pub fn with_customer_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Customer as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_list", selection));
        self
    }

    pub fn without_customer_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Customer as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_list", selection));
        self
    }

    pub fn select_customer_list(mut self) -> Self {
        self.query = self.query.relation("customer_list");
        self
    }

    pub fn select_customer_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_list", selection));
        self
}

    pub fn have_customer_segments(self) -> Self {
        self.with_customer_segment_list_matching(SelectQuery::new("CustomerSegment"))
    }

    pub fn have_no_customer_segments(self) -> Self {
        self.without_customer_segment_list_matching(SelectQuery::new("CustomerSegment"))
    }

    pub fn with_customer_segment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CustomerSegment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_segment_list", selection));
        self
    }

    pub fn without_customer_segment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CustomerSegment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_segment_list", selection));
        self
    }

    pub fn select_customer_segment_list(mut self) -> Self {
        self.query = self.query.relation("customer_segment_list");
        self
    }

    pub fn select_customer_segment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_segment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_segment_list", selection));
        self
}

    pub fn have_products(self) -> Self {
        self.with_product_list_matching(SelectQuery::new("Product"))
    }

    pub fn have_no_products(self) -> Self {
        self.without_product_list_matching(SelectQuery::new("Product"))
    }

    pub fn with_product_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Product as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("product_list", selection));
        self
    }

    pub fn without_product_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Product as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("product_list", selection));
        self
    }

    pub fn select_product_list(mut self) -> Self {
        self.query = self.query.relation("product_list");
        self
    }

    pub fn select_product_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("product_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("product_list", selection));
        self
}

    pub fn have_services(self) -> Self {
        self.with_service_list_matching(SelectQuery::new("Service"))
    }

    pub fn have_no_services(self) -> Self {
        self.without_service_list_matching(SelectQuery::new("Service"))
    }

    pub fn with_service_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Service as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_list", selection));
        self
    }

    pub fn without_service_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Service as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_list", selection));
        self
    }

    pub fn select_service_list(mut self) -> Self {
        self.query = self.query.relation("service_list");
        self
    }

    pub fn select_service_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("service_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("service_list", selection));
        self
}

    pub fn have_price_lists(self) -> Self {
        self.with_price_list_list_matching(SelectQuery::new("PriceList"))
    }

    pub fn have_no_price_lists(self) -> Self {
        self.without_price_list_list_matching(SelectQuery::new("PriceList"))
    }

    pub fn with_price_list_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PriceList as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("price_list_list", selection));
        self
    }

    pub fn without_price_list_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PriceList as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("price_list_list", selection));
        self
    }

    pub fn select_price_list_list(mut self) -> Self {
        self.query = self.query.relation("price_list_list");
        self
    }

    pub fn select_price_list_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("price_list_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("price_list_list", selection));
        self
}

    pub fn have_service_bundles(self) -> Self {
        self.with_service_bundle_list_matching(SelectQuery::new("ServiceBundle"))
    }

    pub fn have_no_service_bundles(self) -> Self {
        self.without_service_bundle_list_matching(SelectQuery::new("ServiceBundle"))
    }

    pub fn with_service_bundle_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ServiceBundle as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_bundle_list", selection));
        self
    }

    pub fn without_service_bundle_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ServiceBundle as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_bundle_list", selection));
        self
    }

    pub fn select_service_bundle_list(mut self) -> Self {
        self.query = self.query.relation("service_bundle_list");
        self
    }

    pub fn select_service_bundle_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("service_bundle_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("service_bundle_list", selection));
        self
}

    pub fn have_storage_units(self) -> Self {
        self.with_storage_unit_list_matching(SelectQuery::new("StorageUnit"))
    }

    pub fn have_no_storage_units(self) -> Self {
        self.without_storage_unit_list_matching(SelectQuery::new("StorageUnit"))
    }

    pub fn with_storage_unit_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::StorageUnit as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("storage_unit_list", selection));
        self
    }

    pub fn without_storage_unit_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::StorageUnit as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("storage_unit_list", selection));
        self
    }

    pub fn select_storage_unit_list(mut self) -> Self {
        self.query = self.query.relation("storage_unit_list");
        self
    }

    pub fn select_storage_unit_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("storage_unit_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("storage_unit_list", selection));
        self
}

    pub fn have_service_areas(self) -> Self {
        self.with_service_area_list_matching(SelectQuery::new("ServiceArea"))
    }

    pub fn have_no_service_areas(self) -> Self {
        self.without_service_area_list_matching(SelectQuery::new("ServiceArea"))
    }

    pub fn with_service_area_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ServiceArea as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_area_list", selection));
        self
    }

    pub fn without_service_area_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ServiceArea as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_area_list", selection));
        self
    }

    pub fn select_service_area_list(mut self) -> Self {
        self.query = self.query.relation("service_area_list");
        self
    }

    pub fn select_service_area_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("service_area_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("service_area_list", selection));
        self
}

    pub fn have_inventory_items(self) -> Self {
        self.with_inventory_item_list_matching(SelectQuery::new("InventoryItem"))
    }

    pub fn have_no_inventory_items(self) -> Self {
        self.without_inventory_item_list_matching(SelectQuery::new("InventoryItem"))
    }

    pub fn with_inventory_item_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::InventoryItem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("inventory_item_list", selection));
        self
    }

    pub fn without_inventory_item_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::InventoryItem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("inventory_item_list", selection));
        self
    }

    pub fn select_inventory_item_list(mut self) -> Self {
        self.query = self.query.relation("inventory_item_list");
        self
    }

    pub fn select_inventory_item_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("inventory_item_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("inventory_item_list", selection));
        self
}

    pub fn have_service_categories(self) -> Self {
        self.with_service_category_list_matching(SelectQuery::new("ServiceCategory"))
    }

    pub fn have_no_service_categories(self) -> Self {
        self.without_service_category_list_matching(SelectQuery::new("ServiceCategory"))
    }

    pub fn with_service_category_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ServiceCategory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_category_list", selection));
        self
    }

    pub fn without_service_category_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ServiceCategory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_category_list", selection));
        self
    }

    pub fn select_service_category_list(mut self) -> Self {
        self.query = self.query.relation("service_category_list");
        self
    }

    pub fn select_service_category_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("service_category_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("service_category_list", selection));
        self
}

    pub fn have_campaigns(self) -> Self {
        self.with_campaign_list_matching(SelectQuery::new("Campaign"))
    }

    pub fn have_no_campaigns(self) -> Self {
        self.without_campaign_list_matching(SelectQuery::new("Campaign"))
    }

    pub fn with_campaign_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Campaign as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("campaign_list", selection));
        self
    }

    pub fn without_campaign_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Campaign as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("campaign_list", selection));
        self
    }

    pub fn select_campaign_list(mut self) -> Self {
        self.query = self.query.relation("campaign_list");
        self
    }

    pub fn select_campaign_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("campaign_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("campaign_list", selection));
        self
}

    pub fn have_marketing_channels(self) -> Self {
        self.with_marketing_channel_list_matching(SelectQuery::new("MarketingChannel"))
    }

    pub fn have_no_marketing_channels(self) -> Self {
        self.without_marketing_channel_list_matching(SelectQuery::new("MarketingChannel"))
    }

    pub fn with_marketing_channel_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MarketingChannel as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("marketing_channel_list", selection));
        self
    }

    pub fn without_marketing_channel_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MarketingChannel as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("marketing_channel_list", selection));
        self
    }

    pub fn select_marketing_channel_list(mut self) -> Self {
        self.query = self.query.relation("marketing_channel_list");
        self
    }

    pub fn select_marketing_channel_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("marketing_channel_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("marketing_channel_list", selection));
        self
}

    pub fn have_payments(self) -> Self {
        self.with_payment_list_matching(SelectQuery::new("Payment"))
    }

    pub fn have_no_payments(self) -> Self {
        self.without_payment_list_matching(SelectQuery::new("Payment"))
    }

    pub fn with_payment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Payment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payment_list", selection));
        self
    }

    pub fn without_payment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Payment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payment_list", selection));
        self
    }

    pub fn select_payment_list(mut self) -> Self {
        self.query = self.query.relation("payment_list");
        self
    }

    pub fn select_payment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payment_list", selection));
        self
}

    pub fn have_expenses(self) -> Self {
        self.with_expense_list_matching(SelectQuery::new("Expense"))
    }

    pub fn have_no_expenses(self) -> Self {
        self.without_expense_list_matching(SelectQuery::new("Expense"))
    }

    pub fn with_expense_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Expense as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("expense_list", selection));
        self
    }

    pub fn without_expense_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Expense as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("expense_list", selection));
        self
    }

    pub fn select_expense_list(mut self) -> Self {
        self.query = self.query.relation("expense_list");
        self
    }

    pub fn select_expense_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("expense_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("expense_list", selection));
        self
}

    pub fn have_vat_rates(self) -> Self {
        self.with_vat_rate_list_matching(SelectQuery::new("VatRate"))
    }

    pub fn have_no_vat_rates(self) -> Self {
        self.without_vat_rate_list_matching(SelectQuery::new("VatRate"))
    }

    pub fn with_vat_rate_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::VatRate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("vat_rate_list", selection));
        self
    }

    pub fn without_vat_rate_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::VatRate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("vat_rate_list", selection));
        self
    }

    pub fn select_vat_rate_list(mut self) -> Self {
        self.query = self.query.relation("vat_rate_list");
        self
    }

    pub fn select_vat_rate_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("vat_rate_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("vat_rate_list", selection));
        self
}

    pub fn have_accounts(self) -> Self {
        self.with_account_list_matching(SelectQuery::new("Account"))
    }

    pub fn have_no_accounts(self) -> Self {
        self.without_account_list_matching(SelectQuery::new("Account"))
    }

    pub fn with_account_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Account as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("account_list", selection));
        self
    }

    pub fn without_account_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Account as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("account_list", selection));
        self
    }

    pub fn select_account_list(mut self) -> Self {
        self.query = self.query.relation("account_list");
        self
    }

    pub fn select_account_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("account_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("account_list", selection));
        self
}

    pub fn have_financial_summaries(self) -> Self {
        self.with_financial_summary_list_matching(SelectQuery::new("FinancialSummary"))
    }

    pub fn have_no_financial_summaries(self) -> Self {
        self.without_financial_summary_list_matching(SelectQuery::new("FinancialSummary"))
    }

    pub fn with_financial_summary_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::FinancialSummary as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("financial_summary_list", selection));
        self
    }

    pub fn without_financial_summary_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::FinancialSummary as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("financial_summary_list", selection));
        self
    }

    pub fn select_financial_summary_list(mut self) -> Self {
        self.query = self.query.relation("financial_summary_list");
        self
    }

    pub fn select_financial_summary_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("financial_summary_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("financial_summary_list", selection));
        self
}

    pub fn have_budgets(self) -> Self {
        self.with_budget_list_matching(SelectQuery::new("Budget"))
    }

    pub fn have_no_budgets(self) -> Self {
        self.without_budget_list_matching(SelectQuery::new("Budget"))
    }

    pub fn with_budget_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Budget as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("budget_list", selection));
        self
    }

    pub fn without_budget_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Budget as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("budget_list", selection));
        self
    }

    pub fn select_budget_list(mut self) -> Self {
        self.query = self.query.relation("budget_list");
        self
    }

    pub fn select_budget_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("budget_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("budget_list", selection));
        self
}

    pub fn have_payables(self) -> Self {
        self.with_payable_list_matching(SelectQuery::new("Payable"))
    }

    pub fn have_no_payables(self) -> Self {
        self.without_payable_list_matching(SelectQuery::new("Payable"))
    }

    pub fn with_payable_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Payable as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payable_list", selection));
        self
    }

    pub fn without_payable_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Payable as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payable_list", selection));
        self
    }

    pub fn select_payable_list(mut self) -> Self {
        self.query = self.query.relation("payable_list");
        self
    }

    pub fn select_payable_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payable_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payable_list", selection));
        self
}

    pub fn have_currency_rates(self) -> Self {
        self.with_currency_rate_list_matching(SelectQuery::new("CurrencyRate"))
    }

    pub fn have_no_currency_rates(self) -> Self {
        self.without_currency_rate_list_matching(SelectQuery::new("CurrencyRate"))
    }

    pub fn with_currency_rate_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CurrencyRate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("currency_rate_list", selection));
        self
    }

    pub fn without_currency_rate_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CurrencyRate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("currency_rate_list", selection));
        self
    }

    pub fn select_currency_rate_list(mut self) -> Self {
        self.query = self.query.relation("currency_rate_list");
        self
    }

    pub fn select_currency_rate_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("currency_rate_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("currency_rate_list", selection));
        self
}

    pub fn have_payment_methods(self) -> Self {
        self.with_payment_method_list_matching(SelectQuery::new("PaymentMethod"))
    }

    pub fn have_no_payment_methods(self) -> Self {
        self.without_payment_method_list_matching(SelectQuery::new("PaymentMethod"))
    }

    pub fn with_payment_method_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PaymentMethod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payment_method_list", selection));
        self
    }

    pub fn without_payment_method_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PaymentMethod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payment_method_list", selection));
        self
    }

    pub fn select_payment_method_list(mut self) -> Self {
        self.query = self.query.relation("payment_method_list");
        self
    }

    pub fn select_payment_method_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payment_method_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payment_method_list", selection));
        self
}

    pub fn have_financial_periods(self) -> Self {
        self.with_financial_period_list_matching(SelectQuery::new("FinancialPeriod"))
    }

    pub fn have_no_financial_periods(self) -> Self {
        self.without_financial_period_list_matching(SelectQuery::new("FinancialPeriod"))
    }

    pub fn with_financial_period_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::FinancialPeriod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("financial_period_list", selection));
        self
    }

    pub fn without_financial_period_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::FinancialPeriod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("financial_period_list", selection));
        self
    }

    pub fn select_financial_period_list(mut self) -> Self {
        self.query = self.query.relation("financial_period_list");
        self
    }

    pub fn select_financial_period_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("financial_period_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("financial_period_list", selection));
        self
}

    pub fn have_vehicles(self) -> Self {
        self.with_vehicle_list_matching(SelectQuery::new("Vehicle"))
    }

    pub fn have_no_vehicles(self) -> Self {
        self.without_vehicle_list_matching(SelectQuery::new("Vehicle"))
    }

    pub fn with_vehicle_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Vehicle as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("vehicle_list", selection));
        self
    }

    pub fn without_vehicle_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Vehicle as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("vehicle_list", selection));
        self
    }

    pub fn select_vehicle_list(mut self) -> Self {
        self.query = self.query.relation("vehicle_list");
        self
    }

    pub fn select_vehicle_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("vehicle_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("vehicle_list", selection));
        self
}

    pub fn have_equipment(self) -> Self {
        self.with_equipment_list_matching(SelectQuery::new("Equipment"))
    }

    pub fn have_no_equipment(self) -> Self {
        self.without_equipment_list_matching(SelectQuery::new("Equipment"))
    }

    pub fn with_equipment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Equipment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("equipment_list", selection));
        self
    }

    pub fn without_equipment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Equipment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("equipment_list", selection));
        self
    }

    pub fn select_equipment_list(mut self) -> Self {
        self.query = self.query.relation("equipment_list");
        self
    }

    pub fn select_equipment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("equipment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("equipment_list", selection));
        self
}

    pub fn have_consumables(self) -> Self {
        self.with_consumable_list_matching(SelectQuery::new("Consumable"))
    }

    pub fn have_no_consumables(self) -> Self {
        self.without_consumable_list_matching(SelectQuery::new("Consumable"))
    }

    pub fn with_consumable_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Consumable as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("consumable_list", selection));
        self
    }

    pub fn without_consumable_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Consumable as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("consumable_list", selection));
        self
    }

    pub fn select_consumable_list(mut self) -> Self {
        self.query = self.query.relation("consumable_list");
        self
    }

    pub fn select_consumable_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("consumable_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("consumable_list", selection));
        self
}

    pub fn have_suppliers(self) -> Self {
        self.with_supplier_list_matching(SelectQuery::new("Supplier"))
    }

    pub fn have_no_suppliers(self) -> Self {
        self.without_supplier_list_matching(SelectQuery::new("Supplier"))
    }

    pub fn with_supplier_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Supplier as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("supplier_list", selection));
        self
    }

    pub fn without_supplier_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Supplier as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("supplier_list", selection));
        self
    }

    pub fn select_supplier_list(mut self) -> Self {
        self.query = self.query.relation("supplier_list");
        self
    }

    pub fn select_supplier_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("supplier_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("supplier_list", selection));
        self
}

    pub fn have_storage_locations(self) -> Self {
        self.with_storage_location_list_matching(SelectQuery::new("StorageLocation"))
    }

    pub fn have_no_storage_locations(self) -> Self {
        self.without_storage_location_list_matching(SelectQuery::new("StorageLocation"))
    }

    pub fn with_storage_location_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::StorageLocation as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("storage_location_list", selection));
        self
    }

    pub fn without_storage_location_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::StorageLocation as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("storage_location_list", selection));
        self
    }

    pub fn select_storage_location_list(mut self) -> Self {
        self.query = self.query.relation("storage_location_list");
        self
    }

    pub fn select_storage_location_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("storage_location_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("storage_location_list", selection));
        self
}

    pub fn have_contracts(self) -> Self {
        self.with_contract_list_matching(SelectQuery::new("Contract"))
    }

    pub fn have_no_contracts(self) -> Self {
        self.without_contract_list_matching(SelectQuery::new("Contract"))
    }

    pub fn with_contract_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Contract as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("contract_list", selection));
        self
    }

    pub fn without_contract_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Contract as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("contract_list", selection));
        self
    }

    pub fn select_contract_list(mut self) -> Self {
        self.query = self.query.relation("contract_list");
        self
    }

    pub fn select_contract_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("contract_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("contract_list", selection));
        self
}

    pub fn have_insurance_policies(self) -> Self {
        self.with_insurance_policy_list_matching(SelectQuery::new("InsurancePolicy"))
    }

    pub fn have_no_insurance_policies(self) -> Self {
        self.without_insurance_policy_list_matching(SelectQuery::new("InsurancePolicy"))
    }

    pub fn with_insurance_policy_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::InsurancePolicy as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("insurance_policy_list", selection));
        self
    }

    pub fn without_insurance_policy_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::InsurancePolicy as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("insurance_policy_list", selection));
        self
    }

    pub fn select_insurance_policy_list(mut self) -> Self {
        self.query = self.query.relation("insurance_policy_list");
        self
    }

    pub fn select_insurance_policy_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("insurance_policy_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("insurance_policy_list", selection));
        self
}

    pub fn have_compliance_checks(self) -> Self {
        self.with_compliance_check_list_matching(SelectQuery::new("ComplianceCheck"))
    }

    pub fn have_no_compliance_checks(self) -> Self {
        self.without_compliance_check_list_matching(SelectQuery::new("ComplianceCheck"))
    }

    pub fn with_compliance_check_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ComplianceCheck as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("compliance_check_list", selection));
        self
    }

    pub fn without_compliance_check_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ComplianceCheck as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("compliance_check_list", selection));
        self
    }

    pub fn select_compliance_check_list(mut self) -> Self {
        self.query = self.query.relation("compliance_check_list");
        self
    }

    pub fn select_compliance_check_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("compliance_check_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("compliance_check_list", selection));
        self
}

    pub fn have_data_retention_policies(self) -> Self {
        self.with_data_retention_policy_list_matching(SelectQuery::new("DataRetentionPolicy"))
    }

    pub fn have_no_data_retention_policies(self) -> Self {
        self.without_data_retention_policy_list_matching(SelectQuery::new("DataRetentionPolicy"))
    }

    pub fn with_data_retention_policy_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DataRetentionPolicy as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("data_retention_policy_list", selection));
        self
    }

    pub fn without_data_retention_policy_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DataRetentionPolicy as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("data_retention_policy_list", selection));
        self
    }

    pub fn select_data_retention_policy_list(mut self) -> Self {
        self.query = self.query.relation("data_retention_policy_list");
        self
    }

    pub fn select_data_retention_policy_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("data_retention_policy_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("data_retention_policy_list", selection));
        self
}

    pub fn have_policy_documents(self) -> Self {
        self.with_policy_document_list_matching(SelectQuery::new("PolicyDocument"))
    }

    pub fn have_no_policy_documents(self) -> Self {
        self.without_policy_document_list_matching(SelectQuery::new("PolicyDocument"))
    }

    pub fn with_policy_document_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PolicyDocument as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("policy_document_list", selection));
        self
    }

    pub fn without_policy_document_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PolicyDocument as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("policy_document_list", selection));
        self
    }

    pub fn select_policy_document_list(mut self) -> Self {
        self.query = self.query.relation("policy_document_list");
        self
    }

    pub fn select_policy_document_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("policy_document_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("policy_document_list", selection));
        self
}

    pub fn have_incident_reports(self) -> Self {
        self.with_incident_report_list_matching(SelectQuery::new("IncidentReport"))
    }

    pub fn have_no_incident_reports(self) -> Self {
        self.without_incident_report_list_matching(SelectQuery::new("IncidentReport"))
    }

    pub fn with_incident_report_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::IncidentReport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("incident_report_list", selection));
        self
    }

    pub fn without_incident_report_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::IncidentReport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("incident_report_list", selection));
        self
    }

    pub fn select_incident_report_list(mut self) -> Self {
        self.query = self.query.relation("incident_report_list");
        self
    }

    pub fn select_incident_report_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("incident_report_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("incident_report_list", selection));
        self
}

    pub fn have_legal_entities(self) -> Self {
        self.with_legal_entity_list_matching(SelectQuery::new("LegalEntity"))
    }

    pub fn have_no_legal_entities(self) -> Self {
        self.without_legal_entity_list_matching(SelectQuery::new("LegalEntity"))
    }

    pub fn with_legal_entity_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::LegalEntity as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("legal_entity_list", selection));
        self
    }

    pub fn without_legal_entity_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::LegalEntity as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("legal_entity_list", selection));
        self
    }

    pub fn select_legal_entity_list(mut self) -> Self {
        self.query = self.query.relation("legal_entity_list");
        self
    }

    pub fn select_legal_entity_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("legal_entity_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("legal_entity_list", selection));
        self
}

    pub fn have_regulatory_requirements(self) -> Self {
        self.with_regulatory_requirement_list_matching(SelectQuery::new("RegulatoryRequirement"))
    }

    pub fn have_no_regulatory_requirements(self) -> Self {
        self.without_regulatory_requirement_list_matching(SelectQuery::new("RegulatoryRequirement"))
    }

    pub fn with_regulatory_requirement_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::RegulatoryRequirement as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("regulatory_requirement_list", selection));
        self
    }

    pub fn without_regulatory_requirement_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::RegulatoryRequirement as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("regulatory_requirement_list", selection));
        self
    }

    pub fn select_regulatory_requirement_list(mut self) -> Self {
        self.query = self.query.relation("regulatory_requirement_list");
        self
    }

    pub fn select_regulatory_requirement_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("regulatory_requirement_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("regulatory_requirement_list", selection));
        self
}

    pub fn have_compliance_certificates(self) -> Self {
        self.with_compliance_certificate_list_matching(SelectQuery::new("ComplianceCertificate"))
    }

    pub fn have_no_compliance_certificates(self) -> Self {
        self.without_compliance_certificate_list_matching(SelectQuery::new("ComplianceCertificate"))
    }

    pub fn with_compliance_certificate_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ComplianceCertificate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("compliance_certificate_list", selection));
        self
    }

    pub fn without_compliance_certificate_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ComplianceCertificate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("compliance_certificate_list", selection));
        self
    }

    pub fn select_compliance_certificate_list(mut self) -> Self {
        self.query = self.query.relation("compliance_certificate_list");
        self
    }

    pub fn select_compliance_certificate_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("compliance_certificate_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("compliance_certificate_list", selection));
        self
}

    pub fn have_roles(self) -> Self {
        self.with_role_list_matching(SelectQuery::new("Role"))
    }

    pub fn have_no_roles(self) -> Self {
        self.without_role_list_matching(SelectQuery::new("Role"))
    }

    pub fn with_role_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Role as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("role_list", selection));
        self
    }

    pub fn without_role_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Role as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("role_list", selection));
        self
    }

    pub fn select_role_list(mut self) -> Self {
        self.query = self.query.relation("role_list");
        self
    }

    pub fn select_role_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("role_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("role_list", selection));
        self
}

    pub fn have_permissions(self) -> Self {
        self.with_permission_list_matching(SelectQuery::new("Permission"))
    }

    pub fn have_no_permissions(self) -> Self {
        self.without_permission_list_matching(SelectQuery::new("Permission"))
    }

    pub fn with_permission_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Permission as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("permission_list", selection));
        self
    }

    pub fn without_permission_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Permission as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("permission_list", selection));
        self
    }

    pub fn select_permission_list(mut self) -> Self {
        self.query = self.query.relation("permission_list");
        self
    }

    pub fn select_permission_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("permission_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("permission_list", selection));
        self
}

    pub fn have_audit_logs(self) -> Self {
        self.with_audit_log_list_matching(SelectQuery::new("AuditLog"))
    }

    pub fn have_no_audit_logs(self) -> Self {
        self.without_audit_log_list_matching(SelectQuery::new("AuditLog"))
    }

    pub fn with_audit_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AuditLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("audit_log_list", selection));
        self
    }

    pub fn without_audit_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AuditLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("audit_log_list", selection));
        self
    }

    pub fn select_audit_log_list(mut self) -> Self {
        self.query = self.query.relation("audit_log_list");
        self
    }

    pub fn select_audit_log_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("audit_log_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("audit_log_list", selection));
        self
}

    pub fn have_system_events(self) -> Self {
        self.with_system_event_list_matching(SelectQuery::new("SystemEvent"))
    }

    pub fn have_no_system_events(self) -> Self {
        self.without_system_event_list_matching(SelectQuery::new("SystemEvent"))
    }

    pub fn with_system_event_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::SystemEvent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("system_event_list", selection));
        self
    }

    pub fn without_system_event_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::SystemEvent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("system_event_list", selection));
        self
    }

    pub fn select_system_event_list(mut self) -> Self {
        self.query = self.query.relation("system_event_list");
        self
    }

    pub fn select_system_event_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("system_event_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("system_event_list", selection));
        self
}

    pub fn have_notification_templates(self) -> Self {
        self.with_notification_template_list_matching(SelectQuery::new("NotificationTemplate"))
    }

    pub fn have_no_notification_templates(self) -> Self {
        self.without_notification_template_list_matching(SelectQuery::new("NotificationTemplate"))
    }

    pub fn with_notification_template_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::NotificationTemplate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("notification_template_list", selection));
        self
    }

    pub fn without_notification_template_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::NotificationTemplate as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("notification_template_list", selection));
        self
    }

    pub fn select_notification_template_list(mut self) -> Self {
        self.query = self.query.relation("notification_template_list");
        self
    }

    pub fn select_notification_template_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("notification_template_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("notification_template_list", selection));
        self
}

    pub fn have_automation_rules(self) -> Self {
        self.with_automation_rule_list_matching(SelectQuery::new("AutomationRule"))
    }

    pub fn have_no_automation_rules(self) -> Self {
        self.without_automation_rule_list_matching(SelectQuery::new("AutomationRule"))
    }

    pub fn with_automation_rule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AutomationRule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("automation_rule_list", selection));
        self
    }

    pub fn without_automation_rule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AutomationRule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("automation_rule_list", selection));
        self
    }

    pub fn select_automation_rule_list(mut self) -> Self {
        self.query = self.query.relation("automation_rule_list");
        self
    }

    pub fn select_automation_rule_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("automation_rule_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("automation_rule_list", selection));
        self
}

    pub fn have_api_clients(self) -> Self {
        self.with_api_client_list_matching(SelectQuery::new("ApiClient"))
    }

    pub fn have_no_api_clients(self) -> Self {
        self.without_api_client_list_matching(SelectQuery::new("ApiClient"))
    }

    pub fn with_api_client_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ApiClient as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("api_client_list", selection));
        self
    }

    pub fn without_api_client_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ApiClient as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("api_client_list", selection));
        self
    }

    pub fn select_api_client_list(mut self) -> Self {
        self.query = self.query.relation("api_client_list");
        self
    }

    pub fn select_api_client_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("api_client_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("api_client_list", selection));
        self
}
    pub fn count_employees(self) -> Self {
        self.count_employees_as("count_employees")
    }

    pub fn count_employees_as(self, alias: impl Into<String>) -> Self {
        self.count_employees_with(alias, crate::Q::employees().unlimited())
    }

    pub fn count_employees_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "employee_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_employees(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees_as("refinements", request)
    }

    pub fn stats_from_employees_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "employee_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_employees_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees(request)
    }




    pub fn count_tenant_configurations(self) -> Self {
        self.count_tenant_configurations_as("count_tenant_configurations")
    }

    pub fn count_tenant_configurations_as(self, alias: impl Into<String>) -> Self {
        self.count_tenant_configurations_with(alias, crate::Q::tenant_configurations().unlimited())
    }

    pub fn count_tenant_configurations_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tenant_configuration_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_tenant_configurations(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tenant_configurations_as("refinements", request)
    }

    pub fn stats_from_tenant_configurations_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tenant_configuration_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_tenant_configurations_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tenant_configurations(request)
    }




    pub fn count_organization_units(self) -> Self {
        self.count_organization_units_as("count_organization_units")
    }

    pub fn count_organization_units_as(self, alias: impl Into<String>) -> Self {
        self.count_organization_units_with(alias, crate::Q::organization_units().unlimited())
    }

    pub fn count_organization_units_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "organization_unit_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_organization_units(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_organization_units_as("refinements", request)
    }

    pub fn stats_from_organization_units_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "organization_unit_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_organization_units_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_organization_units(request)
    }




    pub fn count_department_hierarchies(self) -> Self {
        self.count_department_hierarchies_as("count_department_hierarchies")
    }

    pub fn count_department_hierarchies_as(self, alias: impl Into<String>) -> Self {
        self.count_department_hierarchies_with(alias, crate::Q::department_hierarchies().unlimited())
    }

    pub fn count_department_hierarchies_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "department_hierarchy_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_department_hierarchies(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_department_hierarchies_as("refinements", request)
    }

    pub fn stats_from_department_hierarchies_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "department_hierarchy_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_department_hierarchies_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_department_hierarchies(request)
    }




    pub fn count_branch_offices(self) -> Self {
        self.count_branch_offices_as("count_branch_offices")
    }

    pub fn count_branch_offices_as(self, alias: impl Into<String>) -> Self {
        self.count_branch_offices_with(alias, crate::Q::branch_offices().unlimited())
    }

    pub fn count_branch_offices_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "branch_office_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_branch_offices(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_branch_offices_as("refinements", request)
    }

    pub fn stats_from_branch_offices_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "branch_office_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_branch_offices_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_branch_offices(request)
    }




    pub fn count_move_orders(self) -> Self {
        self.count_move_orders_as("count_move_orders")
    }

    pub fn count_move_orders_as(self, alias: impl Into<String>) -> Self {
        self.count_move_orders_with(alias, crate::Q::move_orders().unlimited())
    }

    pub fn count_move_orders_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_order_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_move_orders(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders_as("refinements", request)
    }

    pub fn stats_from_move_orders_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_order_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_move_orders_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders(request)
    }




    pub fn count_addresses(self) -> Self {
        self.count_addresses_as("count_addresses")
    }

    pub fn count_addresses_as(self, alias: impl Into<String>) -> Self {
        self.count_addresses_with(alias, crate::Q::addresses().unlimited())
    }

    pub fn count_addresses_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "address_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_addresses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_addresses_as("refinements", request)
    }

    pub fn stats_from_addresses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "address_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_addresses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_addresses(request)
    }




    pub fn count_crews(self) -> Self {
        self.count_crews_as("count_crews")
    }

    pub fn count_crews_as(self, alias: impl Into<String>) -> Self {
        self.count_crews_with(alias, crate::Q::crews().unlimited())
    }

    pub fn count_crews_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "crew_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_crews(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crews_as("refinements", request)
    }

    pub fn stats_from_crews_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "crew_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_crews_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crews(request)
    }




    pub fn count_packing_materials(self) -> Self {
        self.count_packing_materials_as("count_packing_materials")
    }

    pub fn count_packing_materials_as(self, alias: impl Into<String>) -> Self {
        self.count_packing_materials_with(alias, crate::Q::packing_materials().unlimited())
    }

    pub fn count_packing_materials_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "packing_material_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_packing_materials(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packing_materials_as("refinements", request)
    }

    pub fn stats_from_packing_materials_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "packing_material_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_packing_materials_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packing_materials(request)
    }




    pub fn count_loading_zones(self) -> Self {
        self.count_loading_zones_as("count_loading_zones")
    }

    pub fn count_loading_zones_as(self, alias: impl Into<String>) -> Self {
        self.count_loading_zones_with(alias, crate::Q::loading_zones().unlimited())
    }

    pub fn count_loading_zones_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "loading_zone_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_loading_zones(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_loading_zones_as("refinements", request)
    }

    pub fn stats_from_loading_zones_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "loading_zone_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_loading_zones_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_loading_zones(request)
    }




    pub fn count_unloading_zones(self) -> Self {
        self.count_unloading_zones_as("count_unloading_zones")
    }

    pub fn count_unloading_zones_as(self, alias: impl Into<String>) -> Self {
        self.count_unloading_zones_with(alias, crate::Q::unloading_zones().unlimited())
    }

    pub fn count_unloading_zones_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "unloading_zone_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_unloading_zones(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_unloading_zones_as("refinements", request)
    }

    pub fn stats_from_unloading_zones_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "unloading_zone_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_unloading_zones_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_unloading_zones(request)
    }




    pub fn count_route_optimization_rules(self) -> Self {
        self.count_route_optimization_rules_as("count_route_optimization_rules")
    }

    pub fn count_route_optimization_rules_as(self, alias: impl Into<String>) -> Self {
        self.count_route_optimization_rules_with(alias, crate::Q::route_optimization_rules().unlimited())
    }

    pub fn count_route_optimization_rules_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "route_optimization_rule_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_route_optimization_rules(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_optimization_rules_as("refinements", request)
    }

    pub fn stats_from_route_optimization_rules_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "route_optimization_rule_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_route_optimization_rules_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_optimization_rules(request)
    }




    pub fn count_move_statuses(self) -> Self {
        self.count_move_statuses_as("count_move_statuses")
    }

    pub fn count_move_statuses_as(self, alias: impl Into<String>) -> Self {
        self.count_move_statuses_with(alias, crate::Q::move_statuses().unlimited())
    }

    pub fn count_move_statuses_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_status_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_move_statuses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_statuses_as("refinements", request)
    }

    pub fn stats_from_move_statuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_status_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_move_statuses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_statuses(request)
    }




    pub fn count_departments(self) -> Self {
        self.count_departments_as("count_departments")
    }

    pub fn count_departments_as(self, alias: impl Into<String>) -> Self {
        self.count_departments_with(alias, crate::Q::departments().unlimited())
    }

    pub fn count_departments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "department_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_departments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments_as("refinements", request)
    }

    pub fn stats_from_departments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "department_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_departments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments(request)
    }




    pub fn count_payroll_periods(self) -> Self {
        self.count_payroll_periods_as("count_payroll_periods")
    }

    pub fn count_payroll_periods_as(self, alias: impl Into<String>) -> Self {
        self.count_payroll_periods_with(alias, crate::Q::payroll_periods().unlimited())
    }

    pub fn count_payroll_periods_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payroll_period_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_payroll_periods(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_periods_as("refinements", request)
    }

    pub fn stats_from_payroll_periods_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payroll_period_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_payroll_periods_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_periods(request)
    }




    pub fn count_customers(self) -> Self {
        self.count_customers_as("count_customers")
    }

    pub fn count_customers_as(self, alias: impl Into<String>) -> Self {
        self.count_customers_with(alias, crate::Q::customers().unlimited())
    }

    pub fn count_customers_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customers(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customers_as("refinements", request)
    }

    pub fn stats_from_customers_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customers_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customers(request)
    }




    pub fn count_customer_segments(self) -> Self {
        self.count_customer_segments_as("count_customer_segments")
    }

    pub fn count_customer_segments_as(self, alias: impl Into<String>) -> Self {
        self.count_customer_segments_with(alias, crate::Q::customer_segments().unlimited())
    }

    pub fn count_customer_segments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_segment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customer_segments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_segments_as("refinements", request)
    }

    pub fn stats_from_customer_segments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_segment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customer_segments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_segments(request)
    }




    pub fn count_products(self) -> Self {
        self.count_products_as("count_products")
    }

    pub fn count_products_as(self, alias: impl Into<String>) -> Self {
        self.count_products_with(alias, crate::Q::products().unlimited())
    }

    pub fn count_products_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "product_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_products(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_products_as("refinements", request)
    }

    pub fn stats_from_products_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "product_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_products_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_products(request)
    }




    pub fn count_services(self) -> Self {
        self.count_services_as("count_services")
    }

    pub fn count_services_as(self, alias: impl Into<String>) -> Self {
        self.count_services_with(alias, crate::Q::services().unlimited())
    }

    pub fn count_services_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_services(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_services_as("refinements", request)
    }

    pub fn stats_from_services_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_services_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_services(request)
    }




    pub fn count_price_lists(self) -> Self {
        self.count_price_lists_as("count_price_lists")
    }

    pub fn count_price_lists_as(self, alias: impl Into<String>) -> Self {
        self.count_price_lists_with(alias, crate::Q::price_lists().unlimited())
    }

    pub fn count_price_lists_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "price_list_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_price_lists(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_price_lists_as("refinements", request)
    }

    pub fn stats_from_price_lists_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "price_list_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_price_lists_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_price_lists(request)
    }




    pub fn count_service_bundles(self) -> Self {
        self.count_service_bundles_as("count_service_bundles")
    }

    pub fn count_service_bundles_as(self, alias: impl Into<String>) -> Self {
        self.count_service_bundles_with(alias, crate::Q::service_bundles().unlimited())
    }

    pub fn count_service_bundles_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_bundle_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_service_bundles(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_bundles_as("refinements", request)
    }

    pub fn stats_from_service_bundles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_bundle_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_service_bundles_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_bundles(request)
    }




    pub fn count_storage_units(self) -> Self {
        self.count_storage_units_as("count_storage_units")
    }

    pub fn count_storage_units_as(self, alias: impl Into<String>) -> Self {
        self.count_storage_units_with(alias, crate::Q::storage_units().unlimited())
    }

    pub fn count_storage_units_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "storage_unit_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_storage_units(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_storage_units_as("refinements", request)
    }

    pub fn stats_from_storage_units_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "storage_unit_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_storage_units_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_storage_units(request)
    }




    pub fn count_service_areas(self) -> Self {
        self.count_service_areas_as("count_service_areas")
    }

    pub fn count_service_areas_as(self, alias: impl Into<String>) -> Self {
        self.count_service_areas_with(alias, crate::Q::service_areas().unlimited())
    }

    pub fn count_service_areas_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_area_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_service_areas(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_areas_as("refinements", request)
    }

    pub fn stats_from_service_areas_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_area_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_service_areas_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_areas(request)
    }




    pub fn count_inventory_items(self) -> Self {
        self.count_inventory_items_as("count_inventory_items")
    }

    pub fn count_inventory_items_as(self, alias: impl Into<String>) -> Self {
        self.count_inventory_items_with(alias, crate::Q::inventory_items().unlimited())
    }

    pub fn count_inventory_items_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "inventory_item_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_inventory_items(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_inventory_items_as("refinements", request)
    }

    pub fn stats_from_inventory_items_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "inventory_item_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_inventory_items_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_inventory_items(request)
    }




    pub fn count_service_categories(self) -> Self {
        self.count_service_categories_as("count_service_categories")
    }

    pub fn count_service_categories_as(self, alias: impl Into<String>) -> Self {
        self.count_service_categories_with(alias, crate::Q::service_categories().unlimited())
    }

    pub fn count_service_categories_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_category_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_service_categories(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_categories_as("refinements", request)
    }

    pub fn stats_from_service_categories_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_category_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_service_categories_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_categories(request)
    }




    pub fn count_campaigns(self) -> Self {
        self.count_campaigns_as("count_campaigns")
    }

    pub fn count_campaigns_as(self, alias: impl Into<String>) -> Self {
        self.count_campaigns_with(alias, crate::Q::campaigns().unlimited())
    }

    pub fn count_campaigns_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "campaign_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_campaigns(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_campaigns_as("refinements", request)
    }

    pub fn stats_from_campaigns_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "campaign_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_campaigns_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_campaigns(request)
    }




    pub fn count_marketing_channels(self) -> Self {
        self.count_marketing_channels_as("count_marketing_channels")
    }

    pub fn count_marketing_channels_as(self, alias: impl Into<String>) -> Self {
        self.count_marketing_channels_with(alias, crate::Q::marketing_channels().unlimited())
    }

    pub fn count_marketing_channels_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "marketing_channel_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_marketing_channels(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_marketing_channels_as("refinements", request)
    }

    pub fn stats_from_marketing_channels_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "marketing_channel_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_marketing_channels_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_marketing_channels(request)
    }




    pub fn count_payments(self) -> Self {
        self.count_payments_as("count_payments")
    }

    pub fn count_payments_as(self, alias: impl Into<String>) -> Self {
        self.count_payments_with(alias, crate::Q::payments().unlimited())
    }

    pub fn count_payments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_payments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payments_as("refinements", request)
    }

    pub fn stats_from_payments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_payments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payments(request)
    }




    pub fn count_expenses(self) -> Self {
        self.count_expenses_as("count_expenses")
    }

    pub fn count_expenses_as(self, alias: impl Into<String>) -> Self {
        self.count_expenses_with(alias, crate::Q::expenses().unlimited())
    }

    pub fn count_expenses_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "expense_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_expenses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expenses_as("refinements", request)
    }

    pub fn stats_from_expenses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "expense_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_expenses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expenses(request)
    }




    pub fn count_vat_rates(self) -> Self {
        self.count_vat_rates_as("count_vat_rates")
    }

    pub fn count_vat_rates_as(self, alias: impl Into<String>) -> Self {
        self.count_vat_rates_with(alias, crate::Q::vat_rates().unlimited())
    }

    pub fn count_vat_rates_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vat_rate_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_vat_rates(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vat_rates_as("refinements", request)
    }

    pub fn stats_from_vat_rates_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vat_rate_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_vat_rates_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vat_rates(request)
    }




    pub fn count_accounts(self) -> Self {
        self.count_accounts_as("count_accounts")
    }

    pub fn count_accounts_as(self, alias: impl Into<String>) -> Self {
        self.count_accounts_with(alias, crate::Q::accounts().unlimited())
    }

    pub fn count_accounts_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "account_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_accounts(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_accounts_as("refinements", request)
    }

    pub fn stats_from_accounts_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "account_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_accounts_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_accounts(request)
    }




    pub fn count_financial_summaries(self) -> Self {
        self.count_financial_summaries_as("count_financial_summaries")
    }

    pub fn count_financial_summaries_as(self, alias: impl Into<String>) -> Self {
        self.count_financial_summaries_with(alias, crate::Q::financial_summaries().unlimited())
    }

    pub fn count_financial_summaries_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "financial_summary_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_financial_summaries(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_financial_summaries_as("refinements", request)
    }

    pub fn stats_from_financial_summaries_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "financial_summary_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_financial_summaries_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_financial_summaries(request)
    }




    pub fn count_budgets(self) -> Self {
        self.count_budgets_as("count_budgets")
    }

    pub fn count_budgets_as(self, alias: impl Into<String>) -> Self {
        self.count_budgets_with(alias, crate::Q::budgets().unlimited())
    }

    pub fn count_budgets_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "budget_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_budgets(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_budgets_as("refinements", request)
    }

    pub fn stats_from_budgets_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "budget_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_budgets_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_budgets(request)
    }




    pub fn count_payables(self) -> Self {
        self.count_payables_as("count_payables")
    }

    pub fn count_payables_as(self, alias: impl Into<String>) -> Self {
        self.count_payables_with(alias, crate::Q::payables().unlimited())
    }

    pub fn count_payables_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payable_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_payables(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payables_as("refinements", request)
    }

    pub fn stats_from_payables_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payable_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_payables_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payables(request)
    }




    pub fn count_currency_rates(self) -> Self {
        self.count_currency_rates_as("count_currency_rates")
    }

    pub fn count_currency_rates_as(self, alias: impl Into<String>) -> Self {
        self.count_currency_rates_with(alias, crate::Q::currency_rates().unlimited())
    }

    pub fn count_currency_rates_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "currency_rate_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_currency_rates(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_currency_rates_as("refinements", request)
    }

    pub fn stats_from_currency_rates_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "currency_rate_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_currency_rates_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_currency_rates(request)
    }




    pub fn count_payment_methods(self) -> Self {
        self.count_payment_methods_as("count_payment_methods")
    }

    pub fn count_payment_methods_as(self, alias: impl Into<String>) -> Self {
        self.count_payment_methods_with(alias, crate::Q::payment_methods().unlimited())
    }

    pub fn count_payment_methods_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payment_method_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_payment_methods(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payment_methods_as("refinements", request)
    }

    pub fn stats_from_payment_methods_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payment_method_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_payment_methods_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payment_methods(request)
    }




    pub fn count_financial_periods(self) -> Self {
        self.count_financial_periods_as("count_financial_periods")
    }

    pub fn count_financial_periods_as(self, alias: impl Into<String>) -> Self {
        self.count_financial_periods_with(alias, crate::Q::financial_periods().unlimited())
    }

    pub fn count_financial_periods_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "financial_period_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_financial_periods(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_financial_periods_as("refinements", request)
    }

    pub fn stats_from_financial_periods_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "financial_period_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_financial_periods_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_financial_periods(request)
    }




    pub fn count_vehicles(self) -> Self {
        self.count_vehicles_as("count_vehicles")
    }

    pub fn count_vehicles_as(self, alias: impl Into<String>) -> Self {
        self.count_vehicles_with(alias, crate::Q::vehicles().unlimited())
    }

    pub fn count_vehicles_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vehicle_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_vehicles(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicles_as("refinements", request)
    }

    pub fn stats_from_vehicles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vehicle_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_vehicles_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicles(request)
    }




    pub fn count_equipment(self) -> Self {
        self.count_equipment_as("count_equipment")
    }

    pub fn count_equipment_as(self, alias: impl Into<String>) -> Self {
        self.count_equipment_with(alias, crate::Q::equipment().unlimited())
    }

    pub fn count_equipment_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "equipment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_equipment(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_equipment_as("refinements", request)
    }

    pub fn stats_from_equipment_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "equipment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_equipment_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_equipment(request)
    }




    pub fn count_consumables(self) -> Self {
        self.count_consumables_as("count_consumables")
    }

    pub fn count_consumables_as(self, alias: impl Into<String>) -> Self {
        self.count_consumables_with(alias, crate::Q::consumables().unlimited())
    }

    pub fn count_consumables_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "consumable_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_consumables(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_consumables_as("refinements", request)
    }

    pub fn stats_from_consumables_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "consumable_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_consumables_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_consumables(request)
    }




    pub fn count_suppliers(self) -> Self {
        self.count_suppliers_as("count_suppliers")
    }

    pub fn count_suppliers_as(self, alias: impl Into<String>) -> Self {
        self.count_suppliers_with(alias, crate::Q::suppliers().unlimited())
    }

    pub fn count_suppliers_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "supplier_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_suppliers(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_suppliers_as("refinements", request)
    }

    pub fn stats_from_suppliers_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "supplier_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_suppliers_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_suppliers(request)
    }




    pub fn count_storage_locations(self) -> Self {
        self.count_storage_locations_as("count_storage_locations")
    }

    pub fn count_storage_locations_as(self, alias: impl Into<String>) -> Self {
        self.count_storage_locations_with(alias, crate::Q::storage_locations().unlimited())
    }

    pub fn count_storage_locations_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "storage_location_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_storage_locations(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_storage_locations_as("refinements", request)
    }

    pub fn stats_from_storage_locations_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "storage_location_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_storage_locations_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_storage_locations(request)
    }




    pub fn count_contracts(self) -> Self {
        self.count_contracts_as("count_contracts")
    }

    pub fn count_contracts_as(self, alias: impl Into<String>) -> Self {
        self.count_contracts_with(alias, crate::Q::contracts().unlimited())
    }

    pub fn count_contracts_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "contract_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_contracts(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts_as("refinements", request)
    }

    pub fn stats_from_contracts_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "contract_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_contracts_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts(request)
    }




    pub fn count_insurance_policies(self) -> Self {
        self.count_insurance_policies_as("count_insurance_policies")
    }

    pub fn count_insurance_policies_as(self, alias: impl Into<String>) -> Self {
        self.count_insurance_policies_with(alias, crate::Q::insurance_policies().unlimited())
    }

    pub fn count_insurance_policies_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "insurance_policy_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_insurance_policies(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_insurance_policies_as("refinements", request)
    }

    pub fn stats_from_insurance_policies_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "insurance_policy_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_insurance_policies_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_insurance_policies(request)
    }




    pub fn count_compliance_checks(self) -> Self {
        self.count_compliance_checks_as("count_compliance_checks")
    }

    pub fn count_compliance_checks_as(self, alias: impl Into<String>) -> Self {
        self.count_compliance_checks_with(alias, crate::Q::compliance_checks().unlimited())
    }

    pub fn count_compliance_checks_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compliance_check_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_compliance_checks(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compliance_checks_as("refinements", request)
    }

    pub fn stats_from_compliance_checks_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compliance_check_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_compliance_checks_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compliance_checks(request)
    }




    pub fn count_data_retention_policies(self) -> Self {
        self.count_data_retention_policies_as("count_data_retention_policies")
    }

    pub fn count_data_retention_policies_as(self, alias: impl Into<String>) -> Self {
        self.count_data_retention_policies_with(alias, crate::Q::data_retention_policies().unlimited())
    }

    pub fn count_data_retention_policies_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "data_retention_policy_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_data_retention_policies(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_data_retention_policies_as("refinements", request)
    }

    pub fn stats_from_data_retention_policies_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "data_retention_policy_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_data_retention_policies_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_data_retention_policies(request)
    }




    pub fn count_policy_documents(self) -> Self {
        self.count_policy_documents_as("count_policy_documents")
    }

    pub fn count_policy_documents_as(self, alias: impl Into<String>) -> Self {
        self.count_policy_documents_with(alias, crate::Q::policy_documents().unlimited())
    }

    pub fn count_policy_documents_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "policy_document_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_policy_documents(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_policy_documents_as("refinements", request)
    }

    pub fn stats_from_policy_documents_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "policy_document_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_policy_documents_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_policy_documents(request)
    }




    pub fn count_incident_reports(self) -> Self {
        self.count_incident_reports_as("count_incident_reports")
    }

    pub fn count_incident_reports_as(self, alias: impl Into<String>) -> Self {
        self.count_incident_reports_with(alias, crate::Q::incident_reports().unlimited())
    }

    pub fn count_incident_reports_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "incident_report_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_incident_reports(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_incident_reports_as("refinements", request)
    }

    pub fn stats_from_incident_reports_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "incident_report_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_incident_reports_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_incident_reports(request)
    }




    pub fn count_legal_entities(self) -> Self {
        self.count_legal_entities_as("count_legal_entities")
    }

    pub fn count_legal_entities_as(self, alias: impl Into<String>) -> Self {
        self.count_legal_entities_with(alias, crate::Q::legal_entities().unlimited())
    }

    pub fn count_legal_entities_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "legal_entity_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_legal_entities(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_legal_entities_as("refinements", request)
    }

    pub fn stats_from_legal_entities_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "legal_entity_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_legal_entities_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_legal_entities(request)
    }




    pub fn count_regulatory_requirements(self) -> Self {
        self.count_regulatory_requirements_as("count_regulatory_requirements")
    }

    pub fn count_regulatory_requirements_as(self, alias: impl Into<String>) -> Self {
        self.count_regulatory_requirements_with(alias, crate::Q::regulatory_requirements().unlimited())
    }

    pub fn count_regulatory_requirements_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "regulatory_requirement_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_regulatory_requirements(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_regulatory_requirements_as("refinements", request)
    }

    pub fn stats_from_regulatory_requirements_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "regulatory_requirement_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_regulatory_requirements_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_regulatory_requirements(request)
    }




    pub fn count_compliance_certificates(self) -> Self {
        self.count_compliance_certificates_as("count_compliance_certificates")
    }

    pub fn count_compliance_certificates_as(self, alias: impl Into<String>) -> Self {
        self.count_compliance_certificates_with(alias, crate::Q::compliance_certificates().unlimited())
    }

    pub fn count_compliance_certificates_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compliance_certificate_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_compliance_certificates(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compliance_certificates_as("refinements", request)
    }

    pub fn stats_from_compliance_certificates_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compliance_certificate_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_compliance_certificates_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compliance_certificates(request)
    }




    pub fn count_roles(self) -> Self {
        self.count_roles_as("count_roles")
    }

    pub fn count_roles_as(self, alias: impl Into<String>) -> Self {
        self.count_roles_with(alias, crate::Q::roles().unlimited())
    }

    pub fn count_roles_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "role_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_roles(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_roles_as("refinements", request)
    }

    pub fn stats_from_roles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "role_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_roles_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_roles(request)
    }




    pub fn count_permissions(self) -> Self {
        self.count_permissions_as("count_permissions")
    }

    pub fn count_permissions_as(self, alias: impl Into<String>) -> Self {
        self.count_permissions_with(alias, crate::Q::permissions().unlimited())
    }

    pub fn count_permissions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "permission_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_permissions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_permissions_as("refinements", request)
    }

    pub fn stats_from_permissions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "permission_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_permissions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_permissions(request)
    }




    pub fn count_audit_logs(self) -> Self {
        self.count_audit_logs_as("count_audit_logs")
    }

    pub fn count_audit_logs_as(self, alias: impl Into<String>) -> Self {
        self.count_audit_logs_with(alias, crate::Q::audit_logs().unlimited())
    }

    pub fn count_audit_logs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "audit_log_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_audit_logs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_audit_logs_as("refinements", request)
    }

    pub fn stats_from_audit_logs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "audit_log_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_audit_logs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_audit_logs(request)
    }




    pub fn count_system_events(self) -> Self {
        self.count_system_events_as("count_system_events")
    }

    pub fn count_system_events_as(self, alias: impl Into<String>) -> Self {
        self.count_system_events_with(alias, crate::Q::system_events().unlimited())
    }

    pub fn count_system_events_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "system_event_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_system_events(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_system_events_as("refinements", request)
    }

    pub fn stats_from_system_events_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "system_event_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_system_events_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_system_events(request)
    }




    pub fn count_notification_templates(self) -> Self {
        self.count_notification_templates_as("count_notification_templates")
    }

    pub fn count_notification_templates_as(self, alias: impl Into<String>) -> Self {
        self.count_notification_templates_with(alias, crate::Q::notification_templates().unlimited())
    }

    pub fn count_notification_templates_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "notification_template_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_notification_templates(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_notification_templates_as("refinements", request)
    }

    pub fn stats_from_notification_templates_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "notification_template_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_notification_templates_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_notification_templates(request)
    }




    pub fn count_automation_rules(self) -> Self {
        self.count_automation_rules_as("count_automation_rules")
    }

    pub fn count_automation_rules_as(self, alias: impl Into<String>) -> Self {
        self.count_automation_rules_with(alias, crate::Q::automation_rules().unlimited())
    }

    pub fn count_automation_rules_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "automation_rule_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_automation_rules(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_automation_rules_as("refinements", request)
    }

    pub fn stats_from_automation_rules_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "automation_rule_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_automation_rules_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_automation_rules(request)
    }




    pub fn count_api_clients(self) -> Self {
        self.count_api_clients_as("count_api_clients")
    }

    pub fn count_api_clients_as(self, alias: impl Into<String>) -> Self {
        self.count_api_clients_with(alias, crate::Q::api_clients().unlimited())
    }

    pub fn count_api_clients_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "api_client_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_api_clients(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_api_clients_as("refinements", request)
    }

    pub fn stats_from_api_clients_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "api_client_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_api_clients_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_api_clients(request)
    }



}

impl<R> Default for MerchantRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< MerchantRequest<R> > for SelectQuery {
    fn from(request: MerchantRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< MerchantRequest<R> > for QuerySelection {
    fn from(request: MerchantRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Merchant> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::MerchantRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<MerchantRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Merchant
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Merchant::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> MerchantRequest<R> {
        self.inner.query.trace_chain.push(teaql_core::TraceNode::new(
            self.inner.query.entity.clone(),
            None,
            self.purpose,
        ));
        self.inner
    }

    pub async fn execute_for_page<'a, C>(
        self,
        ctx: &'a C,
        offset: u64,
        limit: u64,
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
