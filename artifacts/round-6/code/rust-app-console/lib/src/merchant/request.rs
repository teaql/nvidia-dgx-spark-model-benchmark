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
            "tax_number" => Some("tax_number"),
            "address" => Some("address"),
            "external_id" => Some("external_id"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "platform" | "platform_id" => Some("platform_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "platform" => {
                self.with_platform_matching(
                    crate::Q::platforms_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "move_quote_list" => {
                self.with_move_quote_list_matching(
                    crate::Q::move_quotes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "move_order_list" => {
                self.with_move_order_list_matching(
                    crate::Q::move_orders_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "route_stop_list" => {
                self.with_route_stop_list_matching(
                    crate::Q::route_stops_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "crew_list" => {
                self.with_crew_list_matching(
                    crate::Q::crews_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "crew_member_assignment_list" => {
                self.with_crew_member_assignment_list_matching(
                    crate::Q::crew_member_assignments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "vehicle_list" => {
                self.with_vehicle_list_matching(
                    crate::Q::vehicles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "vehicle_assignment_list" => {
                self.with_vehicle_assignment_list_matching(
                    crate::Q::vehicle_assignments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "dispatch_assignment_list" => {
                self.with_dispatch_assignment_list_matching(
                    crate::Q::dispatch_assignments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "damage_report_list" => {
                self.with_damage_report_list_matching(
                    crate::Q::damage_reports_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "proof_of_delivery_list" => {
                self.with_proof_of_delivery_list_matching(
                    crate::Q::proof_of_deliveries_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "operational_exception_list" => {
                self.with_operational_exception_list_matching(
                    crate::Q::operational_exceptions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "pickup_instruction_list" => {
                self.with_pickup_instruction_list_matching(
                    crate::Q::pickup_instructions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "delivery_instruction_list" => {
                self.with_delivery_instruction_list_matching(
                    crate::Q::delivery_instructions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "move_inventory_list" => {
                self.with_move_inventory_list_matching(
                    crate::Q::move_inventory_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "packaging_item_list" => {
                self.with_packaging_item_list_matching(
                    crate::Q::packaging_items_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "logistics_provider_list" => {
                self.with_logistics_provider_list_matching(
                    crate::Q::logistics_providers_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "third_party_dispatch_list" => {
                self.with_third_party_dispatch_list_matching(
                    crate::Q::third_party_dispatches_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "fuel_log_list" => {
                self.with_fuel_log_list_matching(
                    crate::Q::fuel_logs_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "maintenance_record_list" => {
                self.with_maintenance_record_list_matching(
                    crate::Q::maintenance_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "toll_receipt_list" => {
                self.with_toll_receipt_list_matching(
                    crate::Q::toll_receipts_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "shift_log_list" => {
                self.with_shift_log_list_matching(
                    crate::Q::shift_logs_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_feedback_list" => {
                self.with_customer_feedback_list_matching(
                    crate::Q::customer_feedback_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "incident_report_list" => {
                self.with_incident_report_list_matching(
                    crate::Q::incident_reports_minimal()
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
        self.query = self.query.project("tax_number");
        self.query = self.query.project("address");
        self.query = self.query.project("external_id");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("platform_id");
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
        request = request.select_platform();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_move_quote_list();
        request = request.select_move_order_list();
        request = request.select_route_stop_list();
        request = request.select_crew_list();
        request = request.select_crew_member_assignment_list();
        request = request.select_vehicle_list();
        request = request.select_vehicle_assignment_list();
        request = request.select_dispatch_assignment_list();
        request = request.select_damage_report_list();
        request = request.select_proof_of_delivery_list();
        request = request.select_operational_exception_list();
        request = request.select_pickup_instruction_list();
        request = request.select_delivery_instruction_list();
        request = request.select_move_inventory_list();
        request = request.select_packaging_item_list();
        request = request.select_logistics_provider_list();
        request = request.select_third_party_dispatch_list();
        request = request.select_fuel_log_list();
        request = request.select_maintenance_record_list();
        request = request.select_toll_receipt_list();
        request = request.select_shift_log_list();
        request = request.select_customer_feedback_list();
        request = request.select_incident_report_list();
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


    pub fn select_tax_number(mut self) -> Self {
        self.query = self.query.project("tax_number");
        self
    }

    pub fn project_tax_number(self) -> Self {
        self.select_tax_number()
    }

    pub fn select_tax_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_tax_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_tax_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("tax_number", raw_sql_segment));
        self
    }

    pub fn group_by_tax_number(self) -> Self {
        self.group_by("tax_number")
    }

    pub fn group_by_tax_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tax_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tax_number"));
        request
    }

    pub fn group_by_tax_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tax_number")
            .aggregate_with_function("tax_number", alias, function)
    }

    pub fn count_tax_number(self) -> Self {
        self.count_tax_number_as("tax_number_count")
    }

    pub fn count_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tax_number", alias)
    }

    pub fn sum_tax_number(self) -> Self {
        self.sum_tax_number_as("sum_tax_number")
    }

    pub fn sum_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("tax_number", alias)
    }

    pub fn avg_tax_number(self) -> Self {
        self.avg_tax_number_as("avg_tax_number")
    }

    pub fn avg_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("tax_number", alias)
    }

    pub fn min_tax_number(self) -> Self {
        self.min_tax_number_as("min_tax_number")
    }

    pub fn min_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("tax_number", alias)
    }

    pub fn max_tax_number(self) -> Self {
        self.max_tax_number_as("max_tax_number")
    }

    pub fn max_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("tax_number", alias)
    }

    pub fn unselect_tax_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "tax_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "tax_number");
        self
    }


    pub fn with_tax_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "tax_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_tax_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "tax_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_tax_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("tax_number", value));
        self
    }



    pub fn with_tax_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("tax_number", value));
        self
    }

    pub fn with_tax_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tax_number", value));
        self
    }

    pub fn with_tax_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("tax_number", value));
        self
    }

    pub fn with_tax_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tax_number", value));
        self
    }

    pub fn with_tax_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("tax_number", value));
        self
    }

    pub fn with_tax_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("tax_number", lower, upper));
        self
    }

    pub fn with_tax_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "tax_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_tax_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "tax_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tax_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "tax_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tax_number_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("tax_number", value));
        self
    }

    pub fn with_tax_number_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("tax_number", value));
        self
    }

    pub fn with_tax_number_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("tax_number", value));
        self
    }

    pub fn with_tax_number_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("tax_number", value));
        self
    }

    pub fn with_tax_number_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("tax_number", value));
        self
    }

    pub fn with_tax_number_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("tax_number", value));
        self
    }

    pub fn with_tax_number_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("tax_number", value));
        self
    }
    pub fn with_tax_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tax_number", value));
        self
    }

    pub fn with_tax_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tax_number", value));
        self
    }

    pub fn with_tax_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tax_number"));
        self
    }



    pub fn with_tax_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tax_number"));
        self
    }


    pub fn order_by_tax_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("tax_number");
        self
    }

    pub fn order_by_tax_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("tax_number");
        self
    }

    pub fn order_by_tax_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("tax_number");
        self
    }

    pub fn order_by_tax_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("tax_number");
        self
    }


    pub fn select_address(mut self) -> Self {
        self.query = self.query.project("address");
        self
    }

    pub fn project_address(self) -> Self {
        self.select_address()
    }

    pub fn select_address_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_address_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_address_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("address", raw_sql_segment));
        self
    }

    pub fn group_by_address(self) -> Self {
        self.group_by("address")
    }

    pub fn group_by_address_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("address");
        request.query = request
            .query
            .project_expr(alias, Expr::column("address"));
        request
    }

    pub fn group_by_address_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("address")
            .aggregate_with_function("address", alias, function)
    }

    pub fn count_address(self) -> Self {
        self.count_address_as("address_count")
    }

    pub fn count_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("address", alias)
    }

    pub fn sum_address(self) -> Self {
        self.sum_address_as("sum_address")
    }

    pub fn sum_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("address", alias)
    }

    pub fn avg_address(self) -> Self {
        self.avg_address_as("avg_address")
    }

    pub fn avg_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("address", alias)
    }

    pub fn min_address(self) -> Self {
        self.min_address_as("min_address")
    }

    pub fn min_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("address", alias)
    }

    pub fn max_address(self) -> Self {
        self.max_address_as("max_address")
    }

    pub fn max_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("address", alias)
    }

    pub fn unselect_address(mut self) -> Self {
        self.query.projection.retain(|field| field != "address");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "address");
        self
    }


    pub fn with_address(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "address",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_address_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "address",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_address_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("address", value));
        self
    }



    pub fn with_address_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("address", value));
        self
    }

    pub fn with_address_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("address", value));
        self
    }

    pub fn with_address_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("address", value));
        self
    }

    pub fn with_address_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("address", value));
        self
    }

    pub fn with_address_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("address", value));
        self
    }

    pub fn with_address_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("address", lower, upper));
        self
    }

    pub fn with_address_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "address",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_address_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "address",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_address_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "address",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_address_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("address", value));
        self
    }

    pub fn with_address_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("address", value));
        self
    }

    pub fn with_address_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("address", value));
        self
    }

    pub fn with_address_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("address", value));
        self
    }

    pub fn with_address_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("address", value));
        self
    }

    pub fn with_address_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("address", value));
        self
    }

    pub fn with_address_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("address", value));
        self
    }
    pub fn with_address_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("address", value));
        self
    }

    pub fn with_address_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("address", value));
        self
    }

    pub fn with_address_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("address"));
        self
    }



    pub fn with_address_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("address"));
        self
    }


    pub fn order_by_address_asc(mut self) -> Self {
        self.query = self.query.order_asc("address");
        self
    }

    pub fn order_by_address_desc(mut self) -> Self {
        self.query = self.query.order_desc("address");
        self
    }

    pub fn order_by_address_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("address");
        self
    }

    pub fn order_by_address_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("address");
        self
    }


    pub fn select_external_id(mut self) -> Self {
        self.query = self.query.project("external_id");
        self
    }

    pub fn project_external_id(self) -> Self {
        self.select_external_id()
    }

    pub fn select_external_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_external_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_external_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("external_id", raw_sql_segment));
        self
    }

    pub fn group_by_external_id(self) -> Self {
        self.group_by("external_id")
    }

    pub fn group_by_external_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("external_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("external_id"));
        request
    }

    pub fn group_by_external_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("external_id")
            .aggregate_with_function("external_id", alias, function)
    }

    pub fn count_external_id(self) -> Self {
        self.count_external_id_as("external_id_count")
    }

    pub fn count_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("external_id", alias)
    }

    pub fn sum_external_id(self) -> Self {
        self.sum_external_id_as("sum_external_id")
    }

    pub fn sum_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("external_id", alias)
    }

    pub fn avg_external_id(self) -> Self {
        self.avg_external_id_as("avg_external_id")
    }

    pub fn avg_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("external_id", alias)
    }

    pub fn min_external_id(self) -> Self {
        self.min_external_id_as("min_external_id")
    }

    pub fn min_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("external_id", alias)
    }

    pub fn max_external_id(self) -> Self {
        self.max_external_id_as("max_external_id")
    }

    pub fn max_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("external_id", alias)
    }

    pub fn unselect_external_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "external_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "external_id");
        self
    }


    pub fn with_external_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "external_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_external_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "external_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_external_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("external_id", value));
        self
    }



    pub fn with_external_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("external_id", value));
        self
    }

    pub fn with_external_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("external_id", value));
        self
    }

    pub fn with_external_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("external_id", value));
        self
    }

    pub fn with_external_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("external_id", value));
        self
    }

    pub fn with_external_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("external_id", value));
        self
    }

    pub fn with_external_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("external_id", lower, upper));
        self
    }

    pub fn with_external_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "external_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_external_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "external_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_external_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "external_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_external_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("external_id", value));
        self
    }

    pub fn with_external_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("external_id", value));
        self
    }

    pub fn with_external_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("external_id", value));
        self
    }

    pub fn with_external_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("external_id", value));
        self
    }

    pub fn with_external_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("external_id", value));
        self
    }

    pub fn with_external_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("external_id", value));
        self
    }

    pub fn with_external_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("external_id", value));
        self
    }
    pub fn with_external_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("external_id", value));
        self
    }

    pub fn with_external_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("external_id", value));
        self
    }

    pub fn with_external_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("external_id"));
        self
    }



    pub fn with_external_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("external_id"));
        self
    }


    pub fn order_by_external_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("external_id");
        self
    }

    pub fn order_by_external_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("external_id");
        self
    }

    pub fn order_by_external_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("external_id");
        self
    }

    pub fn order_by_external_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("external_id");
        self
    }


    pub fn select_create_time(mut self) -> Self {
        self.query = self.query.project("create_time");
        self
    }

    pub fn project_create_time(self) -> Self {
        self.select_create_time()
    }

    pub fn select_create_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_create_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_create_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("create_time", raw_sql_segment));
        self
    }

    pub fn group_by_create_time(self) -> Self {
        self.group_by("create_time")
    }

    pub fn group_by_create_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("create_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("create_time"));
        request
    }

    pub fn group_by_create_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("create_time")
            .aggregate_with_function("create_time", alias, function)
    }

    pub fn count_create_time(self) -> Self {
        self.count_create_time_as("create_time_count")
    }

    pub fn count_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("create_time", alias)
    }

    pub fn sum_create_time(self) -> Self {
        self.sum_create_time_as("sum_create_time")
    }

    pub fn sum_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("create_time", alias)
    }

    pub fn avg_create_time(self) -> Self {
        self.avg_create_time_as("avg_create_time")
    }

    pub fn avg_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("create_time", alias)
    }

    pub fn min_create_time(self) -> Self {
        self.min_create_time_as("min_create_time")
    }

    pub fn min_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("create_time", alias)
    }

    pub fn max_create_time(self) -> Self {
        self.max_create_time_as("max_create_time")
    }

    pub fn max_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("create_time", alias)
    }

    pub fn unselect_create_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "create_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "create_time");
        self
    }


    pub fn with_create_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_create_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_create_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("create_time", value));
        self
    }



    pub fn with_create_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("create_time", value));
        self
    }

    pub fn with_create_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("create_time", value));
        self
    }

    pub fn with_create_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("create_time", value));
        self
    }

    pub fn with_create_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("create_time", lower, upper));
        self
    }

    pub fn with_create_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "create_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_create_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("create_time"));
        self
    }



    pub fn with_create_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("create_time"));
        self
    }


    pub fn order_by_create_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("create_time");
        self
    }

    pub fn order_by_create_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("create_time");
        self
    }


    pub fn select_update_time(mut self) -> Self {
        self.query = self.query.project("update_time");
        self
    }

    pub fn project_update_time(self) -> Self {
        self.select_update_time()
    }

    pub fn select_update_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_update_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_update_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("update_time", raw_sql_segment));
        self
    }

    pub fn group_by_update_time(self) -> Self {
        self.group_by("update_time")
    }

    pub fn group_by_update_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("update_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("update_time"));
        request
    }

    pub fn group_by_update_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("update_time")
            .aggregate_with_function("update_time", alias, function)
    }

    pub fn count_update_time(self) -> Self {
        self.count_update_time_as("update_time_count")
    }

    pub fn count_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("update_time", alias)
    }

    pub fn sum_update_time(self) -> Self {
        self.sum_update_time_as("sum_update_time")
    }

    pub fn sum_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("update_time", alias)
    }

    pub fn avg_update_time(self) -> Self {
        self.avg_update_time_as("avg_update_time")
    }

    pub fn avg_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("update_time", alias)
    }

    pub fn min_update_time(self) -> Self {
        self.min_update_time_as("min_update_time")
    }

    pub fn min_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("update_time", alias)
    }

    pub fn max_update_time(self) -> Self {
        self.max_update_time_as("max_update_time")
    }

    pub fn max_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("update_time", alias)
    }

    pub fn unselect_update_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "update_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "update_time");
        self
    }


    pub fn with_update_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_update_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_update_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("update_time", value));
        self
    }



    pub fn with_update_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("update_time", value));
        self
    }

    pub fn with_update_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("update_time", value));
        self
    }

    pub fn with_update_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("update_time", value));
        self
    }

    pub fn with_update_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("update_time", lower, upper));
        self
    }

    pub fn with_update_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "update_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_update_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("update_time"));
        self
    }



    pub fn with_update_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("update_time"));
        self
    }


    pub fn order_by_update_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("update_time");
        self
    }

    pub fn order_by_update_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("update_time");
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
    pub fn filter_by_platform(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("platform_id", value.entity_id_value()));
        self
    }

    pub fn with_platform_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "platform_id",
            <crate::Platform as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("platform", selection));
        self
    }


    pub fn without_platform_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "platform_id",
            <crate::Platform as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("platform", selection));
        self
    }


    pub fn have_platform(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("platform_id"));
        self
    }

    pub fn have_no_platform(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("platform_id"));
        self
    }


    pub fn group_by_platform(self) -> Self {
        self.group_by("platform_id")
    }

    pub fn group_by_platform_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("platform_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("platform_id"));
        request
    }

    pub fn group_by_platform_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("platform_id")
            .aggregate_with_function("platform_id", alias, function)
    }

    pub fn group_by_platform_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("platform_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "platform",
            "platform_id",
            request,
        ));
        self
    }

    pub fn group_by_platform_with_details(self) -> Self {
        self.group_by_platform_with_details_from(crate::Q::platforms().unlimited())
    }

    pub fn group_by_platform_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_platform_with(request)
    }


    pub fn roll_up_to_platform(self) -> Self {
        self.roll_up_to_platform_with(crate::Q::platforms().unlimited())
    }

    pub fn roll_up_to_platform_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_platform_matching(selection.clone())
            .group_by_platform_with(selection)
    }

    pub fn count_platform(self) -> Self {
        self.count_platform_as("platform_count")
    }

    pub fn count_platform_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("platform_id", alias)
    }

    pub fn unselect_platform(mut self) -> Self {
        self.query.projection.retain(|field| field != "platform_id");
        self.query.relations.retain(|relation| relation.name != "platform");
        self
    }
    pub fn select_platform(mut self) -> Self {
        self.query = self.query.relation("platform");
        self
    }

    pub fn select_platform_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("platform", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("platform", selection));
        self
}

    pub fn facet_by_platform_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_platform_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_platform_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "platform",
            request,
            include_all_facets,
        ));
        self
    }
    pub fn have_move_quotes(self) -> Self {
        self.with_move_quote_list_matching(SelectQuery::new("MoveQuote"))
    }

    pub fn have_no_move_quotes(self) -> Self {
        self.without_move_quote_list_matching(SelectQuery::new("MoveQuote"))
    }

    pub fn with_move_quote_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MoveQuote as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_quote_list", selection));
        self
    }

    pub fn without_move_quote_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MoveQuote as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_quote_list", selection));
        self
    }

    pub fn select_move_quote_list(mut self) -> Self {
        self.query = self.query.relation("move_quote_list");
        self
    }

    pub fn select_move_quote_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("move_quote_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("move_quote_list", selection));
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
            "merchant_id",
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
            "merchant_id",
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

    pub fn have_route_stops(self) -> Self {
        self.with_route_stop_list_matching(SelectQuery::new("RouteStop"))
    }

    pub fn have_no_route_stops(self) -> Self {
        self.without_route_stop_list_matching(SelectQuery::new("RouteStop"))
    }

    pub fn with_route_stop_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::RouteStop as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("route_stop_list", selection));
        self
    }

    pub fn without_route_stop_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::RouteStop as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("route_stop_list", selection));
        self
    }

    pub fn select_route_stop_list(mut self) -> Self {
        self.query = self.query.relation("route_stop_list");
        self
    }

    pub fn select_route_stop_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("route_stop_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("route_stop_list", selection));
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
            "merchant_id",
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
            "merchant_id",
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

    pub fn have_crew_member_assignments(self) -> Self {
        self.with_crew_member_assignment_list_matching(SelectQuery::new("CrewMemberAssignment"))
    }

    pub fn have_no_crew_member_assignments(self) -> Self {
        self.without_crew_member_assignment_list_matching(SelectQuery::new("CrewMemberAssignment"))
    }

    pub fn with_crew_member_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CrewMemberAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("crew_member_assignment_list", selection));
        self
    }

    pub fn without_crew_member_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CrewMemberAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("crew_member_assignment_list", selection));
        self
    }

    pub fn select_crew_member_assignment_list(mut self) -> Self {
        self.query = self.query.relation("crew_member_assignment_list");
        self
    }

    pub fn select_crew_member_assignment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("crew_member_assignment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("crew_member_assignment_list", selection));
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
            "merchant_id",
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
            "merchant_id",
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

    pub fn have_vehicle_assignments(self) -> Self {
        self.with_vehicle_assignment_list_matching(SelectQuery::new("VehicleAssignment"))
    }

    pub fn have_no_vehicle_assignments(self) -> Self {
        self.without_vehicle_assignment_list_matching(SelectQuery::new("VehicleAssignment"))
    }

    pub fn with_vehicle_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::VehicleAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("vehicle_assignment_list", selection));
        self
    }

    pub fn without_vehicle_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::VehicleAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("vehicle_assignment_list", selection));
        self
    }

    pub fn select_vehicle_assignment_list(mut self) -> Self {
        self.query = self.query.relation("vehicle_assignment_list");
        self
    }

    pub fn select_vehicle_assignment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("vehicle_assignment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("vehicle_assignment_list", selection));
        self
}

    pub fn have_dispatch_assignments(self) -> Self {
        self.with_dispatch_assignment_list_matching(SelectQuery::new("DispatchAssignment"))
    }

    pub fn have_no_dispatch_assignments(self) -> Self {
        self.without_dispatch_assignment_list_matching(SelectQuery::new("DispatchAssignment"))
    }

    pub fn with_dispatch_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DispatchAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("dispatch_assignment_list", selection));
        self
    }

    pub fn without_dispatch_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DispatchAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("dispatch_assignment_list", selection));
        self
    }

    pub fn select_dispatch_assignment_list(mut self) -> Self {
        self.query = self.query.relation("dispatch_assignment_list");
        self
    }

    pub fn select_dispatch_assignment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("dispatch_assignment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("dispatch_assignment_list", selection));
        self
}

    pub fn have_damage_reports(self) -> Self {
        self.with_damage_report_list_matching(SelectQuery::new("DamageReport"))
    }

    pub fn have_no_damage_reports(self) -> Self {
        self.without_damage_report_list_matching(SelectQuery::new("DamageReport"))
    }

    pub fn with_damage_report_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DamageReport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("damage_report_list", selection));
        self
    }

    pub fn without_damage_report_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DamageReport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("damage_report_list", selection));
        self
    }

    pub fn select_damage_report_list(mut self) -> Self {
        self.query = self.query.relation("damage_report_list");
        self
    }

    pub fn select_damage_report_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("damage_report_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("damage_report_list", selection));
        self
}

    pub fn have_proof_of_deliveries(self) -> Self {
        self.with_proof_of_delivery_list_matching(SelectQuery::new("ProofOfDelivery"))
    }

    pub fn have_no_proof_of_deliveries(self) -> Self {
        self.without_proof_of_delivery_list_matching(SelectQuery::new("ProofOfDelivery"))
    }

    pub fn with_proof_of_delivery_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ProofOfDelivery as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("proof_of_delivery_list", selection));
        self
    }

    pub fn without_proof_of_delivery_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ProofOfDelivery as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("proof_of_delivery_list", selection));
        self
    }

    pub fn select_proof_of_delivery_list(mut self) -> Self {
        self.query = self.query.relation("proof_of_delivery_list");
        self
    }

    pub fn select_proof_of_delivery_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("proof_of_delivery_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("proof_of_delivery_list", selection));
        self
}

    pub fn have_operational_exceptions(self) -> Self {
        self.with_operational_exception_list_matching(SelectQuery::new("OperationalException"))
    }

    pub fn have_no_operational_exceptions(self) -> Self {
        self.without_operational_exception_list_matching(SelectQuery::new("OperationalException"))
    }

    pub fn with_operational_exception_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::OperationalException as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("operational_exception_list", selection));
        self
    }

    pub fn without_operational_exception_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::OperationalException as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("operational_exception_list", selection));
        self
    }

    pub fn select_operational_exception_list(mut self) -> Self {
        self.query = self.query.relation("operational_exception_list");
        self
    }

    pub fn select_operational_exception_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("operational_exception_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("operational_exception_list", selection));
        self
}

    pub fn have_pickup_instructions(self) -> Self {
        self.with_pickup_instruction_list_matching(SelectQuery::new("PickupInstruction"))
    }

    pub fn have_no_pickup_instructions(self) -> Self {
        self.without_pickup_instruction_list_matching(SelectQuery::new("PickupInstruction"))
    }

    pub fn with_pickup_instruction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PickupInstruction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("pickup_instruction_list", selection));
        self
    }

    pub fn without_pickup_instruction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PickupInstruction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("pickup_instruction_list", selection));
        self
    }

    pub fn select_pickup_instruction_list(mut self) -> Self {
        self.query = self.query.relation("pickup_instruction_list");
        self
    }

    pub fn select_pickup_instruction_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("pickup_instruction_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("pickup_instruction_list", selection));
        self
}

    pub fn have_delivery_instructions(self) -> Self {
        self.with_delivery_instruction_list_matching(SelectQuery::new("DeliveryInstruction"))
    }

    pub fn have_no_delivery_instructions(self) -> Self {
        self.without_delivery_instruction_list_matching(SelectQuery::new("DeliveryInstruction"))
    }

    pub fn with_delivery_instruction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DeliveryInstruction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("delivery_instruction_list", selection));
        self
    }

    pub fn without_delivery_instruction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DeliveryInstruction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("delivery_instruction_list", selection));
        self
    }

    pub fn select_delivery_instruction_list(mut self) -> Self {
        self.query = self.query.relation("delivery_instruction_list");
        self
    }

    pub fn select_delivery_instruction_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("delivery_instruction_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("delivery_instruction_list", selection));
        self
}

    pub fn have_move_inventory(self) -> Self {
        self.with_move_inventory_list_matching(SelectQuery::new("MoveInventory"))
    }

    pub fn have_no_move_inventory(self) -> Self {
        self.without_move_inventory_list_matching(SelectQuery::new("MoveInventory"))
    }

    pub fn with_move_inventory_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MoveInventory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_inventory_list", selection));
        self
    }

    pub fn without_move_inventory_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MoveInventory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_inventory_list", selection));
        self
    }

    pub fn select_move_inventory_list(mut self) -> Self {
        self.query = self.query.relation("move_inventory_list");
        self
    }

    pub fn select_move_inventory_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("move_inventory_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("move_inventory_list", selection));
        self
}

    pub fn have_packaging_items(self) -> Self {
        self.with_packaging_item_list_matching(SelectQuery::new("PackagingItem"))
    }

    pub fn have_no_packaging_items(self) -> Self {
        self.without_packaging_item_list_matching(SelectQuery::new("PackagingItem"))
    }

    pub fn with_packaging_item_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PackagingItem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("packaging_item_list", selection));
        self
    }

    pub fn without_packaging_item_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PackagingItem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("packaging_item_list", selection));
        self
    }

    pub fn select_packaging_item_list(mut self) -> Self {
        self.query = self.query.relation("packaging_item_list");
        self
    }

    pub fn select_packaging_item_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("packaging_item_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("packaging_item_list", selection));
        self
}

    pub fn have_logistics_providers(self) -> Self {
        self.with_logistics_provider_list_matching(SelectQuery::new("LogisticsProvider"))
    }

    pub fn have_no_logistics_providers(self) -> Self {
        self.without_logistics_provider_list_matching(SelectQuery::new("LogisticsProvider"))
    }

    pub fn with_logistics_provider_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::LogisticsProvider as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("logistics_provider_list", selection));
        self
    }

    pub fn without_logistics_provider_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::LogisticsProvider as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("logistics_provider_list", selection));
        self
    }

    pub fn select_logistics_provider_list(mut self) -> Self {
        self.query = self.query.relation("logistics_provider_list");
        self
    }

    pub fn select_logistics_provider_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("logistics_provider_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("logistics_provider_list", selection));
        self
}

    pub fn have_third_party_dispatches(self) -> Self {
        self.with_third_party_dispatch_list_matching(SelectQuery::new("ThirdPartyDispatch"))
    }

    pub fn have_no_third_party_dispatches(self) -> Self {
        self.without_third_party_dispatch_list_matching(SelectQuery::new("ThirdPartyDispatch"))
    }

    pub fn with_third_party_dispatch_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ThirdPartyDispatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("third_party_dispatch_list", selection));
        self
    }

    pub fn without_third_party_dispatch_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ThirdPartyDispatch as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("third_party_dispatch_list", selection));
        self
    }

    pub fn select_third_party_dispatch_list(mut self) -> Self {
        self.query = self.query.relation("third_party_dispatch_list");
        self
    }

    pub fn select_third_party_dispatch_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("third_party_dispatch_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("third_party_dispatch_list", selection));
        self
}

    pub fn have_fuel_logs(self) -> Self {
        self.with_fuel_log_list_matching(SelectQuery::new("FuelLog"))
    }

    pub fn have_no_fuel_logs(self) -> Self {
        self.without_fuel_log_list_matching(SelectQuery::new("FuelLog"))
    }

    pub fn with_fuel_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::FuelLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("fuel_log_list", selection));
        self
    }

    pub fn without_fuel_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::FuelLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("fuel_log_list", selection));
        self
    }

    pub fn select_fuel_log_list(mut self) -> Self {
        self.query = self.query.relation("fuel_log_list");
        self
    }

    pub fn select_fuel_log_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("fuel_log_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("fuel_log_list", selection));
        self
}

    pub fn have_maintenance_records(self) -> Self {
        self.with_maintenance_record_list_matching(SelectQuery::new("MaintenanceRecord"))
    }

    pub fn have_no_maintenance_records(self) -> Self {
        self.without_maintenance_record_list_matching(SelectQuery::new("MaintenanceRecord"))
    }

    pub fn with_maintenance_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MaintenanceRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("maintenance_record_list", selection));
        self
    }

    pub fn without_maintenance_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MaintenanceRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("maintenance_record_list", selection));
        self
    }

    pub fn select_maintenance_record_list(mut self) -> Self {
        self.query = self.query.relation("maintenance_record_list");
        self
    }

    pub fn select_maintenance_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("maintenance_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("maintenance_record_list", selection));
        self
}

    pub fn have_toll_receipts(self) -> Self {
        self.with_toll_receipt_list_matching(SelectQuery::new("TollReceipt"))
    }

    pub fn have_no_toll_receipts(self) -> Self {
        self.without_toll_receipt_list_matching(SelectQuery::new("TollReceipt"))
    }

    pub fn with_toll_receipt_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TollReceipt as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("toll_receipt_list", selection));
        self
    }

    pub fn without_toll_receipt_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TollReceipt as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("toll_receipt_list", selection));
        self
    }

    pub fn select_toll_receipt_list(mut self) -> Self {
        self.query = self.query.relation("toll_receipt_list");
        self
    }

    pub fn select_toll_receipt_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("toll_receipt_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("toll_receipt_list", selection));
        self
}

    pub fn have_shift_logs(self) -> Self {
        self.with_shift_log_list_matching(SelectQuery::new("ShiftLog"))
    }

    pub fn have_no_shift_logs(self) -> Self {
        self.without_shift_log_list_matching(SelectQuery::new("ShiftLog"))
    }

    pub fn with_shift_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ShiftLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("shift_log_list", selection));
        self
    }

    pub fn without_shift_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ShiftLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("shift_log_list", selection));
        self
    }

    pub fn select_shift_log_list(mut self) -> Self {
        self.query = self.query.relation("shift_log_list");
        self
    }

    pub fn select_shift_log_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("shift_log_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("shift_log_list", selection));
        self
}

    pub fn have_customer_feedback(self) -> Self {
        self.with_customer_feedback_list_matching(SelectQuery::new("CustomerFeedback"))
    }

    pub fn have_no_customer_feedback(self) -> Self {
        self.without_customer_feedback_list_matching(SelectQuery::new("CustomerFeedback"))
    }

    pub fn with_customer_feedback_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CustomerFeedback as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_feedback_list", selection));
        self
    }

    pub fn without_customer_feedback_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CustomerFeedback as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_feedback_list", selection));
        self
    }

    pub fn select_customer_feedback_list(mut self) -> Self {
        self.query = self.query.relation("customer_feedback_list");
        self
    }

    pub fn select_customer_feedback_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_feedback_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_feedback_list", selection));
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
            "merchant_id",
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
            "merchant_id",
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
    pub fn count_move_quotes(self) -> Self {
        self.count_move_quotes_as("count_move_quotes")
    }

    pub fn count_move_quotes_as(self, alias: impl Into<String>) -> Self {
        self.count_move_quotes_with(alias, crate::Q::move_quotes().unlimited())
    }

    pub fn count_move_quotes_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_quote_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_move_quotes(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_quotes_as("refinements", request)
    }

    pub fn stats_from_move_quotes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_quote_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_move_quotes_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_quotes(request)
    }


    pub fn min_create_time_of_move_quotes(self) -> Self {
        self.min_create_time_of_move_quotes_as("min_create_time_of_move_quotes", crate::Q::move_quotes().unlimited())
    }

    pub fn min_create_time_of_move_quotes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_quotes_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_move_quotes(self) -> Self {
        self.max_create_time_of_move_quotes_as("max_create_time_of_move_quotes", crate::Q::move_quotes().unlimited())
    }

    pub fn max_create_time_of_move_quotes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_quotes_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_move_quotes(self) -> Self {
        self.min_update_time_of_move_quotes_as("min_update_time_of_move_quotes", crate::Q::move_quotes().unlimited())
    }

    pub fn min_update_time_of_move_quotes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_quotes_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_move_quotes(self) -> Self {
        self.max_update_time_of_move_quotes_as("max_update_time_of_move_quotes", crate::Q::move_quotes().unlimited())
    }

    pub fn max_update_time_of_move_quotes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_quotes_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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


    pub fn min_create_time_of_move_orders(self) -> Self {
        self.min_create_time_of_move_orders_as("min_create_time_of_move_orders", crate::Q::move_orders().unlimited())
    }

    pub fn min_create_time_of_move_orders_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_move_orders(self) -> Self {
        self.max_create_time_of_move_orders_as("max_create_time_of_move_orders", crate::Q::move_orders().unlimited())
    }

    pub fn max_create_time_of_move_orders_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_move_orders(self) -> Self {
        self.min_update_time_of_move_orders_as("min_update_time_of_move_orders", crate::Q::move_orders().unlimited())
    }

    pub fn min_update_time_of_move_orders_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_move_orders(self) -> Self {
        self.max_update_time_of_move_orders_as("max_update_time_of_move_orders", crate::Q::move_orders().unlimited())
    }

    pub fn max_update_time_of_move_orders_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_route_stops(self) -> Self {
        self.count_route_stops_as("count_route_stops")
    }

    pub fn count_route_stops_as(self, alias: impl Into<String>) -> Self {
        self.count_route_stops_with(alias, crate::Q::route_stops().unlimited())
    }

    pub fn count_route_stops_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "route_stop_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_route_stops(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_stops_as("refinements", request)
    }

    pub fn stats_from_route_stops_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "route_stop_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_route_stops_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_stops(request)
    }


    pub fn min_create_time_of_route_stops(self) -> Self {
        self.min_create_time_of_route_stops_as("min_create_time_of_route_stops", crate::Q::route_stops().unlimited())
    }

    pub fn min_create_time_of_route_stops_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_stops_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_route_stops(self) -> Self {
        self.max_create_time_of_route_stops_as("max_create_time_of_route_stops", crate::Q::route_stops().unlimited())
    }

    pub fn max_create_time_of_route_stops_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_stops_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_route_stops(self) -> Self {
        self.min_update_time_of_route_stops_as("min_update_time_of_route_stops", crate::Q::route_stops().unlimited())
    }

    pub fn min_update_time_of_route_stops_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_stops_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_route_stops(self) -> Self {
        self.max_update_time_of_route_stops_as("max_update_time_of_route_stops", crate::Q::route_stops().unlimited())
    }

    pub fn max_update_time_of_route_stops_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_route_stops_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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


    pub fn min_create_time_of_crews(self) -> Self {
        self.min_create_time_of_crews_as("min_create_time_of_crews", crate::Q::crews().unlimited())
    }

    pub fn min_create_time_of_crews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crews_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_crews(self) -> Self {
        self.max_create_time_of_crews_as("max_create_time_of_crews", crate::Q::crews().unlimited())
    }

    pub fn max_create_time_of_crews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crews_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_crews(self) -> Self {
        self.min_update_time_of_crews_as("min_update_time_of_crews", crate::Q::crews().unlimited())
    }

    pub fn min_update_time_of_crews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crews_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_crews(self) -> Self {
        self.max_update_time_of_crews_as("max_update_time_of_crews", crate::Q::crews().unlimited())
    }

    pub fn max_update_time_of_crews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crews_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_crew_member_assignments(self) -> Self {
        self.count_crew_member_assignments_as("count_crew_member_assignments")
    }

    pub fn count_crew_member_assignments_as(self, alias: impl Into<String>) -> Self {
        self.count_crew_member_assignments_with(alias, crate::Q::crew_member_assignments().unlimited())
    }

    pub fn count_crew_member_assignments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "crew_member_assignment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_crew_member_assignments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crew_member_assignments_as("refinements", request)
    }

    pub fn stats_from_crew_member_assignments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "crew_member_assignment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_crew_member_assignments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crew_member_assignments(request)
    }


    pub fn min_create_time_of_crew_member_assignments(self) -> Self {
        self.min_create_time_of_crew_member_assignments_as("min_create_time_of_crew_member_assignments", crate::Q::crew_member_assignments().unlimited())
    }

    pub fn min_create_time_of_crew_member_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crew_member_assignments_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_crew_member_assignments(self) -> Self {
        self.max_create_time_of_crew_member_assignments_as("max_create_time_of_crew_member_assignments", crate::Q::crew_member_assignments().unlimited())
    }

    pub fn max_create_time_of_crew_member_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crew_member_assignments_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_crew_member_assignments(self) -> Self {
        self.min_update_time_of_crew_member_assignments_as("min_update_time_of_crew_member_assignments", crate::Q::crew_member_assignments().unlimited())
    }

    pub fn min_update_time_of_crew_member_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crew_member_assignments_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_crew_member_assignments(self) -> Self {
        self.max_update_time_of_crew_member_assignments_as("max_update_time_of_crew_member_assignments", crate::Q::crew_member_assignments().unlimited())
    }

    pub fn max_update_time_of_crew_member_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_crew_member_assignments_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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


    pub fn min_create_time_of_vehicles(self) -> Self {
        self.min_create_time_of_vehicles_as("min_create_time_of_vehicles", crate::Q::vehicles().unlimited())
    }

    pub fn min_create_time_of_vehicles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicles_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_vehicles(self) -> Self {
        self.max_create_time_of_vehicles_as("max_create_time_of_vehicles", crate::Q::vehicles().unlimited())
    }

    pub fn max_create_time_of_vehicles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicles_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_vehicles(self) -> Self {
        self.min_update_time_of_vehicles_as("min_update_time_of_vehicles", crate::Q::vehicles().unlimited())
    }

    pub fn min_update_time_of_vehicles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicles_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_vehicles(self) -> Self {
        self.max_update_time_of_vehicles_as("max_update_time_of_vehicles", crate::Q::vehicles().unlimited())
    }

    pub fn max_update_time_of_vehicles_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicles_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_vehicle_assignments(self) -> Self {
        self.count_vehicle_assignments_as("count_vehicle_assignments")
    }

    pub fn count_vehicle_assignments_as(self, alias: impl Into<String>) -> Self {
        self.count_vehicle_assignments_with(alias, crate::Q::vehicle_assignments().unlimited())
    }

    pub fn count_vehicle_assignments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vehicle_assignment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_vehicle_assignments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_assignments_as("refinements", request)
    }

    pub fn stats_from_vehicle_assignments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vehicle_assignment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_vehicle_assignments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_assignments(request)
    }


    pub fn min_create_time_of_vehicle_assignments(self) -> Self {
        self.min_create_time_of_vehicle_assignments_as("min_create_time_of_vehicle_assignments", crate::Q::vehicle_assignments().unlimited())
    }

    pub fn min_create_time_of_vehicle_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_assignments_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_vehicle_assignments(self) -> Self {
        self.max_create_time_of_vehicle_assignments_as("max_create_time_of_vehicle_assignments", crate::Q::vehicle_assignments().unlimited())
    }

    pub fn max_create_time_of_vehicle_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_assignments_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_vehicle_assignments(self) -> Self {
        self.min_update_time_of_vehicle_assignments_as("min_update_time_of_vehicle_assignments", crate::Q::vehicle_assignments().unlimited())
    }

    pub fn min_update_time_of_vehicle_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_assignments_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_vehicle_assignments(self) -> Self {
        self.max_update_time_of_vehicle_assignments_as("max_update_time_of_vehicle_assignments", crate::Q::vehicle_assignments().unlimited())
    }

    pub fn max_update_time_of_vehicle_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_assignments_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_dispatch_assignments(self) -> Self {
        self.count_dispatch_assignments_as("count_dispatch_assignments")
    }

    pub fn count_dispatch_assignments_as(self, alias: impl Into<String>) -> Self {
        self.count_dispatch_assignments_with(alias, crate::Q::dispatch_assignments().unlimited())
    }

    pub fn count_dispatch_assignments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "dispatch_assignment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_dispatch_assignments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_dispatch_assignments_as("refinements", request)
    }

    pub fn stats_from_dispatch_assignments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "dispatch_assignment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_dispatch_assignments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_dispatch_assignments(request)
    }


    pub fn min_create_time_of_dispatch_assignments(self) -> Self {
        self.min_create_time_of_dispatch_assignments_as("min_create_time_of_dispatch_assignments", crate::Q::dispatch_assignments().unlimited())
    }

    pub fn min_create_time_of_dispatch_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_dispatch_assignments_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_dispatch_assignments(self) -> Self {
        self.max_create_time_of_dispatch_assignments_as("max_create_time_of_dispatch_assignments", crate::Q::dispatch_assignments().unlimited())
    }

    pub fn max_create_time_of_dispatch_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_dispatch_assignments_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_dispatch_assignments(self) -> Self {
        self.min_update_time_of_dispatch_assignments_as("min_update_time_of_dispatch_assignments", crate::Q::dispatch_assignments().unlimited())
    }

    pub fn min_update_time_of_dispatch_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_dispatch_assignments_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_dispatch_assignments(self) -> Self {
        self.max_update_time_of_dispatch_assignments_as("max_update_time_of_dispatch_assignments", crate::Q::dispatch_assignments().unlimited())
    }

    pub fn max_update_time_of_dispatch_assignments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_dispatch_assignments_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_damage_reports(self) -> Self {
        self.count_damage_reports_as("count_damage_reports")
    }

    pub fn count_damage_reports_as(self, alias: impl Into<String>) -> Self {
        self.count_damage_reports_with(alias, crate::Q::damage_reports().unlimited())
    }

    pub fn count_damage_reports_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "damage_report_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_damage_reports(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_damage_reports_as("refinements", request)
    }

    pub fn stats_from_damage_reports_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "damage_report_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_damage_reports_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_damage_reports(request)
    }


    pub fn min_create_time_of_damage_reports(self) -> Self {
        self.min_create_time_of_damage_reports_as("min_create_time_of_damage_reports", crate::Q::damage_reports().unlimited())
    }

    pub fn min_create_time_of_damage_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_damage_reports_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_damage_reports(self) -> Self {
        self.max_create_time_of_damage_reports_as("max_create_time_of_damage_reports", crate::Q::damage_reports().unlimited())
    }

    pub fn max_create_time_of_damage_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_damage_reports_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_damage_reports(self) -> Self {
        self.min_update_time_of_damage_reports_as("min_update_time_of_damage_reports", crate::Q::damage_reports().unlimited())
    }

    pub fn min_update_time_of_damage_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_damage_reports_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_damage_reports(self) -> Self {
        self.max_update_time_of_damage_reports_as("max_update_time_of_damage_reports", crate::Q::damage_reports().unlimited())
    }

    pub fn max_update_time_of_damage_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_damage_reports_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_proof_of_deliveries(self) -> Self {
        self.count_proof_of_deliveries_as("count_proof_of_deliveries")
    }

    pub fn count_proof_of_deliveries_as(self, alias: impl Into<String>) -> Self {
        self.count_proof_of_deliveries_with(alias, crate::Q::proof_of_deliveries().unlimited())
    }

    pub fn count_proof_of_deliveries_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "proof_of_delivery_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_proof_of_deliveries(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proof_of_deliveries_as("refinements", request)
    }

    pub fn stats_from_proof_of_deliveries_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "proof_of_delivery_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_proof_of_deliveries_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proof_of_deliveries(request)
    }


    pub fn min_create_time_of_proof_of_deliveries(self) -> Self {
        self.min_create_time_of_proof_of_deliveries_as("min_create_time_of_proof_of_deliveries", crate::Q::proof_of_deliveries().unlimited())
    }

    pub fn min_create_time_of_proof_of_deliveries_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proof_of_deliveries_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_proof_of_deliveries(self) -> Self {
        self.max_create_time_of_proof_of_deliveries_as("max_create_time_of_proof_of_deliveries", crate::Q::proof_of_deliveries().unlimited())
    }

    pub fn max_create_time_of_proof_of_deliveries_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proof_of_deliveries_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_proof_of_deliveries(self) -> Self {
        self.min_update_time_of_proof_of_deliveries_as("min_update_time_of_proof_of_deliveries", crate::Q::proof_of_deliveries().unlimited())
    }

    pub fn min_update_time_of_proof_of_deliveries_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proof_of_deliveries_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_proof_of_deliveries(self) -> Self {
        self.max_update_time_of_proof_of_deliveries_as("max_update_time_of_proof_of_deliveries", crate::Q::proof_of_deliveries().unlimited())
    }

    pub fn max_update_time_of_proof_of_deliveries_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_proof_of_deliveries_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_operational_exceptions(self) -> Self {
        self.count_operational_exceptions_as("count_operational_exceptions")
    }

    pub fn count_operational_exceptions_as(self, alias: impl Into<String>) -> Self {
        self.count_operational_exceptions_with(alias, crate::Q::operational_exceptions().unlimited())
    }

    pub fn count_operational_exceptions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "operational_exception_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_operational_exceptions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operational_exceptions_as("refinements", request)
    }

    pub fn stats_from_operational_exceptions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "operational_exception_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_operational_exceptions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operational_exceptions(request)
    }


    pub fn min_create_time_of_operational_exceptions(self) -> Self {
        self.min_create_time_of_operational_exceptions_as("min_create_time_of_operational_exceptions", crate::Q::operational_exceptions().unlimited())
    }

    pub fn min_create_time_of_operational_exceptions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operational_exceptions_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_operational_exceptions(self) -> Self {
        self.max_create_time_of_operational_exceptions_as("max_create_time_of_operational_exceptions", crate::Q::operational_exceptions().unlimited())
    }

    pub fn max_create_time_of_operational_exceptions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operational_exceptions_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_operational_exceptions(self) -> Self {
        self.min_update_time_of_operational_exceptions_as("min_update_time_of_operational_exceptions", crate::Q::operational_exceptions().unlimited())
    }

    pub fn min_update_time_of_operational_exceptions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operational_exceptions_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_operational_exceptions(self) -> Self {
        self.max_update_time_of_operational_exceptions_as("max_update_time_of_operational_exceptions", crate::Q::operational_exceptions().unlimited())
    }

    pub fn max_update_time_of_operational_exceptions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operational_exceptions_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_pickup_instructions(self) -> Self {
        self.count_pickup_instructions_as("count_pickup_instructions")
    }

    pub fn count_pickup_instructions_as(self, alias: impl Into<String>) -> Self {
        self.count_pickup_instructions_with(alias, crate::Q::pickup_instructions().unlimited())
    }

    pub fn count_pickup_instructions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "pickup_instruction_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_pickup_instructions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_pickup_instructions_as("refinements", request)
    }

    pub fn stats_from_pickup_instructions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "pickup_instruction_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_pickup_instructions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_pickup_instructions(request)
    }


    pub fn min_create_time_of_pickup_instructions(self) -> Self {
        self.min_create_time_of_pickup_instructions_as("min_create_time_of_pickup_instructions", crate::Q::pickup_instructions().unlimited())
    }

    pub fn min_create_time_of_pickup_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_pickup_instructions_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_pickup_instructions(self) -> Self {
        self.max_create_time_of_pickup_instructions_as("max_create_time_of_pickup_instructions", crate::Q::pickup_instructions().unlimited())
    }

    pub fn max_create_time_of_pickup_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_pickup_instructions_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_pickup_instructions(self) -> Self {
        self.min_update_time_of_pickup_instructions_as("min_update_time_of_pickup_instructions", crate::Q::pickup_instructions().unlimited())
    }

    pub fn min_update_time_of_pickup_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_pickup_instructions_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_pickup_instructions(self) -> Self {
        self.max_update_time_of_pickup_instructions_as("max_update_time_of_pickup_instructions", crate::Q::pickup_instructions().unlimited())
    }

    pub fn max_update_time_of_pickup_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_pickup_instructions_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_delivery_instructions(self) -> Self {
        self.count_delivery_instructions_as("count_delivery_instructions")
    }

    pub fn count_delivery_instructions_as(self, alias: impl Into<String>) -> Self {
        self.count_delivery_instructions_with(alias, crate::Q::delivery_instructions().unlimited())
    }

    pub fn count_delivery_instructions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "delivery_instruction_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_delivery_instructions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_instructions_as("refinements", request)
    }

    pub fn stats_from_delivery_instructions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "delivery_instruction_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_delivery_instructions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_instructions(request)
    }


    pub fn min_create_time_of_delivery_instructions(self) -> Self {
        self.min_create_time_of_delivery_instructions_as("min_create_time_of_delivery_instructions", crate::Q::delivery_instructions().unlimited())
    }

    pub fn min_create_time_of_delivery_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_instructions_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_delivery_instructions(self) -> Self {
        self.max_create_time_of_delivery_instructions_as("max_create_time_of_delivery_instructions", crate::Q::delivery_instructions().unlimited())
    }

    pub fn max_create_time_of_delivery_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_instructions_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_delivery_instructions(self) -> Self {
        self.min_update_time_of_delivery_instructions_as("min_update_time_of_delivery_instructions", crate::Q::delivery_instructions().unlimited())
    }

    pub fn min_update_time_of_delivery_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_instructions_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_delivery_instructions(self) -> Self {
        self.max_update_time_of_delivery_instructions_as("max_update_time_of_delivery_instructions", crate::Q::delivery_instructions().unlimited())
    }

    pub fn max_update_time_of_delivery_instructions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_instructions_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_move_inventory(self) -> Self {
        self.count_move_inventory_as("count_move_inventory")
    }

    pub fn count_move_inventory_as(self, alias: impl Into<String>) -> Self {
        self.count_move_inventory_with(alias, crate::Q::move_inventory().unlimited())
    }

    pub fn count_move_inventory_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_inventory_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_move_inventory(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_inventory_as("refinements", request)
    }

    pub fn stats_from_move_inventory_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_inventory_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_move_inventory_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_inventory(request)
    }


    pub fn min_create_time_of_move_inventory(self) -> Self {
        self.min_create_time_of_move_inventory_as("min_create_time_of_move_inventory", crate::Q::move_inventory().unlimited())
    }

    pub fn min_create_time_of_move_inventory_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_inventory_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_move_inventory(self) -> Self {
        self.max_create_time_of_move_inventory_as("max_create_time_of_move_inventory", crate::Q::move_inventory().unlimited())
    }

    pub fn max_create_time_of_move_inventory_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_inventory_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_move_inventory(self) -> Self {
        self.min_update_time_of_move_inventory_as("min_update_time_of_move_inventory", crate::Q::move_inventory().unlimited())
    }

    pub fn min_update_time_of_move_inventory_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_inventory_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_move_inventory(self) -> Self {
        self.max_update_time_of_move_inventory_as("max_update_time_of_move_inventory", crate::Q::move_inventory().unlimited())
    }

    pub fn max_update_time_of_move_inventory_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_inventory_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_packaging_items(self) -> Self {
        self.count_packaging_items_as("count_packaging_items")
    }

    pub fn count_packaging_items_as(self, alias: impl Into<String>) -> Self {
        self.count_packaging_items_with(alias, crate::Q::packaging_items().unlimited())
    }

    pub fn count_packaging_items_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "packaging_item_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_packaging_items(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packaging_items_as("refinements", request)
    }

    pub fn stats_from_packaging_items_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "packaging_item_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_packaging_items_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packaging_items(request)
    }


    pub fn min_create_time_of_packaging_items(self) -> Self {
        self.min_create_time_of_packaging_items_as("min_create_time_of_packaging_items", crate::Q::packaging_items().unlimited())
    }

    pub fn min_create_time_of_packaging_items_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packaging_items_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_packaging_items(self) -> Self {
        self.max_create_time_of_packaging_items_as("max_create_time_of_packaging_items", crate::Q::packaging_items().unlimited())
    }

    pub fn max_create_time_of_packaging_items_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packaging_items_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_packaging_items(self) -> Self {
        self.min_update_time_of_packaging_items_as("min_update_time_of_packaging_items", crate::Q::packaging_items().unlimited())
    }

    pub fn min_update_time_of_packaging_items_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packaging_items_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_packaging_items(self) -> Self {
        self.max_update_time_of_packaging_items_as("max_update_time_of_packaging_items", crate::Q::packaging_items().unlimited())
    }

    pub fn max_update_time_of_packaging_items_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_packaging_items_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_logistics_providers(self) -> Self {
        self.count_logistics_providers_as("count_logistics_providers")
    }

    pub fn count_logistics_providers_as(self, alias: impl Into<String>) -> Self {
        self.count_logistics_providers_with(alias, crate::Q::logistics_providers().unlimited())
    }

    pub fn count_logistics_providers_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "logistics_provider_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_logistics_providers(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_logistics_providers_as("refinements", request)
    }

    pub fn stats_from_logistics_providers_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "logistics_provider_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_logistics_providers_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_logistics_providers(request)
    }


    pub fn min_create_time_of_logistics_providers(self) -> Self {
        self.min_create_time_of_logistics_providers_as("min_create_time_of_logistics_providers", crate::Q::logistics_providers().unlimited())
    }

    pub fn min_create_time_of_logistics_providers_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_logistics_providers_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_logistics_providers(self) -> Self {
        self.max_create_time_of_logistics_providers_as("max_create_time_of_logistics_providers", crate::Q::logistics_providers().unlimited())
    }

    pub fn max_create_time_of_logistics_providers_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_logistics_providers_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_logistics_providers(self) -> Self {
        self.min_update_time_of_logistics_providers_as("min_update_time_of_logistics_providers", crate::Q::logistics_providers().unlimited())
    }

    pub fn min_update_time_of_logistics_providers_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_logistics_providers_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_logistics_providers(self) -> Self {
        self.max_update_time_of_logistics_providers_as("max_update_time_of_logistics_providers", crate::Q::logistics_providers().unlimited())
    }

    pub fn max_update_time_of_logistics_providers_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_logistics_providers_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_third_party_dispatches(self) -> Self {
        self.count_third_party_dispatches_as("count_third_party_dispatches")
    }

    pub fn count_third_party_dispatches_as(self, alias: impl Into<String>) -> Self {
        self.count_third_party_dispatches_with(alias, crate::Q::third_party_dispatches().unlimited())
    }

    pub fn count_third_party_dispatches_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "third_party_dispatch_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_third_party_dispatches(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_third_party_dispatches_as("refinements", request)
    }

    pub fn stats_from_third_party_dispatches_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "third_party_dispatch_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_third_party_dispatches_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_third_party_dispatches(request)
    }


    pub fn min_create_time_of_third_party_dispatches(self) -> Self {
        self.min_create_time_of_third_party_dispatches_as("min_create_time_of_third_party_dispatches", crate::Q::third_party_dispatches().unlimited())
    }

    pub fn min_create_time_of_third_party_dispatches_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_third_party_dispatches_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_third_party_dispatches(self) -> Self {
        self.max_create_time_of_third_party_dispatches_as("max_create_time_of_third_party_dispatches", crate::Q::third_party_dispatches().unlimited())
    }

    pub fn max_create_time_of_third_party_dispatches_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_third_party_dispatches_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_third_party_dispatches(self) -> Self {
        self.min_update_time_of_third_party_dispatches_as("min_update_time_of_third_party_dispatches", crate::Q::third_party_dispatches().unlimited())
    }

    pub fn min_update_time_of_third_party_dispatches_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_third_party_dispatches_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_third_party_dispatches(self) -> Self {
        self.max_update_time_of_third_party_dispatches_as("max_update_time_of_third_party_dispatches", crate::Q::third_party_dispatches().unlimited())
    }

    pub fn max_update_time_of_third_party_dispatches_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_third_party_dispatches_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_fuel_logs(self) -> Self {
        self.count_fuel_logs_as("count_fuel_logs")
    }

    pub fn count_fuel_logs_as(self, alias: impl Into<String>) -> Self {
        self.count_fuel_logs_with(alias, crate::Q::fuel_logs().unlimited())
    }

    pub fn count_fuel_logs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "fuel_log_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_fuel_logs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_logs_as("refinements", request)
    }

    pub fn stats_from_fuel_logs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "fuel_log_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_fuel_logs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_logs(request)
    }


    pub fn min_create_time_of_fuel_logs(self) -> Self {
        self.min_create_time_of_fuel_logs_as("min_create_time_of_fuel_logs", crate::Q::fuel_logs().unlimited())
    }

    pub fn min_create_time_of_fuel_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_logs_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_fuel_logs(self) -> Self {
        self.max_create_time_of_fuel_logs_as("max_create_time_of_fuel_logs", crate::Q::fuel_logs().unlimited())
    }

    pub fn max_create_time_of_fuel_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_logs_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_fuel_logs(self) -> Self {
        self.min_update_time_of_fuel_logs_as("min_update_time_of_fuel_logs", crate::Q::fuel_logs().unlimited())
    }

    pub fn min_update_time_of_fuel_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_logs_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_fuel_logs(self) -> Self {
        self.max_update_time_of_fuel_logs_as("max_update_time_of_fuel_logs", crate::Q::fuel_logs().unlimited())
    }

    pub fn max_update_time_of_fuel_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_logs_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_maintenance_records(self) -> Self {
        self.count_maintenance_records_as("count_maintenance_records")
    }

    pub fn count_maintenance_records_as(self, alias: impl Into<String>) -> Self {
        self.count_maintenance_records_with(alias, crate::Q::maintenance_records().unlimited())
    }

    pub fn count_maintenance_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "maintenance_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_maintenance_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_records_as("refinements", request)
    }

    pub fn stats_from_maintenance_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "maintenance_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_maintenance_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_records(request)
    }


    pub fn min_create_time_of_maintenance_records(self) -> Self {
        self.min_create_time_of_maintenance_records_as("min_create_time_of_maintenance_records", crate::Q::maintenance_records().unlimited())
    }

    pub fn min_create_time_of_maintenance_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_records_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_maintenance_records(self) -> Self {
        self.max_create_time_of_maintenance_records_as("max_create_time_of_maintenance_records", crate::Q::maintenance_records().unlimited())
    }

    pub fn max_create_time_of_maintenance_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_records_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_maintenance_records(self) -> Self {
        self.min_update_time_of_maintenance_records_as("min_update_time_of_maintenance_records", crate::Q::maintenance_records().unlimited())
    }

    pub fn min_update_time_of_maintenance_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_records_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_maintenance_records(self) -> Self {
        self.max_update_time_of_maintenance_records_as("max_update_time_of_maintenance_records", crate::Q::maintenance_records().unlimited())
    }

    pub fn max_update_time_of_maintenance_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_records_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_toll_receipts(self) -> Self {
        self.count_toll_receipts_as("count_toll_receipts")
    }

    pub fn count_toll_receipts_as(self, alias: impl Into<String>) -> Self {
        self.count_toll_receipts_with(alias, crate::Q::toll_receipts().unlimited())
    }

    pub fn count_toll_receipts_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "toll_receipt_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_toll_receipts(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_toll_receipts_as("refinements", request)
    }

    pub fn stats_from_toll_receipts_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "toll_receipt_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_toll_receipts_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_toll_receipts(request)
    }


    pub fn min_create_time_of_toll_receipts(self) -> Self {
        self.min_create_time_of_toll_receipts_as("min_create_time_of_toll_receipts", crate::Q::toll_receipts().unlimited())
    }

    pub fn min_create_time_of_toll_receipts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_toll_receipts_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_toll_receipts(self) -> Self {
        self.max_create_time_of_toll_receipts_as("max_create_time_of_toll_receipts", crate::Q::toll_receipts().unlimited())
    }

    pub fn max_create_time_of_toll_receipts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_toll_receipts_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_toll_receipts(self) -> Self {
        self.min_update_time_of_toll_receipts_as("min_update_time_of_toll_receipts", crate::Q::toll_receipts().unlimited())
    }

    pub fn min_update_time_of_toll_receipts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_toll_receipts_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_toll_receipts(self) -> Self {
        self.max_update_time_of_toll_receipts_as("max_update_time_of_toll_receipts", crate::Q::toll_receipts().unlimited())
    }

    pub fn max_update_time_of_toll_receipts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_toll_receipts_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_shift_logs(self) -> Self {
        self.count_shift_logs_as("count_shift_logs")
    }

    pub fn count_shift_logs_as(self, alias: impl Into<String>) -> Self {
        self.count_shift_logs_with(alias, crate::Q::shift_logs().unlimited())
    }

    pub fn count_shift_logs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "shift_log_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_shift_logs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_logs_as("refinements", request)
    }

    pub fn stats_from_shift_logs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "shift_log_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_shift_logs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_logs(request)
    }


    pub fn min_create_time_of_shift_logs(self) -> Self {
        self.min_create_time_of_shift_logs_as("min_create_time_of_shift_logs", crate::Q::shift_logs().unlimited())
    }

    pub fn min_create_time_of_shift_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_logs_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_shift_logs(self) -> Self {
        self.max_create_time_of_shift_logs_as("max_create_time_of_shift_logs", crate::Q::shift_logs().unlimited())
    }

    pub fn max_create_time_of_shift_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_logs_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_shift_logs(self) -> Self {
        self.min_update_time_of_shift_logs_as("min_update_time_of_shift_logs", crate::Q::shift_logs().unlimited())
    }

    pub fn min_update_time_of_shift_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_logs_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_shift_logs(self) -> Self {
        self.max_update_time_of_shift_logs_as("max_update_time_of_shift_logs", crate::Q::shift_logs().unlimited())
    }

    pub fn max_update_time_of_shift_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_logs_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_customer_feedback(self) -> Self {
        self.count_customer_feedback_as("count_customer_feedback")
    }

    pub fn count_customer_feedback_as(self, alias: impl Into<String>) -> Self {
        self.count_customer_feedback_with(alias, crate::Q::customer_feedback().unlimited())
    }

    pub fn count_customer_feedback_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_feedback_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customer_feedback(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback_as("refinements", request)
    }

    pub fn stats_from_customer_feedback_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_feedback_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customer_feedback_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback(request)
    }


    pub fn min_create_time_of_customer_feedback(self) -> Self {
        self.min_create_time_of_customer_feedback_as("min_create_time_of_customer_feedback", crate::Q::customer_feedback().unlimited())
    }

    pub fn min_create_time_of_customer_feedback_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_customer_feedback(self) -> Self {
        self.max_create_time_of_customer_feedback_as("max_create_time_of_customer_feedback", crate::Q::customer_feedback().unlimited())
    }

    pub fn max_create_time_of_customer_feedback_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_customer_feedback(self) -> Self {
        self.min_update_time_of_customer_feedback_as("min_update_time_of_customer_feedback", crate::Q::customer_feedback().unlimited())
    }

    pub fn min_update_time_of_customer_feedback_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_customer_feedback(self) -> Self {
        self.max_update_time_of_customer_feedback_as("max_update_time_of_customer_feedback", crate::Q::customer_feedback().unlimited())
    }

    pub fn max_update_time_of_customer_feedback_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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


    pub fn min_create_time_of_incident_reports(self) -> Self {
        self.min_create_time_of_incident_reports_as("min_create_time_of_incident_reports", crate::Q::incident_reports().unlimited())
    }

    pub fn min_create_time_of_incident_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_incident_reports_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_incident_reports(self) -> Self {
        self.max_create_time_of_incident_reports_as("max_create_time_of_incident_reports", crate::Q::incident_reports().unlimited())
    }

    pub fn max_create_time_of_incident_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_incident_reports_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_incident_reports(self) -> Self {
        self.min_update_time_of_incident_reports_as("min_update_time_of_incident_reports", crate::Q::incident_reports().unlimited())
    }

    pub fn min_update_time_of_incident_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_incident_reports_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_incident_reports(self) -> Self {
        self.max_update_time_of_incident_reports_as("max_update_time_of_incident_reports", crate::Q::incident_reports().unlimited())
    }

    pub fn max_update_time_of_incident_reports_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_incident_reports_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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
