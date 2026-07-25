use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::DirectDepositInfo {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::DirectDepositInfo {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/direct_deposit_info
#[derive(Debug)]
pub struct DirectDepositInfoRequest<R = crate::DirectDepositInfo> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for DirectDepositInfoRequest<R> {
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

impl<R> DirectDepositInfoRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("DirectDepositInfo")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> DirectDepositInfoRequest<T> {
        DirectDepositInfoRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .direct_deposit_info_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .direct_deposit_info_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .direct_deposit_info_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for DirectDepositInfo is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .direct_deposit_info_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .direct_deposit_info_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
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
            "bank_institution" => Some("bank_institution"),
            "account_number" => Some("account_number"),
            "routing_number" => Some("routing_number"),
            "account_kind" => Some("account_kind"),
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
        self.query = self.query.project("bank_institution");
        self.query = self.query.project("account_number");
        self.query = self.query.project("routing_number");
        self.query = self.query.project("account_kind");
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


    pub fn select_bank_institution(mut self) -> Self {
        self.query = self.query.project("bank_institution");
        self
    }

    pub fn project_bank_institution(self) -> Self {
        self.select_bank_institution()
    }

    pub fn select_bank_institution_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_bank_institution_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_bank_institution_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("bank_institution", raw_sql_segment));
        self
    }

    pub fn group_by_bank_institution(self) -> Self {
        self.group_by("bank_institution")
    }

    pub fn group_by_bank_institution_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("bank_institution");
        request.query = request
            .query
            .project_expr(alias, Expr::column("bank_institution"));
        request
    }

    pub fn group_by_bank_institution_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("bank_institution")
            .aggregate_with_function("bank_institution", alias, function)
    }

    pub fn count_bank_institution(self) -> Self {
        self.count_bank_institution_as("bank_institution_count")
    }

    pub fn count_bank_institution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("bank_institution", alias)
    }

    pub fn sum_bank_institution(self) -> Self {
        self.sum_bank_institution_as("sum_bank_institution")
    }

    pub fn sum_bank_institution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("bank_institution", alias)
    }

    pub fn avg_bank_institution(self) -> Self {
        self.avg_bank_institution_as("avg_bank_institution")
    }

    pub fn avg_bank_institution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("bank_institution", alias)
    }

    pub fn min_bank_institution(self) -> Self {
        self.min_bank_institution_as("min_bank_institution")
    }

    pub fn min_bank_institution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("bank_institution", alias)
    }

    pub fn max_bank_institution(self) -> Self {
        self.max_bank_institution_as("max_bank_institution")
    }

    pub fn max_bank_institution_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("bank_institution", alias)
    }

    pub fn unselect_bank_institution(mut self) -> Self {
        self.query.projection.retain(|field| field != "bank_institution");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "bank_institution");
        self
    }


    pub fn with_bank_institution(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "bank_institution",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_bank_institution_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "bank_institution",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_bank_institution_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("bank_institution", value));
        self
    }



    pub fn with_bank_institution_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("bank_institution", value));
        self
    }

    pub fn with_bank_institution_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("bank_institution", value));
        self
    }

    pub fn with_bank_institution_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("bank_institution", value));
        self
    }

    pub fn with_bank_institution_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("bank_institution", value));
        self
    }

    pub fn with_bank_institution_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("bank_institution", value));
        self
    }

    pub fn with_bank_institution_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("bank_institution", lower, upper));
        self
    }

    pub fn with_bank_institution_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "bank_institution",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_bank_institution_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "bank_institution",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_bank_institution_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "bank_institution",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_bank_institution_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("bank_institution", value));
        self
    }

    pub fn with_bank_institution_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("bank_institution", value));
        self
    }

    pub fn with_bank_institution_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("bank_institution", value));
        self
    }

    pub fn with_bank_institution_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("bank_institution", value));
        self
    }

    pub fn with_bank_institution_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("bank_institution", value));
        self
    }

    pub fn with_bank_institution_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("bank_institution", value));
        self
    }

    pub fn with_bank_institution_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("bank_institution", value));
        self
    }
    pub fn with_bank_institution_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("bank_institution", value));
        self
    }

    pub fn with_bank_institution_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("bank_institution", value));
        self
    }

    pub fn with_bank_institution_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("bank_institution"));
        self
    }



    pub fn with_bank_institution_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("bank_institution"));
        self
    }


    pub fn order_by_bank_institution_asc(mut self) -> Self {
        self.query = self.query.order_asc("bank_institution");
        self
    }

    pub fn order_by_bank_institution_desc(mut self) -> Self {
        self.query = self.query.order_desc("bank_institution");
        self
    }

    pub fn order_by_bank_institution_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("bank_institution");
        self
    }

    pub fn order_by_bank_institution_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("bank_institution");
        self
    }


    pub fn select_account_number(mut self) -> Self {
        self.query = self.query.project("account_number");
        self
    }

    pub fn project_account_number(self) -> Self {
        self.select_account_number()
    }

    pub fn select_account_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_account_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_account_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("account_number", raw_sql_segment));
        self
    }

    pub fn group_by_account_number(self) -> Self {
        self.group_by("account_number")
    }

    pub fn group_by_account_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("account_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("account_number"));
        request
    }

    pub fn group_by_account_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("account_number")
            .aggregate_with_function("account_number", alias, function)
    }

    pub fn count_account_number(self) -> Self {
        self.count_account_number_as("account_number_count")
    }

    pub fn count_account_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("account_number", alias)
    }

    pub fn sum_account_number(self) -> Self {
        self.sum_account_number_as("sum_account_number")
    }

    pub fn sum_account_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("account_number", alias)
    }

    pub fn avg_account_number(self) -> Self {
        self.avg_account_number_as("avg_account_number")
    }

    pub fn avg_account_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("account_number", alias)
    }

    pub fn min_account_number(self) -> Self {
        self.min_account_number_as("min_account_number")
    }

    pub fn min_account_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("account_number", alias)
    }

    pub fn max_account_number(self) -> Self {
        self.max_account_number_as("max_account_number")
    }

    pub fn max_account_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("account_number", alias)
    }

    pub fn unselect_account_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "account_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "account_number");
        self
    }


    pub fn with_account_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "account_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_account_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "account_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_account_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("account_number", value));
        self
    }



    pub fn with_account_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("account_number", value));
        self
    }

    pub fn with_account_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("account_number", value));
        self
    }

    pub fn with_account_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("account_number", value));
        self
    }

    pub fn with_account_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("account_number", value));
        self
    }

    pub fn with_account_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("account_number", value));
        self
    }

    pub fn with_account_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("account_number", lower, upper));
        self
    }

    pub fn with_account_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "account_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_account_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "account_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_account_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "account_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_account_number_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("account_number", value));
        self
    }

    pub fn with_account_number_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("account_number", value));
        self
    }

    pub fn with_account_number_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("account_number", value));
        self
    }

    pub fn with_account_number_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("account_number", value));
        self
    }

    pub fn with_account_number_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("account_number", value));
        self
    }

    pub fn with_account_number_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("account_number", value));
        self
    }

    pub fn with_account_number_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("account_number", value));
        self
    }
    pub fn with_account_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("account_number", value));
        self
    }

    pub fn with_account_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("account_number", value));
        self
    }

    pub fn with_account_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("account_number"));
        self
    }



    pub fn with_account_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("account_number"));
        self
    }


    pub fn order_by_account_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("account_number");
        self
    }

    pub fn order_by_account_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("account_number");
        self
    }

    pub fn order_by_account_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("account_number");
        self
    }

    pub fn order_by_account_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("account_number");
        self
    }


    pub fn select_routing_number(mut self) -> Self {
        self.query = self.query.project("routing_number");
        self
    }

    pub fn project_routing_number(self) -> Self {
        self.select_routing_number()
    }

    pub fn select_routing_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_routing_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_routing_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("routing_number", raw_sql_segment));
        self
    }

    pub fn group_by_routing_number(self) -> Self {
        self.group_by("routing_number")
    }

    pub fn group_by_routing_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("routing_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("routing_number"));
        request
    }

    pub fn group_by_routing_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("routing_number")
            .aggregate_with_function("routing_number", alias, function)
    }

    pub fn count_routing_number(self) -> Self {
        self.count_routing_number_as("routing_number_count")
    }

    pub fn count_routing_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("routing_number", alias)
    }

    pub fn sum_routing_number(self) -> Self {
        self.sum_routing_number_as("sum_routing_number")
    }

    pub fn sum_routing_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("routing_number", alias)
    }

    pub fn avg_routing_number(self) -> Self {
        self.avg_routing_number_as("avg_routing_number")
    }

    pub fn avg_routing_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("routing_number", alias)
    }

    pub fn min_routing_number(self) -> Self {
        self.min_routing_number_as("min_routing_number")
    }

    pub fn min_routing_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("routing_number", alias)
    }

    pub fn max_routing_number(self) -> Self {
        self.max_routing_number_as("max_routing_number")
    }

    pub fn max_routing_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("routing_number", alias)
    }

    pub fn unselect_routing_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "routing_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "routing_number");
        self
    }


    pub fn with_routing_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "routing_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_routing_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "routing_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_routing_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("routing_number", value));
        self
    }



    pub fn with_routing_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("routing_number", value));
        self
    }

    pub fn with_routing_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("routing_number", value));
        self
    }

    pub fn with_routing_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("routing_number", value));
        self
    }

    pub fn with_routing_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("routing_number", value));
        self
    }

    pub fn with_routing_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("routing_number", value));
        self
    }

    pub fn with_routing_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("routing_number", lower, upper));
        self
    }

    pub fn with_routing_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "routing_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_routing_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "routing_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_routing_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "routing_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_routing_number_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("routing_number", value));
        self
    }

    pub fn with_routing_number_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("routing_number", value));
        self
    }

    pub fn with_routing_number_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("routing_number", value));
        self
    }

    pub fn with_routing_number_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("routing_number", value));
        self
    }

    pub fn with_routing_number_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("routing_number", value));
        self
    }

    pub fn with_routing_number_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("routing_number", value));
        self
    }

    pub fn with_routing_number_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("routing_number", value));
        self
    }
    pub fn with_routing_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("routing_number", value));
        self
    }

    pub fn with_routing_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("routing_number", value));
        self
    }

    pub fn with_routing_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("routing_number"));
        self
    }



    pub fn with_routing_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("routing_number"));
        self
    }


    pub fn order_by_routing_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("routing_number");
        self
    }

    pub fn order_by_routing_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("routing_number");
        self
    }

    pub fn order_by_routing_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("routing_number");
        self
    }

    pub fn order_by_routing_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("routing_number");
        self
    }


    pub fn select_account_kind(mut self) -> Self {
        self.query = self.query.project("account_kind");
        self
    }

    pub fn project_account_kind(self) -> Self {
        self.select_account_kind()
    }

    pub fn select_account_kind_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_account_kind_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_account_kind_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("account_kind", raw_sql_segment));
        self
    }

    pub fn group_by_account_kind(self) -> Self {
        self.group_by("account_kind")
    }

    pub fn group_by_account_kind_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("account_kind");
        request.query = request
            .query
            .project_expr(alias, Expr::column("account_kind"));
        request
    }

    pub fn group_by_account_kind_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("account_kind")
            .aggregate_with_function("account_kind", alias, function)
    }

    pub fn count_account_kind(self) -> Self {
        self.count_account_kind_as("account_kind_count")
    }

    pub fn count_account_kind_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("account_kind", alias)
    }

    pub fn sum_account_kind(self) -> Self {
        self.sum_account_kind_as("sum_account_kind")
    }

    pub fn sum_account_kind_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("account_kind", alias)
    }

    pub fn avg_account_kind(self) -> Self {
        self.avg_account_kind_as("avg_account_kind")
    }

    pub fn avg_account_kind_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("account_kind", alias)
    }

    pub fn min_account_kind(self) -> Self {
        self.min_account_kind_as("min_account_kind")
    }

    pub fn min_account_kind_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("account_kind", alias)
    }

    pub fn max_account_kind(self) -> Self {
        self.max_account_kind_as("max_account_kind")
    }

    pub fn max_account_kind_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("account_kind", alias)
    }

    pub fn unselect_account_kind(mut self) -> Self {
        self.query.projection.retain(|field| field != "account_kind");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "account_kind");
        self
    }


    pub fn with_account_kind(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "account_kind",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_account_kind_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "account_kind",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_account_kind_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("account_kind", value));
        self
    }



    pub fn with_account_kind_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("account_kind", value));
        self
    }

    pub fn with_account_kind_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("account_kind", value));
        self
    }

    pub fn with_account_kind_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("account_kind", value));
        self
    }

    pub fn with_account_kind_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("account_kind", value));
        self
    }

    pub fn with_account_kind_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("account_kind", value));
        self
    }

    pub fn with_account_kind_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("account_kind", lower, upper));
        self
    }

    pub fn with_account_kind_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "account_kind",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_account_kind_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "account_kind",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_account_kind_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "account_kind",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_account_kind_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("account_kind", value));
        self
    }

    pub fn with_account_kind_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("account_kind", value));
        self
    }

    pub fn with_account_kind_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("account_kind", value));
        self
    }

    pub fn with_account_kind_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("account_kind", value));
        self
    }

    pub fn with_account_kind_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("account_kind", value));
        self
    }

    pub fn with_account_kind_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("account_kind", value));
        self
    }

    pub fn with_account_kind_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("account_kind", value));
        self
    }
    pub fn with_account_kind_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("account_kind", value));
        self
    }

    pub fn with_account_kind_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("account_kind", value));
        self
    }

    pub fn with_account_kind_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("account_kind"));
        self
    }



    pub fn with_account_kind_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("account_kind"));
        self
    }


    pub fn order_by_account_kind_asc(mut self) -> Self {
        self.query = self.query.order_asc("account_kind");
        self
    }

    pub fn order_by_account_kind_desc(mut self) -> Self {
        self.query = self.query.order_desc("account_kind");
        self
    }

    pub fn order_by_account_kind_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("account_kind");
        self
    }

    pub fn order_by_account_kind_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("account_kind");
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
    pub fn bank_institution_is_string(self) -> Self {
        self.with_bank_institution_is("string()")
    }

    pub fn with_bank_institution_is_string(self) -> Self {
        self.with_bank_institution_is("string()")
    }



    pub fn with_bank_institution_is_not_string(self) -> Self {
        self.with_bank_institution_is_not("string()")
    }



    pub fn account_number_is_string(self) -> Self {
        self.with_account_number_is("string()")
    }

    pub fn with_account_number_is_string(self) -> Self {
        self.with_account_number_is("string()")
    }



    pub fn with_account_number_is_not_string(self) -> Self {
        self.with_account_number_is_not("string()")
    }



    pub fn routing_number_is_string(self) -> Self {
        self.with_routing_number_is("string()")
    }

    pub fn with_routing_number_is_string(self) -> Self {
        self.with_routing_number_is("string()")
    }



    pub fn with_routing_number_is_not_string(self) -> Self {
        self.with_routing_number_is_not("string()")
    }



    pub fn account_kind_is_string(self) -> Self {
        self.with_account_kind_is("string()")
    }

    pub fn with_account_kind_is_string(self) -> Self {
        self.with_account_kind_is("string()")
    }



    pub fn with_account_kind_is_not_string(self) -> Self {
        self.with_account_kind_is_not("string()")
    }




}

impl<R> Default for DirectDepositInfoRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< DirectDepositInfoRequest<R> > for SelectQuery {
    fn from(request: DirectDepositInfoRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< DirectDepositInfoRequest<R> > for QuerySelection {
    fn from(request: DirectDepositInfoRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::DirectDepositInfo> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<DirectDepositInfoRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::DirectDepositInfo
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::DirectDepositInfo::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> DirectDepositInfoRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::DirectDepositInfoRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
