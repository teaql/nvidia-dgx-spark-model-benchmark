use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Employee {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Employee {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/employee
#[derive(Debug)]
pub struct EmployeeRequest<R = crate::Employee> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for EmployeeRequest<R> {
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

impl<R> EmployeeRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Employee")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> EmployeeRequest<T> {
        EmployeeRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .employee_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Employee is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .employee_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::EmployeeRepository<'a>>>
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
            "version" => Some("version"),
            "department" | "department_id" => Some("department_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "department" => {
                self.with_department_matching(
                    crate::Q::departments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "operations_manager_override_list" => {
                self.with_operations_manager_override_list_matching(
                    crate::Q::operations_manager_overrides_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "job_assignment_list" => {
                self.with_job_assignment_list_matching(
                    crate::Q::job_assignments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "bonus_list" => {
                self.with_bonus_list_matching(
                    crate::Q::bonuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "leave_request_list" => {
                self.with_leave_request_list_matching(
                    crate::Q::leave_requests_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "employee_certification_list" => {
                self.with_employee_certification_list_matching(
                    crate::Q::employee_certifications_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tax_withholding_list" => {
                self.with_tax_withholding_list_matching(
                    crate::Q::tax_withholdings_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "direct_deposit_info_list" => {
                self.with_direct_deposit_info_list_matching(
                    crate::Q::direct_deposit_info_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "union_dues_list" => {
                self.with_union_dues_list_matching(
                    crate::Q::union_dueses_minimal()
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
        self.query = self.query.project("version");
        self.query = self.query.project("department_id");
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
        request = request.select_department();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_operations_manager_override_list();
        request = request.select_job_assignment_list();
        request = request.select_bonus_list();
        request = request.select_leave_request_list();
        request = request.select_employee_certification_list();
        request = request.select_tax_withholding_list();
        request = request.select_direct_deposit_info_list();
        request = request.select_union_dues_list();
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
    pub fn filter_by_department(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("department_id", value.entity_id_value()));
        self
    }

    pub fn with_department_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "department_id",
            <crate::Department as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("department", selection));
        self
    }


    pub fn without_department_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "department_id",
            <crate::Department as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("department", selection));
        self
    }


    pub fn have_department(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("department_id"));
        self
    }

    pub fn have_no_department(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("department_id"));
        self
    }


    pub fn group_by_department(self) -> Self {
        self.group_by("department_id")
    }

    pub fn group_by_department_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("department_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("department_id"));
        request
    }

    pub fn group_by_department_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("department_id")
            .aggregate_with_function("department_id", alias, function)
    }

    pub fn group_by_department_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("department_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "department",
            "department_id",
            request,
        ));
        self
    }

    pub fn group_by_department_with_details(self) -> Self {
        self.group_by_department_with_details_from(crate::Q::departments().unlimited())
    }

    pub fn group_by_department_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_department_with(request)
    }


    pub fn roll_up_to_department(self) -> Self {
        self.roll_up_to_department_with(crate::Q::departments().unlimited())
    }

    pub fn roll_up_to_department_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_department_matching(selection.clone())
            .group_by_department_with(selection)
    }

    pub fn count_department(self) -> Self {
        self.count_department_as("department_count")
    }

    pub fn count_department_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("department_id", alias)
    }

    pub fn unselect_department(mut self) -> Self {
        self.query.projection.retain(|field| field != "department_id");
        self.query.relations.retain(|relation| relation.name != "department");
        self
    }
    pub fn select_department(mut self) -> Self {
        self.query = self.query.relation("department");
        self
    }

    pub fn select_department_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("department", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("department", selection));
        self
}

    pub fn facet_by_department_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_department_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_department_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "department",
            request,
            include_all_facets,
        ));
        self
    }
    pub fn have_operations_manager_overrides(self) -> Self {
        self.with_operations_manager_override_list_matching(SelectQuery::new("OperationsManagerOverride"))
    }

    pub fn have_no_operations_manager_overrides(self) -> Self {
        self.without_operations_manager_override_list_matching(SelectQuery::new("OperationsManagerOverride"))
    }

    pub fn with_operations_manager_override_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::OperationsManagerOverride as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("operations_manager_override_list", selection));
        self
    }

    pub fn without_operations_manager_override_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::OperationsManagerOverride as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("operations_manager_override_list", selection));
        self
    }

    pub fn select_operations_manager_override_list(mut self) -> Self {
        self.query = self.query.relation("operations_manager_override_list");
        self
    }

    pub fn select_operations_manager_override_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("operations_manager_override_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("operations_manager_override_list", selection));
        self
}

    pub fn have_job_assignments(self) -> Self {
        self.with_job_assignment_list_matching(SelectQuery::new("JobAssignment"))
    }

    pub fn have_no_job_assignments(self) -> Self {
        self.without_job_assignment_list_matching(SelectQuery::new("JobAssignment"))
    }

    pub fn with_job_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::JobAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("job_assignment_list", selection));
        self
    }

    pub fn without_job_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::JobAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("job_assignment_list", selection));
        self
    }

    pub fn select_job_assignment_list(mut self) -> Self {
        self.query = self.query.relation("job_assignment_list");
        self
    }

    pub fn select_job_assignment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("job_assignment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("job_assignment_list", selection));
        self
}

    pub fn have_bonuses(self) -> Self {
        self.with_bonus_list_matching(SelectQuery::new("Bonus"))
    }

    pub fn have_no_bonuses(self) -> Self {
        self.without_bonus_list_matching(SelectQuery::new("Bonus"))
    }

    pub fn with_bonus_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Bonus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("bonus_list", selection));
        self
    }

    pub fn without_bonus_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Bonus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("bonus_list", selection));
        self
    }

    pub fn select_bonus_list(mut self) -> Self {
        self.query = self.query.relation("bonus_list");
        self
    }

    pub fn select_bonus_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("bonus_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("bonus_list", selection));
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

    pub fn have_employee_certifications(self) -> Self {
        self.with_employee_certification_list_matching(SelectQuery::new("EmployeeCertification"))
    }

    pub fn have_no_employee_certifications(self) -> Self {
        self.without_employee_certification_list_matching(SelectQuery::new("EmployeeCertification"))
    }

    pub fn with_employee_certification_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::EmployeeCertification as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_certification_list", selection));
        self
    }

    pub fn without_employee_certification_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::EmployeeCertification as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_certification_list", selection));
        self
    }

    pub fn select_employee_certification_list(mut self) -> Self {
        self.query = self.query.relation("employee_certification_list");
        self
    }

    pub fn select_employee_certification_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("employee_certification_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("employee_certification_list", selection));
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

    pub fn have_direct_deposit_info(self) -> Self {
        self.with_direct_deposit_info_list_matching(SelectQuery::new("DirectDepositInfo"))
    }

    pub fn have_no_direct_deposit_info(self) -> Self {
        self.without_direct_deposit_info_list_matching(SelectQuery::new("DirectDepositInfo"))
    }

    pub fn with_direct_deposit_info_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DirectDepositInfo as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("direct_deposit_info_list", selection));
        self
    }

    pub fn without_direct_deposit_info_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DirectDepositInfo as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("direct_deposit_info_list", selection));
        self
    }

    pub fn select_direct_deposit_info_list(mut self) -> Self {
        self.query = self.query.relation("direct_deposit_info_list");
        self
    }

    pub fn select_direct_deposit_info_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("direct_deposit_info_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("direct_deposit_info_list", selection));
        self
}

    pub fn have_union_dueses(self) -> Self {
        self.with_union_dues_list_matching(SelectQuery::new("UnionDues"))
    }

    pub fn have_no_union_dueses(self) -> Self {
        self.without_union_dues_list_matching(SelectQuery::new("UnionDues"))
    }

    pub fn with_union_dues_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::UnionDues as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("union_dues_list", selection));
        self
    }

    pub fn without_union_dues_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::UnionDues as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("union_dues_list", selection));
        self
    }

    pub fn select_union_dues_list(mut self) -> Self {
        self.query = self.query.relation("union_dues_list");
        self
    }

    pub fn select_union_dues_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("union_dues_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("union_dues_list", selection));
        self
}
    pub fn count_operations_manager_overrides(self) -> Self {
        self.count_operations_manager_overrides_as("count_operations_manager_overrides")
    }

    pub fn count_operations_manager_overrides_as(self, alias: impl Into<String>) -> Self {
        self.count_operations_manager_overrides_with(alias, crate::Q::operations_manager_overrides().unlimited())
    }

    pub fn count_operations_manager_overrides_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "operations_manager_override_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_operations_manager_overrides(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operations_manager_overrides_as("refinements", request)
    }

    pub fn stats_from_operations_manager_overrides_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "operations_manager_override_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_operations_manager_overrides_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_operations_manager_overrides(request)
    }




    pub fn count_job_assignments(self) -> Self {
        self.count_job_assignments_as("count_job_assignments")
    }

    pub fn count_job_assignments_as(self, alias: impl Into<String>) -> Self {
        self.count_job_assignments_with(alias, crate::Q::job_assignments().unlimited())
    }

    pub fn count_job_assignments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "job_assignment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_job_assignments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_assignments_as("refinements", request)
    }

    pub fn stats_from_job_assignments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "job_assignment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_job_assignments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_assignments(request)
    }




    pub fn count_bonuses(self) -> Self {
        self.count_bonuses_as("count_bonuses")
    }

    pub fn count_bonuses_as(self, alias: impl Into<String>) -> Self {
        self.count_bonuses_with(alias, crate::Q::bonuses().unlimited())
    }

    pub fn count_bonuses_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "bonus_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_bonuses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonuses_as("refinements", request)
    }

    pub fn stats_from_bonuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "bonus_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_bonuses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonuses(request)
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




    pub fn count_employee_certifications(self) -> Self {
        self.count_employee_certifications_as("count_employee_certifications")
    }

    pub fn count_employee_certifications_as(self, alias: impl Into<String>) -> Self {
        self.count_employee_certifications_with(alias, crate::Q::employee_certifications().unlimited())
    }

    pub fn count_employee_certifications_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "employee_certification_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_employee_certifications(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employee_certifications_as("refinements", request)
    }

    pub fn stats_from_employee_certifications_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "employee_certification_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_employee_certifications_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employee_certifications(request)
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




    pub fn count_direct_deposit_info(self) -> Self {
        self.count_direct_deposit_info_as("count_direct_deposit_info")
    }

    pub fn count_direct_deposit_info_as(self, alias: impl Into<String>) -> Self {
        self.count_direct_deposit_info_with(alias, crate::Q::direct_deposit_info().unlimited())
    }

    pub fn count_direct_deposit_info_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "direct_deposit_info_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_direct_deposit_info(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_direct_deposit_info_as("refinements", request)
    }

    pub fn stats_from_direct_deposit_info_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "direct_deposit_info_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_direct_deposit_info_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_direct_deposit_info(request)
    }




    pub fn count_union_dueses(self) -> Self {
        self.count_union_dueses_as("count_union_dueses")
    }

    pub fn count_union_dueses_as(self, alias: impl Into<String>) -> Self {
        self.count_union_dueses_with(alias, crate::Q::union_dueses().unlimited())
    }

    pub fn count_union_dueses_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "union_dues_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_union_dueses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_union_dueses_as("refinements", request)
    }

    pub fn stats_from_union_dueses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "union_dues_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_union_dueses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_union_dueses(request)
    }



}

impl<R> Default for EmployeeRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< EmployeeRequest<R> > for SelectQuery {
    fn from(request: EmployeeRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< EmployeeRequest<R> > for QuerySelection {
    fn from(request: EmployeeRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Employee> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::EmployeeRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<EmployeeRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Employee
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Employee::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> EmployeeRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
