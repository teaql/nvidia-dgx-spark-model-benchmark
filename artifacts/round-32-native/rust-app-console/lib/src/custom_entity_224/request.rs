use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::CustomEntity224 {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::CustomEntity224 {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/custom_entity_224
#[derive(Debug)]
pub struct CustomEntity224Request<R = crate::CustomEntity224> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for CustomEntity224Request<R> {
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

impl<R> CustomEntity224Request<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("CustomEntity224")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> CustomEntity224Request<T> {
        CustomEntity224Request {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .custom_entity_224_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .custom_entity_224_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .custom_entity_224_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for CustomEntity224 is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .custom_entity_224_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .custom_entity_224_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
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
            "summary" => Some("summary"),
            "category" => Some("category"),
            "priority_level" => Some("priority_level"),
            "created_at" => Some("created_at"),
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
        self.query = self.query.project("summary");
        self.query = self.query.project("category");
        self.query = self.query.project("priority_level");
        self.query = self.query.project("created_at");
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


    pub fn select_summary(mut self) -> Self {
        self.query = self.query.project("summary");
        self
    }

    pub fn project_summary(self) -> Self {
        self.select_summary()
    }

    pub fn select_summary_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_summary_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_summary_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("summary", raw_sql_segment));
        self
    }

    pub fn group_by_summary(self) -> Self {
        self.group_by("summary")
    }

    pub fn group_by_summary_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("summary");
        request.query = request
            .query
            .project_expr(alias, Expr::column("summary"));
        request
    }

    pub fn group_by_summary_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("summary")
            .aggregate_with_function("summary", alias, function)
    }

    pub fn count_summary(self) -> Self {
        self.count_summary_as("summary_count")
    }

    pub fn count_summary_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("summary", alias)
    }

    pub fn sum_summary(self) -> Self {
        self.sum_summary_as("sum_summary")
    }

    pub fn sum_summary_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("summary", alias)
    }

    pub fn avg_summary(self) -> Self {
        self.avg_summary_as("avg_summary")
    }

    pub fn avg_summary_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("summary", alias)
    }

    pub fn min_summary(self) -> Self {
        self.min_summary_as("min_summary")
    }

    pub fn min_summary_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("summary", alias)
    }

    pub fn max_summary(self) -> Self {
        self.max_summary_as("max_summary")
    }

    pub fn max_summary_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("summary", alias)
    }

    pub fn unselect_summary(mut self) -> Self {
        self.query.projection.retain(|field| field != "summary");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "summary");
        self
    }


    pub fn with_summary(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "summary",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_summary_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "summary",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_summary_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("summary", value));
        self
    }



    pub fn with_summary_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("summary", value));
        self
    }

    pub fn with_summary_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("summary", value));
        self
    }

    pub fn with_summary_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("summary", value));
        self
    }

    pub fn with_summary_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("summary", value));
        self
    }

    pub fn with_summary_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("summary", value));
        self
    }

    pub fn with_summary_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("summary", lower, upper));
        self
    }

    pub fn with_summary_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "summary",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_summary_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "summary",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_summary_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "summary",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_summary_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("summary", value));
        self
    }

    pub fn with_summary_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("summary", value));
        self
    }

    pub fn with_summary_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("summary", value));
        self
    }

    pub fn with_summary_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("summary", value));
        self
    }

    pub fn with_summary_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("summary", value));
        self
    }

    pub fn with_summary_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("summary", value));
        self
    }

    pub fn with_summary_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("summary", value));
        self
    }
    pub fn with_summary_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("summary", value));
        self
    }

    pub fn with_summary_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("summary", value));
        self
    }

    pub fn with_summary_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("summary"));
        self
    }



    pub fn with_summary_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("summary"));
        self
    }


    pub fn order_by_summary_asc(mut self) -> Self {
        self.query = self.query.order_asc("summary");
        self
    }

    pub fn order_by_summary_desc(mut self) -> Self {
        self.query = self.query.order_desc("summary");
        self
    }

    pub fn order_by_summary_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("summary");
        self
    }

    pub fn order_by_summary_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("summary");
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


    pub fn select_priority_level(mut self) -> Self {
        self.query = self.query.project("priority_level");
        self
    }

    pub fn project_priority_level(self) -> Self {
        self.select_priority_level()
    }

    pub fn select_priority_level_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_priority_level_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_priority_level_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("priority_level", raw_sql_segment));
        self
    }

    pub fn select_priority_level_with_function(self, function: AggregateFunction) -> Self {
        self.select_priority_level_as_with_function("priority_level", function)
    }

    pub fn select_priority_level_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("priority_level", alias, function)
    }

    pub fn group_by_priority_level(self) -> Self {
        self.group_by("priority_level")
    }

    pub fn group_by_priority_level_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("priority_level");
        request.query = request
            .query
            .project_expr(alias, Expr::column("priority_level"));
        request
    }

    pub fn group_by_priority_level_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("priority_level")
            .aggregate_with_function("priority_level", alias, function)
    }

    pub fn count_priority_level(self) -> Self {
        self.count_priority_level_as("priority_level_count")
    }

    pub fn count_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("priority_level", alias)
    }

    pub fn sum_priority_level(self) -> Self {
        self.sum_priority_level_as("sum_priority_level")
    }

    pub fn sum_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("priority_level", alias)
    }

    pub fn avg_priority_level(self) -> Self {
        self.avg_priority_level_as("avg_priority_level")
    }

    pub fn avg_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("priority_level", alias)
    }

    pub fn min_priority_level(self) -> Self {
        self.min_priority_level_as("min_priority_level")
    }

    pub fn min_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("priority_level", alias)
    }

    pub fn max_priority_level(self) -> Self {
        self.max_priority_level_as("max_priority_level")
    }

    pub fn max_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("priority_level", alias)
    }

    pub fn standard_deviation_priority_level(self) -> Self {
        self.standard_deviation_priority_level_as("stdDev_priority_level")
    }

    pub fn standard_deviation_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("priority_level", alias)
    }

    pub fn square_root_of_population_standard_deviation_priority_level(self) -> Self {
        self.square_root_of_population_standard_deviation_priority_level_as("stdDevPop_priority_level")
    }

    pub fn square_root_of_population_standard_deviation_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("priority_level", alias)
    }

    pub fn sample_variance_priority_level(self) -> Self {
        self.sample_variance_priority_level_as("varSamp_priority_level")
    }

    pub fn sample_variance_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("priority_level", alias)
    }

    pub fn sample_population_variance_priority_level(self) -> Self {
        self.sample_population_variance_priority_level_as("varPop_priority_level")
    }

    pub fn sample_population_variance_priority_level_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("priority_level", alias)
    }

    pub fn unselect_priority_level(mut self) -> Self {
        self.query.projection.retain(|field| field != "priority_level");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "priority_level");
        self
    }


    pub fn with_priority_level(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "priority_level",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_priority_level_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "priority_level",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_priority_level_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("priority_level", value));
        self
    }



    pub fn with_priority_level_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("priority_level", value));
        self
    }

    pub fn with_priority_level_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("priority_level", value));
        self
    }

    pub fn with_priority_level_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("priority_level", value));
        self
    }

    pub fn with_priority_level_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("priority_level", value));
        self
    }

    pub fn with_priority_level_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("priority_level", value));
        self
    }

    pub fn with_priority_level_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("priority_level", lower, upper));
        self
    }

    pub fn with_priority_level_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "priority_level",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_priority_level_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "priority_level",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_priority_level_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "priority_level",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_priority_level_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("priority_level", value));
        self
    }

    pub fn with_priority_level_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("priority_level", value));
        self
    }

    pub fn with_priority_level_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("priority_level"));
        self
    }



    pub fn with_priority_level_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("priority_level"));
        self
    }


    pub fn order_by_priority_level_asc(mut self) -> Self {
        self.query = self.query.order_asc("priority_level");
        self
    }

    pub fn order_by_priority_level_desc(mut self) -> Self {
        self.query = self.query.order_desc("priority_level");
        self
    }

    pub fn order_by_priority_level_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("priority_level");
        self
    }

    pub fn order_by_priority_level_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("priority_level");
        self
    }


    pub fn select_created_at(mut self) -> Self {
        self.query = self.query.project("created_at");
        self
    }

    pub fn project_created_at(self) -> Self {
        self.select_created_at()
    }

    pub fn select_created_at_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_created_at_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_created_at_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("created_at", raw_sql_segment));
        self
    }

    pub fn group_by_created_at(self) -> Self {
        self.group_by("created_at")
    }

    pub fn group_by_created_at_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("created_at");
        request.query = request
            .query
            .project_expr(alias, Expr::column("created_at"));
        request
    }

    pub fn group_by_created_at_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("created_at")
            .aggregate_with_function("created_at", alias, function)
    }

    pub fn count_created_at(self) -> Self {
        self.count_created_at_as("created_at_count")
    }

    pub fn count_created_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("created_at", alias)
    }

    pub fn sum_created_at(self) -> Self {
        self.sum_created_at_as("sum_created_at")
    }

    pub fn sum_created_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("created_at", alias)
    }

    pub fn avg_created_at(self) -> Self {
        self.avg_created_at_as("avg_created_at")
    }

    pub fn avg_created_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("created_at", alias)
    }

    pub fn min_created_at(self) -> Self {
        self.min_created_at_as("min_created_at")
    }

    pub fn min_created_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("created_at", alias)
    }

    pub fn max_created_at(self) -> Self {
        self.max_created_at_as("max_created_at")
    }

    pub fn max_created_at_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("created_at", alias)
    }

    pub fn unselect_created_at(mut self) -> Self {
        self.query.projection.retain(|field| field != "created_at");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "created_at");
        self
    }


    pub fn with_created_at(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "created_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_created_at_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "created_at",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_created_at_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("created_at", value));
        self
    }



    pub fn with_created_at_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("created_at", value));
        self
    }

    pub fn with_created_at_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("created_at", value));
        self
    }

    pub fn with_created_at_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("created_at", value));
        self
    }

    pub fn with_created_at_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("created_at", value));
        self
    }

    pub fn with_created_at_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("created_at", value));
        self
    }

    pub fn with_created_at_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("created_at", lower, upper));
        self
    }

    pub fn with_created_at_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "created_at",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_created_at_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "created_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_created_at_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "created_at",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_created_at_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("created_at", value));
        self
    }

    pub fn with_created_at_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("created_at", value));
        self
    }

    pub fn with_created_at_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("created_at"));
        self
    }



    pub fn with_created_at_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("created_at"));
        self
    }


    pub fn order_by_created_at_asc(mut self) -> Self {
        self.query = self.query.order_asc("created_at");
        self
    }

    pub fn order_by_created_at_desc(mut self) -> Self {
        self.query = self.query.order_desc("created_at");
        self
    }

    pub fn order_by_created_at_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("created_at");
        self
    }

    pub fn order_by_created_at_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("created_at");
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
    pub fn summary_is_string(self) -> Self {
        self.with_summary_is("string()")
    }

    pub fn with_summary_is_string(self) -> Self {
        self.with_summary_is("string()")
    }



    pub fn with_summary_is_not_string(self) -> Self {
        self.with_summary_is_not("string()")
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



    pub fn priority_level_is_integer(self) -> Self {
        self.with_priority_level_is("integer()")
    }

    pub fn with_priority_level_is_integer(self) -> Self {
        self.with_priority_level_is("integer()")
    }



    pub fn with_priority_level_is_not_integer(self) -> Self {
        self.with_priority_level_is_not("integer()")
    }



    pub fn created_at_is_datetime(self) -> Self {
        self.with_created_at_is("datetime()")
    }

    pub fn with_created_at_is_datetime(self) -> Self {
        self.with_created_at_is("datetime()")
    }



    pub fn with_created_at_is_not_datetime(self) -> Self {
        self.with_created_at_is_not("datetime()")
    }




}

impl<R> Default for CustomEntity224Request<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< CustomEntity224Request<R> > for SelectQuery {
    fn from(request: CustomEntity224Request<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< CustomEntity224Request<R> > for QuerySelection {
    fn from(request: CustomEntity224Request<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::CustomEntity224> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<CustomEntity224Request<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::CustomEntity224
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::CustomEntity224::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> CustomEntity224Request<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::CustomEntity224Repository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
