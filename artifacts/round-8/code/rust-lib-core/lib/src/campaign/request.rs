use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Campaign {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Campaign {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/campaign
#[derive(Debug)]
pub struct CampaignRequest<R = crate::Campaign> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for CampaignRequest<R> {
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

impl<R> CampaignRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Campaign")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> CampaignRequest<T> {
        CampaignRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .campaign_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .campaign_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CampaignRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CampaignRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CampaignRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .campaign_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Campaign is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .campaign_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .campaign_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::CampaignRepository<'a>>>
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
            "campaign_id" => Some("campaign_id"),
            "name" => Some("name"),
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
            "discount_code_list" => {
                self.with_discount_code_list_matching(
                    crate::Q::discount_codes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "lead_list" => {
                self.with_lead_list_matching(
                    crate::Q::leads_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "conversion_metric_list" => {
                self.with_conversion_metric_list_matching(
                    crate::Q::conversion_metrics_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "audience_segment_list" => {
                self.with_audience_segment_list_matching(
                    crate::Q::audience_segments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "promotional_offer_list" => {
                self.with_promotional_offer_list_matching(
                    crate::Q::promotional_offers_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "attribution_model_list" => {
                self.with_attribution_model_list_matching(
                    crate::Q::attribution_models_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "campaign_budget_list" => {
                self.with_campaign_budget_list_matching(
                    crate::Q::campaign_budgets_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "conversion_report_list" => {
                self.with_conversion_report_list_matching(
                    crate::Q::conversion_reports_minimal()
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
        self.query = self.query.project("campaign_id");
        self.query = self.query.project("name");
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
        request = request.select_discount_code_list();
        request = request.select_lead_list();
        request = request.select_conversion_metric_list();
        request = request.select_audience_segment_list();
        request = request.select_promotional_offer_list();
        request = request.select_attribution_model_list();
        request = request.select_campaign_budget_list();
        request = request.select_conversion_report_list();
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


    pub fn select_campaign_id(mut self) -> Self {
        self.query = self.query.project("campaign_id");
        self
    }

    pub fn project_campaign_id(self) -> Self {
        self.select_campaign_id()
    }

    pub fn select_campaign_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_campaign_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_campaign_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("campaign_id", raw_sql_segment));
        self
    }

    pub fn group_by_campaign_id(self) -> Self {
        self.group_by("campaign_id")
    }

    pub fn group_by_campaign_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("campaign_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("campaign_id"));
        request
    }

    pub fn group_by_campaign_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("campaign_id")
            .aggregate_with_function("campaign_id", alias, function)
    }

    pub fn count_campaign_id(self) -> Self {
        self.count_campaign_id_as("campaign_id_count")
    }

    pub fn count_campaign_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("campaign_id", alias)
    }

    pub fn sum_campaign_id(self) -> Self {
        self.sum_campaign_id_as("sum_campaign_id")
    }

    pub fn sum_campaign_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("campaign_id", alias)
    }

    pub fn avg_campaign_id(self) -> Self {
        self.avg_campaign_id_as("avg_campaign_id")
    }

    pub fn avg_campaign_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("campaign_id", alias)
    }

    pub fn min_campaign_id(self) -> Self {
        self.min_campaign_id_as("min_campaign_id")
    }

    pub fn min_campaign_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("campaign_id", alias)
    }

    pub fn max_campaign_id(self) -> Self {
        self.max_campaign_id_as("max_campaign_id")
    }

    pub fn max_campaign_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("campaign_id", alias)
    }

    pub fn unselect_campaign_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "campaign_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "campaign_id");
        self
    }


    pub fn with_campaign_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "campaign_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_campaign_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "campaign_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_campaign_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("campaign_id", value));
        self
    }



    pub fn with_campaign_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("campaign_id", value));
        self
    }

    pub fn with_campaign_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("campaign_id", value));
        self
    }

    pub fn with_campaign_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("campaign_id", value));
        self
    }

    pub fn with_campaign_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("campaign_id", value));
        self
    }

    pub fn with_campaign_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("campaign_id", value));
        self
    }

    pub fn with_campaign_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("campaign_id", lower, upper));
        self
    }

    pub fn with_campaign_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "campaign_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_campaign_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "campaign_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_campaign_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "campaign_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_campaign_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("campaign_id", value));
        self
    }

    pub fn with_campaign_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("campaign_id", value));
        self
    }

    pub fn with_campaign_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("campaign_id", value));
        self
    }

    pub fn with_campaign_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("campaign_id", value));
        self
    }

    pub fn with_campaign_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("campaign_id", value));
        self
    }

    pub fn with_campaign_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("campaign_id", value));
        self
    }

    pub fn with_campaign_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("campaign_id", value));
        self
    }
    pub fn with_campaign_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("campaign_id", value));
        self
    }

    pub fn with_campaign_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("campaign_id", value));
        self
    }

    pub fn with_campaign_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("campaign_id"));
        self
    }



    pub fn with_campaign_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("campaign_id"));
        self
    }


    pub fn order_by_campaign_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("campaign_id");
        self
    }

    pub fn order_by_campaign_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("campaign_id");
        self
    }

    pub fn order_by_campaign_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("campaign_id");
        self
    }

    pub fn order_by_campaign_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("campaign_id");
        self
    }


    pub fn select_name(mut self) -> Self {
        self.query = self.query.project("name");
        self
    }

    pub fn project_name(self) -> Self {
        self.select_name()
    }

    pub fn select_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("name", raw_sql_segment));
        self
    }

    pub fn group_by_name(self) -> Self {
        self.group_by("name")
    }

    pub fn group_by_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("name"));
        request
    }

    pub fn group_by_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("name")
            .aggregate_with_function("name", alias, function)
    }

    pub fn count_name(self) -> Self {
        self.count_name_as("name_count")
    }

    pub fn count_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("name", alias)
    }

    pub fn sum_name(self) -> Self {
        self.sum_name_as("sum_name")
    }

    pub fn sum_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("name", alias)
    }

    pub fn avg_name(self) -> Self {
        self.avg_name_as("avg_name")
    }

    pub fn avg_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("name", alias)
    }

    pub fn min_name(self) -> Self {
        self.min_name_as("min_name")
    }

    pub fn min_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("name", alias)
    }

    pub fn max_name(self) -> Self {
        self.max_name_as("max_name")
    }

    pub fn max_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("name", alias)
    }

    pub fn unselect_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "name");
        self
    }


    pub fn with_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("name", value));
        self
    }



    pub fn with_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("name", value));
        self
    }

    pub fn with_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("name", value));
        self
    }

    pub fn with_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("name", value));
        self
    }

    pub fn with_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("name", value));
        self
    }

    pub fn with_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("name", value));
        self
    }

    pub fn with_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("name", lower, upper));
        self
    }

    pub fn with_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("name", value));
        self
    }

    pub fn with_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("name", value));
        self
    }

    pub fn with_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("name", value));
        self
    }

    pub fn with_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("name", value));
        self
    }

    pub fn with_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("name", value));
        self
    }

    pub fn with_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("name", value));
        self
    }

    pub fn with_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("name", value));
        self
    }
    pub fn with_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("name", value));
        self
    }

    pub fn with_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("name", value));
        self
    }

    pub fn with_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("name"));
        self
    }



    pub fn with_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("name"));
        self
    }


    pub fn order_by_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("name");
        self
    }

    pub fn order_by_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("name");
        self
    }

    pub fn order_by_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("name");
        self
    }

    pub fn order_by_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("name");
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
    pub fn have_discount_codes(self) -> Self {
        self.with_discount_code_list_matching(SelectQuery::new("DiscountCode"))
    }

    pub fn have_no_discount_codes(self) -> Self {
        self.without_discount_code_list_matching(SelectQuery::new("DiscountCode"))
    }

    pub fn with_discount_code_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DiscountCode as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("discount_code_list", selection));
        self
    }

    pub fn without_discount_code_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DiscountCode as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("discount_code_list", selection));
        self
    }

    pub fn select_discount_code_list(mut self) -> Self {
        self.query = self.query.relation("discount_code_list");
        self
    }

    pub fn select_discount_code_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("discount_code_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("discount_code_list", selection));
        self
}

    pub fn have_leads(self) -> Self {
        self.with_lead_list_matching(SelectQuery::new("Lead"))
    }

    pub fn have_no_leads(self) -> Self {
        self.without_lead_list_matching(SelectQuery::new("Lead"))
    }

    pub fn with_lead_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Lead as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("lead_list", selection));
        self
    }

    pub fn without_lead_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Lead as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("lead_list", selection));
        self
    }

    pub fn select_lead_list(mut self) -> Self {
        self.query = self.query.relation("lead_list");
        self
    }

    pub fn select_lead_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("lead_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("lead_list", selection));
        self
}

    pub fn have_conversion_metrics(self) -> Self {
        self.with_conversion_metric_list_matching(SelectQuery::new("ConversionMetric"))
    }

    pub fn have_no_conversion_metrics(self) -> Self {
        self.without_conversion_metric_list_matching(SelectQuery::new("ConversionMetric"))
    }

    pub fn with_conversion_metric_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ConversionMetric as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("conversion_metric_list", selection));
        self
    }

    pub fn without_conversion_metric_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ConversionMetric as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("conversion_metric_list", selection));
        self
    }

    pub fn select_conversion_metric_list(mut self) -> Self {
        self.query = self.query.relation("conversion_metric_list");
        self
    }

    pub fn select_conversion_metric_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("conversion_metric_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("conversion_metric_list", selection));
        self
}

    pub fn have_audience_segments(self) -> Self {
        self.with_audience_segment_list_matching(SelectQuery::new("AudienceSegment"))
    }

    pub fn have_no_audience_segments(self) -> Self {
        self.without_audience_segment_list_matching(SelectQuery::new("AudienceSegment"))
    }

    pub fn with_audience_segment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AudienceSegment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("audience_segment_list", selection));
        self
    }

    pub fn without_audience_segment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AudienceSegment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("audience_segment_list", selection));
        self
    }

    pub fn select_audience_segment_list(mut self) -> Self {
        self.query = self.query.relation("audience_segment_list");
        self
    }

    pub fn select_audience_segment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("audience_segment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("audience_segment_list", selection));
        self
}

    pub fn have_promotional_offers(self) -> Self {
        self.with_promotional_offer_list_matching(SelectQuery::new("PromotionalOffer"))
    }

    pub fn have_no_promotional_offers(self) -> Self {
        self.without_promotional_offer_list_matching(SelectQuery::new("PromotionalOffer"))
    }

    pub fn with_promotional_offer_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PromotionalOffer as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("promotional_offer_list", selection));
        self
    }

    pub fn without_promotional_offer_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PromotionalOffer as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("promotional_offer_list", selection));
        self
    }

    pub fn select_promotional_offer_list(mut self) -> Self {
        self.query = self.query.relation("promotional_offer_list");
        self
    }

    pub fn select_promotional_offer_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("promotional_offer_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("promotional_offer_list", selection));
        self
}

    pub fn have_attribution_models(self) -> Self {
        self.with_attribution_model_list_matching(SelectQuery::new("AttributionModel"))
    }

    pub fn have_no_attribution_models(self) -> Self {
        self.without_attribution_model_list_matching(SelectQuery::new("AttributionModel"))
    }

    pub fn with_attribution_model_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AttributionModel as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("attribution_model_list", selection));
        self
    }

    pub fn without_attribution_model_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AttributionModel as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("attribution_model_list", selection));
        self
    }

    pub fn select_attribution_model_list(mut self) -> Self {
        self.query = self.query.relation("attribution_model_list");
        self
    }

    pub fn select_attribution_model_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("attribution_model_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("attribution_model_list", selection));
        self
}

    pub fn have_campaign_budgets(self) -> Self {
        self.with_campaign_budget_list_matching(SelectQuery::new("CampaignBudget"))
    }

    pub fn have_no_campaign_budgets(self) -> Self {
        self.without_campaign_budget_list_matching(SelectQuery::new("CampaignBudget"))
    }

    pub fn with_campaign_budget_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CampaignBudget as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("campaign_budget_list", selection));
        self
    }

    pub fn without_campaign_budget_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CampaignBudget as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("campaign_budget_list", selection));
        self
    }

    pub fn select_campaign_budget_list(mut self) -> Self {
        self.query = self.query.relation("campaign_budget_list");
        self
    }

    pub fn select_campaign_budget_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("campaign_budget_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("campaign_budget_list", selection));
        self
}

    pub fn have_conversion_reports(self) -> Self {
        self.with_conversion_report_list_matching(SelectQuery::new("ConversionReport"))
    }

    pub fn have_no_conversion_reports(self) -> Self {
        self.without_conversion_report_list_matching(SelectQuery::new("ConversionReport"))
    }

    pub fn with_conversion_report_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ConversionReport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("conversion_report_list", selection));
        self
    }

    pub fn without_conversion_report_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ConversionReport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "campaign_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("conversion_report_list", selection));
        self
    }

    pub fn select_conversion_report_list(mut self) -> Self {
        self.query = self.query.relation("conversion_report_list");
        self
    }

    pub fn select_conversion_report_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("conversion_report_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("conversion_report_list", selection));
        self
}
    pub fn count_discount_codes(self) -> Self {
        self.count_discount_codes_as("count_discount_codes")
    }

    pub fn count_discount_codes_as(self, alias: impl Into<String>) -> Self {
        self.count_discount_codes_with(alias, crate::Q::discount_codes().unlimited())
    }

    pub fn count_discount_codes_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "discount_code_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_discount_codes(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_discount_codes_as("refinements", request)
    }

    pub fn stats_from_discount_codes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "discount_code_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_discount_codes_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_discount_codes(request)
    }




    pub fn count_leads(self) -> Self {
        self.count_leads_as("count_leads")
    }

    pub fn count_leads_as(self, alias: impl Into<String>) -> Self {
        self.count_leads_with(alias, crate::Q::leads().unlimited())
    }

    pub fn count_leads_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "lead_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_leads(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leads_as("refinements", request)
    }

    pub fn stats_from_leads_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "lead_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_leads_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leads(request)
    }




    pub fn count_conversion_metrics(self) -> Self {
        self.count_conversion_metrics_as("count_conversion_metrics")
    }

    pub fn count_conversion_metrics_as(self, alias: impl Into<String>) -> Self {
        self.count_conversion_metrics_with(alias, crate::Q::conversion_metrics().unlimited())
    }

    pub fn count_conversion_metrics_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "conversion_metric_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_conversion_metrics(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_conversion_metrics_as("refinements", request)
    }

    pub fn stats_from_conversion_metrics_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "conversion_metric_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_conversion_metrics_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_conversion_metrics(request)
    }




    pub fn count_audience_segments(self) -> Self {
        self.count_audience_segments_as("count_audience_segments")
    }

    pub fn count_audience_segments_as(self, alias: impl Into<String>) -> Self {
        self.count_audience_segments_with(alias, crate::Q::audience_segments().unlimited())
    }

    pub fn count_audience_segments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "audience_segment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_audience_segments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_audience_segments_as("refinements", request)
    }

    pub fn stats_from_audience_segments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "audience_segment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_audience_segments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_audience_segments(request)
    }




    pub fn count_promotional_offers(self) -> Self {
        self.count_promotional_offers_as("count_promotional_offers")
    }

    pub fn count_promotional_offers_as(self, alias: impl Into<String>) -> Self {
        self.count_promotional_offers_with(alias, crate::Q::promotional_offers().unlimited())
    }

    pub fn count_promotional_offers_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "promotional_offer_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_promotional_offers(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_promotional_offers_as("refinements", request)
    }

    pub fn stats_from_promotional_offers_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "promotional_offer_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_promotional_offers_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_promotional_offers(request)
    }




    pub fn count_attribution_models(self) -> Self {
        self.count_attribution_models_as("count_attribution_models")
    }

    pub fn count_attribution_models_as(self, alias: impl Into<String>) -> Self {
        self.count_attribution_models_with(alias, crate::Q::attribution_models().unlimited())
    }

    pub fn count_attribution_models_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "attribution_model_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_attribution_models(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attribution_models_as("refinements", request)
    }

    pub fn stats_from_attribution_models_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "attribution_model_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_attribution_models_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attribution_models(request)
    }




    pub fn count_campaign_budgets(self) -> Self {
        self.count_campaign_budgets_as("count_campaign_budgets")
    }

    pub fn count_campaign_budgets_as(self, alias: impl Into<String>) -> Self {
        self.count_campaign_budgets_with(alias, crate::Q::campaign_budgets().unlimited())
    }

    pub fn count_campaign_budgets_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "campaign_budget_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_campaign_budgets(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_campaign_budgets_as("refinements", request)
    }

    pub fn stats_from_campaign_budgets_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "campaign_budget_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_campaign_budgets_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_campaign_budgets(request)
    }




    pub fn count_conversion_reports(self) -> Self {
        self.count_conversion_reports_as("count_conversion_reports")
    }

    pub fn count_conversion_reports_as(self, alias: impl Into<String>) -> Self {
        self.count_conversion_reports_with(alias, crate::Q::conversion_reports().unlimited())
    }

    pub fn count_conversion_reports_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "conversion_report_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_conversion_reports(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_conversion_reports_as("refinements", request)
    }

    pub fn stats_from_conversion_reports_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "conversion_report_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_conversion_reports_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_conversion_reports(request)
    }



}

impl<R> Default for CampaignRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< CampaignRequest<R> > for SelectQuery {
    fn from(request: CampaignRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< CampaignRequest<R> > for QuerySelection {
    fn from(request: CampaignRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Campaign> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::CampaignRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<CampaignRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Campaign
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Campaign::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> CampaignRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::CampaignRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
