use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::AutomationTrigger {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::AutomationTrigger {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/automation_trigger
#[derive(Debug)]
pub struct AutomationTriggerRequest<R = crate::AutomationTrigger> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for AutomationTriggerRequest<R> {
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

impl<R> AutomationTriggerRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("AutomationTrigger")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> AutomationTriggerRequest<T> {
        AutomationTriggerRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .automation_trigger_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .automation_trigger_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .automation_trigger_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for AutomationTrigger is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .automation_trigger_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .automation_trigger_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
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
            "trigger_event" => Some("trigger_event"),
            "condition_expression" => Some("condition_expression"),
            "is_active" => Some("is_active"),
            "execution_order" => Some("execution_order"),
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
        self.query = self.query.project("trigger_event");
        self.query = self.query.project("condition_expression");
        self.query = self.query.project("is_active");
        self.query = self.query.project("execution_order");
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


    pub fn select_trigger_event(mut self) -> Self {
        self.query = self.query.project("trigger_event");
        self
    }

    pub fn project_trigger_event(self) -> Self {
        self.select_trigger_event()
    }

    pub fn select_trigger_event_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_trigger_event_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_trigger_event_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("trigger_event", raw_sql_segment));
        self
    }

    pub fn group_by_trigger_event(self) -> Self {
        self.group_by("trigger_event")
    }

    pub fn group_by_trigger_event_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("trigger_event");
        request.query = request
            .query
            .project_expr(alias, Expr::column("trigger_event"));
        request
    }

    pub fn group_by_trigger_event_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("trigger_event")
            .aggregate_with_function("trigger_event", alias, function)
    }

    pub fn count_trigger_event(self) -> Self {
        self.count_trigger_event_as("trigger_event_count")
    }

    pub fn count_trigger_event_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("trigger_event", alias)
    }

    pub fn sum_trigger_event(self) -> Self {
        self.sum_trigger_event_as("sum_trigger_event")
    }

    pub fn sum_trigger_event_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("trigger_event", alias)
    }

    pub fn avg_trigger_event(self) -> Self {
        self.avg_trigger_event_as("avg_trigger_event")
    }

    pub fn avg_trigger_event_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("trigger_event", alias)
    }

    pub fn min_trigger_event(self) -> Self {
        self.min_trigger_event_as("min_trigger_event")
    }

    pub fn min_trigger_event_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("trigger_event", alias)
    }

    pub fn max_trigger_event(self) -> Self {
        self.max_trigger_event_as("max_trigger_event")
    }

    pub fn max_trigger_event_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("trigger_event", alias)
    }

    pub fn unselect_trigger_event(mut self) -> Self {
        self.query.projection.retain(|field| field != "trigger_event");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "trigger_event");
        self
    }


    pub fn with_trigger_event(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "trigger_event",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_trigger_event_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "trigger_event",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_trigger_event_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("trigger_event", value));
        self
    }



    pub fn with_trigger_event_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("trigger_event", value));
        self
    }

    pub fn with_trigger_event_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("trigger_event", value));
        self
    }

    pub fn with_trigger_event_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("trigger_event", value));
        self
    }

    pub fn with_trigger_event_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("trigger_event", value));
        self
    }

    pub fn with_trigger_event_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("trigger_event", value));
        self
    }

    pub fn with_trigger_event_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("trigger_event", lower, upper));
        self
    }

    pub fn with_trigger_event_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "trigger_event",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_trigger_event_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "trigger_event",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_trigger_event_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "trigger_event",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_trigger_event_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("trigger_event", value));
        self
    }

    pub fn with_trigger_event_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("trigger_event", value));
        self
    }

    pub fn with_trigger_event_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("trigger_event", value));
        self
    }

    pub fn with_trigger_event_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("trigger_event", value));
        self
    }

    pub fn with_trigger_event_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("trigger_event", value));
        self
    }

    pub fn with_trigger_event_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("trigger_event", value));
        self
    }

    pub fn with_trigger_event_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("trigger_event", value));
        self
    }
    pub fn with_trigger_event_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("trigger_event", value));
        self
    }

    pub fn with_trigger_event_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("trigger_event", value));
        self
    }

    pub fn with_trigger_event_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("trigger_event"));
        self
    }



    pub fn with_trigger_event_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("trigger_event"));
        self
    }


    pub fn order_by_trigger_event_asc(mut self) -> Self {
        self.query = self.query.order_asc("trigger_event");
        self
    }

    pub fn order_by_trigger_event_desc(mut self) -> Self {
        self.query = self.query.order_desc("trigger_event");
        self
    }

    pub fn order_by_trigger_event_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("trigger_event");
        self
    }

    pub fn order_by_trigger_event_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("trigger_event");
        self
    }


    pub fn select_condition_expression(mut self) -> Self {
        self.query = self.query.project("condition_expression");
        self
    }

    pub fn project_condition_expression(self) -> Self {
        self.select_condition_expression()
    }

    pub fn select_condition_expression_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_condition_expression_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_condition_expression_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("condition_expression", raw_sql_segment));
        self
    }

    pub fn group_by_condition_expression(self) -> Self {
        self.group_by("condition_expression")
    }

    pub fn group_by_condition_expression_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("condition_expression");
        request.query = request
            .query
            .project_expr(alias, Expr::column("condition_expression"));
        request
    }

    pub fn group_by_condition_expression_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("condition_expression")
            .aggregate_with_function("condition_expression", alias, function)
    }

    pub fn count_condition_expression(self) -> Self {
        self.count_condition_expression_as("condition_expression_count")
    }

    pub fn count_condition_expression_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("condition_expression", alias)
    }

    pub fn sum_condition_expression(self) -> Self {
        self.sum_condition_expression_as("sum_condition_expression")
    }

    pub fn sum_condition_expression_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("condition_expression", alias)
    }

    pub fn avg_condition_expression(self) -> Self {
        self.avg_condition_expression_as("avg_condition_expression")
    }

    pub fn avg_condition_expression_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("condition_expression", alias)
    }

    pub fn min_condition_expression(self) -> Self {
        self.min_condition_expression_as("min_condition_expression")
    }

    pub fn min_condition_expression_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("condition_expression", alias)
    }

    pub fn max_condition_expression(self) -> Self {
        self.max_condition_expression_as("max_condition_expression")
    }

    pub fn max_condition_expression_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("condition_expression", alias)
    }

    pub fn unselect_condition_expression(mut self) -> Self {
        self.query.projection.retain(|field| field != "condition_expression");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "condition_expression");
        self
    }


    pub fn with_condition_expression(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "condition_expression",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_condition_expression_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "condition_expression",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_condition_expression_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("condition_expression", value));
        self
    }



    pub fn with_condition_expression_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("condition_expression", value));
        self
    }

    pub fn with_condition_expression_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("condition_expression", value));
        self
    }

    pub fn with_condition_expression_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("condition_expression", value));
        self
    }

    pub fn with_condition_expression_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("condition_expression", value));
        self
    }

    pub fn with_condition_expression_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("condition_expression", value));
        self
    }

    pub fn with_condition_expression_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("condition_expression", lower, upper));
        self
    }

    pub fn with_condition_expression_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "condition_expression",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_condition_expression_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "condition_expression",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_condition_expression_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "condition_expression",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_condition_expression_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("condition_expression", value));
        self
    }

    pub fn with_condition_expression_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("condition_expression", value));
        self
    }

    pub fn with_condition_expression_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("condition_expression", value));
        self
    }

    pub fn with_condition_expression_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("condition_expression", value));
        self
    }

    pub fn with_condition_expression_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("condition_expression", value));
        self
    }

    pub fn with_condition_expression_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("condition_expression", value));
        self
    }

    pub fn with_condition_expression_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("condition_expression", value));
        self
    }
    pub fn with_condition_expression_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("condition_expression", value));
        self
    }

    pub fn with_condition_expression_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("condition_expression", value));
        self
    }

    pub fn with_condition_expression_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("condition_expression"));
        self
    }



    pub fn with_condition_expression_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("condition_expression"));
        self
    }


    pub fn order_by_condition_expression_asc(mut self) -> Self {
        self.query = self.query.order_asc("condition_expression");
        self
    }

    pub fn order_by_condition_expression_desc(mut self) -> Self {
        self.query = self.query.order_desc("condition_expression");
        self
    }

    pub fn order_by_condition_expression_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("condition_expression");
        self
    }

    pub fn order_by_condition_expression_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("condition_expression");
        self
    }


    pub fn select_is_active(mut self) -> Self {
        self.query = self.query.project("is_active");
        self
    }

    pub fn project_is_active(self) -> Self {
        self.select_is_active()
    }

    pub fn select_is_active_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_is_active_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_is_active_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("is_active", raw_sql_segment));
        self
    }

    pub fn group_by_is_active(self) -> Self {
        self.group_by("is_active")
    }

    pub fn group_by_is_active_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("is_active");
        request.query = request
            .query
            .project_expr(alias, Expr::column("is_active"));
        request
    }

    pub fn group_by_is_active_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("is_active")
            .aggregate_with_function("is_active", alias, function)
    }

    pub fn count_is_active(self) -> Self {
        self.count_is_active_as("is_active_count")
    }

    pub fn count_is_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("is_active", alias)
    }

    pub fn sum_is_active(self) -> Self {
        self.sum_is_active_as("sum_is_active")
    }

    pub fn sum_is_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("is_active", alias)
    }

    pub fn avg_is_active(self) -> Self {
        self.avg_is_active_as("avg_is_active")
    }

    pub fn avg_is_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("is_active", alias)
    }

    pub fn min_is_active(self) -> Self {
        self.min_is_active_as("min_is_active")
    }

    pub fn min_is_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("is_active", alias)
    }

    pub fn max_is_active(self) -> Self {
        self.max_is_active_as("max_is_active")
    }

    pub fn max_is_active_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("is_active", alias)
    }

    pub fn unselect_is_active(mut self) -> Self {
        self.query.projection.retain(|field| field != "is_active");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "is_active");
        self
    }


    pub fn with_is_active(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "is_active",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_is_active_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "is_active",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_is_active_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("is_active", value));
        self
    }



    pub fn with_is_active_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("is_active", value));
        self
    }

    pub fn with_is_active_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("is_active", value));
        self
    }

    pub fn with_is_active_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("is_active", value));
        self
    }

    pub fn with_is_active_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("is_active", value));
        self
    }

    pub fn with_is_active_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("is_active", value));
        self
    }

    pub fn with_is_active_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("is_active", lower, upper));
        self
    }

    pub fn with_is_active_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "is_active",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_is_active_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "is_active",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_is_active_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "is_active",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_is_active_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("is_active", value));
        self
    }

    pub fn with_is_active_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("is_active", value));
        self
    }

    pub fn with_is_active_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("is_active", value));
        self
    }

    pub fn with_is_active_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("is_active", value));
        self
    }

    pub fn with_is_active_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("is_active", value));
        self
    }

    pub fn with_is_active_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("is_active", value));
        self
    }

    pub fn with_is_active_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("is_active", value));
        self
    }
    pub fn with_is_active_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("is_active", value));
        self
    }

    pub fn with_is_active_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("is_active", value));
        self
    }

    pub fn with_is_active_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("is_active"));
        self
    }



    pub fn with_is_active_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("is_active"));
        self
    }


    pub fn order_by_is_active_asc(mut self) -> Self {
        self.query = self.query.order_asc("is_active");
        self
    }

    pub fn order_by_is_active_desc(mut self) -> Self {
        self.query = self.query.order_desc("is_active");
        self
    }

    pub fn order_by_is_active_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("is_active");
        self
    }

    pub fn order_by_is_active_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("is_active");
        self
    }


    pub fn select_execution_order(mut self) -> Self {
        self.query = self.query.project("execution_order");
        self
    }

    pub fn project_execution_order(self) -> Self {
        self.select_execution_order()
    }

    pub fn select_execution_order_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_execution_order_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_execution_order_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("execution_order", raw_sql_segment));
        self
    }

    pub fn select_execution_order_with_function(self, function: AggregateFunction) -> Self {
        self.select_execution_order_as_with_function("execution_order", function)
    }

    pub fn select_execution_order_as_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.aggregate_with_function("execution_order", alias, function)
    }

    pub fn group_by_execution_order(self) -> Self {
        self.group_by("execution_order")
    }

    pub fn group_by_execution_order_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("execution_order");
        request.query = request
            .query
            .project_expr(alias, Expr::column("execution_order"));
        request
    }

    pub fn group_by_execution_order_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("execution_order")
            .aggregate_with_function("execution_order", alias, function)
    }

    pub fn count_execution_order(self) -> Self {
        self.count_execution_order_as("execution_order_count")
    }

    pub fn count_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("execution_order", alias)
    }

    pub fn sum_execution_order(self) -> Self {
        self.sum_execution_order_as("sum_execution_order")
    }

    pub fn sum_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("execution_order", alias)
    }

    pub fn avg_execution_order(self) -> Self {
        self.avg_execution_order_as("avg_execution_order")
    }

    pub fn avg_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("execution_order", alias)
    }

    pub fn min_execution_order(self) -> Self {
        self.min_execution_order_as("min_execution_order")
    }

    pub fn min_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("execution_order", alias)
    }

    pub fn max_execution_order(self) -> Self {
        self.max_execution_order_as("max_execution_order")
    }

    pub fn max_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("execution_order", alias)
    }

    pub fn standard_deviation_execution_order(self) -> Self {
        self.standard_deviation_execution_order_as("stdDev_execution_order")
    }

    pub fn standard_deviation_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev("execution_order", alias)
    }

    pub fn square_root_of_population_standard_deviation_execution_order(self) -> Self {
        self.square_root_of_population_standard_deviation_execution_order_as("stdDevPop_execution_order")
    }

    pub fn square_root_of_population_standard_deviation_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_stddev_pop("execution_order", alias)
    }

    pub fn sample_variance_execution_order(self) -> Self {
        self.sample_variance_execution_order_as("varSamp_execution_order")
    }

    pub fn sample_variance_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_samp("execution_order", alias)
    }

    pub fn sample_population_variance_execution_order(self) -> Self {
        self.sample_population_variance_execution_order_as("varPop_execution_order")
    }

    pub fn sample_population_variance_execution_order_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_var_pop("execution_order", alias)
    }

    pub fn unselect_execution_order(mut self) -> Self {
        self.query.projection.retain(|field| field != "execution_order");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "execution_order");
        self
    }


    pub fn with_execution_order(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "execution_order",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_execution_order_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "execution_order",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_execution_order_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("execution_order", value));
        self
    }



    pub fn with_execution_order_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("execution_order", value));
        self
    }

    pub fn with_execution_order_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("execution_order", value));
        self
    }

    pub fn with_execution_order_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("execution_order", value));
        self
    }

    pub fn with_execution_order_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("execution_order", value));
        self
    }

    pub fn with_execution_order_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("execution_order", value));
        self
    }

    pub fn with_execution_order_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("execution_order", lower, upper));
        self
    }

    pub fn with_execution_order_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "execution_order",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_execution_order_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "execution_order",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_execution_order_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "execution_order",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_execution_order_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("execution_order", value));
        self
    }

    pub fn with_execution_order_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("execution_order", value));
        self
    }

    pub fn with_execution_order_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("execution_order"));
        self
    }



    pub fn with_execution_order_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("execution_order"));
        self
    }


    pub fn order_by_execution_order_asc(mut self) -> Self {
        self.query = self.query.order_asc("execution_order");
        self
    }

    pub fn order_by_execution_order_desc(mut self) -> Self {
        self.query = self.query.order_desc("execution_order");
        self
    }

    pub fn order_by_execution_order_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("execution_order");
        self
    }

    pub fn order_by_execution_order_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("execution_order");
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
    pub fn trigger_event_is_string(self) -> Self {
        self.with_trigger_event_is("string()")
    }

    pub fn with_trigger_event_is_string(self) -> Self {
        self.with_trigger_event_is("string()")
    }



    pub fn with_trigger_event_is_not_string(self) -> Self {
        self.with_trigger_event_is_not("string()")
    }



    pub fn condition_expression_is_string(self) -> Self {
        self.with_condition_expression_is("string()")
    }

    pub fn with_condition_expression_is_string(self) -> Self {
        self.with_condition_expression_is("string()")
    }



    pub fn with_condition_expression_is_not_string(self) -> Self {
        self.with_condition_expression_is_not("string()")
    }



    pub fn is_active_is_boolean(self) -> Self {
        self.with_is_active_is("boolean()")
    }

    pub fn with_is_active_is_boolean(self) -> Self {
        self.with_is_active_is("boolean()")
    }



    pub fn with_is_active_is_not_boolean(self) -> Self {
        self.with_is_active_is_not("boolean()")
    }



    pub fn execution_order_is_integer(self) -> Self {
        self.with_execution_order_is("integer()")
    }

    pub fn with_execution_order_is_integer(self) -> Self {
        self.with_execution_order_is("integer()")
    }



    pub fn with_execution_order_is_not_integer(self) -> Self {
        self.with_execution_order_is_not("integer()")
    }




}

impl<R> Default for AutomationTriggerRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< AutomationTriggerRequest<R> > for SelectQuery {
    fn from(request: AutomationTriggerRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< AutomationTriggerRequest<R> > for QuerySelection {
    fn from(request: AutomationTriggerRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::AutomationTrigger> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<AutomationTriggerRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::AutomationTrigger
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::AutomationTrigger::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> AutomationTriggerRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::AutomationTriggerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
