use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::DashcamFootage {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::DashcamFootage {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/dashcam_footage
#[derive(Debug)]
pub struct DashcamFootageRequest<R = crate::DashcamFootage> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for DashcamFootageRequest<R> {
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

impl<R> DashcamFootageRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("DashcamFootage")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> DashcamFootageRequest<T> {
        DashcamFootageRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .dashcam_footage_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .dashcam_footage_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .dashcam_footage_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for DashcamFootage is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .dashcam_footage_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .dashcam_footage_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
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
            "duration_seconds" => Some("duration_seconds"),
            "file_path" => Some("file_path"),
            "resolution" => Some("resolution"),
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
        self.query = self.query.project("duration_seconds");
        self.query = self.query.project("file_path");
        self.query = self.query.project("resolution");
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


    pub fn select_duration_seconds(mut self) -> Self {
        self.query = self.query.project("duration_seconds");
        self
    }

    pub fn project_duration_seconds(self) -> Self {
        self.select_duration_seconds()
    }

    pub fn select_duration_seconds_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_duration_seconds_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_duration_seconds_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("duration_seconds", raw_sql_segment));
        self
    }

    pub fn select_duration_seconds_with_function(self, function: AggregateFunction) -> Self {
        self.select_duration_seconds_as_with_function("duration_seconds", function)
    }

    pub fn select_duration_seconds_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("duration_seconds", alias, function)
    }

    pub fn group_by_duration_seconds(self) -> Self {
        self.group_by("duration_seconds")
    }

    pub fn group_by_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("duration_seconds");
        request.query = request
            .query
            .project_expr(alias, Expr::column("duration_seconds"));
        request
    }

    pub fn group_by_duration_seconds_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("duration_seconds")
            .aggregate_with_function("duration_seconds", alias, function)
    }

    pub fn count_duration_seconds(self) -> Self {
        self.count_duration_seconds_as("duration_seconds_count")
    }

    pub fn count_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("duration_seconds", alias)
    }

    pub fn sum_duration_seconds(self) -> Self {
        self.sum_duration_seconds_as("sum_duration_seconds")
    }

    pub fn sum_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("duration_seconds", alias)
    }

    pub fn avg_duration_seconds(self) -> Self {
        self.avg_duration_seconds_as("avg_duration_seconds")
    }

    pub fn avg_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("duration_seconds", alias)
    }

    pub fn min_duration_seconds(self) -> Self {
        self.min_duration_seconds_as("min_duration_seconds")
    }

    pub fn min_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("duration_seconds", alias)
    }

    pub fn max_duration_seconds(self) -> Self {
        self.max_duration_seconds_as("max_duration_seconds")
    }

    pub fn max_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("duration_seconds", alias)
    }

    pub fn standard_deviation_duration_seconds(self) -> Self {
        self.standard_deviation_duration_seconds_as("stdDev_duration_seconds")
    }

    pub fn standard_deviation_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("duration_seconds", alias)
    }

    pub fn square_root_of_population_standard_deviation_duration_seconds(self) -> Self {
        self.square_root_of_population_standard_deviation_duration_seconds_as("stdDevPop_duration_seconds")
    }

    pub fn square_root_of_population_standard_deviation_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("duration_seconds", alias)
    }

    pub fn sample_variance_duration_seconds(self) -> Self {
        self.sample_variance_duration_seconds_as("varSamp_duration_seconds")
    }

    pub fn sample_variance_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("duration_seconds", alias)
    }

    pub fn sample_population_variance_duration_seconds(self) -> Self {
        self.sample_population_variance_duration_seconds_as("varPop_duration_seconds")
    }

    pub fn sample_population_variance_duration_seconds_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("duration_seconds", alias)
    }

    pub fn unselect_duration_seconds(mut self) -> Self {
        self.query.projection.retain(|field| field != "duration_seconds");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "duration_seconds");
        self
    }


    pub fn with_duration_seconds(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "duration_seconds",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_duration_seconds_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "duration_seconds",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_duration_seconds_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("duration_seconds", value));
        self
    }



    pub fn with_duration_seconds_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("duration_seconds", value));
        self
    }

    pub fn with_duration_seconds_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("duration_seconds", value));
        self
    }

    pub fn with_duration_seconds_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("duration_seconds", value));
        self
    }

    pub fn with_duration_seconds_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("duration_seconds", value));
        self
    }

    pub fn with_duration_seconds_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("duration_seconds", value));
        self
    }

    pub fn with_duration_seconds_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("duration_seconds", lower, upper));
        self
    }

    pub fn with_duration_seconds_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "duration_seconds",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_duration_seconds_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "duration_seconds",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_duration_seconds_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "duration_seconds",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_duration_seconds_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("duration_seconds", value));
        self
    }

    pub fn with_duration_seconds_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("duration_seconds", value));
        self
    }

    pub fn with_duration_seconds_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("duration_seconds"));
        self
    }



    pub fn with_duration_seconds_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("duration_seconds"));
        self
    }


    pub fn order_by_duration_seconds_asc(mut self) -> Self {
        self.query = self.query.order_asc("duration_seconds");
        self
    }

    pub fn order_by_duration_seconds_desc(mut self) -> Self {
        self.query = self.query.order_desc("duration_seconds");
        self
    }

    pub fn order_by_duration_seconds_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("duration_seconds");
        self
    }

    pub fn order_by_duration_seconds_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("duration_seconds");
        self
    }


    pub fn select_file_path(mut self) -> Self {
        self.query = self.query.project("file_path");
        self
    }

    pub fn project_file_path(self) -> Self {
        self.select_file_path()
    }

    pub fn select_file_path_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_file_path_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_file_path_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("file_path", raw_sql_segment));
        self
    }

    pub fn group_by_file_path(self) -> Self {
        self.group_by("file_path")
    }

    pub fn group_by_file_path_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("file_path");
        request.query = request
            .query
            .project_expr(alias, Expr::column("file_path"));
        request
    }

    pub fn group_by_file_path_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("file_path")
            .aggregate_with_function("file_path", alias, function)
    }

    pub fn count_file_path(self) -> Self {
        self.count_file_path_as("file_path_count")
    }

    pub fn count_file_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("file_path", alias)
    }

    pub fn sum_file_path(self) -> Self {
        self.sum_file_path_as("sum_file_path")
    }

    pub fn sum_file_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("file_path", alias)
    }

    pub fn avg_file_path(self) -> Self {
        self.avg_file_path_as("avg_file_path")
    }

    pub fn avg_file_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("file_path", alias)
    }

    pub fn min_file_path(self) -> Self {
        self.min_file_path_as("min_file_path")
    }

    pub fn min_file_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("file_path", alias)
    }

    pub fn max_file_path(self) -> Self {
        self.max_file_path_as("max_file_path")
    }

    pub fn max_file_path_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("file_path", alias)
    }

    pub fn unselect_file_path(mut self) -> Self {
        self.query.projection.retain(|field| field != "file_path");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "file_path");
        self
    }


    pub fn with_file_path(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "file_path",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_file_path_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "file_path",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_file_path_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("file_path", value));
        self
    }



    pub fn with_file_path_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("file_path", value));
        self
    }

    pub fn with_file_path_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("file_path", value));
        self
    }

    pub fn with_file_path_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("file_path", value));
        self
    }

    pub fn with_file_path_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("file_path", value));
        self
    }

    pub fn with_file_path_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("file_path", value));
        self
    }

    pub fn with_file_path_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("file_path", lower, upper));
        self
    }

    pub fn with_file_path_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "file_path",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_file_path_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "file_path",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_file_path_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "file_path",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_file_path_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("file_path", value));
        self
    }

    pub fn with_file_path_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("file_path", value));
        self
    }

    pub fn with_file_path_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("file_path", value));
        self
    }

    pub fn with_file_path_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("file_path", value));
        self
    }

    pub fn with_file_path_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("file_path", value));
        self
    }

    pub fn with_file_path_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("file_path", value));
        self
    }

    pub fn with_file_path_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("file_path", value));
        self
    }
    pub fn with_file_path_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("file_path", value));
        self
    }

    pub fn with_file_path_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("file_path", value));
        self
    }

    pub fn with_file_path_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("file_path"));
        self
    }



    pub fn with_file_path_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("file_path"));
        self
    }


    pub fn order_by_file_path_asc(mut self) -> Self {
        self.query = self.query.order_asc("file_path");
        self
    }

    pub fn order_by_file_path_desc(mut self) -> Self {
        self.query = self.query.order_desc("file_path");
        self
    }

    pub fn order_by_file_path_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("file_path");
        self
    }

    pub fn order_by_file_path_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("file_path");
        self
    }


    pub fn select_resolution(mut self) -> Self {
        self.query = self.query.project("resolution");
        self
    }

    pub fn project_resolution(self) -> Self {
        self.select_resolution()
    }

    pub fn select_resolution_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_resolution_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_resolution_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("resolution", raw_sql_segment));
        self
    }

    pub fn group_by_resolution(self) -> Self {
        self.group_by("resolution")
    }

    pub fn group_by_resolution_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("resolution");
        request.query = request
            .query
            .project_expr(alias, Expr::column("resolution"));
        request
    }

    pub fn group_by_resolution_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("resolution")
            .aggregate_with_function("resolution", alias, function)
    }

    pub fn count_resolution(self) -> Self {
        self.count_resolution_as("resolution_count")
    }

    pub fn count_resolution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("resolution", alias)
    }

    pub fn sum_resolution(self) -> Self {
        self.sum_resolution_as("sum_resolution")
    }

    pub fn sum_resolution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("resolution", alias)
    }

    pub fn avg_resolution(self) -> Self {
        self.avg_resolution_as("avg_resolution")
    }

    pub fn avg_resolution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("resolution", alias)
    }

    pub fn min_resolution(self) -> Self {
        self.min_resolution_as("min_resolution")
    }

    pub fn min_resolution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("resolution", alias)
    }

    pub fn max_resolution(self) -> Self {
        self.max_resolution_as("max_resolution")
    }

    pub fn max_resolution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("resolution", alias)
    }

    pub fn unselect_resolution(mut self) -> Self {
        self.query.projection.retain(|field| field != "resolution");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "resolution");
        self
    }


    pub fn with_resolution(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "resolution",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_resolution_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "resolution",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_resolution_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("resolution", value));
        self
    }



    pub fn with_resolution_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("resolution", value));
        self
    }

    pub fn with_resolution_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("resolution", value));
        self
    }

    pub fn with_resolution_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("resolution", value));
        self
    }

    pub fn with_resolution_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("resolution", value));
        self
    }

    pub fn with_resolution_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("resolution", value));
        self
    }

    pub fn with_resolution_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("resolution", lower, upper));
        self
    }

    pub fn with_resolution_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "resolution",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_resolution_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "resolution",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_resolution_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "resolution",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_resolution_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("resolution", value));
        self
    }

    pub fn with_resolution_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("resolution", value));
        self
    }

    pub fn with_resolution_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("resolution", value));
        self
    }

    pub fn with_resolution_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("resolution", value));
        self
    }

    pub fn with_resolution_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("resolution", value));
        self
    }

    pub fn with_resolution_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("resolution", value));
        self
    }

    pub fn with_resolution_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("resolution", value));
        self
    }
    pub fn with_resolution_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("resolution", value));
        self
    }

    pub fn with_resolution_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("resolution", value));
        self
    }

    pub fn with_resolution_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("resolution"));
        self
    }



    pub fn with_resolution_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("resolution"));
        self
    }


    pub fn order_by_resolution_asc(mut self) -> Self {
        self.query = self.query.order_asc("resolution");
        self
    }

    pub fn order_by_resolution_desc(mut self) -> Self {
        self.query = self.query.order_desc("resolution");
        self
    }

    pub fn order_by_resolution_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("resolution");
        self
    }

    pub fn order_by_resolution_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("resolution");
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



    pub fn duration_seconds_is_integer(self) -> Self {
        self.with_duration_seconds_is("integer()")
    }

    pub fn with_duration_seconds_is_integer(self) -> Self {
        self.with_duration_seconds_is("integer()")
    }



    pub fn with_duration_seconds_is_not_integer(self) -> Self {
        self.with_duration_seconds_is_not("integer()")
    }



    pub fn file_path_is_string(self) -> Self {
        self.with_file_path_is("string()")
    }

    pub fn with_file_path_is_string(self) -> Self {
        self.with_file_path_is("string()")
    }



    pub fn with_file_path_is_not_string(self) -> Self {
        self.with_file_path_is_not("string()")
    }



    pub fn resolution_is_string(self) -> Self {
        self.with_resolution_is("string()")
    }

    pub fn with_resolution_is_string(self) -> Self {
        self.with_resolution_is("string()")
    }



    pub fn with_resolution_is_not_string(self) -> Self {
        self.with_resolution_is_not("string()")
    }




}

impl<R> Default for DashcamFootageRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< DashcamFootageRequest<R> > for SelectQuery {
    fn from(request: DashcamFootageRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< DashcamFootageRequest<R> > for QuerySelection {
    fn from(request: DashcamFootageRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::DashcamFootage> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<DashcamFootageRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::DashcamFootage
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::DashcamFootage::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> DashcamFootageRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::DashcamFootageRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
