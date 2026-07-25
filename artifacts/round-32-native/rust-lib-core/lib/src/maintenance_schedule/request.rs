use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::MaintenanceSchedule {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::MaintenanceSchedule {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/maintenance_schedule
#[derive(Debug)]
pub struct MaintenanceScheduleRequest<R = crate::MaintenanceSchedule> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for MaintenanceScheduleRequest<R> {
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

impl<R> MaintenanceScheduleRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("MaintenanceSchedule")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> MaintenanceScheduleRequest<T> {
        MaintenanceScheduleRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .maintenance_schedule_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .maintenance_schedule_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .maintenance_schedule_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for MaintenanceSchedule is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .maintenance_schedule_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .maintenance_schedule_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
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
            "frequency_days" => Some("frequency_days"),
            "last_service_date" => Some("last_service_date"),
            "next_due_date" => Some("next_due_date"),
            "service_details" => Some("service_details"),
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
        self.query = self.query.project("frequency_days");
        self.query = self.query.project("last_service_date");
        self.query = self.query.project("next_due_date");
        self.query = self.query.project("service_details");
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


    pub fn select_frequency_days(mut self) -> Self {
        self.query = self.query.project("frequency_days");
        self
    }

    pub fn project_frequency_days(self) -> Self {
        self.select_frequency_days()
    }

    pub fn select_frequency_days_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_frequency_days_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_frequency_days_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("frequency_days", raw_sql_segment));
        self
    }

    pub fn select_frequency_days_with_function(self, function: AggregateFunction) -> Self {
        self.select_frequency_days_as_with_function("frequency_days", function)
    }

    pub fn select_frequency_days_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("frequency_days", alias, function)
    }

    pub fn group_by_frequency_days(self) -> Self {
        self.group_by("frequency_days")
    }

    pub fn group_by_frequency_days_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("frequency_days");
        request.query = request
            .query
            .project_expr(alias, Expr::column("frequency_days"));
        request
    }

    pub fn group_by_frequency_days_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("frequency_days")
            .aggregate_with_function("frequency_days", alias, function)
    }

    pub fn count_frequency_days(self) -> Self {
        self.count_frequency_days_as("frequency_days_count")
    }

    pub fn count_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("frequency_days", alias)
    }

    pub fn sum_frequency_days(self) -> Self {
        self.sum_frequency_days_as("sum_frequency_days")
    }

    pub fn sum_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("frequency_days", alias)
    }

    pub fn avg_frequency_days(self) -> Self {
        self.avg_frequency_days_as("avg_frequency_days")
    }

    pub fn avg_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("frequency_days", alias)
    }

    pub fn min_frequency_days(self) -> Self {
        self.min_frequency_days_as("min_frequency_days")
    }

    pub fn min_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("frequency_days", alias)
    }

    pub fn max_frequency_days(self) -> Self {
        self.max_frequency_days_as("max_frequency_days")
    }

    pub fn max_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("frequency_days", alias)
    }

    pub fn standard_deviation_frequency_days(self) -> Self {
        self.standard_deviation_frequency_days_as("stdDev_frequency_days")
    }

    pub fn standard_deviation_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("frequency_days", alias)
    }

    pub fn square_root_of_population_standard_deviation_frequency_days(self) -> Self {
        self.square_root_of_population_standard_deviation_frequency_days_as("stdDevPop_frequency_days")
    }

    pub fn square_root_of_population_standard_deviation_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("frequency_days", alias)
    }

    pub fn sample_variance_frequency_days(self) -> Self {
        self.sample_variance_frequency_days_as("varSamp_frequency_days")
    }

    pub fn sample_variance_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("frequency_days", alias)
    }

    pub fn sample_population_variance_frequency_days(self) -> Self {
        self.sample_population_variance_frequency_days_as("varPop_frequency_days")
    }

    pub fn sample_population_variance_frequency_days_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("frequency_days", alias)
    }

    pub fn unselect_frequency_days(mut self) -> Self {
        self.query.projection.retain(|field| field != "frequency_days");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "frequency_days");
        self
    }


    pub fn with_frequency_days(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "frequency_days",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_frequency_days_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "frequency_days",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_frequency_days_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("frequency_days", value));
        self
    }



    pub fn with_frequency_days_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("frequency_days", value));
        self
    }

    pub fn with_frequency_days_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("frequency_days", value));
        self
    }

    pub fn with_frequency_days_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("frequency_days", value));
        self
    }

    pub fn with_frequency_days_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("frequency_days", value));
        self
    }

    pub fn with_frequency_days_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("frequency_days", value));
        self
    }

    pub fn with_frequency_days_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("frequency_days", lower, upper));
        self
    }

    pub fn with_frequency_days_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "frequency_days",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_frequency_days_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "frequency_days",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_frequency_days_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "frequency_days",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_frequency_days_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("frequency_days", value));
        self
    }

    pub fn with_frequency_days_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("frequency_days", value));
        self
    }

    pub fn with_frequency_days_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("frequency_days"));
        self
    }



    pub fn with_frequency_days_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("frequency_days"));
        self
    }


    pub fn order_by_frequency_days_asc(mut self) -> Self {
        self.query = self.query.order_asc("frequency_days");
        self
    }

    pub fn order_by_frequency_days_desc(mut self) -> Self {
        self.query = self.query.order_desc("frequency_days");
        self
    }

    pub fn order_by_frequency_days_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("frequency_days");
        self
    }

    pub fn order_by_frequency_days_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("frequency_days");
        self
    }


    pub fn select_last_service_date(mut self) -> Self {
        self.query = self.query.project("last_service_date");
        self
    }

    pub fn project_last_service_date(self) -> Self {
        self.select_last_service_date()
    }

    pub fn select_last_service_date_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_last_service_date_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_last_service_date_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("last_service_date", raw_sql_segment));
        self
    }

    pub fn group_by_last_service_date(self) -> Self {
        self.group_by("last_service_date")
    }

    pub fn group_by_last_service_date_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("last_service_date");
        request.query = request
            .query
            .project_expr(alias, Expr::column("last_service_date"));
        request
    }

    pub fn group_by_last_service_date_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("last_service_date")
            .aggregate_with_function("last_service_date", alias, function)
    }

    pub fn count_last_service_date(self) -> Self {
        self.count_last_service_date_as("last_service_date_count")
    }

    pub fn count_last_service_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("last_service_date", alias)
    }

    pub fn sum_last_service_date(self) -> Self {
        self.sum_last_service_date_as("sum_last_service_date")
    }

    pub fn sum_last_service_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("last_service_date", alias)
    }

    pub fn avg_last_service_date(self) -> Self {
        self.avg_last_service_date_as("avg_last_service_date")
    }

    pub fn avg_last_service_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("last_service_date", alias)
    }

    pub fn min_last_service_date(self) -> Self {
        self.min_last_service_date_as("min_last_service_date")
    }

    pub fn min_last_service_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("last_service_date", alias)
    }

    pub fn max_last_service_date(self) -> Self {
        self.max_last_service_date_as("max_last_service_date")
    }

    pub fn max_last_service_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("last_service_date", alias)
    }

    pub fn unselect_last_service_date(mut self) -> Self {
        self.query.projection.retain(|field| field != "last_service_date");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "last_service_date");
        self
    }


    pub fn with_last_service_date(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "last_service_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_last_service_date_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "last_service_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_last_service_date_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("last_service_date", value));
        self
    }



    pub fn with_last_service_date_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("last_service_date", value));
        self
    }

    pub fn with_last_service_date_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("last_service_date", value));
        self
    }

    pub fn with_last_service_date_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("last_service_date", value));
        self
    }

    pub fn with_last_service_date_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("last_service_date", value));
        self
    }

    pub fn with_last_service_date_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("last_service_date", value));
        self
    }

    pub fn with_last_service_date_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("last_service_date", lower, upper));
        self
    }

    pub fn with_last_service_date_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "last_service_date",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_last_service_date_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "last_service_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_last_service_date_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "last_service_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_last_service_date_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("last_service_date", value));
        self
    }

    pub fn with_last_service_date_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("last_service_date", value));
        self
    }

    pub fn with_last_service_date_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("last_service_date"));
        self
    }



    pub fn with_last_service_date_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("last_service_date"));
        self
    }


    pub fn order_by_last_service_date_asc(mut self) -> Self {
        self.query = self.query.order_asc("last_service_date");
        self
    }

    pub fn order_by_last_service_date_desc(mut self) -> Self {
        self.query = self.query.order_desc("last_service_date");
        self
    }

    pub fn order_by_last_service_date_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("last_service_date");
        self
    }

    pub fn order_by_last_service_date_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("last_service_date");
        self
    }


    pub fn select_next_due_date(mut self) -> Self {
        self.query = self.query.project("next_due_date");
        self
    }

    pub fn project_next_due_date(self) -> Self {
        self.select_next_due_date()
    }

    pub fn select_next_due_date_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_next_due_date_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_next_due_date_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("next_due_date", raw_sql_segment));
        self
    }

    pub fn group_by_next_due_date(self) -> Self {
        self.group_by("next_due_date")
    }

    pub fn group_by_next_due_date_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("next_due_date");
        request.query = request
            .query
            .project_expr(alias, Expr::column("next_due_date"));
        request
    }

    pub fn group_by_next_due_date_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("next_due_date")
            .aggregate_with_function("next_due_date", alias, function)
    }

    pub fn count_next_due_date(self) -> Self {
        self.count_next_due_date_as("next_due_date_count")
    }

    pub fn count_next_due_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("next_due_date", alias)
    }

    pub fn sum_next_due_date(self) -> Self {
        self.sum_next_due_date_as("sum_next_due_date")
    }

    pub fn sum_next_due_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("next_due_date", alias)
    }

    pub fn avg_next_due_date(self) -> Self {
        self.avg_next_due_date_as("avg_next_due_date")
    }

    pub fn avg_next_due_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("next_due_date", alias)
    }

    pub fn min_next_due_date(self) -> Self {
        self.min_next_due_date_as("min_next_due_date")
    }

    pub fn min_next_due_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("next_due_date", alias)
    }

    pub fn max_next_due_date(self) -> Self {
        self.max_next_due_date_as("max_next_due_date")
    }

    pub fn max_next_due_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("next_due_date", alias)
    }

    pub fn unselect_next_due_date(mut self) -> Self {
        self.query.projection.retain(|field| field != "next_due_date");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "next_due_date");
        self
    }


    pub fn with_next_due_date(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "next_due_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_next_due_date_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "next_due_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_next_due_date_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("next_due_date", value));
        self
    }



    pub fn with_next_due_date_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("next_due_date", value));
        self
    }

    pub fn with_next_due_date_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("next_due_date", value));
        self
    }

    pub fn with_next_due_date_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("next_due_date", value));
        self
    }

    pub fn with_next_due_date_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("next_due_date", value));
        self
    }

    pub fn with_next_due_date_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("next_due_date", value));
        self
    }

    pub fn with_next_due_date_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("next_due_date", lower, upper));
        self
    }

    pub fn with_next_due_date_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "next_due_date",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_next_due_date_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "next_due_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_next_due_date_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "next_due_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_next_due_date_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("next_due_date", value));
        self
    }

    pub fn with_next_due_date_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("next_due_date", value));
        self
    }

    pub fn with_next_due_date_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("next_due_date"));
        self
    }



    pub fn with_next_due_date_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("next_due_date"));
        self
    }


    pub fn order_by_next_due_date_asc(mut self) -> Self {
        self.query = self.query.order_asc("next_due_date");
        self
    }

    pub fn order_by_next_due_date_desc(mut self) -> Self {
        self.query = self.query.order_desc("next_due_date");
        self
    }

    pub fn order_by_next_due_date_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("next_due_date");
        self
    }

    pub fn order_by_next_due_date_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("next_due_date");
        self
    }


    pub fn select_service_details(mut self) -> Self {
        self.query = self.query.project("service_details");
        self
    }

    pub fn project_service_details(self) -> Self {
        self.select_service_details()
    }

    pub fn select_service_details_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_service_details_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_service_details_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("service_details", raw_sql_segment));
        self
    }

    pub fn group_by_service_details(self) -> Self {
        self.group_by("service_details")
    }

    pub fn group_by_service_details_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("service_details");
        request.query = request
            .query
            .project_expr(alias, Expr::column("service_details"));
        request
    }

    pub fn group_by_service_details_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("service_details")
            .aggregate_with_function("service_details", alias, function)
    }

    pub fn count_service_details(self) -> Self {
        self.count_service_details_as("service_details_count")
    }

    pub fn count_service_details_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("service_details", alias)
    }

    pub fn sum_service_details(self) -> Self {
        self.sum_service_details_as("sum_service_details")
    }

    pub fn sum_service_details_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("service_details", alias)
    }

    pub fn avg_service_details(self) -> Self {
        self.avg_service_details_as("avg_service_details")
    }

    pub fn avg_service_details_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("service_details", alias)
    }

    pub fn min_service_details(self) -> Self {
        self.min_service_details_as("min_service_details")
    }

    pub fn min_service_details_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("service_details", alias)
    }

    pub fn max_service_details(self) -> Self {
        self.max_service_details_as("max_service_details")
    }

    pub fn max_service_details_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("service_details", alias)
    }

    pub fn unselect_service_details(mut self) -> Self {
        self.query.projection.retain(|field| field != "service_details");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "service_details");
        self
    }


    pub fn with_service_details(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "service_details",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_service_details_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "service_details",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_service_details_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("service_details", value));
        self
    }



    pub fn with_service_details_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("service_details", value));
        self
    }

    pub fn with_service_details_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("service_details", value));
        self
    }

    pub fn with_service_details_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("service_details", value));
        self
    }

    pub fn with_service_details_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("service_details", value));
        self
    }

    pub fn with_service_details_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("service_details", value));
        self
    }

    pub fn with_service_details_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("service_details", lower, upper));
        self
    }

    pub fn with_service_details_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "service_details",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_service_details_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "service_details",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_service_details_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "service_details",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_service_details_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("service_details", value));
        self
    }

    pub fn with_service_details_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("service_details", value));
        self
    }

    pub fn with_service_details_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("service_details", value));
        self
    }

    pub fn with_service_details_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("service_details", value));
        self
    }

    pub fn with_service_details_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("service_details", value));
        self
    }

    pub fn with_service_details_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("service_details", value));
        self
    }

    pub fn with_service_details_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("service_details", value));
        self
    }
    pub fn with_service_details_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("service_details", value));
        self
    }

    pub fn with_service_details_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("service_details", value));
        self
    }

    pub fn with_service_details_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("service_details"));
        self
    }



    pub fn with_service_details_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("service_details"));
        self
    }


    pub fn order_by_service_details_asc(mut self) -> Self {
        self.query = self.query.order_asc("service_details");
        self
    }

    pub fn order_by_service_details_desc(mut self) -> Self {
        self.query = self.query.order_desc("service_details");
        self
    }

    pub fn order_by_service_details_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("service_details");
        self
    }

    pub fn order_by_service_details_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("service_details");
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
    pub fn frequency_days_is_integer(self) -> Self {
        self.with_frequency_days_is("integer()")
    }

    pub fn with_frequency_days_is_integer(self) -> Self {
        self.with_frequency_days_is("integer()")
    }



    pub fn with_frequency_days_is_not_integer(self) -> Self {
        self.with_frequency_days_is_not("integer()")
    }



    pub fn last_service_date_is_date(self) -> Self {
        self.with_last_service_date_is("date()")
    }

    pub fn with_last_service_date_is_date(self) -> Self {
        self.with_last_service_date_is("date()")
    }



    pub fn with_last_service_date_is_not_date(self) -> Self {
        self.with_last_service_date_is_not("date()")
    }



    pub fn next_due_date_is_date(self) -> Self {
        self.with_next_due_date_is("date()")
    }

    pub fn with_next_due_date_is_date(self) -> Self {
        self.with_next_due_date_is("date()")
    }



    pub fn with_next_due_date_is_not_date(self) -> Self {
        self.with_next_due_date_is_not("date()")
    }



    pub fn service_details_is_string(self) -> Self {
        self.with_service_details_is("string()")
    }

    pub fn with_service_details_is_string(self) -> Self {
        self.with_service_details_is("string()")
    }



    pub fn with_service_details_is_not_string(self) -> Self {
        self.with_service_details_is_not("string()")
    }




}

impl<R> Default for MaintenanceScheduleRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< MaintenanceScheduleRequest<R> > for SelectQuery {
    fn from(request: MaintenanceScheduleRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< MaintenanceScheduleRequest<R> > for QuerySelection {
    fn from(request: MaintenanceScheduleRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::MaintenanceSchedule> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<MaintenanceScheduleRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::MaintenanceSchedule
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::MaintenanceSchedule::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> MaintenanceScheduleRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::MaintenanceScheduleRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
