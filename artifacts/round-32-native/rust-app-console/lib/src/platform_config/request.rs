use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::PlatformConfig {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::PlatformConfig {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/platform_config
#[derive(Debug)]
pub struct PlatformConfigRequest<R = crate::PlatformConfig> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for PlatformConfigRequest<R> {
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

impl<R> PlatformConfigRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("PlatformConfig")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> PlatformConfigRequest<T> {
        PlatformConfigRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .platform_config_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .platform_config_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .platform_config_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for PlatformConfig is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .platform_config_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .platform_config_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
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
            "config_key" => Some("config_key"),
            "config_value" => Some("config_value"),
            "category" => Some("category"),
            "is_enabled" => Some("is_enabled"),
            "secret_key" => Some("secret_key"),
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
        self.query = self.query.project("config_key");
        self.query = self.query.project("config_value");
        self.query = self.query.project("category");
        self.query = self.query.project("is_enabled");
        self.query = self.query.project("secret_key");
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


    pub fn select_config_key(mut self) -> Self {
        self.query = self.query.project("config_key");
        self
    }

    pub fn project_config_key(self) -> Self {
        self.select_config_key()
    }

    pub fn select_config_key_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_config_key_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_config_key_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("config_key", raw_sql_segment));
        self
    }

    pub fn group_by_config_key(self) -> Self {
        self.group_by("config_key")
    }

    pub fn group_by_config_key_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("config_key");
        request.query = request
            .query
            .project_expr(alias, Expr::column("config_key"));
        request
    }

    pub fn group_by_config_key_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("config_key")
            .aggregate_with_function("config_key", alias, function)
    }

    pub fn count_config_key(self) -> Self {
        self.count_config_key_as("config_key_count")
    }

    pub fn count_config_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("config_key", alias)
    }

    pub fn sum_config_key(self) -> Self {
        self.sum_config_key_as("sum_config_key")
    }

    pub fn sum_config_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("config_key", alias)
    }

    pub fn avg_config_key(self) -> Self {
        self.avg_config_key_as("avg_config_key")
    }

    pub fn avg_config_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("config_key", alias)
    }

    pub fn min_config_key(self) -> Self {
        self.min_config_key_as("min_config_key")
    }

    pub fn min_config_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("config_key", alias)
    }

    pub fn max_config_key(self) -> Self {
        self.max_config_key_as("max_config_key")
    }

    pub fn max_config_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("config_key", alias)
    }

    pub fn unselect_config_key(mut self) -> Self {
        self.query.projection.retain(|field| field != "config_key");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "config_key");
        self
    }


    pub fn with_config_key(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "config_key",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_config_key_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "config_key",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_config_key_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("config_key", value));
        self
    }



    pub fn with_config_key_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("config_key", value));
        self
    }

    pub fn with_config_key_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("config_key", value));
        self
    }

    pub fn with_config_key_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("config_key", value));
        self
    }

    pub fn with_config_key_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("config_key", value));
        self
    }

    pub fn with_config_key_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("config_key", value));
        self
    }

    pub fn with_config_key_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("config_key", lower, upper));
        self
    }

    pub fn with_config_key_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "config_key",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_config_key_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "config_key",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_config_key_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "config_key",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_config_key_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("config_key", value));
        self
    }

    pub fn with_config_key_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("config_key", value));
        self
    }

    pub fn with_config_key_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("config_key", value));
        self
    }

    pub fn with_config_key_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("config_key", value));
        self
    }

    pub fn with_config_key_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("config_key", value));
        self
    }

    pub fn with_config_key_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("config_key", value));
        self
    }

    pub fn with_config_key_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("config_key", value));
        self
    }
    pub fn with_config_key_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("config_key", value));
        self
    }

    pub fn with_config_key_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("config_key", value));
        self
    }

    pub fn with_config_key_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("config_key"));
        self
    }



    pub fn with_config_key_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("config_key"));
        self
    }


    pub fn order_by_config_key_asc(mut self) -> Self {
        self.query = self.query.order_asc("config_key");
        self
    }

    pub fn order_by_config_key_desc(mut self) -> Self {
        self.query = self.query.order_desc("config_key");
        self
    }

    pub fn order_by_config_key_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("config_key");
        self
    }

    pub fn order_by_config_key_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("config_key");
        self
    }


    pub fn select_config_value(mut self) -> Self {
        self.query = self.query.project("config_value");
        self
    }

    pub fn project_config_value(self) -> Self {
        self.select_config_value()
    }

    pub fn select_config_value_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_config_value_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_config_value_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("config_value", raw_sql_segment));
        self
    }

    pub fn group_by_config_value(self) -> Self {
        self.group_by("config_value")
    }

    pub fn group_by_config_value_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("config_value");
        request.query = request
            .query
            .project_expr(alias, Expr::column("config_value"));
        request
    }

    pub fn group_by_config_value_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("config_value")
            .aggregate_with_function("config_value", alias, function)
    }

    pub fn count_config_value(self) -> Self {
        self.count_config_value_as("config_value_count")
    }

    pub fn count_config_value_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("config_value", alias)
    }

    pub fn sum_config_value(self) -> Self {
        self.sum_config_value_as("sum_config_value")
    }

    pub fn sum_config_value_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("config_value", alias)
    }

    pub fn avg_config_value(self) -> Self {
        self.avg_config_value_as("avg_config_value")
    }

    pub fn avg_config_value_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("config_value", alias)
    }

    pub fn min_config_value(self) -> Self {
        self.min_config_value_as("min_config_value")
    }

    pub fn min_config_value_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("config_value", alias)
    }

    pub fn max_config_value(self) -> Self {
        self.max_config_value_as("max_config_value")
    }

    pub fn max_config_value_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("config_value", alias)
    }

    pub fn unselect_config_value(mut self) -> Self {
        self.query.projection.retain(|field| field != "config_value");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "config_value");
        self
    }


    pub fn with_config_value(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "config_value",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_config_value_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "config_value",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_config_value_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("config_value", value));
        self
    }



    pub fn with_config_value_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("config_value", value));
        self
    }

    pub fn with_config_value_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("config_value", value));
        self
    }

    pub fn with_config_value_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("config_value", value));
        self
    }

    pub fn with_config_value_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("config_value", value));
        self
    }

    pub fn with_config_value_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("config_value", value));
        self
    }

    pub fn with_config_value_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("config_value", lower, upper));
        self
    }

    pub fn with_config_value_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "config_value",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_config_value_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "config_value",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_config_value_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "config_value",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_config_value_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("config_value", value));
        self
    }

    pub fn with_config_value_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("config_value", value));
        self
    }

    pub fn with_config_value_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("config_value", value));
        self
    }

    pub fn with_config_value_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("config_value", value));
        self
    }

    pub fn with_config_value_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("config_value", value));
        self
    }

    pub fn with_config_value_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("config_value", value));
        self
    }

    pub fn with_config_value_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("config_value", value));
        self
    }
    pub fn with_config_value_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("config_value", value));
        self
    }

    pub fn with_config_value_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("config_value", value));
        self
    }

    pub fn with_config_value_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("config_value"));
        self
    }



    pub fn with_config_value_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("config_value"));
        self
    }


    pub fn order_by_config_value_asc(mut self) -> Self {
        self.query = self.query.order_asc("config_value");
        self
    }

    pub fn order_by_config_value_desc(mut self) -> Self {
        self.query = self.query.order_desc("config_value");
        self
    }

    pub fn order_by_config_value_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("config_value");
        self
    }

    pub fn order_by_config_value_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("config_value");
        self
    }


    pub fn select_category(mut self) -> Self {
        self.query = self.query.project("category");
        self
    }

    pub fn project_category(self) -> Self {
        self.select_category()
    }

    pub fn select_category_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_category_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_category_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("category", raw_sql_segment));
        self
    }

    pub fn group_by_category(self) -> Self {
        self.group_by("category")
    }

    pub fn group_by_category_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("category");
        request.query = request
            .query
            .project_expr(alias, Expr::column("category"));
        request
    }

    pub fn group_by_category_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("category")
            .aggregate_with_function("category", alias, function)
    }

    pub fn count_category(self) -> Self {
        self.count_category_as("category_count")
    }

    pub fn count_category_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("category", alias)
    }

    pub fn sum_category(self) -> Self {
        self.sum_category_as("sum_category")
    }

    pub fn sum_category_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("category", alias)
    }

    pub fn avg_category(self) -> Self {
        self.avg_category_as("avg_category")
    }

    pub fn avg_category_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("category", alias)
    }

    pub fn min_category(self) -> Self {
        self.min_category_as("min_category")
    }

    pub fn min_category_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("category", alias)
    }

    pub fn max_category(self) -> Self {
        self.max_category_as("max_category")
    }

    pub fn max_category_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("category", alias)
    }

    pub fn unselect_category(mut self) -> Self {
        self.query.projection.retain(|field| field != "category");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "category");
        self
    }


    pub fn with_category(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "category",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_category_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "category",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_category_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("category", value));
        self
    }



    pub fn with_category_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("category", value));
        self
    }

    pub fn with_category_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("category", value));
        self
    }

    pub fn with_category_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("category", value));
        self
    }

    pub fn with_category_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("category", value));
        self
    }

    pub fn with_category_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("category", value));
        self
    }

    pub fn with_category_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("category", lower, upper));
        self
    }

    pub fn with_category_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "category",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_category_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "category",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_category_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "category",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_category_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("category", value));
        self
    }

    pub fn with_category_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("category", value));
        self
    }

    pub fn with_category_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("category", value));
        self
    }

    pub fn with_category_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("category", value));
        self
    }

    pub fn with_category_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("category", value));
        self
    }

    pub fn with_category_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("category", value));
        self
    }

    pub fn with_category_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("category", value));
        self
    }
    pub fn with_category_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("category", value));
        self
    }

    pub fn with_category_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("category", value));
        self
    }

    pub fn with_category_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("category"));
        self
    }



    pub fn with_category_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("category"));
        self
    }


    pub fn order_by_category_asc(mut self) -> Self {
        self.query = self.query.order_asc("category");
        self
    }

    pub fn order_by_category_desc(mut self) -> Self {
        self.query = self.query.order_desc("category");
        self
    }

    pub fn order_by_category_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("category");
        self
    }

    pub fn order_by_category_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("category");
        self
    }


    pub fn select_is_enabled(mut self) -> Self {
        self.query = self.query.project("is_enabled");
        self
    }

    pub fn project_is_enabled(self) -> Self {
        self.select_is_enabled()
    }

    pub fn select_is_enabled_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_is_enabled_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_is_enabled_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("is_enabled", raw_sql_segment));
        self
    }

    pub fn group_by_is_enabled(self) -> Self {
        self.group_by("is_enabled")
    }

    pub fn group_by_is_enabled_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("is_enabled");
        request.query = request
            .query
            .project_expr(alias, Expr::column("is_enabled"));
        request
    }

    pub fn group_by_is_enabled_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("is_enabled")
            .aggregate_with_function("is_enabled", alias, function)
    }

    pub fn count_is_enabled(self) -> Self {
        self.count_is_enabled_as("is_enabled_count")
    }

    pub fn count_is_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("is_enabled", alias)
    }

    pub fn sum_is_enabled(self) -> Self {
        self.sum_is_enabled_as("sum_is_enabled")
    }

    pub fn sum_is_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("is_enabled", alias)
    }

    pub fn avg_is_enabled(self) -> Self {
        self.avg_is_enabled_as("avg_is_enabled")
    }

    pub fn avg_is_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("is_enabled", alias)
    }

    pub fn min_is_enabled(self) -> Self {
        self.min_is_enabled_as("min_is_enabled")
    }

    pub fn min_is_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("is_enabled", alias)
    }

    pub fn max_is_enabled(self) -> Self {
        self.max_is_enabled_as("max_is_enabled")
    }

    pub fn max_is_enabled_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("is_enabled", alias)
    }

    pub fn unselect_is_enabled(mut self) -> Self {
        self.query.projection.retain(|field| field != "is_enabled");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "is_enabled");
        self
    }


    pub fn with_is_enabled(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "is_enabled",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_is_enabled_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "is_enabled",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_is_enabled_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("is_enabled", value));
        self
    }



    pub fn with_is_enabled_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("is_enabled", value));
        self
    }

    pub fn with_is_enabled_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("is_enabled", value));
        self
    }

    pub fn with_is_enabled_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("is_enabled", value));
        self
    }

    pub fn with_is_enabled_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("is_enabled", value));
        self
    }

    pub fn with_is_enabled_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("is_enabled", value));
        self
    }

    pub fn with_is_enabled_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("is_enabled", lower, upper));
        self
    }

    pub fn with_is_enabled_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "is_enabled",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_is_enabled_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "is_enabled",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_is_enabled_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "is_enabled",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_is_enabled_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("is_enabled", value));
        self
    }

    pub fn with_is_enabled_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("is_enabled", value));
        self
    }

    pub fn with_is_enabled_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("is_enabled", value));
        self
    }

    pub fn with_is_enabled_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("is_enabled", value));
        self
    }

    pub fn with_is_enabled_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("is_enabled", value));
        self
    }

    pub fn with_is_enabled_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("is_enabled", value));
        self
    }

    pub fn with_is_enabled_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("is_enabled", value));
        self
    }
    pub fn with_is_enabled_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("is_enabled", value));
        self
    }

    pub fn with_is_enabled_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("is_enabled", value));
        self
    }

    pub fn with_is_enabled_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("is_enabled"));
        self
    }



    pub fn with_is_enabled_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("is_enabled"));
        self
    }


    pub fn order_by_is_enabled_asc(mut self) -> Self {
        self.query = self.query.order_asc("is_enabled");
        self
    }

    pub fn order_by_is_enabled_desc(mut self) -> Self {
        self.query = self.query.order_desc("is_enabled");
        self
    }

    pub fn order_by_is_enabled_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("is_enabled");
        self
    }

    pub fn order_by_is_enabled_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("is_enabled");
        self
    }


    pub fn select_secret_key(mut self) -> Self {
        self.query = self.query.project("secret_key");
        self
    }

    pub fn project_secret_key(self) -> Self {
        self.select_secret_key()
    }

    pub fn select_secret_key_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_secret_key_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_secret_key_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("secret_key", raw_sql_segment));
        self
    }

    pub fn group_by_secret_key(self) -> Self {
        self.group_by("secret_key")
    }

    pub fn group_by_secret_key_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("secret_key");
        request.query = request
            .query
            .project_expr(alias, Expr::column("secret_key"));
        request
    }

    pub fn group_by_secret_key_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("secret_key")
            .aggregate_with_function("secret_key", alias, function)
    }

    pub fn count_secret_key(self) -> Self {
        self.count_secret_key_as("secret_key_count")
    }

    pub fn count_secret_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("secret_key", alias)
    }

    pub fn sum_secret_key(self) -> Self {
        self.sum_secret_key_as("sum_secret_key")
    }

    pub fn sum_secret_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("secret_key", alias)
    }

    pub fn avg_secret_key(self) -> Self {
        self.avg_secret_key_as("avg_secret_key")
    }

    pub fn avg_secret_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("secret_key", alias)
    }

    pub fn min_secret_key(self) -> Self {
        self.min_secret_key_as("min_secret_key")
    }

    pub fn min_secret_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("secret_key", alias)
    }

    pub fn max_secret_key(self) -> Self {
        self.max_secret_key_as("max_secret_key")
    }

    pub fn max_secret_key_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("secret_key", alias)
    }

    pub fn unselect_secret_key(mut self) -> Self {
        self.query.projection.retain(|field| field != "secret_key");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "secret_key");
        self
    }


    pub fn with_secret_key(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "secret_key",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_secret_key_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "secret_key",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_secret_key_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("secret_key", value));
        self
    }



    pub fn with_secret_key_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("secret_key", value));
        self
    }

    pub fn with_secret_key_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("secret_key", value));
        self
    }

    pub fn with_secret_key_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("secret_key", value));
        self
    }

    pub fn with_secret_key_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("secret_key", value));
        self
    }

    pub fn with_secret_key_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("secret_key", value));
        self
    }

    pub fn with_secret_key_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("secret_key", lower, upper));
        self
    }

    pub fn with_secret_key_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "secret_key",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_secret_key_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "secret_key",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_secret_key_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "secret_key",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_secret_key_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("secret_key", value));
        self
    }

    pub fn with_secret_key_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("secret_key", value));
        self
    }

    pub fn with_secret_key_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("secret_key", value));
        self
    }

    pub fn with_secret_key_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("secret_key", value));
        self
    }

    pub fn with_secret_key_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("secret_key", value));
        self
    }

    pub fn with_secret_key_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("secret_key", value));
        self
    }

    pub fn with_secret_key_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("secret_key", value));
        self
    }
    pub fn with_secret_key_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("secret_key", value));
        self
    }

    pub fn with_secret_key_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("secret_key", value));
        self
    }

    pub fn with_secret_key_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("secret_key"));
        self
    }



    pub fn with_secret_key_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("secret_key"));
        self
    }


    pub fn order_by_secret_key_asc(mut self) -> Self {
        self.query = self.query.order_asc("secret_key");
        self
    }

    pub fn order_by_secret_key_desc(mut self) -> Self {
        self.query = self.query.order_desc("secret_key");
        self
    }

    pub fn order_by_secret_key_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("secret_key");
        self
    }

    pub fn order_by_secret_key_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("secret_key");
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
    pub fn config_key_is_string(self) -> Self {
        self.with_config_key_is("string()")
    }

    pub fn with_config_key_is_string(self) -> Self {
        self.with_config_key_is("string()")
    }



    pub fn with_config_key_is_not_string(self) -> Self {
        self.with_config_key_is_not("string()")
    }



    pub fn config_value_is_string(self) -> Self {
        self.with_config_value_is("string()")
    }

    pub fn with_config_value_is_string(self) -> Self {
        self.with_config_value_is("string()")
    }



    pub fn with_config_value_is_not_string(self) -> Self {
        self.with_config_value_is_not("string()")
    }



    pub fn category_is_string(self) -> Self {
        self.with_category_is("string()")
    }

    pub fn with_category_is_string(self) -> Self {
        self.with_category_is("string()")
    }



    pub fn with_category_is_not_string(self) -> Self {
        self.with_category_is_not("string()")
    }



    pub fn is_enabled_is_boolean(self) -> Self {
        self.with_is_enabled_is("boolean()")
    }

    pub fn with_is_enabled_is_boolean(self) -> Self {
        self.with_is_enabled_is("boolean()")
    }



    pub fn with_is_enabled_is_not_boolean(self) -> Self {
        self.with_is_enabled_is_not("boolean()")
    }



    pub fn secret_key_is_string(self) -> Self {
        self.with_secret_key_is("string()")
    }

    pub fn with_secret_key_is_string(self) -> Self {
        self.with_secret_key_is("string()")
    }



    pub fn with_secret_key_is_not_string(self) -> Self {
        self.with_secret_key_is_not("string()")
    }




}

impl<R> Default for PlatformConfigRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< PlatformConfigRequest<R> > for SelectQuery {
    fn from(request: PlatformConfigRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< PlatformConfigRequest<R> > for QuerySelection {
    fn from(request: PlatformConfigRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::PlatformConfig> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<PlatformConfigRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::PlatformConfig
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::PlatformConfig::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> PlatformConfigRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::PlatformConfigRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
