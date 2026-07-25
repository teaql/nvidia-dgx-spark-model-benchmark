use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::FuelRecord {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::FuelRecord {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/fuel_record
#[derive(Debug)]
pub struct FuelRecordRequest<R = crate::FuelRecord> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for FuelRecordRequest<R> {
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

impl<R> FuelRecordRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("FuelRecord")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> FuelRecordRequest<T> {
        FuelRecordRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .fuel_record_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .fuel_record_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .fuel_record_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for FuelRecord is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .fuel_record_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .fuel_record_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
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
            "recorded_at" => Some("recorded_at"),
            "gallons" => Some("gallons"),
            "unit_price" => Some("unit_price"),
            "total_cost" => Some("total_cost"),
            "odometer" => Some("odometer"),
            "version" => Some("version"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
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
        self.query = self.query.project("recorded_at");
        self.query = self.query.project("gallons");
        self.query = self.query.project("unit_price");
        self.query = self.query.project("total_cost");
        self.query = self.query.project("odometer");
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
        self.select_all()
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


    pub fn select_recorded_at(mut self) -> Self {
        self.query = self.query.project("recorded_at");
        self
    }

    pub fn project_recorded_at(self) -> Self {
        self.select_recorded_at()
    }

    pub fn select_recorded_at_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_recorded_at_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_recorded_at_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("recorded_at", raw_sql_segment));
        self
    }

    pub fn group_by_recorded_at(self) -> Self {
        self.group_by("recorded_at")
    }

    pub fn group_by_recorded_at_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("recorded_at");
        request.query = request
            .query
            .project_expr(alias, Expr::column("recorded_at"));
        request
    }

    pub fn group_by_recorded_at_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("recorded_at")
            .aggregate_with_function("recorded_at", alias, function)
    }

    pub fn count_recorded_at(self) -> Self {
        self.count_recorded_at_as("recorded_at_count")
    }

    pub fn count_recorded_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("recorded_at", alias)
    }

    pub fn sum_recorded_at(self) -> Self {
        self.sum_recorded_at_as("sum_recorded_at")
    }

    pub fn sum_recorded_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("recorded_at", alias)
    }

    pub fn avg_recorded_at(self) -> Self {
        self.avg_recorded_at_as("avg_recorded_at")
    }

    pub fn avg_recorded_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("recorded_at", alias)
    }

    pub fn min_recorded_at(self) -> Self {
        self.min_recorded_at_as("min_recorded_at")
    }

    pub fn min_recorded_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("recorded_at", alias)
    }

    pub fn max_recorded_at(self) -> Self {
        self.max_recorded_at_as("max_recorded_at")
    }

    pub fn max_recorded_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("recorded_at", alias)
    }

    pub fn unselect_recorded_at(mut self) -> Self {
        self.query.projection.retain(|field| field != "recorded_at");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "recorded_at");
        self
    }


    pub fn with_recorded_at(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "recorded_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_recorded_at_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "recorded_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_recorded_at_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("recorded_at", value));
        self
    }



    pub fn with_recorded_at_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("recorded_at", value));
        self
    }

    pub fn with_recorded_at_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("recorded_at", value));
        self
    }

    pub fn with_recorded_at_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("recorded_at", value));
        self
    }

    pub fn with_recorded_at_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("recorded_at", value));
        self
    }

    pub fn with_recorded_at_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("recorded_at", value));
        self
    }

    pub fn with_recorded_at_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("recorded_at", lower, upper));
        self
    }

    pub fn with_recorded_at_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "recorded_at",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_recorded_at_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "recorded_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_recorded_at_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "recorded_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_recorded_at_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("recorded_at", value));
        self
    }

    pub fn with_recorded_at_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("recorded_at", value));
        self
    }

    pub fn with_recorded_at_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("recorded_at"));
        self
    }



    pub fn with_recorded_at_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("recorded_at"));
        self
    }


    pub fn order_by_recorded_at_asc(mut self) -> Self {
        self.query = self.query.order_asc("recorded_at");
        self
    }

    pub fn order_by_recorded_at_desc(mut self) -> Self {
        self.query = self.query.order_desc("recorded_at");
        self
    }

    pub fn order_by_recorded_at_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("recorded_at");
        self
    }

    pub fn order_by_recorded_at_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("recorded_at");
        self
    }


    pub fn select_gallons(mut self) -> Self {
        self.query = self.query.project("gallons");
        self
    }

    pub fn project_gallons(self) -> Self {
        self.select_gallons()
    }

    pub fn select_gallons_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_gallons_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_gallons_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("gallons", raw_sql_segment));
        self
    }

    pub fn group_by_gallons(self) -> Self {
        self.group_by("gallons")
    }

    pub fn group_by_gallons_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("gallons");
        request.query = request
            .query
            .project_expr(alias, Expr::column("gallons"));
        request
    }

    pub fn group_by_gallons_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("gallons")
            .aggregate_with_function("gallons", alias, function)
    }

    pub fn count_gallons(self) -> Self {
        self.count_gallons_as("gallons_count")
    }

    pub fn count_gallons_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("gallons", alias)
    }

    pub fn sum_gallons(self) -> Self {
        self.sum_gallons_as("sum_gallons")
    }

    pub fn sum_gallons_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("gallons", alias)
    }

    pub fn avg_gallons(self) -> Self {
        self.avg_gallons_as("avg_gallons")
    }

    pub fn avg_gallons_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("gallons", alias)
    }

    pub fn min_gallons(self) -> Self {
        self.min_gallons_as("min_gallons")
    }

    pub fn min_gallons_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("gallons", alias)
    }

    pub fn max_gallons(self) -> Self {
        self.max_gallons_as("max_gallons")
    }

    pub fn max_gallons_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("gallons", alias)
    }

    pub fn unselect_gallons(mut self) -> Self {
        self.query.projection.retain(|field| field != "gallons");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "gallons");
        self
    }


    pub fn with_gallons(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "gallons",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_gallons_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "gallons",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_gallons_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("gallons", value));
        self
    }



    pub fn with_gallons_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("gallons", value));
        self
    }

    pub fn with_gallons_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("gallons", value));
        self
    }

    pub fn with_gallons_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("gallons", value));
        self
    }

    pub fn with_gallons_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("gallons", value));
        self
    }

    pub fn with_gallons_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("gallons", value));
        self
    }

    pub fn with_gallons_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("gallons", lower, upper));
        self
    }

    pub fn with_gallons_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "gallons",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_gallons_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "gallons",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_gallons_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "gallons",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_gallons_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("gallons", value));
        self
    }

    pub fn with_gallons_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("gallons", value));
        self
    }

    pub fn with_gallons_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("gallons", value));
        self
    }

    pub fn with_gallons_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("gallons", value));
        self
    }

    pub fn with_gallons_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("gallons", value));
        self
    }

    pub fn with_gallons_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("gallons", value));
        self
    }

    pub fn with_gallons_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("gallons", value));
        self
    }
    pub fn with_gallons_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("gallons", value));
        self
    }

    pub fn with_gallons_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("gallons", value));
        self
    }

    pub fn with_gallons_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("gallons"));
        self
    }



    pub fn with_gallons_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("gallons"));
        self
    }


    pub fn order_by_gallons_asc(mut self) -> Self {
        self.query = self.query.order_asc("gallons");
        self
    }

    pub fn order_by_gallons_desc(mut self) -> Self {
        self.query = self.query.order_desc("gallons");
        self
    }

    pub fn order_by_gallons_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("gallons");
        self
    }

    pub fn order_by_gallons_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("gallons");
        self
    }


    pub fn select_unit_price(mut self) -> Self {
        self.query = self.query.project("unit_price");
        self
    }

    pub fn project_unit_price(self) -> Self {
        self.select_unit_price()
    }

    pub fn select_unit_price_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_unit_price_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_unit_price_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("unit_price", raw_sql_segment));
        self
    }

    pub fn group_by_unit_price(self) -> Self {
        self.group_by("unit_price")
    }

    pub fn group_by_unit_price_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("unit_price");
        request.query = request
            .query
            .project_expr(alias, Expr::column("unit_price"));
        request
    }

    pub fn group_by_unit_price_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("unit_price")
            .aggregate_with_function("unit_price", alias, function)
    }

    pub fn count_unit_price(self) -> Self {
        self.count_unit_price_as("unit_price_count")
    }

    pub fn count_unit_price_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("unit_price", alias)
    }

    pub fn sum_unit_price(self) -> Self {
        self.sum_unit_price_as("sum_unit_price")
    }

    pub fn sum_unit_price_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("unit_price", alias)
    }

    pub fn avg_unit_price(self) -> Self {
        self.avg_unit_price_as("avg_unit_price")
    }

    pub fn avg_unit_price_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("unit_price", alias)
    }

    pub fn min_unit_price(self) -> Self {
        self.min_unit_price_as("min_unit_price")
    }

    pub fn min_unit_price_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("unit_price", alias)
    }

    pub fn max_unit_price(self) -> Self {
        self.max_unit_price_as("max_unit_price")
    }

    pub fn max_unit_price_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("unit_price", alias)
    }

    pub fn unselect_unit_price(mut self) -> Self {
        self.query.projection.retain(|field| field != "unit_price");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "unit_price");
        self
    }


    pub fn with_unit_price(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "unit_price",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_unit_price_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "unit_price",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_unit_price_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("unit_price", value));
        self
    }



    pub fn with_unit_price_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("unit_price", value));
        self
    }

    pub fn with_unit_price_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("unit_price", value));
        self
    }

    pub fn with_unit_price_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("unit_price", value));
        self
    }

    pub fn with_unit_price_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("unit_price", value));
        self
    }

    pub fn with_unit_price_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("unit_price", value));
        self
    }

    pub fn with_unit_price_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("unit_price", lower, upper));
        self
    }

    pub fn with_unit_price_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "unit_price",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_unit_price_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "unit_price",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_unit_price_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "unit_price",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_unit_price_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("unit_price", value));
        self
    }

    pub fn with_unit_price_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("unit_price", value));
        self
    }

    pub fn with_unit_price_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("unit_price", value));
        self
    }

    pub fn with_unit_price_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("unit_price", value));
        self
    }

    pub fn with_unit_price_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("unit_price", value));
        self
    }

    pub fn with_unit_price_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("unit_price", value));
        self
    }

    pub fn with_unit_price_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("unit_price", value));
        self
    }
    pub fn with_unit_price_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("unit_price", value));
        self
    }

    pub fn with_unit_price_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("unit_price", value));
        self
    }

    pub fn with_unit_price_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("unit_price"));
        self
    }



    pub fn with_unit_price_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("unit_price"));
        self
    }


    pub fn order_by_unit_price_asc(mut self) -> Self {
        self.query = self.query.order_asc("unit_price");
        self
    }

    pub fn order_by_unit_price_desc(mut self) -> Self {
        self.query = self.query.order_desc("unit_price");
        self
    }

    pub fn order_by_unit_price_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("unit_price");
        self
    }

    pub fn order_by_unit_price_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("unit_price");
        self
    }


    pub fn select_total_cost(mut self) -> Self {
        self.query = self.query.project("total_cost");
        self
    }

    pub fn project_total_cost(self) -> Self {
        self.select_total_cost()
    }

    pub fn select_total_cost_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_total_cost_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_total_cost_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("total_cost", raw_sql_segment));
        self
    }

    pub fn group_by_total_cost(self) -> Self {
        self.group_by("total_cost")
    }

    pub fn group_by_total_cost_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("total_cost");
        request.query = request
            .query
            .project_expr(alias, Expr::column("total_cost"));
        request
    }

    pub fn group_by_total_cost_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("total_cost")
            .aggregate_with_function("total_cost", alias, function)
    }

    pub fn count_total_cost(self) -> Self {
        self.count_total_cost_as("total_cost_count")
    }

    pub fn count_total_cost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("total_cost", alias)
    }

    pub fn sum_total_cost(self) -> Self {
        self.sum_total_cost_as("sum_total_cost")
    }

    pub fn sum_total_cost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("total_cost", alias)
    }

    pub fn avg_total_cost(self) -> Self {
        self.avg_total_cost_as("avg_total_cost")
    }

    pub fn avg_total_cost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("total_cost", alias)
    }

    pub fn min_total_cost(self) -> Self {
        self.min_total_cost_as("min_total_cost")
    }

    pub fn min_total_cost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("total_cost", alias)
    }

    pub fn max_total_cost(self) -> Self {
        self.max_total_cost_as("max_total_cost")
    }

    pub fn max_total_cost_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("total_cost", alias)
    }

    pub fn unselect_total_cost(mut self) -> Self {
        self.query.projection.retain(|field| field != "total_cost");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "total_cost");
        self
    }


    pub fn with_total_cost(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "total_cost",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_total_cost_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "total_cost",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_total_cost_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("total_cost", value));
        self
    }



    pub fn with_total_cost_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("total_cost", value));
        self
    }

    pub fn with_total_cost_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("total_cost", value));
        self
    }

    pub fn with_total_cost_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("total_cost", value));
        self
    }

    pub fn with_total_cost_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("total_cost", value));
        self
    }

    pub fn with_total_cost_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("total_cost", value));
        self
    }

    pub fn with_total_cost_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("total_cost", lower, upper));
        self
    }

    pub fn with_total_cost_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "total_cost",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_total_cost_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "total_cost",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_total_cost_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "total_cost",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_total_cost_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("total_cost", value));
        self
    }

    pub fn with_total_cost_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("total_cost", value));
        self
    }

    pub fn with_total_cost_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("total_cost", value));
        self
    }

    pub fn with_total_cost_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("total_cost", value));
        self
    }

    pub fn with_total_cost_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("total_cost", value));
        self
    }

    pub fn with_total_cost_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("total_cost", value));
        self
    }

    pub fn with_total_cost_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("total_cost", value));
        self
    }
    pub fn with_total_cost_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("total_cost", value));
        self
    }

    pub fn with_total_cost_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("total_cost", value));
        self
    }

    pub fn with_total_cost_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("total_cost"));
        self
    }



    pub fn with_total_cost_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("total_cost"));
        self
    }


    pub fn order_by_total_cost_asc(mut self) -> Self {
        self.query = self.query.order_asc("total_cost");
        self
    }

    pub fn order_by_total_cost_desc(mut self) -> Self {
        self.query = self.query.order_desc("total_cost");
        self
    }

    pub fn order_by_total_cost_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("total_cost");
        self
    }

    pub fn order_by_total_cost_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("total_cost");
        self
    }


    pub fn select_odometer(mut self) -> Self {
        self.query = self.query.project("odometer");
        self
    }

    pub fn project_odometer(self) -> Self {
        self.select_odometer()
    }

    pub fn select_odometer_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_odometer_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_odometer_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("odometer", raw_sql_segment));
        self
    }

    pub fn select_odometer_with_function(self, function: AggregateFunction) -> Self {
        self.select_odometer_as_with_function("odometer", function)
    }

    pub fn select_odometer_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("odometer", alias, function)
    }

    pub fn group_by_odometer(self) -> Self {
        self.group_by("odometer")
    }

    pub fn group_by_odometer_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("odometer");
        request.query = request
            .query
            .project_expr(alias, Expr::column("odometer"));
        request
    }

    pub fn group_by_odometer_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("odometer")
            .aggregate_with_function("odometer", alias, function)
    }

    pub fn count_odometer(self) -> Self {
        self.count_odometer_as("odometer_count")
    }

    pub fn count_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("odometer", alias)
    }

    pub fn sum_odometer(self) -> Self {
        self.sum_odometer_as("sum_odometer")
    }

    pub fn sum_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("odometer", alias)
    }

    pub fn avg_odometer(self) -> Self {
        self.avg_odometer_as("avg_odometer")
    }

    pub fn avg_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("odometer", alias)
    }

    pub fn min_odometer(self) -> Self {
        self.min_odometer_as("min_odometer")
    }

    pub fn min_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("odometer", alias)
    }

    pub fn max_odometer(self) -> Self {
        self.max_odometer_as("max_odometer")
    }

    pub fn max_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("odometer", alias)
    }

    pub fn standard_deviation_odometer(self) -> Self {
        self.standard_deviation_odometer_as("stdDev_odometer")
    }

    pub fn standard_deviation_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("odometer", alias)
    }

    pub fn square_root_of_population_standard_deviation_odometer(self) -> Self {
        self.square_root_of_population_standard_deviation_odometer_as("stdDevPop_odometer")
    }

    pub fn square_root_of_population_standard_deviation_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("odometer", alias)
    }

    pub fn sample_variance_odometer(self) -> Self {
        self.sample_variance_odometer_as("varSamp_odometer")
    }

    pub fn sample_variance_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("odometer", alias)
    }

    pub fn sample_population_variance_odometer(self) -> Self {
        self.sample_population_variance_odometer_as("varPop_odometer")
    }

    pub fn sample_population_variance_odometer_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("odometer", alias)
    }

    pub fn unselect_odometer(mut self) -> Self {
        self.query.projection.retain(|field| field != "odometer");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "odometer");
        self
    }


    pub fn with_odometer(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "odometer",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_odometer_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "odometer",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_odometer_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("odometer", value));
        self
    }



    pub fn with_odometer_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("odometer", value));
        self
    }

    pub fn with_odometer_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("odometer", value));
        self
    }

    pub fn with_odometer_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("odometer", value));
        self
    }

    pub fn with_odometer_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("odometer", value));
        self
    }

    pub fn with_odometer_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("odometer", value));
        self
    }

    pub fn with_odometer_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("odometer", lower, upper));
        self
    }

    pub fn with_odometer_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "odometer",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_odometer_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "odometer",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_odometer_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "odometer",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_odometer_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("odometer", value));
        self
    }

    pub fn with_odometer_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("odometer", value));
        self
    }

    pub fn with_odometer_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("odometer"));
        self
    }



    pub fn with_odometer_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("odometer"));
        self
    }


    pub fn order_by_odometer_asc(mut self) -> Self {
        self.query = self.query.order_asc("odometer");
        self
    }

    pub fn order_by_odometer_desc(mut self) -> Self {
        self.query = self.query.order_desc("odometer");
        self
    }

    pub fn order_by_odometer_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("odometer");
        self
    }

    pub fn order_by_odometer_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("odometer");
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
    pub fn recorded_at_is_datetime(self) -> Self {
        self.with_recorded_at_is("datetime()")
    }

    pub fn with_recorded_at_is_datetime(self) -> Self {
        self.with_recorded_at_is("datetime()")
    }



    pub fn with_recorded_at_is_not_datetime(self) -> Self {
        self.with_recorded_at_is_not("datetime()")
    }



    pub fn gallons_is_double(self) -> Self {
        self.with_gallons_is("double()")
    }

    pub fn with_gallons_is_double(self) -> Self {
        self.with_gallons_is("double()")
    }



    pub fn with_gallons_is_not_double(self) -> Self {
        self.with_gallons_is_not("double()")
    }



    pub fn unit_price_is_double(self) -> Self {
        self.with_unit_price_is("double()")
    }

    pub fn with_unit_price_is_double(self) -> Self {
        self.with_unit_price_is("double()")
    }



    pub fn with_unit_price_is_not_double(self) -> Self {
        self.with_unit_price_is_not("double()")
    }



    pub fn total_cost_is_double(self) -> Self {
        self.with_total_cost_is("double()")
    }

    pub fn with_total_cost_is_double(self) -> Self {
        self.with_total_cost_is("double()")
    }



    pub fn with_total_cost_is_not_double(self) -> Self {
        self.with_total_cost_is_not("double()")
    }



    pub fn odometer_is_integer(self) -> Self {
        self.with_odometer_is("integer()")
    }

    pub fn with_odometer_is_integer(self) -> Self {
        self.with_odometer_is("integer()")
    }



    pub fn with_odometer_is_not_integer(self) -> Self {
        self.with_odometer_is_not("integer()")
    }




}

impl<R> Default for FuelRecordRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< FuelRecordRequest<R> > for SelectQuery {
    fn from(request: FuelRecordRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< FuelRecordRequest<R> > for QuerySelection {
    fn from(request: FuelRecordRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::FuelRecord> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::FuelRecordRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<FuelRecordRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::FuelRecord
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::FuelRecord::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> FuelRecordRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::FuelRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
