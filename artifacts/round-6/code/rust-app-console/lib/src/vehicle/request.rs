use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Vehicle {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Vehicle {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/vehicle
#[derive(Debug)]
pub struct VehicleRequest<R = crate::Vehicle> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for VehicleRequest<R> {
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

impl<R> VehicleRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Vehicle")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> VehicleRequest<T> {
        VehicleRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .vehicle_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .vehicle_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::VehicleRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::VehicleRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::VehicleRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .vehicle_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Vehicle is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .vehicle_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .vehicle_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::VehicleRepository<'a>>>
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
            "license_plate" => Some("license_plate"),
            "model" => Some("model"),
            "capacity_kg" => Some("capacity_kg"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "merchant" | "merchant_id" => Some("merchant_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "merchant" => {
                self.with_merchant_matching(
                    crate::Q::merchants_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "vehicle_assignment_list" => {
                self.with_vehicle_assignment_list_matching(
                    crate::Q::vehicle_assignments_minimal()
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
        self.query = self.query.project("license_plate");
        self.query = self.query.project("model");
        self.query = self.query.project("capacity_kg");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
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
        request = request.select_merchant();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_vehicle_assignment_list();
        request = request.select_fuel_log_list();
        request = request.select_maintenance_record_list();
        request = request.select_toll_receipt_list();
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


    pub fn select_license_plate(mut self) -> Self {
        self.query = self.query.project("license_plate");
        self
    }

    pub fn project_license_plate(self) -> Self {
        self.select_license_plate()
    }

    pub fn select_license_plate_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_license_plate_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_license_plate_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("license_plate", raw_sql_segment));
        self
    }

    pub fn group_by_license_plate(self) -> Self {
        self.group_by("license_plate")
    }

    pub fn group_by_license_plate_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("license_plate");
        request.query = request
            .query
            .project_expr(alias, Expr::column("license_plate"));
        request
    }

    pub fn group_by_license_plate_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("license_plate")
            .aggregate_with_function("license_plate", alias, function)
    }

    pub fn count_license_plate(self) -> Self {
        self.count_license_plate_as("license_plate_count")
    }

    pub fn count_license_plate_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("license_plate", alias)
    }

    pub fn sum_license_plate(self) -> Self {
        self.sum_license_plate_as("sum_license_plate")
    }

    pub fn sum_license_plate_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("license_plate", alias)
    }

    pub fn avg_license_plate(self) -> Self {
        self.avg_license_plate_as("avg_license_plate")
    }

    pub fn avg_license_plate_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("license_plate", alias)
    }

    pub fn min_license_plate(self) -> Self {
        self.min_license_plate_as("min_license_plate")
    }

    pub fn min_license_plate_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("license_plate", alias)
    }

    pub fn max_license_plate(self) -> Self {
        self.max_license_plate_as("max_license_plate")
    }

    pub fn max_license_plate_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("license_plate", alias)
    }

    pub fn unselect_license_plate(mut self) -> Self {
        self.query.projection.retain(|field| field != "license_plate");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "license_plate");
        self
    }


    pub fn with_license_plate(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "license_plate",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_license_plate_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "license_plate",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_license_plate_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("license_plate", value));
        self
    }



    pub fn with_license_plate_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("license_plate", value));
        self
    }

    pub fn with_license_plate_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("license_plate", value));
        self
    }

    pub fn with_license_plate_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("license_plate", value));
        self
    }

    pub fn with_license_plate_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("license_plate", value));
        self
    }

    pub fn with_license_plate_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("license_plate", value));
        self
    }

    pub fn with_license_plate_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("license_plate", lower, upper));
        self
    }

    pub fn with_license_plate_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "license_plate",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_license_plate_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "license_plate",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_license_plate_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "license_plate",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_license_plate_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("license_plate", value));
        self
    }

    pub fn with_license_plate_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("license_plate", value));
        self
    }

    pub fn with_license_plate_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("license_plate", value));
        self
    }

    pub fn with_license_plate_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("license_plate", value));
        self
    }

    pub fn with_license_plate_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("license_plate", value));
        self
    }

    pub fn with_license_plate_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("license_plate", value));
        self
    }

    pub fn with_license_plate_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("license_plate", value));
        self
    }
    pub fn with_license_plate_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("license_plate", value));
        self
    }

    pub fn with_license_plate_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("license_plate", value));
        self
    }

    pub fn with_license_plate_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("license_plate"));
        self
    }



    pub fn with_license_plate_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("license_plate"));
        self
    }


    pub fn order_by_license_plate_asc(mut self) -> Self {
        self.query = self.query.order_asc("license_plate");
        self
    }

    pub fn order_by_license_plate_desc(mut self) -> Self {
        self.query = self.query.order_desc("license_plate");
        self
    }

    pub fn order_by_license_plate_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("license_plate");
        self
    }

    pub fn order_by_license_plate_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("license_plate");
        self
    }


    pub fn select_model(mut self) -> Self {
        self.query = self.query.project("model");
        self
    }

    pub fn project_model(self) -> Self {
        self.select_model()
    }

    pub fn select_model_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_model_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_model_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("model", raw_sql_segment));
        self
    }

    pub fn group_by_model(self) -> Self {
        self.group_by("model")
    }

    pub fn group_by_model_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("model");
        request.query = request
            .query
            .project_expr(alias, Expr::column("model"));
        request
    }

    pub fn group_by_model_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("model")
            .aggregate_with_function("model", alias, function)
    }

    pub fn count_model(self) -> Self {
        self.count_model_as("model_count")
    }

    pub fn count_model_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("model", alias)
    }

    pub fn sum_model(self) -> Self {
        self.sum_model_as("sum_model")
    }

    pub fn sum_model_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("model", alias)
    }

    pub fn avg_model(self) -> Self {
        self.avg_model_as("avg_model")
    }

    pub fn avg_model_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("model", alias)
    }

    pub fn min_model(self) -> Self {
        self.min_model_as("min_model")
    }

    pub fn min_model_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("model", alias)
    }

    pub fn max_model(self) -> Self {
        self.max_model_as("max_model")
    }

    pub fn max_model_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("model", alias)
    }

    pub fn unselect_model(mut self) -> Self {
        self.query.projection.retain(|field| field != "model");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "model");
        self
    }


    pub fn with_model(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "model",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_model_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "model",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_model_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("model", value));
        self
    }



    pub fn with_model_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("model", value));
        self
    }

    pub fn with_model_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("model", value));
        self
    }

    pub fn with_model_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("model", value));
        self
    }

    pub fn with_model_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("model", value));
        self
    }

    pub fn with_model_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("model", value));
        self
    }

    pub fn with_model_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("model", lower, upper));
        self
    }

    pub fn with_model_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "model",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_model_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "model",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_model_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "model",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_model_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("model", value));
        self
    }

    pub fn with_model_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("model", value));
        self
    }

    pub fn with_model_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("model", value));
        self
    }

    pub fn with_model_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("model", value));
        self
    }

    pub fn with_model_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("model", value));
        self
    }

    pub fn with_model_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("model", value));
        self
    }

    pub fn with_model_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("model", value));
        self
    }
    pub fn with_model_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("model", value));
        self
    }

    pub fn with_model_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("model", value));
        self
    }

    pub fn with_model_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("model"));
        self
    }



    pub fn with_model_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("model"));
        self
    }


    pub fn order_by_model_asc(mut self) -> Self {
        self.query = self.query.order_asc("model");
        self
    }

    pub fn order_by_model_desc(mut self) -> Self {
        self.query = self.query.order_desc("model");
        self
    }

    pub fn order_by_model_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("model");
        self
    }

    pub fn order_by_model_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("model");
        self
    }


    pub fn select_capacity_kg(mut self) -> Self {
        self.query = self.query.project("capacity_kg");
        self
    }

    pub fn project_capacity_kg(self) -> Self {
        self.select_capacity_kg()
    }

    pub fn select_capacity_kg_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_capacity_kg_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_capacity_kg_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("capacity_kg", raw_sql_segment));
        self
    }

    pub fn group_by_capacity_kg(self) -> Self {
        self.group_by("capacity_kg")
    }

    pub fn group_by_capacity_kg_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("capacity_kg");
        request.query = request
            .query
            .project_expr(alias, Expr::column("capacity_kg"));
        request
    }

    pub fn group_by_capacity_kg_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("capacity_kg")
            .aggregate_with_function("capacity_kg", alias, function)
    }

    pub fn count_capacity_kg(self) -> Self {
        self.count_capacity_kg_as("capacity_kg_count")
    }

    pub fn count_capacity_kg_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("capacity_kg", alias)
    }

    pub fn sum_capacity_kg(self) -> Self {
        self.sum_capacity_kg_as("sum_capacity_kg")
    }

    pub fn sum_capacity_kg_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("capacity_kg", alias)
    }

    pub fn avg_capacity_kg(self) -> Self {
        self.avg_capacity_kg_as("avg_capacity_kg")
    }

    pub fn avg_capacity_kg_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("capacity_kg", alias)
    }

    pub fn min_capacity_kg(self) -> Self {
        self.min_capacity_kg_as("min_capacity_kg")
    }

    pub fn min_capacity_kg_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("capacity_kg", alias)
    }

    pub fn max_capacity_kg(self) -> Self {
        self.max_capacity_kg_as("max_capacity_kg")
    }

    pub fn max_capacity_kg_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("capacity_kg", alias)
    }

    pub fn unselect_capacity_kg(mut self) -> Self {
        self.query.projection.retain(|field| field != "capacity_kg");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "capacity_kg");
        self
    }


    pub fn with_capacity_kg(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "capacity_kg",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_capacity_kg_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "capacity_kg",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_capacity_kg_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("capacity_kg", value));
        self
    }



    pub fn with_capacity_kg_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("capacity_kg", lower, upper));
        self
    }

    pub fn with_capacity_kg_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "capacity_kg",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_capacity_kg_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "capacity_kg",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_capacity_kg_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "capacity_kg",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_capacity_kg_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("capacity_kg", value));
        self
    }
    pub fn with_capacity_kg_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("capacity_kg", value));
        self
    }

    pub fn with_capacity_kg_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("capacity_kg"));
        self
    }



    pub fn with_capacity_kg_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("capacity_kg"));
        self
    }


    pub fn order_by_capacity_kg_asc(mut self) -> Self {
        self.query = self.query.order_asc("capacity_kg");
        self
    }

    pub fn order_by_capacity_kg_desc(mut self) -> Self {
        self.query = self.query.order_desc("capacity_kg");
        self
    }

    pub fn order_by_capacity_kg_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("capacity_kg");
        self
    }

    pub fn order_by_capacity_kg_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("capacity_kg");
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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
            "vehicle_id",
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

impl<R> Default for VehicleRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< VehicleRequest<R> > for SelectQuery {
    fn from(request: VehicleRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< VehicleRequest<R> > for QuerySelection {
    fn from(request: VehicleRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Vehicle> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::VehicleRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<VehicleRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Vehicle
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Vehicle::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> VehicleRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::VehicleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
