use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::EmployeeRecord {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::EmployeeRecord {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/employee_record
#[derive(Debug)]
pub struct EmployeeRecordRequest<R = crate::EmployeeRecord> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for EmployeeRecordRequest<R> {
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

impl<R> EmployeeRecordRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("EmployeeRecord")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> EmployeeRecordRequest<T> {
        EmployeeRecordRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .employee_record_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_record_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_record_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for EmployeeRecord is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_record_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_record_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
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
            "employee_number" => Some("employee_number"),
            "first_name" => Some("first_name"),
            "last_name" => Some("last_name"),
            "hire_date" => Some("hire_date"),
            "department" => Some("department"),
            "employment_status" => Some("employment_status"),
            "version" => Some("version"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "timesheet_entry_list" => {
                self.with_timesheet_entry_list_matching(
                    crate::Q::timesheet_entries_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tax_withholding_list" => {
                self.with_tax_withholding_list_matching(
                    crate::Q::tax_withholdings_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "leave_request_list" => {
                self.with_leave_request_list_matching(
                    crate::Q::leave_requests_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "training_record_list" => {
                self.with_training_record_list_matching(
                    crate::Q::training_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "performance_review_list_as_employee" => {
                self.with_performance_review_list_as_employee_matching(
                    crate::Q::performance_reviews_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "performance_review_list_as_reviewer" => {
                self.with_performance_review_list_as_reviewer_matching(
                    crate::Q::performance_reviews_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "compensation_adjustment_list_as_employee" => {
                self.with_compensation_adjustment_list_as_employee_matching(
                    crate::Q::compensation_adjustments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "compensation_adjustment_list_as_approved_by" => {
                self.with_compensation_adjustment_list_as_approved_by_matching(
                    crate::Q::compensation_adjustments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "onboarding_checklist_list" => {
                self.with_onboarding_checklist_list_matching(
                    crate::Q::onboarding_checklists_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "offboarding_process_list" => {
                self.with_offboarding_process_list_matching(
                    crate::Q::offboarding_processes_minimal()
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
        self.query = self.query.project("employee_number");
        self.query = self.query.project("first_name");
        self.query = self.query.project("last_name");
        self.query = self.query.project("hire_date");
        self.query = self.query.project("department");
        self.query = self.query.project("employment_status");
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
        let mut request = self.select_all();
        request = request.select_timesheet_entry_list();
        request = request.select_tax_withholding_list();
        request = request.select_leave_request_list();
        request = request.select_training_record_list();
        request = request.select_performance_review_list_as_employee();
        request = request.select_performance_review_list_as_reviewer();
        request = request.select_compensation_adjustment_list_as_employee();
        request = request.select_compensation_adjustment_list_as_approved_by();
        request = request.select_onboarding_checklist_list();
        request = request.select_offboarding_process_list();
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


    pub fn select_employee_number(mut self) -> Self {
        self.query = self.query.project("employee_number");
        self
    }

    pub fn project_employee_number(self) -> Self {
        self.select_employee_number()
    }

    pub fn select_employee_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_employee_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_employee_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("employee_number", raw_sql_segment));
        self
    }

    pub fn group_by_employee_number(self) -> Self {
        self.group_by("employee_number")
    }

    pub fn group_by_employee_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("employee_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("employee_number"));
        request
    }

    pub fn group_by_employee_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("employee_number")
            .aggregate_with_function("employee_number", alias, function)
    }

    pub fn count_employee_number(self) -> Self {
        self.count_employee_number_as("employee_number_count")
    }

    pub fn count_employee_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("employee_number", alias)
    }

    pub fn sum_employee_number(self) -> Self {
        self.sum_employee_number_as("sum_employee_number")
    }

    pub fn sum_employee_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("employee_number", alias)
    }

    pub fn avg_employee_number(self) -> Self {
        self.avg_employee_number_as("avg_employee_number")
    }

    pub fn avg_employee_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("employee_number", alias)
    }

    pub fn min_employee_number(self) -> Self {
        self.min_employee_number_as("min_employee_number")
    }

    pub fn min_employee_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("employee_number", alias)
    }

    pub fn max_employee_number(self) -> Self {
        self.max_employee_number_as("max_employee_number")
    }

    pub fn max_employee_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("employee_number", alias)
    }

    pub fn unselect_employee_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "employee_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "employee_number");
        self
    }


    pub fn with_employee_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "employee_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_employee_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "employee_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_employee_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("employee_number", value));
        self
    }



    pub fn with_employee_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("employee_number", value));
        self
    }

    pub fn with_employee_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("employee_number", value));
        self
    }

    pub fn with_employee_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("employee_number", value));
        self
    }

    pub fn with_employee_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("employee_number", value));
        self
    }

    pub fn with_employee_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("employee_number", value));
        self
    }

    pub fn with_employee_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("employee_number", lower, upper));
        self
    }

    pub fn with_employee_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "employee_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_employee_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "employee_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_employee_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "employee_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_employee_number_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("employee_number", value));
        self
    }

    pub fn with_employee_number_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("employee_number", value));
        self
    }

    pub fn with_employee_number_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("employee_number", value));
        self
    }

    pub fn with_employee_number_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("employee_number", value));
        self
    }

    pub fn with_employee_number_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("employee_number", value));
        self
    }

    pub fn with_employee_number_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("employee_number", value));
        self
    }

    pub fn with_employee_number_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("employee_number", value));
        self
    }
    pub fn with_employee_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("employee_number", value));
        self
    }

    pub fn with_employee_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("employee_number", value));
        self
    }

    pub fn with_employee_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("employee_number"));
        self
    }



    pub fn with_employee_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("employee_number"));
        self
    }


    pub fn order_by_employee_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("employee_number");
        self
    }

    pub fn order_by_employee_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("employee_number");
        self
    }

    pub fn order_by_employee_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("employee_number");
        self
    }

    pub fn order_by_employee_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("employee_number");
        self
    }


    pub fn select_first_name(mut self) -> Self {
        self.query = self.query.project("first_name");
        self
    }

    pub fn project_first_name(self) -> Self {
        self.select_first_name()
    }

    pub fn select_first_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_first_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_first_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("first_name", raw_sql_segment));
        self
    }

    pub fn group_by_first_name(self) -> Self {
        self.group_by("first_name")
    }

    pub fn group_by_first_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("first_name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("first_name"));
        request
    }

    pub fn group_by_first_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("first_name")
            .aggregate_with_function("first_name", alias, function)
    }

    pub fn count_first_name(self) -> Self {
        self.count_first_name_as("first_name_count")
    }

    pub fn count_first_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("first_name", alias)
    }

    pub fn sum_first_name(self) -> Self {
        self.sum_first_name_as("sum_first_name")
    }

    pub fn sum_first_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("first_name", alias)
    }

    pub fn avg_first_name(self) -> Self {
        self.avg_first_name_as("avg_first_name")
    }

    pub fn avg_first_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("first_name", alias)
    }

    pub fn min_first_name(self) -> Self {
        self.min_first_name_as("min_first_name")
    }

    pub fn min_first_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("first_name", alias)
    }

    pub fn max_first_name(self) -> Self {
        self.max_first_name_as("max_first_name")
    }

    pub fn max_first_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("first_name", alias)
    }

    pub fn unselect_first_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "first_name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "first_name");
        self
    }


    pub fn with_first_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "first_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_first_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "first_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_first_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("first_name", value));
        self
    }



    pub fn with_first_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("first_name", value));
        self
    }

    pub fn with_first_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("first_name", value));
        self
    }

    pub fn with_first_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("first_name", value));
        self
    }

    pub fn with_first_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("first_name", value));
        self
    }

    pub fn with_first_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("first_name", value));
        self
    }

    pub fn with_first_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("first_name", lower, upper));
        self
    }

    pub fn with_first_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "first_name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_first_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "first_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_first_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "first_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_first_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("first_name", value));
        self
    }

    pub fn with_first_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("first_name", value));
        self
    }

    pub fn with_first_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("first_name", value));
        self
    }

    pub fn with_first_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("first_name", value));
        self
    }

    pub fn with_first_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("first_name", value));
        self
    }

    pub fn with_first_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("first_name", value));
        self
    }

    pub fn with_first_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("first_name", value));
        self
    }
    pub fn with_first_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("first_name", value));
        self
    }

    pub fn with_first_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("first_name", value));
        self
    }

    pub fn with_first_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("first_name"));
        self
    }



    pub fn with_first_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("first_name"));
        self
    }


    pub fn order_by_first_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("first_name");
        self
    }

    pub fn order_by_first_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("first_name");
        self
    }

    pub fn order_by_first_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("first_name");
        self
    }

    pub fn order_by_first_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("first_name");
        self
    }


    pub fn select_last_name(mut self) -> Self {
        self.query = self.query.project("last_name");
        self
    }

    pub fn project_last_name(self) -> Self {
        self.select_last_name()
    }

    pub fn select_last_name_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_last_name_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_last_name_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("last_name", raw_sql_segment));
        self
    }

    pub fn group_by_last_name(self) -> Self {
        self.group_by("last_name")
    }

    pub fn group_by_last_name_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("last_name");
        request.query = request
            .query
            .project_expr(alias, Expr::column("last_name"));
        request
    }

    pub fn group_by_last_name_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("last_name")
            .aggregate_with_function("last_name", alias, function)
    }

    pub fn count_last_name(self) -> Self {
        self.count_last_name_as("last_name_count")
    }

    pub fn count_last_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("last_name", alias)
    }

    pub fn sum_last_name(self) -> Self {
        self.sum_last_name_as("sum_last_name")
    }

    pub fn sum_last_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("last_name", alias)
    }

    pub fn avg_last_name(self) -> Self {
        self.avg_last_name_as("avg_last_name")
    }

    pub fn avg_last_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("last_name", alias)
    }

    pub fn min_last_name(self) -> Self {
        self.min_last_name_as("min_last_name")
    }

    pub fn min_last_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("last_name", alias)
    }

    pub fn max_last_name(self) -> Self {
        self.max_last_name_as("max_last_name")
    }

    pub fn max_last_name_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("last_name", alias)
    }

    pub fn unselect_last_name(mut self) -> Self {
        self.query.projection.retain(|field| field != "last_name");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "last_name");
        self
    }


    pub fn with_last_name(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "last_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_last_name_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "last_name",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_last_name_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("last_name", value));
        self
    }



    pub fn with_last_name_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("last_name", value));
        self
    }

    pub fn with_last_name_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("last_name", value));
        self
    }

    pub fn with_last_name_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("last_name", value));
        self
    }

    pub fn with_last_name_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("last_name", value));
        self
    }

    pub fn with_last_name_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("last_name", value));
        self
    }

    pub fn with_last_name_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("last_name", lower, upper));
        self
    }

    pub fn with_last_name_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "last_name",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_last_name_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "last_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_last_name_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "last_name",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_last_name_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("last_name", value));
        self
    }

    pub fn with_last_name_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("last_name", value));
        self
    }

    pub fn with_last_name_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("last_name", value));
        self
    }

    pub fn with_last_name_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("last_name", value));
        self
    }

    pub fn with_last_name_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("last_name", value));
        self
    }

    pub fn with_last_name_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("last_name", value));
        self
    }

    pub fn with_last_name_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("last_name", value));
        self
    }
    pub fn with_last_name_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("last_name", value));
        self
    }

    pub fn with_last_name_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("last_name", value));
        self
    }

    pub fn with_last_name_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("last_name"));
        self
    }



    pub fn with_last_name_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("last_name"));
        self
    }


    pub fn order_by_last_name_asc(mut self) -> Self {
        self.query = self.query.order_asc("last_name");
        self
    }

    pub fn order_by_last_name_desc(mut self) -> Self {
        self.query = self.query.order_desc("last_name");
        self
    }

    pub fn order_by_last_name_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("last_name");
        self
    }

    pub fn order_by_last_name_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("last_name");
        self
    }


    pub fn select_hire_date(mut self) -> Self {
        self.query = self.query.project("hire_date");
        self
    }

    pub fn project_hire_date(self) -> Self {
        self.select_hire_date()
    }

    pub fn select_hire_date_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_hire_date_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_hire_date_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("hire_date", raw_sql_segment));
        self
    }

    pub fn group_by_hire_date(self) -> Self {
        self.group_by("hire_date")
    }

    pub fn group_by_hire_date_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("hire_date");
        request.query = request
            .query
            .project_expr(alias, Expr::column("hire_date"));
        request
    }

    pub fn group_by_hire_date_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("hire_date")
            .aggregate_with_function("hire_date", alias, function)
    }

    pub fn count_hire_date(self) -> Self {
        self.count_hire_date_as("hire_date_count")
    }

    pub fn count_hire_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("hire_date", alias)
    }

    pub fn sum_hire_date(self) -> Self {
        self.sum_hire_date_as("sum_hire_date")
    }

    pub fn sum_hire_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("hire_date", alias)
    }

    pub fn avg_hire_date(self) -> Self {
        self.avg_hire_date_as("avg_hire_date")
    }

    pub fn avg_hire_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("hire_date", alias)
    }

    pub fn min_hire_date(self) -> Self {
        self.min_hire_date_as("min_hire_date")
    }

    pub fn min_hire_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("hire_date", alias)
    }

    pub fn max_hire_date(self) -> Self {
        self.max_hire_date_as("max_hire_date")
    }

    pub fn max_hire_date_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("hire_date", alias)
    }

    pub fn unselect_hire_date(mut self) -> Self {
        self.query.projection.retain(|field| field != "hire_date");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "hire_date");
        self
    }


    pub fn with_hire_date(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "hire_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_hire_date_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "hire_date",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_hire_date_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("hire_date", value));
        self
    }



    pub fn with_hire_date_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("hire_date", value));
        self
    }

    pub fn with_hire_date_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("hire_date", value));
        self
    }

    pub fn with_hire_date_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("hire_date", value));
        self
    }

    pub fn with_hire_date_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("hire_date", value));
        self
    }

    pub fn with_hire_date_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("hire_date", value));
        self
    }

    pub fn with_hire_date_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("hire_date", lower, upper));
        self
    }

    pub fn with_hire_date_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "hire_date",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_hire_date_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "hire_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_hire_date_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "hire_date",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_hire_date_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("hire_date", value));
        self
    }

    pub fn with_hire_date_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("hire_date", value));
        self
    }

    pub fn with_hire_date_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("hire_date"));
        self
    }



    pub fn with_hire_date_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("hire_date"));
        self
    }


    pub fn order_by_hire_date_asc(mut self) -> Self {
        self.query = self.query.order_asc("hire_date");
        self
    }

    pub fn order_by_hire_date_desc(mut self) -> Self {
        self.query = self.query.order_desc("hire_date");
        self
    }

    pub fn order_by_hire_date_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("hire_date");
        self
    }

    pub fn order_by_hire_date_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("hire_date");
        self
    }


    pub fn select_department(mut self) -> Self {
        self.query = self.query.project("department");
        self
    }

    pub fn project_department(self) -> Self {
        self.select_department()
    }

    pub fn select_department_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_department_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_department_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("department", raw_sql_segment));
        self
    }

    pub fn group_by_department(self) -> Self {
        self.group_by("department")
    }

    pub fn group_by_department_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("department");
        request.query = request
            .query
            .project_expr(alias, Expr::column("department"));
        request
    }

    pub fn group_by_department_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("department")
            .aggregate_with_function("department", alias, function)
    }

    pub fn count_department(self) -> Self {
        self.count_department_as("department_count")
    }

    pub fn count_department_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("department", alias)
    }

    pub fn sum_department(self) -> Self {
        self.sum_department_as("sum_department")
    }

    pub fn sum_department_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("department", alias)
    }

    pub fn avg_department(self) -> Self {
        self.avg_department_as("avg_department")
    }

    pub fn avg_department_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("department", alias)
    }

    pub fn min_department(self) -> Self {
        self.min_department_as("min_department")
    }

    pub fn min_department_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("department", alias)
    }

    pub fn max_department(self) -> Self {
        self.max_department_as("max_department")
    }

    pub fn max_department_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("department", alias)
    }

    pub fn unselect_department(mut self) -> Self {
        self.query.projection.retain(|field| field != "department");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "department");
        self
    }


    pub fn with_department(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "department",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_department_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "department",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_department_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("department", value));
        self
    }



    pub fn with_department_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("department", value));
        self
    }

    pub fn with_department_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("department", value));
        self
    }

    pub fn with_department_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("department", value));
        self
    }

    pub fn with_department_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("department", value));
        self
    }

    pub fn with_department_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("department", value));
        self
    }

    pub fn with_department_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("department", lower, upper));
        self
    }

    pub fn with_department_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "department",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_department_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "department",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_department_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "department",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_department_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("department", value));
        self
    }

    pub fn with_department_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("department", value));
        self
    }

    pub fn with_department_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("department", value));
        self
    }

    pub fn with_department_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("department", value));
        self
    }

    pub fn with_department_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("department", value));
        self
    }

    pub fn with_department_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("department", value));
        self
    }

    pub fn with_department_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("department", value));
        self
    }
    pub fn with_department_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("department", value));
        self
    }

    pub fn with_department_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("department", value));
        self
    }

    pub fn with_department_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("department"));
        self
    }



    pub fn with_department_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("department"));
        self
    }


    pub fn order_by_department_asc(mut self) -> Self {
        self.query = self.query.order_asc("department");
        self
    }

    pub fn order_by_department_desc(mut self) -> Self {
        self.query = self.query.order_desc("department");
        self
    }

    pub fn order_by_department_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("department");
        self
    }

    pub fn order_by_department_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("department");
        self
    }


    pub fn select_employment_status(mut self) -> Self {
        self.query = self.query.project("employment_status");
        self
    }

    pub fn project_employment_status(self) -> Self {
        self.select_employment_status()
    }

    pub fn select_employment_status_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_employment_status_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_employment_status_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("employment_status", raw_sql_segment));
        self
    }

    pub fn group_by_employment_status(self) -> Self {
        self.group_by("employment_status")
    }

    pub fn group_by_employment_status_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("employment_status");
        request.query = request
            .query
            .project_expr(alias, Expr::column("employment_status"));
        request
    }

    pub fn group_by_employment_status_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("employment_status")
            .aggregate_with_function("employment_status", alias, function)
    }

    pub fn count_employment_status(self) -> Self {
        self.count_employment_status_as("employment_status_count")
    }

    pub fn count_employment_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("employment_status", alias)
    }

    pub fn sum_employment_status(self) -> Self {
        self.sum_employment_status_as("sum_employment_status")
    }

    pub fn sum_employment_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("employment_status", alias)
    }

    pub fn avg_employment_status(self) -> Self {
        self.avg_employment_status_as("avg_employment_status")
    }

    pub fn avg_employment_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("employment_status", alias)
    }

    pub fn min_employment_status(self) -> Self {
        self.min_employment_status_as("min_employment_status")
    }

    pub fn min_employment_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("employment_status", alias)
    }

    pub fn max_employment_status(self) -> Self {
        self.max_employment_status_as("max_employment_status")
    }

    pub fn max_employment_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("employment_status", alias)
    }

    pub fn unselect_employment_status(mut self) -> Self {
        self.query.projection.retain(|field| field != "employment_status");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "employment_status");
        self
    }


    pub fn with_employment_status(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "employment_status",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_employment_status_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "employment_status",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_employment_status_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("employment_status", value));
        self
    }



    pub fn with_employment_status_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("employment_status", value));
        self
    }

    pub fn with_employment_status_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("employment_status", value));
        self
    }

    pub fn with_employment_status_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("employment_status", value));
        self
    }

    pub fn with_employment_status_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("employment_status", value));
        self
    }

    pub fn with_employment_status_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("employment_status", value));
        self
    }

    pub fn with_employment_status_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("employment_status", lower, upper));
        self
    }

    pub fn with_employment_status_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "employment_status",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_employment_status_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "employment_status",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_employment_status_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "employment_status",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_employment_status_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("employment_status", value));
        self
    }

    pub fn with_employment_status_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("employment_status", value));
        self
    }

    pub fn with_employment_status_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("employment_status", value));
        self
    }

    pub fn with_employment_status_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("employment_status", value));
        self
    }

    pub fn with_employment_status_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("employment_status", value));
        self
    }

    pub fn with_employment_status_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("employment_status", value));
        self
    }

    pub fn with_employment_status_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("employment_status", value));
        self
    }
    pub fn with_employment_status_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("employment_status", value));
        self
    }

    pub fn with_employment_status_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("employment_status", value));
        self
    }

    pub fn with_employment_status_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("employment_status"));
        self
    }



    pub fn with_employment_status_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("employment_status"));
        self
    }


    pub fn order_by_employment_status_asc(mut self) -> Self {
        self.query = self.query.order_asc("employment_status");
        self
    }

    pub fn order_by_employment_status_desc(mut self) -> Self {
        self.query = self.query.order_desc("employment_status");
        self
    }

    pub fn order_by_employment_status_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("employment_status");
        self
    }

    pub fn order_by_employment_status_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("employment_status");
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
    pub fn employee_number_is_string(self) -> Self {
        self.with_employee_number_is("string()")
    }

    pub fn with_employee_number_is_string(self) -> Self {
        self.with_employee_number_is("string()")
    }



    pub fn with_employee_number_is_not_string(self) -> Self {
        self.with_employee_number_is_not("string()")
    }



    pub fn first_name_is_string(self) -> Self {
        self.with_first_name_is("string()")
    }

    pub fn with_first_name_is_string(self) -> Self {
        self.with_first_name_is("string()")
    }



    pub fn with_first_name_is_not_string(self) -> Self {
        self.with_first_name_is_not("string()")
    }



    pub fn last_name_is_string(self) -> Self {
        self.with_last_name_is("string()")
    }

    pub fn with_last_name_is_string(self) -> Self {
        self.with_last_name_is("string()")
    }



    pub fn with_last_name_is_not_string(self) -> Self {
        self.with_last_name_is_not("string()")
    }



    pub fn hire_date_is_date(self) -> Self {
        self.with_hire_date_is("date()")
    }

    pub fn with_hire_date_is_date(self) -> Self {
        self.with_hire_date_is("date()")
    }



    pub fn with_hire_date_is_not_date(self) -> Self {
        self.with_hire_date_is_not("date()")
    }



    pub fn department_is_string(self) -> Self {
        self.with_department_is("string()")
    }

    pub fn with_department_is_string(self) -> Self {
        self.with_department_is("string()")
    }



    pub fn with_department_is_not_string(self) -> Self {
        self.with_department_is_not("string()")
    }



    pub fn employment_status_is_string(self) -> Self {
        self.with_employment_status_is("string()")
    }

    pub fn with_employment_status_is_string(self) -> Self {
        self.with_employment_status_is("string()")
    }



    pub fn with_employment_status_is_not_string(self) -> Self {
        self.with_employment_status_is_not("string()")
    }




    pub fn have_timesheet_entries(self) -> Self {
        self.with_timesheet_entry_list_matching(SelectQuery::new("TimesheetEntry"))
    }

    pub fn have_no_timesheet_entries(self) -> Self {
        self.without_timesheet_entry_list_matching(SelectQuery::new("TimesheetEntry"))
    }

    pub fn with_timesheet_entry_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TimesheetEntry as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("timesheet_entry_list", selection));
        self
    }

    pub fn without_timesheet_entry_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TimesheetEntry as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("timesheet_entry_list", selection));
        self
    }

    pub fn select_timesheet_entry_list(mut self) -> Self {
        self.query = self.query.relation("timesheet_entry_list");
        self
    }

    pub fn select_timesheet_entry_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("timesheet_entry_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("timesheet_entry_list", selection));
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
            "employee_id",
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
            "employee_id",
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

    pub fn have_leave_requests(self) -> Self {
        self.with_leave_request_list_matching(SelectQuery::new("LeaveRequest"))
    }

    pub fn have_no_leave_requests(self) -> Self {
        self.without_leave_request_list_matching(SelectQuery::new("LeaveRequest"))
    }

    pub fn with_leave_request_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::LeaveRequest as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("leave_request_list", selection));
        self
    }

    pub fn without_leave_request_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::LeaveRequest as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("leave_request_list", selection));
        self
    }

    pub fn select_leave_request_list(mut self) -> Self {
        self.query = self.query.relation("leave_request_list");
        self
    }

    pub fn select_leave_request_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("leave_request_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("leave_request_list", selection));
        self
}

    pub fn have_training_records(self) -> Self {
        self.with_training_record_list_matching(SelectQuery::new("TrainingRecord"))
    }

    pub fn have_no_training_records(self) -> Self {
        self.without_training_record_list_matching(SelectQuery::new("TrainingRecord"))
    }

    pub fn with_training_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TrainingRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("training_record_list", selection));
        self
    }

    pub fn without_training_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TrainingRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("training_record_list", selection));
        self
    }

    pub fn select_training_record_list(mut self) -> Self {
        self.query = self.query.relation("training_record_list");
        self
    }

    pub fn select_training_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("training_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("training_record_list", selection));
        self
}

    pub fn have_performance_reviews_as_employee(self) -> Self {
        self.with_performance_review_list_as_employee_matching(SelectQuery::new("PerformanceReview"))
    }

    pub fn have_no_performance_reviews_as_employee(self) -> Self {
        self.without_performance_review_list_as_employee_matching(SelectQuery::new("PerformanceReview"))
    }

    pub fn with_performance_review_list_as_employee_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PerformanceReview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("performance_review_list_as_employee", selection));
        self
    }

    pub fn without_performance_review_list_as_employee_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PerformanceReview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("performance_review_list_as_employee", selection));
        self
    }

    pub fn select_performance_review_list_as_employee(mut self) -> Self {
        self.query = self.query.relation("performance_review_list_as_employee");
        self
    }

    pub fn select_performance_review_list_as_employee_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("performance_review_list_as_employee", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("performance_review_list_as_employee", selection));
        self
}

    pub fn have_performance_reviews_as_reviewer(self) -> Self {
        self.with_performance_review_list_as_reviewer_matching(SelectQuery::new("PerformanceReview"))
    }

    pub fn have_no_performance_reviews_as_reviewer(self) -> Self {
        self.without_performance_review_list_as_reviewer_matching(SelectQuery::new("PerformanceReview"))
    }

    pub fn with_performance_review_list_as_reviewer_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PerformanceReview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "reviewer_id",
        ));
        self.relation_filters.push(RelationFilter::new("performance_review_list_as_reviewer", selection));
        self
    }

    pub fn without_performance_review_list_as_reviewer_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PerformanceReview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "reviewer_id",
        ));
        self.relation_filters.push(RelationFilter::new("performance_review_list_as_reviewer", selection));
        self
    }

    pub fn select_performance_review_list_as_reviewer(mut self) -> Self {
        self.query = self.query.relation("performance_review_list_as_reviewer");
        self
    }

    pub fn select_performance_review_list_as_reviewer_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("performance_review_list_as_reviewer", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("performance_review_list_as_reviewer", selection));
        self
}

    pub fn have_compensation_adjustments_as_employee(self) -> Self {
        self.with_compensation_adjustment_list_as_employee_matching(SelectQuery::new("CompensationAdjustment"))
    }

    pub fn have_no_compensation_adjustments_as_employee(self) -> Self {
        self.without_compensation_adjustment_list_as_employee_matching(SelectQuery::new("CompensationAdjustment"))
    }

    pub fn with_compensation_adjustment_list_as_employee_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CompensationAdjustment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("compensation_adjustment_list_as_employee", selection));
        self
    }

    pub fn without_compensation_adjustment_list_as_employee_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CompensationAdjustment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("compensation_adjustment_list_as_employee", selection));
        self
    }

    pub fn select_compensation_adjustment_list_as_employee(mut self) -> Self {
        self.query = self.query.relation("compensation_adjustment_list_as_employee");
        self
    }

    pub fn select_compensation_adjustment_list_as_employee_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("compensation_adjustment_list_as_employee", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("compensation_adjustment_list_as_employee", selection));
        self
}

    pub fn have_compensation_adjustments_as_approved_by(self) -> Self {
        self.with_compensation_adjustment_list_as_approved_by_matching(SelectQuery::new("CompensationAdjustment"))
    }

    pub fn have_no_compensation_adjustments_as_approved_by(self) -> Self {
        self.without_compensation_adjustment_list_as_approved_by_matching(SelectQuery::new("CompensationAdjustment"))
    }

    pub fn with_compensation_adjustment_list_as_approved_by_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CompensationAdjustment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "approved_by_id",
        ));
        self.relation_filters.push(RelationFilter::new("compensation_adjustment_list_as_approved_by", selection));
        self
    }

    pub fn without_compensation_adjustment_list_as_approved_by_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CompensationAdjustment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "approved_by_id",
        ));
        self.relation_filters.push(RelationFilter::new("compensation_adjustment_list_as_approved_by", selection));
        self
    }

    pub fn select_compensation_adjustment_list_as_approved_by(mut self) -> Self {
        self.query = self.query.relation("compensation_adjustment_list_as_approved_by");
        self
    }

    pub fn select_compensation_adjustment_list_as_approved_by_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("compensation_adjustment_list_as_approved_by", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("compensation_adjustment_list_as_approved_by", selection));
        self
}

    pub fn have_onboarding_checklists(self) -> Self {
        self.with_onboarding_checklist_list_matching(SelectQuery::new("OnboardingChecklist"))
    }

    pub fn have_no_onboarding_checklists(self) -> Self {
        self.without_onboarding_checklist_list_matching(SelectQuery::new("OnboardingChecklist"))
    }

    pub fn with_onboarding_checklist_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::OnboardingChecklist as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("onboarding_checklist_list", selection));
        self
    }

    pub fn without_onboarding_checklist_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::OnboardingChecklist as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("onboarding_checklist_list", selection));
        self
    }

    pub fn select_onboarding_checklist_list(mut self) -> Self {
        self.query = self.query.relation("onboarding_checklist_list");
        self
    }

    pub fn select_onboarding_checklist_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("onboarding_checklist_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("onboarding_checklist_list", selection));
        self
}

    pub fn have_offboarding_processes(self) -> Self {
        self.with_offboarding_process_list_matching(SelectQuery::new("OffboardingProcess"))
    }

    pub fn have_no_offboarding_processes(self) -> Self {
        self.without_offboarding_process_list_matching(SelectQuery::new("OffboardingProcess"))
    }

    pub fn with_offboarding_process_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::OffboardingProcess as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("offboarding_process_list", selection));
        self
    }

    pub fn without_offboarding_process_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::OffboardingProcess as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("offboarding_process_list", selection));
        self
    }

    pub fn select_offboarding_process_list(mut self) -> Self {
        self.query = self.query.relation("offboarding_process_list");
        self
    }

    pub fn select_offboarding_process_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("offboarding_process_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("offboarding_process_list", selection));
        self
}
    pub fn count_timesheet_entries(self) -> Self {
        self.count_timesheet_entries_as("count_timesheet_entries")
    }

    pub fn count_timesheet_entries_as(self, alias: impl Into<String>) -> Self {
        self.count_timesheet_entries_with(alias, crate::Q::timesheet_entries().unlimited())
    }

    pub fn count_timesheet_entries_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "timesheet_entry_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_timesheet_entries(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_timesheet_entries_as("refinements", request)
    }

    pub fn stats_from_timesheet_entries_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "timesheet_entry_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_timesheet_entries_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_timesheet_entries(request)
    }


    pub fn min_work_date_of_timesheet_entries(self) -> Self {
        self.min_work_date_of_timesheet_entries_as("min_work_date_of_timesheet_entries", crate::Q::timesheet_entries().unlimited())
    }

    pub fn min_work_date_of_timesheet_entries_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_timesheet_entries_as(alias, request.into().into_query().min("work_date", "min_work_date"))
    }
    pub fn max_work_date_of_timesheet_entries(self) -> Self {
        self.max_work_date_of_timesheet_entries_as("max_work_date_of_timesheet_entries", crate::Q::timesheet_entries().unlimited())
    }

    pub fn max_work_date_of_timesheet_entries_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_timesheet_entries_as(alias, request.into().into_query().max("work_date", "max_work_date"))
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


    pub fn min_effective_date_of_tax_withholdings(self) -> Self {
        self.min_effective_date_of_tax_withholdings_as("min_effective_date_of_tax_withholdings", crate::Q::tax_withholdings().unlimited())
    }

    pub fn min_effective_date_of_tax_withholdings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_withholdings_as(alias, request.into().into_query().min("effective_date", "min_effective_date"))
    }
    pub fn max_effective_date_of_tax_withholdings(self) -> Self {
        self.max_effective_date_of_tax_withholdings_as("max_effective_date_of_tax_withholdings", crate::Q::tax_withholdings().unlimited())
    }

    pub fn max_effective_date_of_tax_withholdings_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_withholdings_as(alias, request.into().into_query().max("effective_date", "max_effective_date"))
    }

    pub fn count_leave_requests(self) -> Self {
        self.count_leave_requests_as("count_leave_requests")
    }

    pub fn count_leave_requests_as(self, alias: impl Into<String>) -> Self {
        self.count_leave_requests_with(alias, crate::Q::leave_requests().unlimited())
    }

    pub fn count_leave_requests_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "leave_request_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_leave_requests(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as("refinements", request)
    }

    pub fn stats_from_leave_requests_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "leave_request_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_leave_requests_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests(request)
    }


    pub fn min_start_date_of_leave_requests(self) -> Self {
        self.min_start_date_of_leave_requests_as("min_start_date_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn min_start_date_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().min("start_date", "min_start_date"))
    }
    pub fn max_start_date_of_leave_requests(self) -> Self {
        self.max_start_date_of_leave_requests_as("max_start_date_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn max_start_date_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().max("start_date", "max_start_date"))
    }
    pub fn min_end_date_of_leave_requests(self) -> Self {
        self.min_end_date_of_leave_requests_as("min_end_date_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn min_end_date_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().min("end_date", "min_end_date"))
    }
    pub fn max_end_date_of_leave_requests(self) -> Self {
        self.max_end_date_of_leave_requests_as("max_end_date_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn max_end_date_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().max("end_date", "max_end_date"))
    }

    pub fn count_training_records(self) -> Self {
        self.count_training_records_as("count_training_records")
    }

    pub fn count_training_records_as(self, alias: impl Into<String>) -> Self {
        self.count_training_records_with(alias, crate::Q::training_records().unlimited())
    }

    pub fn count_training_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "training_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_training_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records_as("refinements", request)
    }

    pub fn stats_from_training_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "training_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_training_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records(request)
    }


    pub fn min_completion_date_of_training_records(self) -> Self {
        self.min_completion_date_of_training_records_as("min_completion_date_of_training_records", crate::Q::training_records().unlimited())
    }

    pub fn min_completion_date_of_training_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records_as(alias, request.into().into_query().min("completion_date", "min_completion_date"))
    }
    pub fn max_completion_date_of_training_records(self) -> Self {
        self.max_completion_date_of_training_records_as("max_completion_date_of_training_records", crate::Q::training_records().unlimited())
    }

    pub fn max_completion_date_of_training_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records_as(alias, request.into().into_query().max("completion_date", "max_completion_date"))
    }

    pub fn count_performance_reviews_as_employee(self) -> Self {
        self.count_performance_reviews_as_employee_as("count_performance_reviews_as_employee")
    }

    pub fn count_performance_reviews_as_employee_as(self, alias: impl Into<String>) -> Self {
        self.count_performance_reviews_as_employee_with(alias, crate::Q::performance_reviews().unlimited())
    }

    pub fn count_performance_reviews_as_employee_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "performance_review_list_as_employee",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_performance_reviews_as_employee(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_employee_as("refinements", request)
    }

    pub fn stats_from_performance_reviews_as_employee_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "performance_review_list_as_employee",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_performance_reviews_as_employee_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_employee(request)
    }


    pub fn min_review_date_of_performance_reviews_as_employee(self) -> Self {
        self.min_review_date_of_performance_reviews_as_employee_as("min_review_date_of_performance_reviews_as_employee", crate::Q::performance_reviews().unlimited())
    }

    pub fn min_review_date_of_performance_reviews_as_employee_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_employee_as(alias, request.into().into_query().min("review_date", "min_review_date"))
    }
    pub fn max_review_date_of_performance_reviews_as_employee(self) -> Self {
        self.max_review_date_of_performance_reviews_as_employee_as("max_review_date_of_performance_reviews_as_employee", crate::Q::performance_reviews().unlimited())
    }

    pub fn max_review_date_of_performance_reviews_as_employee_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_employee_as(alias, request.into().into_query().max("review_date", "max_review_date"))
    }

    pub fn count_performance_reviews_as_reviewer(self) -> Self {
        self.count_performance_reviews_as_reviewer_as("count_performance_reviews_as_reviewer")
    }

    pub fn count_performance_reviews_as_reviewer_as(self, alias: impl Into<String>) -> Self {
        self.count_performance_reviews_as_reviewer_with(alias, crate::Q::performance_reviews().unlimited())
    }

    pub fn count_performance_reviews_as_reviewer_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "performance_review_list_as_reviewer",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_performance_reviews_as_reviewer(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_reviewer_as("refinements", request)
    }

    pub fn stats_from_performance_reviews_as_reviewer_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "performance_review_list_as_reviewer",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_performance_reviews_as_reviewer_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_reviewer(request)
    }


    pub fn min_review_date_of_performance_reviews_as_reviewer(self) -> Self {
        self.min_review_date_of_performance_reviews_as_reviewer_as("min_review_date_of_performance_reviews_as_reviewer", crate::Q::performance_reviews().unlimited())
    }

    pub fn min_review_date_of_performance_reviews_as_reviewer_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_reviewer_as(alias, request.into().into_query().min("review_date", "min_review_date"))
    }
    pub fn max_review_date_of_performance_reviews_as_reviewer(self) -> Self {
        self.max_review_date_of_performance_reviews_as_reviewer_as("max_review_date_of_performance_reviews_as_reviewer", crate::Q::performance_reviews().unlimited())
    }

    pub fn max_review_date_of_performance_reviews_as_reviewer_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as_reviewer_as(alias, request.into().into_query().max("review_date", "max_review_date"))
    }

    pub fn count_compensation_adjustments_as_employee(self) -> Self {
        self.count_compensation_adjustments_as_employee_as("count_compensation_adjustments_as_employee")
    }

    pub fn count_compensation_adjustments_as_employee_as(self, alias: impl Into<String>) -> Self {
        self.count_compensation_adjustments_as_employee_with(alias, crate::Q::compensation_adjustments().unlimited())
    }

    pub fn count_compensation_adjustments_as_employee_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compensation_adjustment_list_as_employee",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_compensation_adjustments_as_employee(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_employee_as("refinements", request)
    }

    pub fn stats_from_compensation_adjustments_as_employee_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compensation_adjustment_list_as_employee",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_compensation_adjustments_as_employee_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_employee(request)
    }


    pub fn min_effective_date_of_compensation_adjustments_as_employee(self) -> Self {
        self.min_effective_date_of_compensation_adjustments_as_employee_as("min_effective_date_of_compensation_adjustments_as_employee", crate::Q::compensation_adjustments().unlimited())
    }

    pub fn min_effective_date_of_compensation_adjustments_as_employee_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_employee_as(alias, request.into().into_query().min("effective_date", "min_effective_date"))
    }
    pub fn max_effective_date_of_compensation_adjustments_as_employee(self) -> Self {
        self.max_effective_date_of_compensation_adjustments_as_employee_as("max_effective_date_of_compensation_adjustments_as_employee", crate::Q::compensation_adjustments().unlimited())
    }

    pub fn max_effective_date_of_compensation_adjustments_as_employee_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_employee_as(alias, request.into().into_query().max("effective_date", "max_effective_date"))
    }

    pub fn count_compensation_adjustments_as_approved_by(self) -> Self {
        self.count_compensation_adjustments_as_approved_by_as("count_compensation_adjustments_as_approved_by")
    }

    pub fn count_compensation_adjustments_as_approved_by_as(self, alias: impl Into<String>) -> Self {
        self.count_compensation_adjustments_as_approved_by_with(alias, crate::Q::compensation_adjustments().unlimited())
    }

    pub fn count_compensation_adjustments_as_approved_by_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compensation_adjustment_list_as_approved_by",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_compensation_adjustments_as_approved_by(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_approved_by_as("refinements", request)
    }

    pub fn stats_from_compensation_adjustments_as_approved_by_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "compensation_adjustment_list_as_approved_by",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_compensation_adjustments_as_approved_by_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_approved_by(request)
    }


    pub fn min_effective_date_of_compensation_adjustments_as_approved_by(self) -> Self {
        self.min_effective_date_of_compensation_adjustments_as_approved_by_as("min_effective_date_of_compensation_adjustments_as_approved_by", crate::Q::compensation_adjustments().unlimited())
    }

    pub fn min_effective_date_of_compensation_adjustments_as_approved_by_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_approved_by_as(alias, request.into().into_query().min("effective_date", "min_effective_date"))
    }
    pub fn max_effective_date_of_compensation_adjustments_as_approved_by(self) -> Self {
        self.max_effective_date_of_compensation_adjustments_as_approved_by_as("max_effective_date_of_compensation_adjustments_as_approved_by", crate::Q::compensation_adjustments().unlimited())
    }

    pub fn max_effective_date_of_compensation_adjustments_as_approved_by_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_compensation_adjustments_as_approved_by_as(alias, request.into().into_query().max("effective_date", "max_effective_date"))
    }

    pub fn count_onboarding_checklists(self) -> Self {
        self.count_onboarding_checklists_as("count_onboarding_checklists")
    }

    pub fn count_onboarding_checklists_as(self, alias: impl Into<String>) -> Self {
        self.count_onboarding_checklists_with(alias, crate::Q::onboarding_checklists().unlimited())
    }

    pub fn count_onboarding_checklists_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "onboarding_checklist_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_onboarding_checklists(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists_as("refinements", request)
    }

    pub fn stats_from_onboarding_checklists_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "onboarding_checklist_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_onboarding_checklists_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists(request)
    }


    pub fn min_due_date_of_onboarding_checklists(self) -> Self {
        self.min_due_date_of_onboarding_checklists_as("min_due_date_of_onboarding_checklists", crate::Q::onboarding_checklists().unlimited())
    }

    pub fn min_due_date_of_onboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists_as(alias, request.into().into_query().min("due_date", "min_due_date"))
    }
    pub fn max_due_date_of_onboarding_checklists(self) -> Self {
        self.max_due_date_of_onboarding_checklists_as("max_due_date_of_onboarding_checklists", crate::Q::onboarding_checklists().unlimited())
    }

    pub fn max_due_date_of_onboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists_as(alias, request.into().into_query().max("due_date", "max_due_date"))
    }

    pub fn count_offboarding_processes(self) -> Self {
        self.count_offboarding_processes_as("count_offboarding_processes")
    }

    pub fn count_offboarding_processes_as(self, alias: impl Into<String>) -> Self {
        self.count_offboarding_processes_with(alias, crate::Q::offboarding_processes().unlimited())
    }

    pub fn count_offboarding_processes_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "offboarding_process_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_offboarding_processes(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_processes_as("refinements", request)
    }

    pub fn stats_from_offboarding_processes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "offboarding_process_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_offboarding_processes_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_processes(request)
    }


    pub fn min_due_date_of_offboarding_processes(self) -> Self {
        self.min_due_date_of_offboarding_processes_as("min_due_date_of_offboarding_processes", crate::Q::offboarding_processes().unlimited())
    }

    pub fn min_due_date_of_offboarding_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_processes_as(alias, request.into().into_query().min("due_date", "min_due_date"))
    }
    pub fn max_due_date_of_offboarding_processes(self) -> Self {
        self.max_due_date_of_offboarding_processes_as("max_due_date_of_offboarding_processes", crate::Q::offboarding_processes().unlimited())
    }

    pub fn max_due_date_of_offboarding_processes_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_processes_as(alias, request.into().into_query().max("due_date", "max_due_date"))
    }
}

impl<R> Default for EmployeeRecordRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< EmployeeRecordRequest<R> > for SelectQuery {
    fn from(request: EmployeeRecordRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< EmployeeRecordRequest<R> > for QuerySelection {
    fn from(request: EmployeeRecordRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::EmployeeRecord> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<EmployeeRecordRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::EmployeeRecord
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::EmployeeRecord::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> EmployeeRecordRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::EmployeeRecordRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
