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
            "vehicle_id" => Some("vehicle_id"),
            "make" => Some("make"),
            "version" => Some("version"),
            "merchant_ref" | "merchant_ref_id" => Some("merchant_ref_id"),
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
            "asset_assignment_list" => {
                self.with_asset_assignment_list_matching(
                    crate::Q::asset_assignments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "asset_inspection_list" => {
                self.with_asset_inspection_list_matching(
                    crate::Q::asset_inspections_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "maintenance_schedule_list" => {
                self.with_maintenance_schedule_list_matching(
                    crate::Q::maintenance_schedules_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "maintenance_event_list" => {
                self.with_maintenance_event_list_matching(
                    crate::Q::maintenance_events_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "fuel_record_list" => {
                self.with_fuel_record_list_matching(
                    crate::Q::fuel_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "vehicle_registration_list" => {
                self.with_vehicle_registration_list_matching(
                    crate::Q::vehicle_registrations_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "asset_condition_list" => {
                self.with_asset_condition_list_matching(
                    crate::Q::asset_conditions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "depreciation_record_list" => {
                self.with_depreciation_record_list_matching(
                    crate::Q::depreciation_records_minimal()
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
        self.query = self.query.project("vehicle_id");
        self.query = self.query.project("make");
        self.query = self.query.project("version");
        self.query = self.query.project("merchant_ref_id");
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
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_asset_assignment_list();
        request = request.select_asset_inspection_list();
        request = request.select_maintenance_schedule_list();
        request = request.select_maintenance_event_list();
        request = request.select_fuel_record_list();
        request = request.select_vehicle_registration_list();
        request = request.select_asset_condition_list();
        request = request.select_depreciation_record_list();
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


    pub fn select_vehicle_id(mut self) -> Self {
        self.query = self.query.project("vehicle_id");
        self
    }

    pub fn project_vehicle_id(self) -> Self {
        self.select_vehicle_id()
    }

    pub fn select_vehicle_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_vehicle_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_vehicle_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("vehicle_id", raw_sql_segment));
        self
    }

    pub fn group_by_vehicle_id(self) -> Self {
        self.group_by("vehicle_id")
    }

    pub fn group_by_vehicle_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("vehicle_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("vehicle_id"));
        request
    }

    pub fn group_by_vehicle_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("vehicle_id")
            .aggregate_with_function("vehicle_id", alias, function)
    }

    pub fn count_vehicle_id(self) -> Self {
        self.count_vehicle_id_as("vehicle_id_count")
    }

    pub fn count_vehicle_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("vehicle_id", alias)
    }

    pub fn sum_vehicle_id(self) -> Self {
        self.sum_vehicle_id_as("sum_vehicle_id")
    }

    pub fn sum_vehicle_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("vehicle_id", alias)
    }

    pub fn avg_vehicle_id(self) -> Self {
        self.avg_vehicle_id_as("avg_vehicle_id")
    }

    pub fn avg_vehicle_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("vehicle_id", alias)
    }

    pub fn min_vehicle_id(self) -> Self {
        self.min_vehicle_id_as("min_vehicle_id")
    }

    pub fn min_vehicle_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("vehicle_id", alias)
    }

    pub fn max_vehicle_id(self) -> Self {
        self.max_vehicle_id_as("max_vehicle_id")
    }

    pub fn max_vehicle_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("vehicle_id", alias)
    }

    pub fn unselect_vehicle_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "vehicle_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "vehicle_id");
        self
    }


    pub fn with_vehicle_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "vehicle_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_vehicle_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "vehicle_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_vehicle_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("vehicle_id", value));
        self
    }



    pub fn with_vehicle_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("vehicle_id", lower, upper));
        self
    }

    pub fn with_vehicle_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "vehicle_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_vehicle_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "vehicle_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_vehicle_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "vehicle_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_vehicle_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("vehicle_id", value));
        self
    }
    pub fn with_vehicle_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("vehicle_id", value));
        self
    }

    pub fn with_vehicle_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("vehicle_id"));
        self
    }



    pub fn with_vehicle_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("vehicle_id"));
        self
    }


    pub fn order_by_vehicle_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("vehicle_id");
        self
    }

    pub fn order_by_vehicle_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("vehicle_id");
        self
    }

    pub fn order_by_vehicle_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("vehicle_id");
        self
    }

    pub fn order_by_vehicle_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("vehicle_id");
        self
    }


    pub fn select_make(mut self) -> Self {
        self.query = self.query.project("make");
        self
    }

    pub fn project_make(self) -> Self {
        self.select_make()
    }

    pub fn select_make_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_make_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_make_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("make", raw_sql_segment));
        self
    }

    pub fn group_by_make(self) -> Self {
        self.group_by("make")
    }

    pub fn group_by_make_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("make");
        request.query = request
            .query
            .project_expr(alias, Expr::column("make"));
        request
    }

    pub fn group_by_make_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("make")
            .aggregate_with_function("make", alias, function)
    }

    pub fn count_make(self) -> Self {
        self.count_make_as("make_count")
    }

    pub fn count_make_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("make", alias)
    }

    pub fn sum_make(self) -> Self {
        self.sum_make_as("sum_make")
    }

    pub fn sum_make_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("make", alias)
    }

    pub fn avg_make(self) -> Self {
        self.avg_make_as("avg_make")
    }

    pub fn avg_make_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("make", alias)
    }

    pub fn min_make(self) -> Self {
        self.min_make_as("min_make")
    }

    pub fn min_make_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("make", alias)
    }

    pub fn max_make(self) -> Self {
        self.max_make_as("max_make")
    }

    pub fn max_make_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("make", alias)
    }

    pub fn unselect_make(mut self) -> Self {
        self.query.projection.retain(|field| field != "make");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "make");
        self
    }


    pub fn with_make(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "make",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_make_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "make",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_make_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("make", value));
        self
    }



    pub fn with_make_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("make", value));
        self
    }

    pub fn with_make_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("make", value));
        self
    }

    pub fn with_make_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("make", value));
        self
    }

    pub fn with_make_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("make", value));
        self
    }

    pub fn with_make_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("make", value));
        self
    }

    pub fn with_make_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("make", lower, upper));
        self
    }

    pub fn with_make_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "make",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_make_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "make",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_make_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "make",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_make_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("make", value));
        self
    }

    pub fn with_make_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("make", value));
        self
    }

    pub fn with_make_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("make", value));
        self
    }

    pub fn with_make_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("make", value));
        self
    }

    pub fn with_make_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("make", value));
        self
    }

    pub fn with_make_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("make", value));
        self
    }

    pub fn with_make_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("make", value));
        self
    }
    pub fn with_make_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("make", value));
        self
    }

    pub fn with_make_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("make", value));
        self
    }

    pub fn with_make_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("make"));
        self
    }



    pub fn with_make_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("make"));
        self
    }


    pub fn order_by_make_asc(mut self) -> Self {
        self.query = self.query.order_asc("make");
        self
    }

    pub fn order_by_make_desc(mut self) -> Self {
        self.query = self.query.order_desc("make");
        self
    }

    pub fn order_by_make_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("make");
        self
    }

    pub fn order_by_make_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("make");
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
    pub fn have_asset_assignments(self) -> Self {
        self.with_asset_assignment_list_matching(SelectQuery::new("AssetAssignment"))
    }

    pub fn have_no_asset_assignments(self) -> Self {
        self.without_asset_assignment_list_matching(SelectQuery::new("AssetAssignment"))
    }

    pub fn with_asset_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AssetAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("asset_assignment_list", selection));
        self
    }

    pub fn without_asset_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AssetAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("asset_assignment_list", selection));
        self
    }

    pub fn select_asset_assignment_list(mut self) -> Self {
        self.query = self.query.relation("asset_assignment_list");
        self
    }

    pub fn select_asset_assignment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("asset_assignment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("asset_assignment_list", selection));
        self
}

    pub fn have_asset_inspections(self) -> Self {
        self.with_asset_inspection_list_matching(SelectQuery::new("AssetInspection"))
    }

    pub fn have_no_asset_inspections(self) -> Self {
        self.without_asset_inspection_list_matching(SelectQuery::new("AssetInspection"))
    }

    pub fn with_asset_inspection_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AssetInspection as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("asset_inspection_list", selection));
        self
    }

    pub fn without_asset_inspection_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AssetInspection as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("asset_inspection_list", selection));
        self
    }

    pub fn select_asset_inspection_list(mut self) -> Self {
        self.query = self.query.relation("asset_inspection_list");
        self
    }

    pub fn select_asset_inspection_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("asset_inspection_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("asset_inspection_list", selection));
        self
}

    pub fn have_maintenance_schedules(self) -> Self {
        self.with_maintenance_schedule_list_matching(SelectQuery::new("MaintenanceSchedule"))
    }

    pub fn have_no_maintenance_schedules(self) -> Self {
        self.without_maintenance_schedule_list_matching(SelectQuery::new("MaintenanceSchedule"))
    }

    pub fn with_maintenance_schedule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MaintenanceSchedule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("maintenance_schedule_list", selection));
        self
    }

    pub fn without_maintenance_schedule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MaintenanceSchedule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("maintenance_schedule_list", selection));
        self
    }

    pub fn select_maintenance_schedule_list(mut self) -> Self {
        self.query = self.query.relation("maintenance_schedule_list");
        self
    }

    pub fn select_maintenance_schedule_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("maintenance_schedule_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("maintenance_schedule_list", selection));
        self
}

    pub fn have_maintenance_events(self) -> Self {
        self.with_maintenance_event_list_matching(SelectQuery::new("MaintenanceEvent"))
    }

    pub fn have_no_maintenance_events(self) -> Self {
        self.without_maintenance_event_list_matching(SelectQuery::new("MaintenanceEvent"))
    }

    pub fn with_maintenance_event_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MaintenanceEvent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("maintenance_event_list", selection));
        self
    }

    pub fn without_maintenance_event_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MaintenanceEvent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("maintenance_event_list", selection));
        self
    }

    pub fn select_maintenance_event_list(mut self) -> Self {
        self.query = self.query.relation("maintenance_event_list");
        self
    }

    pub fn select_maintenance_event_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("maintenance_event_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("maintenance_event_list", selection));
        self
}

    pub fn have_fuel_records(self) -> Self {
        self.with_fuel_record_list_matching(SelectQuery::new("FuelRecord"))
    }

    pub fn have_no_fuel_records(self) -> Self {
        self.without_fuel_record_list_matching(SelectQuery::new("FuelRecord"))
    }

    pub fn with_fuel_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::FuelRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("fuel_record_list", selection));
        self
    }

    pub fn without_fuel_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::FuelRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("fuel_record_list", selection));
        self
    }

    pub fn select_fuel_record_list(mut self) -> Self {
        self.query = self.query.relation("fuel_record_list");
        self
    }

    pub fn select_fuel_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("fuel_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("fuel_record_list", selection));
        self
}

    pub fn have_vehicle_registrations(self) -> Self {
        self.with_vehicle_registration_list_matching(SelectQuery::new("VehicleRegistration"))
    }

    pub fn have_no_vehicle_registrations(self) -> Self {
        self.without_vehicle_registration_list_matching(SelectQuery::new("VehicleRegistration"))
    }

    pub fn with_vehicle_registration_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::VehicleRegistration as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("vehicle_registration_list", selection));
        self
    }

    pub fn without_vehicle_registration_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::VehicleRegistration as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("vehicle_registration_list", selection));
        self
    }

    pub fn select_vehicle_registration_list(mut self) -> Self {
        self.query = self.query.relation("vehicle_registration_list");
        self
    }

    pub fn select_vehicle_registration_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("vehicle_registration_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("vehicle_registration_list", selection));
        self
}

    pub fn have_asset_conditions(self) -> Self {
        self.with_asset_condition_list_matching(SelectQuery::new("AssetCondition"))
    }

    pub fn have_no_asset_conditions(self) -> Self {
        self.without_asset_condition_list_matching(SelectQuery::new("AssetCondition"))
    }

    pub fn with_asset_condition_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AssetCondition as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("asset_condition_list", selection));
        self
    }

    pub fn without_asset_condition_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AssetCondition as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("asset_condition_list", selection));
        self
    }

    pub fn select_asset_condition_list(mut self) -> Self {
        self.query = self.query.relation("asset_condition_list");
        self
    }

    pub fn select_asset_condition_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("asset_condition_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("asset_condition_list", selection));
        self
}

    pub fn have_depreciation_records(self) -> Self {
        self.with_depreciation_record_list_matching(SelectQuery::new("DepreciationRecord"))
    }

    pub fn have_no_depreciation_records(self) -> Self {
        self.without_depreciation_record_list_matching(SelectQuery::new("DepreciationRecord"))
    }

    pub fn with_depreciation_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DepreciationRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("depreciation_record_list", selection));
        self
    }

    pub fn without_depreciation_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DepreciationRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "vehicle_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("depreciation_record_list", selection));
        self
    }

    pub fn select_depreciation_record_list(mut self) -> Self {
        self.query = self.query.relation("depreciation_record_list");
        self
    }

    pub fn select_depreciation_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("depreciation_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("depreciation_record_list", selection));
        self
}
    pub fn count_asset_assignments(self) -> Self {
        self.count_asset_assignments_as("count_asset_assignments")
    }

    pub fn count_asset_assignments_as(self, alias: impl Into<String>) -> Self {
        self.count_asset_assignments_with(alias, crate::Q::asset_assignments().unlimited())
    }

    pub fn count_asset_assignments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "asset_assignment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_asset_assignments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_asset_assignments_as("refinements", request)
    }

    pub fn stats_from_asset_assignments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "asset_assignment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_asset_assignments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_asset_assignments(request)
    }




    pub fn count_asset_inspections(self) -> Self {
        self.count_asset_inspections_as("count_asset_inspections")
    }

    pub fn count_asset_inspections_as(self, alias: impl Into<String>) -> Self {
        self.count_asset_inspections_with(alias, crate::Q::asset_inspections().unlimited())
    }

    pub fn count_asset_inspections_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "asset_inspection_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_asset_inspections(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_asset_inspections_as("refinements", request)
    }

    pub fn stats_from_asset_inspections_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "asset_inspection_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_asset_inspections_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_asset_inspections(request)
    }




    pub fn count_maintenance_schedules(self) -> Self {
        self.count_maintenance_schedules_as("count_maintenance_schedules")
    }

    pub fn count_maintenance_schedules_as(self, alias: impl Into<String>) -> Self {
        self.count_maintenance_schedules_with(alias, crate::Q::maintenance_schedules().unlimited())
    }

    pub fn count_maintenance_schedules_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "maintenance_schedule_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_maintenance_schedules(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_schedules_as("refinements", request)
    }

    pub fn stats_from_maintenance_schedules_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "maintenance_schedule_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_maintenance_schedules_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_schedules(request)
    }




    pub fn count_maintenance_events(self) -> Self {
        self.count_maintenance_events_as("count_maintenance_events")
    }

    pub fn count_maintenance_events_as(self, alias: impl Into<String>) -> Self {
        self.count_maintenance_events_with(alias, crate::Q::maintenance_events().unlimited())
    }

    pub fn count_maintenance_events_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "maintenance_event_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_maintenance_events(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_events_as("refinements", request)
    }

    pub fn stats_from_maintenance_events_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "maintenance_event_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_maintenance_events_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_maintenance_events(request)
    }




    pub fn count_fuel_records(self) -> Self {
        self.count_fuel_records_as("count_fuel_records")
    }

    pub fn count_fuel_records_as(self, alias: impl Into<String>) -> Self {
        self.count_fuel_records_with(alias, crate::Q::fuel_records().unlimited())
    }

    pub fn count_fuel_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "fuel_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_fuel_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_records_as("refinements", request)
    }

    pub fn stats_from_fuel_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "fuel_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_fuel_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_fuel_records(request)
    }




    pub fn count_vehicle_registrations(self) -> Self {
        self.count_vehicle_registrations_as("count_vehicle_registrations")
    }

    pub fn count_vehicle_registrations_as(self, alias: impl Into<String>) -> Self {
        self.count_vehicle_registrations_with(alias, crate::Q::vehicle_registrations().unlimited())
    }

    pub fn count_vehicle_registrations_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vehicle_registration_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_vehicle_registrations(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_registrations_as("refinements", request)
    }

    pub fn stats_from_vehicle_registrations_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "vehicle_registration_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_vehicle_registrations_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_vehicle_registrations(request)
    }




    pub fn count_asset_conditions(self) -> Self {
        self.count_asset_conditions_as("count_asset_conditions")
    }

    pub fn count_asset_conditions_as(self, alias: impl Into<String>) -> Self {
        self.count_asset_conditions_with(alias, crate::Q::asset_conditions().unlimited())
    }

    pub fn count_asset_conditions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "asset_condition_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_asset_conditions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_asset_conditions_as("refinements", request)
    }

    pub fn stats_from_asset_conditions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "asset_condition_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_asset_conditions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_asset_conditions(request)
    }




    pub fn count_depreciation_records(self) -> Self {
        self.count_depreciation_records_as("count_depreciation_records")
    }

    pub fn count_depreciation_records_as(self, alias: impl Into<String>) -> Self {
        self.count_depreciation_records_with(alias, crate::Q::depreciation_records().unlimited())
    }

    pub fn count_depreciation_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "depreciation_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_depreciation_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_depreciation_records_as("refinements", request)
    }

    pub fn stats_from_depreciation_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "depreciation_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_depreciation_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_depreciation_records(request)
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
