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
            "order_number" => Some("order_number"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "status" | "status_id" => Some("status_id"),
            "quote" | "quote_id" => Some("quote_id"),
            "merchant" | "merchant_id" => Some("merchant_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "status" => {
                self.with_status_matching(
                    crate::Q::order_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "quote" => {
                self.with_quote_matching(
                    crate::Q::move_quotes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "merchant" => {
                self.with_merchant_matching(
                    crate::Q::merchants_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "route_stop_list" => {
                self.with_route_stop_list_matching(
                    crate::Q::route_stops_minimal()
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
            "third_party_dispatch_list" => {
                self.with_third_party_dispatch_list_matching(
                    crate::Q::third_party_dispatches_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_feedback_list" => {
                self.with_customer_feedback_list_matching(
                    crate::Q::customer_feedback_minimal()
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
        self.query = self.query.project("order_number");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("status_id");
        self.query = self.query.project("quote_id");
        self.query = self.query.project("merchant_id");
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
        request = request.select_status();
        request = request.select_quote();
        request = request.select_merchant();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_route_stop_list();
        request = request.select_dispatch_assignment_list();
        request = request.select_damage_report_list();
        request = request.select_proof_of_delivery_list();
        request = request.select_operational_exception_list();
        request = request.select_move_inventory_list();
        request = request.select_packaging_item_list();
        request = request.select_third_party_dispatch_list();
        request = request.select_customer_feedback_list();
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


    pub fn select_order_number(mut self) -> Self {
        self.query = self.query.project("order_number");
        self
    }

    pub fn project_order_number(self) -> Self {
        self.select_order_number()
    }

    pub fn select_order_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_order_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_order_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("order_number", raw_sql_segment));
        self
    }

    pub fn group_by_order_number(self) -> Self {
        self.group_by("order_number")
    }

    pub fn group_by_order_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("order_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("order_number"));
        request
    }

    pub fn group_by_order_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("order_number")
            .aggregate_with_function("order_number", alias, function)
    }

    pub fn count_order_number(self) -> Self {
        self.count_order_number_as("order_number_count")
    }

    pub fn count_order_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("order_number", alias)
    }

    pub fn sum_order_number(self) -> Self {
        self.sum_order_number_as("sum_order_number")
    }

    pub fn sum_order_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("order_number", alias)
    }

    pub fn avg_order_number(self) -> Self {
        self.avg_order_number_as("avg_order_number")
    }

    pub fn avg_order_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("order_number", alias)
    }

    pub fn min_order_number(self) -> Self {
        self.min_order_number_as("min_order_number")
    }

    pub fn min_order_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("order_number", alias)
    }

    pub fn max_order_number(self) -> Self {
        self.max_order_number_as("max_order_number")
    }

    pub fn max_order_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("order_number", alias)
    }

    pub fn unselect_order_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "order_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "order_number");
        self
    }


    pub fn with_order_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "order_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_order_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "order_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_order_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("order_number", value));
        self
    }



    pub fn with_order_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("order_number", value));
        self
    }

    pub fn with_order_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("order_number", value));
        self
    }

    pub fn with_order_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("order_number", value));
        self
    }

    pub fn with_order_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("order_number", value));
        self
    }

    pub fn with_order_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("order_number", value));
        self
    }

    pub fn with_order_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("order_number", lower, upper));
        self
    }

    pub fn with_order_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "order_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_order_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "order_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_order_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "order_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_order_number_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("order_number", value));
        self
    }

    pub fn with_order_number_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("order_number", value));
        self
    }

    pub fn with_order_number_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("order_number", value));
        self
    }

    pub fn with_order_number_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("order_number", value));
        self
    }

    pub fn with_order_number_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("order_number", value));
        self
    }

    pub fn with_order_number_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("order_number", value));
        self
    }

    pub fn with_order_number_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("order_number", value));
        self
    }
    pub fn with_order_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("order_number", value));
        self
    }

    pub fn with_order_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("order_number", value));
        self
    }

    pub fn with_order_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("order_number"));
        self
    }



    pub fn with_order_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("order_number"));
        self
    }


    pub fn order_by_order_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("order_number");
        self
    }

    pub fn order_by_order_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("order_number");
        self
    }

    pub fn order_by_order_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("order_number");
        self
    }

    pub fn order_by_order_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("order_number");
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
    /// Please use `with_status_is` instead
    pub(crate) fn filter_by_status(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("status_id", value.entity_id_value()));
        self
    }
    /// Complex relation filter for `status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_status_is_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::order_statuses_minimal().filter(...);
    /// let request = crate::Q::move_orders().with_status_matching(dynamic_query);
    /// ```
    pub fn with_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "status_id",
            <crate::OrderStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("status", selection));
        self
    }


    /// Complex relation filter for `status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_status_is_not_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::order_statuses_minimal().filter(...);
    /// let request = crate::Q::move_orders().without_status_matching(dynamic_query);
    /// ```
    pub fn without_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "status_id",
            <crate::OrderStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("status", selection));
        self
    }


    pub fn have_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("status_id"));
        self
    }

    pub fn have_no_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("status_id"));
        self
    }


    pub fn group_by_status(self) -> Self {
        self.group_by("status_id")
    }

    pub fn group_by_status_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("status_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("status_id"));
        request
    }

    pub fn group_by_status_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("status_id")
            .aggregate_with_function("status_id", alias, function)
    }

    pub fn group_by_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("status_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "status",
            "status_id",
            request,
        ));
        self
    }

    pub fn group_by_status_with_details(self) -> Self {
        self.group_by_status_with_details_from(crate::Q::order_statuses().unlimited())
    }

    pub fn group_by_status_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_status_with(request)
    }


    pub fn roll_up_to_status(self) -> Self {
        self.roll_up_to_status_with(crate::Q::order_statuses().unlimited())
    }

    pub fn roll_up_to_status_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_status_matching(selection.clone())
            .group_by_status_with(selection)
    }

    pub fn count_status(self) -> Self {
        self.count_status_as("status_count")
    }

    pub fn count_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("status_id", alias)
    }

    pub fn unselect_status(mut self) -> Self {
        self.query.projection.retain(|field| field != "status_id");
        self.query.relations.retain(|relation| relation.name != "status");
        self
    }


    pub fn filter_by_quote(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("quote_id", value.entity_id_value()));
        self
    }

    pub fn with_quote_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "quote_id",
            <crate::MoveQuote as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("quote", selection));
        self
    }


    pub fn without_quote_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "quote_id",
            <crate::MoveQuote as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("quote", selection));
        self
    }


    pub fn have_quote(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("quote_id"));
        self
    }

    pub fn have_no_quote(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("quote_id"));
        self
    }


    pub fn group_by_quote(self) -> Self {
        self.group_by("quote_id")
    }

    pub fn group_by_quote_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("quote_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("quote_id"));
        request
    }

    pub fn group_by_quote_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("quote_id")
            .aggregate_with_function("quote_id", alias, function)
    }

    pub fn group_by_quote_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("quote_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "quote",
            "quote_id",
            request,
        ));
        self
    }

    pub fn group_by_quote_with_details(self) -> Self {
        self.group_by_quote_with_details_from(crate::Q::move_quotes().unlimited())
    }

    pub fn group_by_quote_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_quote_with(request)
    }


    pub fn roll_up_to_quote(self) -> Self {
        self.roll_up_to_quote_with(crate::Q::move_quotes().unlimited())
    }

    pub fn roll_up_to_quote_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_quote_matching(selection.clone())
            .group_by_quote_with(selection)
    }

    pub fn count_quote(self) -> Self {
        self.count_quote_as("quote_count")
    }

    pub fn count_quote_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("quote_id", alias)
    }

    pub fn unselect_quote(mut self) -> Self {
        self.query.projection.retain(|field| field != "quote_id");
        self.query.relations.retain(|relation| relation.name != "quote");
        self
    }


    pub fn filter_by_merchant(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("merchant_id", value.entity_id_value()));
        self
    }

    pub fn with_merchant_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "merchant_id",
            <crate::Merchant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("merchant", selection));
        self
    }


    pub fn without_merchant_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "merchant_id",
            <crate::Merchant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("merchant", selection));
        self
    }


    pub fn have_merchant(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("merchant_id"));
        self
    }

    pub fn have_no_merchant(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("merchant_id"));
        self
    }


    pub fn group_by_merchant(self) -> Self {
        self.group_by("merchant_id")
    }

    pub fn group_by_merchant_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("merchant_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("merchant_id"));
        request
    }

    pub fn group_by_merchant_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("merchant_id")
            .aggregate_with_function("merchant_id", alias, function)
    }

    pub fn group_by_merchant_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("merchant_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "merchant",
            "merchant_id",
            request,
        ));
        self
    }

    pub fn group_by_merchant_with_details(self) -> Self {
        self.group_by_merchant_with_details_from(crate::Q::merchants().unlimited())
    }

    pub fn group_by_merchant_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_merchant_with(request)
    }


    pub fn roll_up_to_merchant(self) -> Self {
        self.roll_up_to_merchant_with(crate::Q::merchants().unlimited())
    }

    pub fn roll_up_to_merchant_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_merchant_matching(selection.clone())
            .group_by_merchant_with(selection)
    }

    pub fn count_merchant(self) -> Self {
        self.count_merchant_as("merchant_count")
    }

    pub fn count_merchant_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("merchant_id", alias)
    }

    pub fn unselect_merchant(mut self) -> Self {
        self.query.projection.retain(|field| field != "merchant_id");
        self.query.relations.retain(|relation| relation.name != "merchant");
        self
    }
    pub fn status_is_draft(self) -> Self {
        self.filter_by_status(1001_u64)
    }

    pub fn with_status_is_draft(self) -> Self {
        self.filter_by_status(1001_u64)
    }



    pub fn with_status_is_not_draft(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("status_id", 1001_u64));
        self
    }


    pub fn status_is_confirmed(self) -> Self {
        self.filter_by_status(1002_u64)
    }

    pub fn with_status_is_confirmed(self) -> Self {
        self.filter_by_status(1002_u64)
    }



    pub fn with_status_is_not_confirmed(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("status_id", 1002_u64));
        self
    }


    pub fn status_is_completed(self) -> Self {
        self.filter_by_status(1003_u64)
    }

    pub fn with_status_is_completed(self) -> Self {
        self.filter_by_status(1003_u64)
    }



    pub fn with_status_is_not_completed(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("status_id", 1003_u64));
        self
    }






    pub fn select_status(mut self) -> Self {
        self.query = self.query.relation("status");
        self
    }

    pub fn select_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("status", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("status", selection));
        self
}

    pub fn facet_by_status_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_status_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_status_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "status",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_quote(mut self) -> Self {
        self.query = self.query.relation("quote");
        self
    }

    pub fn select_quote_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("quote", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("quote", selection));
        self
}

    pub fn facet_by_quote_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_quote_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_quote_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "quote",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_merchant(mut self) -> Self {
        self.query = self.query.relation("merchant");
        self
    }

    pub fn select_merchant_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("merchant", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("merchant", selection));
        self
}

    pub fn facet_by_merchant_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_merchant_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_merchant_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "merchant",
            request,
            include_all_facets,
        ));
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
            "move_order_id",
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
