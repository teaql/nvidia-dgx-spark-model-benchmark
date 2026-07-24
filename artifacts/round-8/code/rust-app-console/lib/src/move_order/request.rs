use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::MoveOrder {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::MoveOrder {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/move_order
#[derive(Debug)]
pub struct MoveOrderRequest<R = crate::MoveOrder> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for MoveOrderRequest<R> {
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

impl<R> MoveOrderRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("MoveOrder")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> MoveOrderRequest<T> {
        MoveOrderRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .move_order_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .move_order_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .move_order_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for MoveOrder is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .move_order_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .move_order_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
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
            "order_id" => Some("order_id"),
            "status" => Some("status"),
            "version" => Some("version"),
            "merchant_ref" | "merchant_ref_id" => Some("merchant_ref_id"),
            "customer_ref" | "customer_ref_id" => Some("customer_ref_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "merchant_ref" => {
                self.with_merchant_ref_matching(
                    crate::Q::merchants_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_ref" => {
                self.with_customer_ref_matching(
                    crate::Q::customers_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "move_quote_list" => {
                self.with_move_quote_list_matching(
                    crate::Q::move_quotes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "route_list" => {
                self.with_route_list_matching(
                    crate::Q::routes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "time_slot_list" => {
                self.with_time_slot_list_matching(
                    crate::Q::time_slots_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "fulfillment_event_list" => {
                self.with_fulfillment_event_list_matching(
                    crate::Q::fulfillment_events_minimal()
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
            "move_item_list" => {
                self.with_move_item_list_matching(
                    crate::Q::move_items_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "inventory_list_list" => {
                self.with_inventory_list_list_matching(
                    crate::Q::inventory_lists_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "transit_log_list" => {
                self.with_transit_log_list_matching(
                    crate::Q::transit_logs_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "delay_record_list" => {
                self.with_delay_record_list_matching(
                    crate::Q::delay_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "vehicle_assignment_list" => {
                self.with_vehicle_assignment_list_matching(
                    crate::Q::vehicle_assignments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "cargo_weight_record_list" => {
                self.with_cargo_weight_record_list_matching(
                    crate::Q::cargo_weight_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "special_handling_instruction_list" => {
                self.with_special_handling_instruction_list_matching(
                    crate::Q::special_handling_instructions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "delivery_window_list" => {
                self.with_delivery_window_list_matching(
                    crate::Q::delivery_windows_minimal()
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
        self.query = self.query.project("order_id");
        self.query = self.query.project("status");
        self.query = self.query.project("version");
        self.query = self.query.project("merchant_ref_id");
        self.query = self.query.project("customer_ref_id");
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
        request = request.select_merchant_ref();
        request = request.select_customer_ref();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_move_quote_list();
        request = request.select_route_list();
        request = request.select_time_slot_list();
        request = request.select_fulfillment_event_list();
        request = request.select_dispatch_assignment_list();
        request = request.select_damage_report_list();
        request = request.select_proof_of_delivery_list();
        request = request.select_move_item_list();
        request = request.select_inventory_list_list();
        request = request.select_transit_log_list();
        request = request.select_delay_record_list();
        request = request.select_vehicle_assignment_list();
        request = request.select_cargo_weight_record_list();
        request = request.select_special_handling_instruction_list();
        request = request.select_delivery_window_list();
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


    pub fn select_order_id(mut self) -> Self {
        self.query = self.query.project("order_id");
        self
    }

    pub fn project_order_id(self) -> Self {
        self.select_order_id()
    }

    pub fn select_order_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_order_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_order_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("order_id", raw_sql_segment));
        self
    }

    pub fn group_by_order_id(self) -> Self {
        self.group_by("order_id")
    }

    pub fn group_by_order_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("order_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("order_id"));
        request
    }

    pub fn group_by_order_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("order_id")
            .aggregate_with_function("order_id", alias, function)
    }

    pub fn count_order_id(self) -> Self {
        self.count_order_id_as("order_id_count")
    }

    pub fn count_order_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("order_id", alias)
    }

    pub fn sum_order_id(self) -> Self {
        self.sum_order_id_as("sum_order_id")
    }

    pub fn sum_order_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("order_id", alias)
    }

    pub fn avg_order_id(self) -> Self {
        self.avg_order_id_as("avg_order_id")
    }

    pub fn avg_order_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("order_id", alias)
    }

    pub fn min_order_id(self) -> Self {
        self.min_order_id_as("min_order_id")
    }

    pub fn min_order_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("order_id", alias)
    }

    pub fn max_order_id(self) -> Self {
        self.max_order_id_as("max_order_id")
    }

    pub fn max_order_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("order_id", alias)
    }

    pub fn unselect_order_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "order_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "order_id");
        self
    }


    pub fn with_order_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "order_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_order_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "order_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_order_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("order_id", value));
        self
    }



    pub fn with_order_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("order_id", value));
        self
    }

    pub fn with_order_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("order_id", value));
        self
    }

    pub fn with_order_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("order_id", value));
        self
    }

    pub fn with_order_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("order_id", value));
        self
    }

    pub fn with_order_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("order_id", value));
        self
    }

    pub fn with_order_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("order_id", lower, upper));
        self
    }

    pub fn with_order_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "order_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_order_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "order_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_order_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "order_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_order_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("order_id", value));
        self
    }

    pub fn with_order_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("order_id", value));
        self
    }

    pub fn with_order_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("order_id", value));
        self
    }

    pub fn with_order_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("order_id", value));
        self
    }

    pub fn with_order_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("order_id", value));
        self
    }

    pub fn with_order_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("order_id", value));
        self
    }

    pub fn with_order_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("order_id", value));
        self
    }
    pub fn with_order_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("order_id", value));
        self
    }

    pub fn with_order_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("order_id", value));
        self
    }

    pub fn with_order_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("order_id"));
        self
    }



    pub fn with_order_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("order_id"));
        self
    }


    pub fn order_by_order_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("order_id");
        self
    }

    pub fn order_by_order_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("order_id");
        self
    }

    pub fn order_by_order_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("order_id");
        self
    }

    pub fn order_by_order_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("order_id");
        self
    }


    pub fn select_status(mut self) -> Self {
        self.query = self.query.project("status");
        self
    }

    pub fn project_status(self) -> Self {
        self.select_status()
    }

    pub fn select_status_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_status_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_status_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("status", raw_sql_segment));
        self
    }

    pub fn group_by_status(self) -> Self {
        self.group_by("status")
    }

    pub fn group_by_status_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("status");
        request.query = request
            .query
            .project_expr(alias, Expr::column("status"));
        request
    }

    pub fn group_by_status_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("status")
            .aggregate_with_function("status", alias, function)
    }

    pub fn count_status(self) -> Self {
        self.count_status_as("status_count")
    }

    pub fn count_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("status", alias)
    }

    pub fn sum_status(self) -> Self {
        self.sum_status_as("sum_status")
    }

    pub fn sum_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("status", alias)
    }

    pub fn avg_status(self) -> Self {
        self.avg_status_as("avg_status")
    }

    pub fn avg_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("status", alias)
    }

    pub fn min_status(self) -> Self {
        self.min_status_as("min_status")
    }

    pub fn min_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("status", alias)
    }

    pub fn max_status(self) -> Self {
        self.max_status_as("max_status")
    }

    pub fn max_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("status", alias)
    }

    pub fn unselect_status(mut self) -> Self {
        self.query.projection.retain(|field| field != "status");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "status");
        self
    }


    pub fn with_status(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "status",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_status_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "status",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_status_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("status", value));
        self
    }



    pub fn with_status_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("status", value));
        self
    }

    pub fn with_status_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("status", value));
        self
    }

    pub fn with_status_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("status", value));
        self
    }

    pub fn with_status_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("status", value));
        self
    }

    pub fn with_status_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("status", value));
        self
    }

    pub fn with_status_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("status", lower, upper));
        self
    }

    pub fn with_status_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "status",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_status_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "status",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_status_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "status",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_status_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("status", value));
        self
    }

    pub fn with_status_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("status", value));
        self
    }

    pub fn with_status_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("status", value));
        self
    }

    pub fn with_status_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("status", value));
        self
    }

    pub fn with_status_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("status", value));
        self
    }

    pub fn with_status_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("status", value));
        self
    }

    pub fn with_status_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("status", value));
        self
    }
    pub fn with_status_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("status", value));
        self
    }

    pub fn with_status_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("status", value));
        self
    }

    pub fn with_status_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("status"));
        self
    }



    pub fn with_status_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("status"));
        self
    }


    pub fn order_by_status_asc(mut self) -> Self {
        self.query = self.query.order_asc("status");
        self
    }

    pub fn order_by_status_desc(mut self) -> Self {
        self.query = self.query.order_desc("status");
        self
    }

    pub fn order_by_status_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("status");
        self
    }

    pub fn order_by_status_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("status");
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
    pub fn filter_by_merchant_ref(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("merchant_ref_id", value.entity_id_value()));
        self
    }

    pub fn with_merchant_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "merchant_ref_id",
            <crate::Merchant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("merchant_ref", selection));
        self
    }


    pub fn without_merchant_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "merchant_ref_id",
            <crate::Merchant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("merchant_ref", selection));
        self
    }


    pub fn have_merchant_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("merchant_ref_id"));
        self
    }

    pub fn have_no_merchant_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("merchant_ref_id"));
        self
    }


    pub fn group_by_merchant_ref(self) -> Self {
        self.group_by("merchant_ref_id")
    }

    pub fn group_by_merchant_ref_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("merchant_ref_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("merchant_ref_id"));
        request
    }

    pub fn group_by_merchant_ref_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("merchant_ref_id")
            .aggregate_with_function("merchant_ref_id", alias, function)
    }

    pub fn group_by_merchant_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("merchant_ref_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "merchant_ref",
            "merchant_ref_id",
            request,
        ));
        self
    }

    pub fn group_by_merchant_ref_with_details(self) -> Self {
        self.group_by_merchant_ref_with_details_from(crate::Q::merchants().unlimited())
    }

    pub fn group_by_merchant_ref_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_merchant_ref_with(request)
    }


    pub fn roll_up_to_merchant_ref(self) -> Self {
        self.roll_up_to_merchant_ref_with(crate::Q::merchants().unlimited())
    }

    pub fn roll_up_to_merchant_ref_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_merchant_ref_matching(selection.clone())
            .group_by_merchant_ref_with(selection)
    }

    pub fn count_merchant_ref(self) -> Self {
        self.count_merchant_ref_as("merchant_ref_count")
    }

    pub fn count_merchant_ref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("merchant_ref_id", alias)
    }

    pub fn unselect_merchant_ref(mut self) -> Self {
        self.query.projection.retain(|field| field != "merchant_ref_id");
        self.query.relations.retain(|relation| relation.name != "merchant_ref");
        self
    }


    pub fn filter_by_customer_ref(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("customer_ref_id", value.entity_id_value()));
        self
    }

    pub fn with_customer_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "customer_ref_id",
            <crate::Customer as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_ref", selection));
        self
    }


    pub fn without_customer_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "customer_ref_id",
            <crate::Customer as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_ref", selection));
        self
    }


    pub fn have_customer_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("customer_ref_id"));
        self
    }

    pub fn have_no_customer_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("customer_ref_id"));
        self
    }


    pub fn group_by_customer_ref(self) -> Self {
        self.group_by("customer_ref_id")
    }

    pub fn group_by_customer_ref_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("customer_ref_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("customer_ref_id"));
        request
    }

    pub fn group_by_customer_ref_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("customer_ref_id")
            .aggregate_with_function("customer_ref_id", alias, function)
    }

    pub fn group_by_customer_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("customer_ref_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "customer_ref",
            "customer_ref_id",
            request,
        ));
        self
    }

    pub fn group_by_customer_ref_with_details(self) -> Self {
        self.group_by_customer_ref_with_details_from(crate::Q::customers().unlimited())
    }

    pub fn group_by_customer_ref_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_customer_ref_with(request)
    }


    pub fn roll_up_to_customer_ref(self) -> Self {
        self.roll_up_to_customer_ref_with(crate::Q::customers().unlimited())
    }

    pub fn roll_up_to_customer_ref_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_customer_ref_matching(selection.clone())
            .group_by_customer_ref_with(selection)
    }

    pub fn count_customer_ref(self) -> Self {
        self.count_customer_ref_as("customer_ref_count")
    }

    pub fn count_customer_ref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("customer_ref_id", alias)
    }

    pub fn unselect_customer_ref(mut self) -> Self {
        self.query.projection.retain(|field| field != "customer_ref_id");
        self.query.relations.retain(|relation| relation.name != "customer_ref");
        self
    }
    pub fn select_merchant_ref(mut self) -> Self {
        self.query = self.query.relation("merchant_ref");
        self
    }

    pub fn select_merchant_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("merchant_ref", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("merchant_ref", selection));
        self
}

    pub fn facet_by_merchant_ref_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_merchant_ref_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_merchant_ref_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "merchant_ref",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_customer_ref(mut self) -> Self {
        self.query = self.query.relation("customer_ref");
        self
    }

    pub fn select_customer_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_ref", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_ref", selection));
        self
}

    pub fn facet_by_customer_ref_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_customer_ref_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_customer_ref_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "customer_ref",
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
            "move_order_ref_id",
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
            "move_order_ref_id",
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

    pub fn have_routes(self) -> Self {
        self.with_route_list_matching(SelectQuery::new("Route"))
    }

    pub fn have_no_routes(self) -> Self {
        self.without_route_list_matching(SelectQuery::new("Route"))
    }

    pub fn with_route_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Route as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("route_list", selection));
        self
    }

    pub fn without_route_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Route as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("route_list", selection));
        self
    }

    pub fn select_route_list(mut self) -> Self {
        self.query = self.query.relation("route_list");
        self
    }

    pub fn select_route_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("route_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("route_list", selection));
        self
}

    pub fn have_time_slots(self) -> Self {
        self.with_time_slot_list_matching(SelectQuery::new("TimeSlot"))
    }

    pub fn have_no_time_slots(self) -> Self {
        self.without_time_slot_list_matching(SelectQuery::new("TimeSlot"))
    }

    pub fn with_time_slot_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TimeSlot as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("time_slot_list", selection));
        self
    }

    pub fn without_time_slot_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TimeSlot as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("time_slot_list", selection));
        self
    }

    pub fn select_time_slot_list(mut self) -> Self {
        self.query = self.query.relation("time_slot_list");
        self
    }

    pub fn select_time_slot_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("time_slot_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("time_slot_list", selection));
        self
}

    pub fn have_fulfillment_events(self) -> Self {
        self.with_fulfillment_event_list_matching(SelectQuery::new("FulfillmentEvent"))
    }

    pub fn have_no_fulfillment_events(self) -> Self {
        self.without_fulfillment_event_list_matching(SelectQuery::new("FulfillmentEvent"))
    }

    pub fn with_fulfillment_event_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::FulfillmentEvent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("fulfillment_event_list", selection));
        self
    }

    pub fn without_fulfillment_event_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::FulfillmentEvent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("fulfillment_event_list", selection));
        self
    }

    pub fn select_fulfillment_event_list(mut self) -> Self {
        self.query = self.query.relation("fulfillment_event_list");
        self
    }

    pub fn select_fulfillment_event_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("fulfillment_event_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("fulfillment_event_list", selection));
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
            "move_order_ref_id",
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
            "move_order_ref_id",
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
            "move_order_ref_id",
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
            "move_order_ref_id",
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
            "move_order_ref_id",
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
            "move_order_ref_id",
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

    pub fn have_move_items(self) -> Self {
        self.with_move_item_list_matching(SelectQuery::new("MoveItem"))
    }

    pub fn have_no_move_items(self) -> Self {
        self.without_move_item_list_matching(SelectQuery::new("MoveItem"))
    }

    pub fn with_move_item_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MoveItem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_item_list", selection));
        self
    }

    pub fn without_move_item_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MoveItem as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_item_list", selection));
        self
    }

    pub fn select_move_item_list(mut self) -> Self {
        self.query = self.query.relation("move_item_list");
        self
    }

    pub fn select_move_item_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("move_item_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("move_item_list", selection));
        self
}

    pub fn have_inventory_lists(self) -> Self {
        self.with_inventory_list_list_matching(SelectQuery::new("InventoryList"))
    }

    pub fn have_no_inventory_lists(self) -> Self {
        self.without_inventory_list_list_matching(SelectQuery::new("InventoryList"))
    }

    pub fn with_inventory_list_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::InventoryList as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("inventory_list_list", selection));
        self
    }

    pub fn without_inventory_list_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::InventoryList as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("inventory_list_list", selection));
        self
    }

    pub fn select_inventory_list_list(mut self) -> Self {
        self.query = self.query.relation("inventory_list_list");
        self
    }

    pub fn select_inventory_list_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("inventory_list_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("inventory_list_list", selection));
        self
}

    pub fn have_transit_logs(self) -> Self {
        self.with_transit_log_list_matching(SelectQuery::new("TransitLog"))
    }

    pub fn have_no_transit_logs(self) -> Self {
        self.without_transit_log_list_matching(SelectQuery::new("TransitLog"))
    }

    pub fn with_transit_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TransitLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("transit_log_list", selection));
        self
    }

    pub fn without_transit_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TransitLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("transit_log_list", selection));
        self
    }

    pub fn select_transit_log_list(mut self) -> Self {
        self.query = self.query.relation("transit_log_list");
        self
    }

    pub fn select_transit_log_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("transit_log_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("transit_log_list", selection));
        self
}

    pub fn have_delay_records(self) -> Self {
        self.with_delay_record_list_matching(SelectQuery::new("DelayRecord"))
    }

    pub fn have_no_delay_records(self) -> Self {
        self.without_delay_record_list_matching(SelectQuery::new("DelayRecord"))
    }

    pub fn with_delay_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DelayRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("delay_record_list", selection));
        self
    }

    pub fn without_delay_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DelayRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("delay_record_list", selection));
        self
    }

    pub fn select_delay_record_list(mut self) -> Self {
        self.query = self.query.relation("delay_record_list");
        self
    }

    pub fn select_delay_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("delay_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("delay_record_list", selection));
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
            "move_order_ref_id",
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
            "move_order_ref_id",
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

    pub fn have_cargo_weight_records(self) -> Self {
        self.with_cargo_weight_record_list_matching(SelectQuery::new("CargoWeightRecord"))
    }

    pub fn have_no_cargo_weight_records(self) -> Self {
        self.without_cargo_weight_record_list_matching(SelectQuery::new("CargoWeightRecord"))
    }

    pub fn with_cargo_weight_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CargoWeightRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("cargo_weight_record_list", selection));
        self
    }

    pub fn without_cargo_weight_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CargoWeightRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("cargo_weight_record_list", selection));
        self
    }

    pub fn select_cargo_weight_record_list(mut self) -> Self {
        self.query = self.query.relation("cargo_weight_record_list");
        self
    }

    pub fn select_cargo_weight_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("cargo_weight_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("cargo_weight_record_list", selection));
        self
}

    pub fn have_special_handling_instructions(self) -> Self {
        self.with_special_handling_instruction_list_matching(SelectQuery::new("SpecialHandlingInstruction"))
    }

    pub fn have_no_special_handling_instructions(self) -> Self {
        self.without_special_handling_instruction_list_matching(SelectQuery::new("SpecialHandlingInstruction"))
    }

    pub fn with_special_handling_instruction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::SpecialHandlingInstruction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("special_handling_instruction_list", selection));
        self
    }

    pub fn without_special_handling_instruction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::SpecialHandlingInstruction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("special_handling_instruction_list", selection));
        self
    }

    pub fn select_special_handling_instruction_list(mut self) -> Self {
        self.query = self.query.relation("special_handling_instruction_list");
        self
    }

    pub fn select_special_handling_instruction_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("special_handling_instruction_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("special_handling_instruction_list", selection));
        self
}

    pub fn have_delivery_windows(self) -> Self {
        self.with_delivery_window_list_matching(SelectQuery::new("DeliveryWindow"))
    }

    pub fn have_no_delivery_windows(self) -> Self {
        self.without_delivery_window_list_matching(SelectQuery::new("DeliveryWindow"))
    }

    pub fn with_delivery_window_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DeliveryWindow as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("delivery_window_list", selection));
        self
    }

    pub fn without_delivery_window_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DeliveryWindow as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "move_order_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("delivery_window_list", selection));
        self
    }

    pub fn select_delivery_window_list(mut self) -> Self {
        self.query = self.query.relation("delivery_window_list");
        self
    }

    pub fn select_delivery_window_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("delivery_window_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("delivery_window_list", selection));
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




    pub fn count_routes(self) -> Self {
        self.count_routes_as("count_routes")
    }

    pub fn count_routes_as(self, alias: impl Into<String>) -> Self {
        self.count_routes_with(alias, crate::Q::routes().unlimited())
    }

    pub fn count_routes_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "route_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_routes(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_routes_as("refinements", request)
    }

    pub fn stats_from_routes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "route_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_routes_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_routes(request)
    }




    pub fn count_time_slots(self) -> Self {
        self.count_time_slots_as("count_time_slots")
    }

    pub fn count_time_slots_as(self, alias: impl Into<String>) -> Self {
        self.count_time_slots_with(alias, crate::Q::time_slots().unlimited())
    }

    pub fn count_time_slots_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "time_slot_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_time_slots(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_slots_as("refinements", request)
    }

    pub fn stats_from_time_slots_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "time_slot_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_time_slots_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_slots(request)
    }




    pub fn count_fulfillment_events(self) -> Self {
        self.count_fulfillment_events_as("count_fulfillment_events")
    }

    pub fn count_fulfillment_events_as(self, alias: impl Into<String>) -> Self {
        self.count_fulfillment_events_with(alias, crate::Q::fulfillment_events().unlimited())
    }

    pub fn count_fulfillment_events_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "fulfillment_event_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_fulfillment_events(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fulfillment_events_as("refinements", request)
    }

    pub fn stats_from_fulfillment_events_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "fulfillment_event_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_fulfillment_events_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fulfillment_events(request)
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




    pub fn count_move_items(self) -> Self {
        self.count_move_items_as("count_move_items")
    }

    pub fn count_move_items_as(self, alias: impl Into<String>) -> Self {
        self.count_move_items_with(alias, crate::Q::move_items().unlimited())
    }

    pub fn count_move_items_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_item_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_move_items(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_items_as("refinements", request)
    }

    pub fn stats_from_move_items_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_item_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_move_items_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_items(request)
    }




    pub fn count_inventory_lists(self) -> Self {
        self.count_inventory_lists_as("count_inventory_lists")
    }

    pub fn count_inventory_lists_as(self, alias: impl Into<String>) -> Self {
        self.count_inventory_lists_with(alias, crate::Q::inventory_lists().unlimited())
    }

    pub fn count_inventory_lists_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "inventory_list_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_inventory_lists(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_inventory_lists_as("refinements", request)
    }

    pub fn stats_from_inventory_lists_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "inventory_list_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_inventory_lists_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_inventory_lists(request)
    }




    pub fn count_transit_logs(self) -> Self {
        self.count_transit_logs_as("count_transit_logs")
    }

    pub fn count_transit_logs_as(self, alias: impl Into<String>) -> Self {
        self.count_transit_logs_with(alias, crate::Q::transit_logs().unlimited())
    }

    pub fn count_transit_logs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "transit_log_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_transit_logs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_transit_logs_as("refinements", request)
    }

    pub fn stats_from_transit_logs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "transit_log_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_transit_logs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_transit_logs(request)
    }




    pub fn count_delay_records(self) -> Self {
        self.count_delay_records_as("count_delay_records")
    }

    pub fn count_delay_records_as(self, alias: impl Into<String>) -> Self {
        self.count_delay_records_with(alias, crate::Q::delay_records().unlimited())
    }

    pub fn count_delay_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "delay_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_delay_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delay_records_as("refinements", request)
    }

    pub fn stats_from_delay_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "delay_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_delay_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delay_records(request)
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




    pub fn count_cargo_weight_records(self) -> Self {
        self.count_cargo_weight_records_as("count_cargo_weight_records")
    }

    pub fn count_cargo_weight_records_as(self, alias: impl Into<String>) -> Self {
        self.count_cargo_weight_records_with(alias, crate::Q::cargo_weight_records().unlimited())
    }

    pub fn count_cargo_weight_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "cargo_weight_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_cargo_weight_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_cargo_weight_records_as("refinements", request)
    }

    pub fn stats_from_cargo_weight_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "cargo_weight_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_cargo_weight_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_cargo_weight_records(request)
    }




    pub fn count_special_handling_instructions(self) -> Self {
        self.count_special_handling_instructions_as("count_special_handling_instructions")
    }

    pub fn count_special_handling_instructions_as(self, alias: impl Into<String>) -> Self {
        self.count_special_handling_instructions_with(alias, crate::Q::special_handling_instructions().unlimited())
    }

    pub fn count_special_handling_instructions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "special_handling_instruction_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_special_handling_instructions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_special_handling_instructions_as("refinements", request)
    }

    pub fn stats_from_special_handling_instructions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "special_handling_instruction_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_special_handling_instructions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_special_handling_instructions(request)
    }




    pub fn count_delivery_windows(self) -> Self {
        self.count_delivery_windows_as("count_delivery_windows")
    }

    pub fn count_delivery_windows_as(self, alias: impl Into<String>) -> Self {
        self.count_delivery_windows_with(alias, crate::Q::delivery_windows().unlimited())
    }

    pub fn count_delivery_windows_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "delivery_window_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_delivery_windows(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_windows_as("refinements", request)
    }

    pub fn stats_from_delivery_windows_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "delivery_window_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_delivery_windows_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_delivery_windows(request)
    }



}

impl<R> Default for MoveOrderRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< MoveOrderRequest<R> > for SelectQuery {
    fn from(request: MoveOrderRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< MoveOrderRequest<R> > for QuerySelection {
    fn from(request: MoveOrderRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::MoveOrder> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::MoveOrderRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<MoveOrderRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::MoveOrder
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::MoveOrder::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> MoveOrderRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::MoveOrderRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
