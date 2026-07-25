use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Customer {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Customer {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/customer
#[derive(Debug)]
pub struct CustomerRequest<R = crate::Customer> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for CustomerRequest<R> {
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

impl<R> CustomerRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Customer")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> CustomerRequest<T> {
        CustomerRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .customer_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Customer is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
            "version" => Some("version"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "employee_list" => {
                self.with_employee_list_matching(
                    crate::Q::employees_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "truck_list" => {
                self.with_truck_list_matching(
                    crate::Q::trucks_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "inventory_item_list" => {
                self.with_inventory_item_list_matching(
                    crate::Q::inventory_items_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "move_order_list" => {
                self.with_move_order_list_matching(
                    crate::Q::move_orders_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "route_list" => {
                self.with_route_list_matching(
                    crate::Q::routes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "payment_list" => {
                self.with_payment_list_matching(
                    crate::Q::payments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "invoice_list" => {
                self.with_invoice_list_matching(
                    crate::Q::invoices_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "feedback_list" => {
                self.with_feedback_list_matching(
                    crate::Q::feedback_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "schedule_list" => {
                self.with_schedule_list_matching(
                    crate::Q::schedules_minimal()
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
        self.query = self.query.project("version");
        self
    }

    pub fn select_self_fields(self) -> Self {
        self.select_self()
    }

    pub fn select_self_without_parent(self) -> Self {
        self.select_self_fields()
    }

    pub fn select_all(self) -> Self {
        self.select_self()
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_employee_list();
        request = request.select_truck_list();
        request = request.select_inventory_item_list();
        request = request.select_move_order_list();
        request = request.select_route_list();
        request = request.select_payment_list();
        request = request.select_invoice_list();
        request = request.select_feedback_list();
        request = request.select_schedule_list();
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
            "customer_id",
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
            "customer_id",
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

    pub fn have_trucks(self) -> Self {
        self.with_truck_list_matching(SelectQuery::new("Truck"))
    }

    pub fn have_no_trucks(self) -> Self {
        self.without_truck_list_matching(SelectQuery::new("Truck"))
    }

    pub fn with_truck_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Truck as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("truck_list", selection));
        self
    }

    pub fn without_truck_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Truck as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("truck_list", selection));
        self
    }

    pub fn select_truck_list(mut self) -> Self {
        self.query = self.query.relation("truck_list");
        self
    }

    pub fn select_truck_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("truck_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("truck_list", selection));
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
            "customer_id",
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
            "customer_id",
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
            "customer_id",
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
            "customer_id",
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
            "customer_id",
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
            "customer_id",
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
            "customer_id",
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
            "customer_id",
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

    pub fn have_invoices(self) -> Self {
        self.with_invoice_list_matching(SelectQuery::new("Invoice"))
    }

    pub fn have_no_invoices(self) -> Self {
        self.without_invoice_list_matching(SelectQuery::new("Invoice"))
    }

    pub fn with_invoice_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Invoice as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("invoice_list", selection));
        self
    }

    pub fn without_invoice_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Invoice as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("invoice_list", selection));
        self
    }

    pub fn select_invoice_list(mut self) -> Self {
        self.query = self.query.relation("invoice_list");
        self
    }

    pub fn select_invoice_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("invoice_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("invoice_list", selection));
        self
}

    pub fn have_feedback(self) -> Self {
        self.with_feedback_list_matching(SelectQuery::new("Feedback"))
    }

    pub fn have_no_feedback(self) -> Self {
        self.without_feedback_list_matching(SelectQuery::new("Feedback"))
    }

    pub fn with_feedback_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Feedback as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("feedback_list", selection));
        self
    }

    pub fn without_feedback_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Feedback as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("feedback_list", selection));
        self
    }

    pub fn select_feedback_list(mut self) -> Self {
        self.query = self.query.relation("feedback_list");
        self
    }

    pub fn select_feedback_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("feedback_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("feedback_list", selection));
        self
}

    pub fn have_schedules(self) -> Self {
        self.with_schedule_list_matching(SelectQuery::new("Schedule"))
    }

    pub fn have_no_schedules(self) -> Self {
        self.without_schedule_list_matching(SelectQuery::new("Schedule"))
    }

    pub fn with_schedule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Schedule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("schedule_list", selection));
        self
    }

    pub fn without_schedule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Schedule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_id",
        ));
        self.relation_filters.push(RelationFilter::new("schedule_list", selection));
        self
    }

    pub fn select_schedule_list(mut self) -> Self {
        self.query = self.query.relation("schedule_list");
        self
    }

    pub fn select_schedule_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("schedule_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("schedule_list", selection));
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




    pub fn count_trucks(self) -> Self {
        self.count_trucks_as("count_trucks")
    }

    pub fn count_trucks_as(self, alias: impl Into<String>) -> Self {
        self.count_trucks_with(alias, crate::Q::trucks().unlimited())
    }

    pub fn count_trucks_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "truck_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_trucks(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_trucks_as("refinements", request)
    }

    pub fn stats_from_trucks_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "truck_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_trucks_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_trucks(request)
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




    pub fn count_invoices(self) -> Self {
        self.count_invoices_as("count_invoices")
    }

    pub fn count_invoices_as(self, alias: impl Into<String>) -> Self {
        self.count_invoices_with(alias, crate::Q::invoices().unlimited())
    }

    pub fn count_invoices_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invoice_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_invoices(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_invoices_as("refinements", request)
    }

    pub fn stats_from_invoices_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "invoice_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_invoices_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_invoices(request)
    }




    pub fn count_feedback(self) -> Self {
        self.count_feedback_as("count_feedback")
    }

    pub fn count_feedback_as(self, alias: impl Into<String>) -> Self {
        self.count_feedback_with(alias, crate::Q::feedback().unlimited())
    }

    pub fn count_feedback_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "feedback_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_feedback(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_feedback_as("refinements", request)
    }

    pub fn stats_from_feedback_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "feedback_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_feedback_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_feedback(request)
    }




    pub fn count_schedules(self) -> Self {
        self.count_schedules_as("count_schedules")
    }

    pub fn count_schedules_as(self, alias: impl Into<String>) -> Self {
        self.count_schedules_with(alias, crate::Q::schedules().unlimited())
    }

    pub fn count_schedules_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "schedule_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_schedules(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_schedules_as("refinements", request)
    }

    pub fn stats_from_schedules_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "schedule_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_schedules_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_schedules(request)
    }



}

impl<R> Default for CustomerRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< CustomerRequest<R> > for SelectQuery {
    fn from(request: CustomerRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< CustomerRequest<R> > for QuerySelection {
    fn from(request: CustomerRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Customer> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::CustomerRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<CustomerRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Customer
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Customer::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> CustomerRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
