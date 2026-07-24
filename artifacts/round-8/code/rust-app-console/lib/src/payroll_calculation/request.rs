use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::PayrollCalculation {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::PayrollCalculation {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/payroll_calculation
#[derive(Debug)]
pub struct PayrollCalculationRequest<R = crate::PayrollCalculation> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for PayrollCalculationRequest<R> {
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

impl<R> PayrollCalculationRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("PayrollCalculation")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> PayrollCalculationRequest<T> {
        PayrollCalculationRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .payroll_calculation_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .payroll_calculation_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .payroll_calculation_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for PayrollCalculation is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .payroll_calculation_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .payroll_calculation_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
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
            "calc_id" => Some("calc_id"),
            "gross_pay" => Some("gross_pay"),
            "version" => Some("version"),
            "payroll_period_ref" | "payroll_period_ref_id" => Some("payroll_period_ref_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "payroll_period_ref" => {
                self.with_payroll_period_ref_matching(
                    crate::Q::payroll_periods_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "payslip_list" => {
                self.with_payslip_list_matching(
                    crate::Q::payslips_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "deduction_list" => {
                self.with_deduction_list_matching(
                    crate::Q::deductions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tax_withholding_list" => {
                self.with_tax_withholding_list_matching(
                    crate::Q::tax_withholdings_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "payroll_adjustment_list" => {
                self.with_payroll_adjustment_list_matching(
                    crate::Q::payroll_adjustments_minimal()
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
        self.query = self.query.project("calc_id");
        self.query = self.query.project("gross_pay");
        self.query = self.query.project("version");
        self.query = self.query.project("payroll_period_ref_id");
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
        request = request.select_payroll_period_ref();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_payslip_list();
        request = request.select_deduction_list();
        request = request.select_tax_withholding_list();
        request = request.select_payroll_adjustment_list();
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


    pub fn select_calc_id(mut self) -> Self {
        self.query = self.query.project("calc_id");
        self
    }

    pub fn project_calc_id(self) -> Self {
        self.select_calc_id()
    }

    pub fn select_calc_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_calc_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_calc_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("calc_id", raw_sql_segment));
        self
    }

    pub fn group_by_calc_id(self) -> Self {
        self.group_by("calc_id")
    }

    pub fn group_by_calc_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("calc_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("calc_id"));
        request
    }

    pub fn group_by_calc_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("calc_id")
            .aggregate_with_function("calc_id", alias, function)
    }

    pub fn count_calc_id(self) -> Self {
        self.count_calc_id_as("calc_id_count")
    }

    pub fn count_calc_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("calc_id", alias)
    }

    pub fn sum_calc_id(self) -> Self {
        self.sum_calc_id_as("sum_calc_id")
    }

    pub fn sum_calc_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("calc_id", alias)
    }

    pub fn avg_calc_id(self) -> Self {
        self.avg_calc_id_as("avg_calc_id")
    }

    pub fn avg_calc_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("calc_id", alias)
    }

    pub fn min_calc_id(self) -> Self {
        self.min_calc_id_as("min_calc_id")
    }

    pub fn min_calc_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("calc_id", alias)
    }

    pub fn max_calc_id(self) -> Self {
        self.max_calc_id_as("max_calc_id")
    }

    pub fn max_calc_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("calc_id", alias)
    }

    pub fn unselect_calc_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "calc_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "calc_id");
        self
    }


    pub fn with_calc_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "calc_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_calc_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "calc_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_calc_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("calc_id", value));
        self
    }



    pub fn with_calc_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("calc_id", value));
        self
    }

    pub fn with_calc_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("calc_id", value));
        self
    }

    pub fn with_calc_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("calc_id", value));
        self
    }

    pub fn with_calc_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("calc_id", value));
        self
    }

    pub fn with_calc_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("calc_id", value));
        self
    }

    pub fn with_calc_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("calc_id", lower, upper));
        self
    }

    pub fn with_calc_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "calc_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_calc_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "calc_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_calc_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "calc_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_calc_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("calc_id", value));
        self
    }

    pub fn with_calc_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("calc_id", value));
        self
    }

    pub fn with_calc_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("calc_id", value));
        self
    }

    pub fn with_calc_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("calc_id", value));
        self
    }

    pub fn with_calc_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("calc_id", value));
        self
    }

    pub fn with_calc_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("calc_id", value));
        self
    }

    pub fn with_calc_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("calc_id", value));
        self
    }
    pub fn with_calc_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("calc_id", value));
        self
    }

    pub fn with_calc_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("calc_id", value));
        self
    }

    pub fn with_calc_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("calc_id"));
        self
    }



    pub fn with_calc_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("calc_id"));
        self
    }


    pub fn order_by_calc_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("calc_id");
        self
    }

    pub fn order_by_calc_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("calc_id");
        self
    }

    pub fn order_by_calc_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("calc_id");
        self
    }

    pub fn order_by_calc_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("calc_id");
        self
    }


    pub fn select_gross_pay(mut self) -> Self {
        self.query = self.query.project("gross_pay");
        self
    }

    pub fn project_gross_pay(self) -> Self {
        self.select_gross_pay()
    }

    pub fn select_gross_pay_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_gross_pay_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_gross_pay_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("gross_pay", raw_sql_segment));
        self
    }

    pub fn group_by_gross_pay(self) -> Self {
        self.group_by("gross_pay")
    }

    pub fn group_by_gross_pay_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("gross_pay");
        request.query = request
            .query
            .project_expr(alias, Expr::column("gross_pay"));
        request
    }

    pub fn group_by_gross_pay_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("gross_pay")
            .aggregate_with_function("gross_pay", alias, function)
    }

    pub fn count_gross_pay(self) -> Self {
        self.count_gross_pay_as("gross_pay_count")
    }

    pub fn count_gross_pay_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("gross_pay", alias)
    }

    pub fn sum_gross_pay(self) -> Self {
        self.sum_gross_pay_as("sum_gross_pay")
    }

    pub fn sum_gross_pay_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("gross_pay", alias)
    }

    pub fn avg_gross_pay(self) -> Self {
        self.avg_gross_pay_as("avg_gross_pay")
    }

    pub fn avg_gross_pay_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("gross_pay", alias)
    }

    pub fn min_gross_pay(self) -> Self {
        self.min_gross_pay_as("min_gross_pay")
    }

    pub fn min_gross_pay_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("gross_pay", alias)
    }

    pub fn max_gross_pay(self) -> Self {
        self.max_gross_pay_as("max_gross_pay")
    }

    pub fn max_gross_pay_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("gross_pay", alias)
    }

    pub fn unselect_gross_pay(mut self) -> Self {
        self.query.projection.retain(|field| field != "gross_pay");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "gross_pay");
        self
    }


    pub fn with_gross_pay(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "gross_pay",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_gross_pay_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "gross_pay",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_gross_pay_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("gross_pay", value));
        self
    }



    pub fn with_gross_pay_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("gross_pay", value));
        self
    }

    pub fn with_gross_pay_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("gross_pay", value));
        self
    }

    pub fn with_gross_pay_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("gross_pay", value));
        self
    }

    pub fn with_gross_pay_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("gross_pay", value));
        self
    }

    pub fn with_gross_pay_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("gross_pay", value));
        self
    }

    pub fn with_gross_pay_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("gross_pay", lower, upper));
        self
    }

    pub fn with_gross_pay_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "gross_pay",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_gross_pay_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "gross_pay",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_gross_pay_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "gross_pay",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_gross_pay_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("gross_pay", value));
        self
    }

    pub fn with_gross_pay_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("gross_pay", value));
        self
    }

    pub fn with_gross_pay_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("gross_pay", value));
        self
    }

    pub fn with_gross_pay_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("gross_pay", value));
        self
    }

    pub fn with_gross_pay_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("gross_pay", value));
        self
    }

    pub fn with_gross_pay_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("gross_pay", value));
        self
    }

    pub fn with_gross_pay_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("gross_pay", value));
        self
    }
    pub fn with_gross_pay_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("gross_pay", value));
        self
    }

    pub fn with_gross_pay_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("gross_pay", value));
        self
    }

    pub fn with_gross_pay_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("gross_pay"));
        self
    }



    pub fn with_gross_pay_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("gross_pay"));
        self
    }


    pub fn order_by_gross_pay_asc(mut self) -> Self {
        self.query = self.query.order_asc("gross_pay");
        self
    }

    pub fn order_by_gross_pay_desc(mut self) -> Self {
        self.query = self.query.order_desc("gross_pay");
        self
    }

    pub fn order_by_gross_pay_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("gross_pay");
        self
    }

    pub fn order_by_gross_pay_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("gross_pay");
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
    pub fn filter_by_payroll_period_ref(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("payroll_period_ref_id", value.entity_id_value()));
        self
    }

    pub fn with_payroll_period_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "payroll_period_ref_id",
            <crate::PayrollPeriod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_period_ref", selection));
        self
    }


    pub fn without_payroll_period_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "payroll_period_ref_id",
            <crate::PayrollPeriod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_period_ref", selection));
        self
    }


    pub fn have_payroll_period_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("payroll_period_ref_id"));
        self
    }

    pub fn have_no_payroll_period_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("payroll_period_ref_id"));
        self
    }


    pub fn group_by_payroll_period_ref(self) -> Self {
        self.group_by("payroll_period_ref_id")
    }

    pub fn group_by_payroll_period_ref_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("payroll_period_ref_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("payroll_period_ref_id"));
        request
    }

    pub fn group_by_payroll_period_ref_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("payroll_period_ref_id")
            .aggregate_with_function("payroll_period_ref_id", alias, function)
    }

    pub fn group_by_payroll_period_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("payroll_period_ref_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "payroll_period_ref",
            "payroll_period_ref_id",
            request,
        ));
        self
    }

    pub fn group_by_payroll_period_ref_with_details(self) -> Self {
        self.group_by_payroll_period_ref_with_details_from(crate::Q::payroll_periods().unlimited())
    }

    pub fn group_by_payroll_period_ref_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_payroll_period_ref_with(request)
    }


    pub fn roll_up_to_payroll_period_ref(self) -> Self {
        self.roll_up_to_payroll_period_ref_with(crate::Q::payroll_periods().unlimited())
    }

    pub fn roll_up_to_payroll_period_ref_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_payroll_period_ref_matching(selection.clone())
            .group_by_payroll_period_ref_with(selection)
    }

    pub fn count_payroll_period_ref(self) -> Self {
        self.count_payroll_period_ref_as("payroll_period_ref_count")
    }

    pub fn count_payroll_period_ref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("payroll_period_ref_id", alias)
    }

    pub fn unselect_payroll_period_ref(mut self) -> Self {
        self.query.projection.retain(|field| field != "payroll_period_ref_id");
        self.query.relations.retain(|relation| relation.name != "payroll_period_ref");
        self
    }
    pub fn select_payroll_period_ref(mut self) -> Self {
        self.query = self.query.relation("payroll_period_ref");
        self
    }

    pub fn select_payroll_period_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payroll_period_ref", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payroll_period_ref", selection));
        self
}

    pub fn facet_by_payroll_period_ref_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_payroll_period_ref_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_payroll_period_ref_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "payroll_period_ref",
            request,
            include_all_facets,
        ));
        self
    }
    pub fn have_payslips(self) -> Self {
        self.with_payslip_list_matching(SelectQuery::new("Payslip"))
    }

    pub fn have_no_payslips(self) -> Self {
        self.without_payslip_list_matching(SelectQuery::new("Payslip"))
    }

    pub fn with_payslip_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Payslip as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payslip_list", selection));
        self
    }

    pub fn without_payslip_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Payslip as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payslip_list", selection));
        self
    }

    pub fn select_payslip_list(mut self) -> Self {
        self.query = self.query.relation("payslip_list");
        self
    }

    pub fn select_payslip_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payslip_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payslip_list", selection));
        self
}

    pub fn have_deductions(self) -> Self {
        self.with_deduction_list_matching(SelectQuery::new("Deduction"))
    }

    pub fn have_no_deductions(self) -> Self {
        self.without_deduction_list_matching(SelectQuery::new("Deduction"))
    }

    pub fn with_deduction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Deduction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("deduction_list", selection));
        self
    }

    pub fn without_deduction_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Deduction as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("deduction_list", selection));
        self
    }

    pub fn select_deduction_list(mut self) -> Self {
        self.query = self.query.relation("deduction_list");
        self
    }

    pub fn select_deduction_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("deduction_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("deduction_list", selection));
        self
}

    pub fn have_tax_withholdings(self) -> Self {
        self.with_tax_withholding_list_matching(SelectQuery::new("TaxWithholding"))
    }

    pub fn have_no_tax_withholdings(self) -> Self {
        self.without_tax_withholding_list_matching(SelectQuery::new("TaxWithholding"))
    }

    pub fn with_tax_withholding_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TaxWithholding as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("tax_withholding_list", selection));
        self
    }

    pub fn without_tax_withholding_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TaxWithholding as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("tax_withholding_list", selection));
        self
    }

    pub fn select_tax_withholding_list(mut self) -> Self {
        self.query = self.query.relation("tax_withholding_list");
        self
    }

    pub fn select_tax_withholding_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tax_withholding_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tax_withholding_list", selection));
        self
}

    pub fn have_payroll_adjustments(self) -> Self {
        self.with_payroll_adjustment_list_matching(SelectQuery::new("PayrollAdjustment"))
    }

    pub fn have_no_payroll_adjustments(self) -> Self {
        self.without_payroll_adjustment_list_matching(SelectQuery::new("PayrollAdjustment"))
    }

    pub fn with_payroll_adjustment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PayrollAdjustment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_adjustment_list", selection));
        self
    }

    pub fn without_payroll_adjustment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PayrollAdjustment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "payroll_calculation_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_adjustment_list", selection));
        self
    }

    pub fn select_payroll_adjustment_list(mut self) -> Self {
        self.query = self.query.relation("payroll_adjustment_list");
        self
    }

    pub fn select_payroll_adjustment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payroll_adjustment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payroll_adjustment_list", selection));
        self
}
    pub fn count_payslips(self) -> Self {
        self.count_payslips_as("count_payslips")
    }

    pub fn count_payslips_as(self, alias: impl Into<String>) -> Self {
        self.count_payslips_with(alias, crate::Q::payslips().unlimited())
    }

    pub fn count_payslips_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payslip_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_payslips(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payslips_as("refinements", request)
    }

    pub fn stats_from_payslips_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payslip_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_payslips_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payslips(request)
    }




    pub fn count_deductions(self) -> Self {
        self.count_deductions_as("count_deductions")
    }

    pub fn count_deductions_as(self, alias: impl Into<String>) -> Self {
        self.count_deductions_with(alias, crate::Q::deductions().unlimited())
    }

    pub fn count_deductions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "deduction_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_deductions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_deductions_as("refinements", request)
    }

    pub fn stats_from_deductions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "deduction_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_deductions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_deductions(request)
    }




    pub fn count_tax_withholdings(self) -> Self {
        self.count_tax_withholdings_as("count_tax_withholdings")
    }

    pub fn count_tax_withholdings_as(self, alias: impl Into<String>) -> Self {
        self.count_tax_withholdings_with(alias, crate::Q::tax_withholdings().unlimited())
    }

    pub fn count_tax_withholdings_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tax_withholding_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_tax_withholdings(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_withholdings_as("refinements", request)
    }

    pub fn stats_from_tax_withholdings_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tax_withholding_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_tax_withholdings_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_withholdings(request)
    }




    pub fn count_payroll_adjustments(self) -> Self {
        self.count_payroll_adjustments_as("count_payroll_adjustments")
    }

    pub fn count_payroll_adjustments_as(self, alias: impl Into<String>) -> Self {
        self.count_payroll_adjustments_with(alias, crate::Q::payroll_adjustments().unlimited())
    }

    pub fn count_payroll_adjustments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payroll_adjustment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_payroll_adjustments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_adjustments_as("refinements", request)
    }

    pub fn stats_from_payroll_adjustments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payroll_adjustment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_payroll_adjustments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_adjustments(request)
    }



}

impl<R> Default for PayrollCalculationRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< PayrollCalculationRequest<R> > for SelectQuery {
    fn from(request: PayrollCalculationRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< PayrollCalculationRequest<R> > for QuerySelection {
    fn from(request: PayrollCalculationRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::PayrollCalculation> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<PayrollCalculationRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::PayrollCalculation
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::PayrollCalculation::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> PayrollCalculationRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::PayrollCalculationRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
