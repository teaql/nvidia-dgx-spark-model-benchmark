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
            "first_name" => Some("first_name"),
            "last_name" => Some("last_name"),
            "email" => Some("email"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "status" | "status_id" => Some("status_id"),
            "position" | "position_id" => Some("position_id"),
            "merchant" | "merchant_id" => Some("merchant_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "status" => {
                self.with_status_matching(
                    crate::Q::employee_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "position" => {
                self.with_position_matching(
                    crate::Q::positions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "merchant" => {
                self.with_merchant_matching(
                    crate::Q::merchants_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "salary_record_list" => {
                self.with_salary_record_list_matching(
                    crate::Q::salary_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "attendance_log_list" => {
                self.with_attendance_log_list_matching(
                    crate::Q::attendance_logs_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "leave_request_list" => {
                self.with_leave_request_list_matching(
                    crate::Q::leave_requests_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "performance_review_list" => {
                self.with_performance_review_list_matching(
                    crate::Q::performance_reviews_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "training_record_list" => {
                self.with_training_record_list_matching(
                    crate::Q::training_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "benefit_plan_list" => {
                self.with_benefit_plan_list_matching(
                    crate::Q::benefit_plans_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "expense_claim_list" => {
                self.with_expense_claim_list_matching(
                    crate::Q::expense_claims_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "tax_form_list" => {
                self.with_tax_form_list_matching(
                    crate::Q::tax_forms_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "contract_list" => {
                self.with_contract_list_matching(
                    crate::Q::contracts_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "resignation_list" => {
                self.with_resignation_list_matching(
                    crate::Q::resignations_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "warning_letter_list" => {
                self.with_warning_letter_list_matching(
                    crate::Q::warning_letters_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "bonus_record_list" => {
                self.with_bonus_record_list_matching(
                    crate::Q::bonus_records_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "shift_schedule_list" => {
                self.with_shift_schedule_list_matching(
                    crate::Q::shift_schedules_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "time_off_balance_list" => {
                self.with_time_off_balance_list_matching(
                    crate::Q::time_off_balances_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "onboarding_checklist_list" => {
                self.with_onboarding_checklist_list_matching(
                    crate::Q::onboarding_checklists_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "offboarding_checklist_list" => {
                self.with_offboarding_checklist_list_matching(
                    crate::Q::offboarding_checklists_minimal()
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
        self.query = self.query.project("first_name");
        self.query = self.query.project("last_name");
        self.query = self.query.project("email");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("status_id");
        self.query = self.query.project("position_id");
        self.query = self.query.project("merchant_id");
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
        request = request.select_status();
        request = request.select_position();
        request = request.select_merchant();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_salary_record_list();
        request = request.select_attendance_log_list();
        request = request.select_leave_request_list();
        request = request.select_performance_review_list();
        request = request.select_training_record_list();
        request = request.select_benefit_plan_list();
        request = request.select_expense_claim_list();
        request = request.select_tax_form_list();
        request = request.select_contract_list();
        request = request.select_resignation_list();
        request = request.select_warning_letter_list();
        request = request.select_bonus_record_list();
        request = request.select_shift_schedule_list();
        request = request.select_time_off_balance_list();
        request = request.select_onboarding_checklist_list();
        request = request.select_offboarding_checklist_list();
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


    pub fn select_email(mut self) -> Self {
        self.query = self.query.project("email");
        self
    }

    pub fn project_email(self) -> Self {
        self.select_email()
    }

    pub fn select_email_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_email_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_email_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("email", raw_sql_segment));
        self
    }

    pub fn group_by_email(self) -> Self {
        self.group_by("email")
    }

    pub fn group_by_email_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("email");
        request.query = request
            .query
            .project_expr(alias, Expr::column("email"));
        request
    }

    pub fn group_by_email_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("email")
            .aggregate_with_function("email", alias, function)
    }

    pub fn count_email(self) -> Self {
        self.count_email_as("email_count")
    }

    pub fn count_email_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("email", alias)
    }

    pub fn sum_email(self) -> Self {
        self.sum_email_as("sum_email")
    }

    pub fn sum_email_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("email", alias)
    }

    pub fn avg_email(self) -> Self {
        self.avg_email_as("avg_email")
    }

    pub fn avg_email_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("email", alias)
    }

    pub fn min_email(self) -> Self {
        self.min_email_as("min_email")
    }

    pub fn min_email_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("email", alias)
    }

    pub fn max_email(self) -> Self {
        self.max_email_as("max_email")
    }

    pub fn max_email_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("email", alias)
    }

    pub fn unselect_email(mut self) -> Self {
        self.query.projection.retain(|field| field != "email");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "email");
        self
    }


    pub fn with_email(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "email",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_email_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "email",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_email_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("email", value));
        self
    }



    pub fn with_email_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("email", value));
        self
    }

    pub fn with_email_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("email", value));
        self
    }

    pub fn with_email_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("email", value));
        self
    }

    pub fn with_email_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("email", value));
        self
    }

    pub fn with_email_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("email", value));
        self
    }

    pub fn with_email_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("email", lower, upper));
        self
    }

    pub fn with_email_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "email",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_email_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "email",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_email_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "email",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_email_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("email", value));
        self
    }

    pub fn with_email_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("email", value));
        self
    }

    pub fn with_email_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("email", value));
        self
    }

    pub fn with_email_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("email", value));
        self
    }

    pub fn with_email_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("email", value));
        self
    }

    pub fn with_email_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("email", value));
        self
    }

    pub fn with_email_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("email", value));
        self
    }
    pub fn with_email_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("email", value));
        self
    }

    pub fn with_email_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("email", value));
        self
    }

    pub fn with_email_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("email"));
        self
    }



    pub fn with_email_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("email"));
        self
    }


    pub fn order_by_email_asc(mut self) -> Self {
        self.query = self.query.order_asc("email");
        self
    }

    pub fn order_by_email_desc(mut self) -> Self {
        self.query = self.query.order_desc("email");
        self
    }

    pub fn order_by_email_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("email");
        self
    }

    pub fn order_by_email_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("email");
        self
    }


    pub fn select_create_time(mut self) -> Self {
        self.query = self.query.project("create_time");
        self
    }

    pub fn project_create_time(self) -> Self {
        self.select_create_time()
    }

    pub fn select_create_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_create_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_create_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("create_time", raw_sql_segment));
        self
    }

    pub fn group_by_create_time(self) -> Self {
        self.group_by("create_time")
    }

    pub fn group_by_create_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("create_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("create_time"));
        request
    }

    pub fn group_by_create_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("create_time")
            .aggregate_with_function("create_time", alias, function)
    }

    pub fn count_create_time(self) -> Self {
        self.count_create_time_as("create_time_count")
    }

    pub fn count_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("create_time", alias)
    }

    pub fn sum_create_time(self) -> Self {
        self.sum_create_time_as("sum_create_time")
    }

    pub fn sum_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("create_time", alias)
    }

    pub fn avg_create_time(self) -> Self {
        self.avg_create_time_as("avg_create_time")
    }

    pub fn avg_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("create_time", alias)
    }

    pub fn min_create_time(self) -> Self {
        self.min_create_time_as("min_create_time")
    }

    pub fn min_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("create_time", alias)
    }

    pub fn max_create_time(self) -> Self {
        self.max_create_time_as("max_create_time")
    }

    pub fn max_create_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("create_time", alias)
    }

    pub fn unselect_create_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "create_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "create_time");
        self
    }


    pub fn with_create_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_create_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "create_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_create_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("create_time", value));
        self
    }



    pub fn with_create_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("create_time", value));
        self
    }

    pub fn with_create_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("create_time", value));
        self
    }

    pub fn with_create_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("create_time", value));
        self
    }

    pub fn with_create_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("create_time", lower, upper));
        self
    }

    pub fn with_create_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "create_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_create_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "create_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_create_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("create_time", value));
        self
    }

    pub fn with_create_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("create_time", value));
        self
    }

    pub fn with_create_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("create_time"));
        self
    }



    pub fn with_create_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("create_time"));
        self
    }


    pub fn order_by_create_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("create_time");
        self
    }

    pub fn order_by_create_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("create_time");
        self
    }

    pub fn order_by_create_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("create_time");
        self
    }


    pub fn select_update_time(mut self) -> Self {
        self.query = self.query.project("update_time");
        self
    }

    pub fn project_update_time(self) -> Self {
        self.select_update_time()
    }

    pub fn select_update_time_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_update_time_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_update_time_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("update_time", raw_sql_segment));
        self
    }

    pub fn group_by_update_time(self) -> Self {
        self.group_by("update_time")
    }

    pub fn group_by_update_time_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("update_time");
        request.query = request
            .query
            .project_expr(alias, Expr::column("update_time"));
        request
    }

    pub fn group_by_update_time_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("update_time")
            .aggregate_with_function("update_time", alias, function)
    }

    pub fn count_update_time(self) -> Self {
        self.count_update_time_as("update_time_count")
    }

    pub fn count_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("update_time", alias)
    }

    pub fn sum_update_time(self) -> Self {
        self.sum_update_time_as("sum_update_time")
    }

    pub fn sum_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("update_time", alias)
    }

    pub fn avg_update_time(self) -> Self {
        self.avg_update_time_as("avg_update_time")
    }

    pub fn avg_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("update_time", alias)
    }

    pub fn min_update_time(self) -> Self {
        self.min_update_time_as("min_update_time")
    }

    pub fn min_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("update_time", alias)
    }

    pub fn max_update_time(self) -> Self {
        self.max_update_time_as("max_update_time")
    }

    pub fn max_update_time_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("update_time", alias)
    }

    pub fn unselect_update_time(mut self) -> Self {
        self.query.projection.retain(|field| field != "update_time");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "update_time");
        self
    }


    pub fn with_update_time(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_update_time_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "update_time",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_update_time_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("update_time", value));
        self
    }



    pub fn with_update_time_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("update_time", value));
        self
    }

    pub fn with_update_time_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("update_time", value));
        self
    }

    pub fn with_update_time_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("update_time", value));
        self
    }

    pub fn with_update_time_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("update_time", lower, upper));
        self
    }

    pub fn with_update_time_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "update_time",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_update_time_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "update_time",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_update_time_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("update_time", value));
        self
    }

    pub fn with_update_time_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("update_time", value));
        self
    }

    pub fn with_update_time_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("update_time"));
        self
    }



    pub fn with_update_time_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("update_time"));
        self
    }


    pub fn order_by_update_time_asc(mut self) -> Self {
        self.query = self.query.order_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc(mut self) -> Self {
        self.query = self.query.order_desc("update_time");
        self
    }

    pub fn order_by_update_time_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("update_time");
        self
    }

    pub fn order_by_update_time_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("update_time");
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
    /// Please use `with_status_is` instead
    pub(crate) fn filter_by_status(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("status_id", value.entity_id_value()));
        self
    }
    /// Complex relation filter for `status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_status_is_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::employee_statuses_minimal().filter(...);
    /// let request = crate::Q::employees().with_status_matching(dynamic_query);
    /// ```
    pub fn with_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "status_id",
            <crate::EmployeeStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("status", selection));
        self
    }


    /// Complex relation filter for `status`.
    ///
    /// **Usage Priority:**
    ///
    /// 1. **Preferred**: If you only want to filter by specific known constants, please **prefer** the generated semantic shortcut methods, such as:
    ///    - [`Self::with_status_is_not_xxx`]
    ///
    ///    This gives the best code readability.
    ///
    /// 2. **Advanced**: Only use this method when you need to perform advanced searches, dynamic subqueries, or filter based on complex relation conditions.
    ///
    /// # Example
    /// ```rust
    /// // Only use when building dynamic queries
    /// let dynamic_query = crate::Q::employee_statuses_minimal().filter(...);
    /// let request = crate::Q::employees().without_status_matching(dynamic_query);
    /// ```
    pub fn without_status_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "status_id",
            <crate::EmployeeStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("status", selection));
        self
    }


    pub fn have_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("status_id"));
        self
    }

    pub fn have_no_status(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("status_id"));
        self
    }


    pub fn group_by_status(self) -> Self {
        self.group_by("status_id")
    }

    pub fn group_by_status_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("status_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("status_id"));
        request
    }

    pub fn group_by_status_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("status_id")
            .aggregate_with_function("status_id", alias, function)
    }

    pub fn group_by_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("status_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "status",
            "status_id",
            request,
        ));
        self
    }

    pub fn group_by_status_with_details(self) -> Self {
        self.group_by_status_with_details_from(crate::Q::employee_statuses().unlimited())
    }

    pub fn group_by_status_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_status_with(request)
    }


    pub fn roll_up_to_status(self) -> Self {
        self.roll_up_to_status_with(crate::Q::employee_statuses().unlimited())
    }

    pub fn roll_up_to_status_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_status_matching(selection.clone())
            .group_by_status_with(selection)
    }

    pub fn count_status(self) -> Self {
        self.count_status_as("status_count")
    }

    pub fn count_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("status_id", alias)
    }

    pub fn unselect_status(mut self) -> Self {
        self.query.projection.retain(|field| field != "status_id");
        self.query.relations.retain(|relation| relation.name != "status");
        self
    }


    pub fn filter_by_position(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("position_id", value.entity_id_value()));
        self
    }

    pub fn with_position_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "position_id",
            <crate::Position as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("position", selection));
        self
    }


    pub fn without_position_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "position_id",
            <crate::Position as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("position", selection));
        self
    }


    pub fn have_position(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("position_id"));
        self
    }

    pub fn have_no_position(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("position_id"));
        self
    }


    pub fn group_by_position(self) -> Self {
        self.group_by("position_id")
    }

    pub fn group_by_position_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("position_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("position_id"));
        request
    }

    pub fn group_by_position_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("position_id")
            .aggregate_with_function("position_id", alias, function)
    }

    pub fn group_by_position_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("position_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "position",
            "position_id",
            request,
        ));
        self
    }

    pub fn group_by_position_with_details(self) -> Self {
        self.group_by_position_with_details_from(crate::Q::positions().unlimited())
    }

    pub fn group_by_position_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_position_with(request)
    }


    pub fn roll_up_to_position(self) -> Self {
        self.roll_up_to_position_with(crate::Q::positions().unlimited())
    }

    pub fn roll_up_to_position_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_position_matching(selection.clone())
            .group_by_position_with(selection)
    }

    pub fn count_position(self) -> Self {
        self.count_position_as("position_count")
    }

    pub fn count_position_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("position_id", alias)
    }

    pub fn unselect_position(mut self) -> Self {
        self.query.projection.retain(|field| field != "position_id");
        self.query.relations.retain(|relation| relation.name != "position");
        self
    }


    pub fn filter_by_merchant(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("merchant_id", value.entity_id_value()));
        self
    }

    pub fn with_merchant_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "merchant_id",
            <crate::Merchant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("merchant", selection));
        self
    }


    pub fn without_merchant_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "merchant_id",
            <crate::Merchant as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("merchant", selection));
        self
    }


    pub fn have_merchant(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("merchant_id"));
        self
    }

    pub fn have_no_merchant(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("merchant_id"));
        self
    }


    pub fn group_by_merchant(self) -> Self {
        self.group_by("merchant_id")
    }

    pub fn group_by_merchant_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("merchant_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("merchant_id"));
        request
    }

    pub fn group_by_merchant_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("merchant_id")
            .aggregate_with_function("merchant_id", alias, function)
    }

    pub fn group_by_merchant_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("merchant_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "merchant",
            "merchant_id",
            request,
        ));
        self
    }

    pub fn group_by_merchant_with_details(self) -> Self {
        self.group_by_merchant_with_details_from(crate::Q::merchants().unlimited())
    }

    pub fn group_by_merchant_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_merchant_with(request)
    }


    pub fn roll_up_to_merchant(self) -> Self {
        self.roll_up_to_merchant_with(crate::Q::merchants().unlimited())
    }

    pub fn roll_up_to_merchant_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_merchant_matching(selection.clone())
            .group_by_merchant_with(selection)
    }

    pub fn count_merchant(self) -> Self {
        self.count_merchant_as("merchant_count")
    }

    pub fn count_merchant_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("merchant_id", alias)
    }

    pub fn unselect_merchant(mut self) -> Self {
        self.query.projection.retain(|field| field != "merchant_id");
        self.query.relations.retain(|relation| relation.name != "merchant");
        self
    }
    pub fn status_is_active(self) -> Self {
        self.filter_by_status(1001_u64)
    }

    pub fn with_status_is_active(self) -> Self {
        self.filter_by_status(1001_u64)
    }



    pub fn with_status_is_not_active(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("status_id", 1001_u64));
        self
    }


    pub fn status_is_probation(self) -> Self {
        self.filter_by_status(1002_u64)
    }

    pub fn with_status_is_probation(self) -> Self {
        self.filter_by_status(1002_u64)
    }



    pub fn with_status_is_not_probation(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("status_id", 1002_u64));
        self
    }


    pub fn status_is_terminated(self) -> Self {
        self.filter_by_status(1003_u64)
    }

    pub fn with_status_is_terminated(self) -> Self {
        self.filter_by_status(1003_u64)
    }



    pub fn with_status_is_not_terminated(mut self) -> Self {
        self.query = self.query.and_filter(Expr::ne("status_id", 1003_u64));
        self
    }






    pub fn select_status(mut self) -> Self {
        self.query = self.query.relation("status");
        self
    }

    pub fn select_status_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("status", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("status", selection));
        self
}

    pub fn facet_by_status_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_status_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_status_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "status",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_position(mut self) -> Self {
        self.query = self.query.relation("position");
        self
    }

    pub fn select_position_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("position", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("position", selection));
        self
}

    pub fn facet_by_position_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_position_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_position_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "position",
            request,
            include_all_facets,
        ));
        self
    }

    pub fn select_merchant(mut self) -> Self {
        self.query = self.query.relation("merchant");
        self
    }

    pub fn select_merchant_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("merchant", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("merchant", selection));
        self
}

    pub fn facet_by_merchant_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_merchant_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_merchant_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "merchant",
            request,
            include_all_facets,
        ));
        self
    }
    pub fn have_salary_records(self) -> Self {
        self.with_salary_record_list_matching(SelectQuery::new("SalaryRecord"))
    }

    pub fn have_no_salary_records(self) -> Self {
        self.without_salary_record_list_matching(SelectQuery::new("SalaryRecord"))
    }

    pub fn with_salary_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::SalaryRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("salary_record_list", selection));
        self
    }

    pub fn without_salary_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::SalaryRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("salary_record_list", selection));
        self
    }

    pub fn select_salary_record_list(mut self) -> Self {
        self.query = self.query.relation("salary_record_list");
        self
    }

    pub fn select_salary_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("salary_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("salary_record_list", selection));
        self
}

    pub fn have_attendance_logs(self) -> Self {
        self.with_attendance_log_list_matching(SelectQuery::new("AttendanceLog"))
    }

    pub fn have_no_attendance_logs(self) -> Self {
        self.without_attendance_log_list_matching(SelectQuery::new("AttendanceLog"))
    }

    pub fn with_attendance_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AttendanceLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("attendance_log_list", selection));
        self
    }

    pub fn without_attendance_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AttendanceLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("attendance_log_list", selection));
        self
    }

    pub fn select_attendance_log_list(mut self) -> Self {
        self.query = self.query.relation("attendance_log_list");
        self
    }

    pub fn select_attendance_log_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("attendance_log_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("attendance_log_list", selection));
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

    pub fn have_performance_reviews(self) -> Self {
        self.with_performance_review_list_matching(SelectQuery::new("PerformanceReview"))
    }

    pub fn have_no_performance_reviews(self) -> Self {
        self.without_performance_review_list_matching(SelectQuery::new("PerformanceReview"))
    }

    pub fn with_performance_review_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PerformanceReview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("performance_review_list", selection));
        self
    }

    pub fn without_performance_review_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PerformanceReview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("performance_review_list", selection));
        self
    }

    pub fn select_performance_review_list(mut self) -> Self {
        self.query = self.query.relation("performance_review_list");
        self
    }

    pub fn select_performance_review_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("performance_review_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("performance_review_list", selection));
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

    pub fn have_benefit_plans(self) -> Self {
        self.with_benefit_plan_list_matching(SelectQuery::new("BenefitPlan"))
    }

    pub fn have_no_benefit_plans(self) -> Self {
        self.without_benefit_plan_list_matching(SelectQuery::new("BenefitPlan"))
    }

    pub fn with_benefit_plan_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::BenefitPlan as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("benefit_plan_list", selection));
        self
    }

    pub fn without_benefit_plan_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::BenefitPlan as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("benefit_plan_list", selection));
        self
    }

    pub fn select_benefit_plan_list(mut self) -> Self {
        self.query = self.query.relation("benefit_plan_list");
        self
    }

    pub fn select_benefit_plan_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("benefit_plan_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("benefit_plan_list", selection));
        self
}

    pub fn have_expense_claims(self) -> Self {
        self.with_expense_claim_list_matching(SelectQuery::new("ExpenseClaim"))
    }

    pub fn have_no_expense_claims(self) -> Self {
        self.without_expense_claim_list_matching(SelectQuery::new("ExpenseClaim"))
    }

    pub fn with_expense_claim_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ExpenseClaim as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("expense_claim_list", selection));
        self
    }

    pub fn without_expense_claim_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ExpenseClaim as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("expense_claim_list", selection));
        self
    }

    pub fn select_expense_claim_list(mut self) -> Self {
        self.query = self.query.relation("expense_claim_list");
        self
    }

    pub fn select_expense_claim_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("expense_claim_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("expense_claim_list", selection));
        self
}

    pub fn have_tax_forms(self) -> Self {
        self.with_tax_form_list_matching(SelectQuery::new("TaxForm"))
    }

    pub fn have_no_tax_forms(self) -> Self {
        self.without_tax_form_list_matching(SelectQuery::new("TaxForm"))
    }

    pub fn with_tax_form_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TaxForm as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("tax_form_list", selection));
        self
    }

    pub fn without_tax_form_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TaxForm as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("tax_form_list", selection));
        self
    }

    pub fn select_tax_form_list(mut self) -> Self {
        self.query = self.query.relation("tax_form_list");
        self
    }

    pub fn select_tax_form_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("tax_form_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("tax_form_list", selection));
        self
}

    pub fn have_contracts(self) -> Self {
        self.with_contract_list_matching(SelectQuery::new("Contract"))
    }

    pub fn have_no_contracts(self) -> Self {
        self.without_contract_list_matching(SelectQuery::new("Contract"))
    }

    pub fn with_contract_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Contract as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("contract_list", selection));
        self
    }

    pub fn without_contract_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Contract as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("contract_list", selection));
        self
    }

    pub fn select_contract_list(mut self) -> Self {
        self.query = self.query.relation("contract_list");
        self
    }

    pub fn select_contract_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("contract_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("contract_list", selection));
        self
}

    pub fn have_resignations(self) -> Self {
        self.with_resignation_list_matching(SelectQuery::new("Resignation"))
    }

    pub fn have_no_resignations(self) -> Self {
        self.without_resignation_list_matching(SelectQuery::new("Resignation"))
    }

    pub fn with_resignation_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Resignation as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("resignation_list", selection));
        self
    }

    pub fn without_resignation_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Resignation as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("resignation_list", selection));
        self
    }

    pub fn select_resignation_list(mut self) -> Self {
        self.query = self.query.relation("resignation_list");
        self
    }

    pub fn select_resignation_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("resignation_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("resignation_list", selection));
        self
}

    pub fn have_warning_letters(self) -> Self {
        self.with_warning_letter_list_matching(SelectQuery::new("WarningLetter"))
    }

    pub fn have_no_warning_letters(self) -> Self {
        self.without_warning_letter_list_matching(SelectQuery::new("WarningLetter"))
    }

    pub fn with_warning_letter_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::WarningLetter as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("warning_letter_list", selection));
        self
    }

    pub fn without_warning_letter_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::WarningLetter as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("warning_letter_list", selection));
        self
    }

    pub fn select_warning_letter_list(mut self) -> Self {
        self.query = self.query.relation("warning_letter_list");
        self
    }

    pub fn select_warning_letter_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("warning_letter_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("warning_letter_list", selection));
        self
}

    pub fn have_bonus_records(self) -> Self {
        self.with_bonus_record_list_matching(SelectQuery::new("BonusRecord"))
    }

    pub fn have_no_bonus_records(self) -> Self {
        self.without_bonus_record_list_matching(SelectQuery::new("BonusRecord"))
    }

    pub fn with_bonus_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::BonusRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("bonus_record_list", selection));
        self
    }

    pub fn without_bonus_record_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::BonusRecord as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("bonus_record_list", selection));
        self
    }

    pub fn select_bonus_record_list(mut self) -> Self {
        self.query = self.query.relation("bonus_record_list");
        self
    }

    pub fn select_bonus_record_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("bonus_record_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("bonus_record_list", selection));
        self
}

    pub fn have_shift_schedules(self) -> Self {
        self.with_shift_schedule_list_matching(SelectQuery::new("ShiftSchedule"))
    }

    pub fn have_no_shift_schedules(self) -> Self {
        self.without_shift_schedule_list_matching(SelectQuery::new("ShiftSchedule"))
    }

    pub fn with_shift_schedule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ShiftSchedule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("shift_schedule_list", selection));
        self
    }

    pub fn without_shift_schedule_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ShiftSchedule as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("shift_schedule_list", selection));
        self
    }

    pub fn select_shift_schedule_list(mut self) -> Self {
        self.query = self.query.relation("shift_schedule_list");
        self
    }

    pub fn select_shift_schedule_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("shift_schedule_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("shift_schedule_list", selection));
        self
}

    pub fn have_time_off_balances(self) -> Self {
        self.with_time_off_balance_list_matching(SelectQuery::new("TimeOffBalance"))
    }

    pub fn have_no_time_off_balances(self) -> Self {
        self.without_time_off_balance_list_matching(SelectQuery::new("TimeOffBalance"))
    }

    pub fn with_time_off_balance_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TimeOffBalance as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("time_off_balance_list", selection));
        self
    }

    pub fn without_time_off_balance_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TimeOffBalance as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("time_off_balance_list", selection));
        self
    }

    pub fn select_time_off_balance_list(mut self) -> Self {
        self.query = self.query.relation("time_off_balance_list");
        self
    }

    pub fn select_time_off_balance_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("time_off_balance_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("time_off_balance_list", selection));
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

    pub fn have_offboarding_checklists(self) -> Self {
        self.with_offboarding_checklist_list_matching(SelectQuery::new("OffboardingChecklist"))
    }

    pub fn have_no_offboarding_checklists(self) -> Self {
        self.without_offboarding_checklist_list_matching(SelectQuery::new("OffboardingChecklist"))
    }

    pub fn with_offboarding_checklist_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::OffboardingChecklist as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("offboarding_checklist_list", selection));
        self
    }

    pub fn without_offboarding_checklist_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::OffboardingChecklist as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "employee_id",
        ));
        self.relation_filters.push(RelationFilter::new("offboarding_checklist_list", selection));
        self
    }

    pub fn select_offboarding_checklist_list(mut self) -> Self {
        self.query = self.query.relation("offboarding_checklist_list");
        self
    }

    pub fn select_offboarding_checklist_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("offboarding_checklist_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("offboarding_checklist_list", selection));
        self
}
    pub fn count_salary_records(self) -> Self {
        self.count_salary_records_as("count_salary_records")
    }

    pub fn count_salary_records_as(self, alias: impl Into<String>) -> Self {
        self.count_salary_records_with(alias, crate::Q::salary_records().unlimited())
    }

    pub fn count_salary_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "salary_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_salary_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_salary_records_as("refinements", request)
    }

    pub fn stats_from_salary_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "salary_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_salary_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_salary_records(request)
    }


    pub fn min_create_time_of_salary_records(self) -> Self {
        self.min_create_time_of_salary_records_as("min_create_time_of_salary_records", crate::Q::salary_records().unlimited())
    }

    pub fn min_create_time_of_salary_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_salary_records_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_salary_records(self) -> Self {
        self.max_create_time_of_salary_records_as("max_create_time_of_salary_records", crate::Q::salary_records().unlimited())
    }

    pub fn max_create_time_of_salary_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_salary_records_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_salary_records(self) -> Self {
        self.min_update_time_of_salary_records_as("min_update_time_of_salary_records", crate::Q::salary_records().unlimited())
    }

    pub fn min_update_time_of_salary_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_salary_records_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_salary_records(self) -> Self {
        self.max_update_time_of_salary_records_as("max_update_time_of_salary_records", crate::Q::salary_records().unlimited())
    }

    pub fn max_update_time_of_salary_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_salary_records_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_attendance_logs(self) -> Self {
        self.count_attendance_logs_as("count_attendance_logs")
    }

    pub fn count_attendance_logs_as(self, alias: impl Into<String>) -> Self {
        self.count_attendance_logs_with(alias, crate::Q::attendance_logs().unlimited())
    }

    pub fn count_attendance_logs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "attendance_log_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_attendance_logs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attendance_logs_as("refinements", request)
    }

    pub fn stats_from_attendance_logs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "attendance_log_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_attendance_logs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attendance_logs(request)
    }


    pub fn min_create_time_of_attendance_logs(self) -> Self {
        self.min_create_time_of_attendance_logs_as("min_create_time_of_attendance_logs", crate::Q::attendance_logs().unlimited())
    }

    pub fn min_create_time_of_attendance_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attendance_logs_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_attendance_logs(self) -> Self {
        self.max_create_time_of_attendance_logs_as("max_create_time_of_attendance_logs", crate::Q::attendance_logs().unlimited())
    }

    pub fn max_create_time_of_attendance_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attendance_logs_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_attendance_logs(self) -> Self {
        self.min_update_time_of_attendance_logs_as("min_update_time_of_attendance_logs", crate::Q::attendance_logs().unlimited())
    }

    pub fn min_update_time_of_attendance_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attendance_logs_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_attendance_logs(self) -> Self {
        self.max_update_time_of_attendance_logs_as("max_update_time_of_attendance_logs", crate::Q::attendance_logs().unlimited())
    }

    pub fn max_update_time_of_attendance_logs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_attendance_logs_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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


    pub fn min_create_time_of_leave_requests(self) -> Self {
        self.min_create_time_of_leave_requests_as("min_create_time_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn min_create_time_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_leave_requests(self) -> Self {
        self.max_create_time_of_leave_requests_as("max_create_time_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn max_create_time_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_leave_requests(self) -> Self {
        self.min_update_time_of_leave_requests_as("min_update_time_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn min_update_time_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_leave_requests(self) -> Self {
        self.max_update_time_of_leave_requests_as("max_update_time_of_leave_requests", crate::Q::leave_requests().unlimited())
    }

    pub fn max_update_time_of_leave_requests_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_leave_requests_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_performance_reviews(self) -> Self {
        self.count_performance_reviews_as("count_performance_reviews")
    }

    pub fn count_performance_reviews_as(self, alias: impl Into<String>) -> Self {
        self.count_performance_reviews_with(alias, crate::Q::performance_reviews().unlimited())
    }

    pub fn count_performance_reviews_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "performance_review_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_performance_reviews(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as("refinements", request)
    }

    pub fn stats_from_performance_reviews_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "performance_review_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_performance_reviews_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews(request)
    }


    pub fn min_create_time_of_performance_reviews(self) -> Self {
        self.min_create_time_of_performance_reviews_as("min_create_time_of_performance_reviews", crate::Q::performance_reviews().unlimited())
    }

    pub fn min_create_time_of_performance_reviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_performance_reviews(self) -> Self {
        self.max_create_time_of_performance_reviews_as("max_create_time_of_performance_reviews", crate::Q::performance_reviews().unlimited())
    }

    pub fn max_create_time_of_performance_reviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_performance_reviews(self) -> Self {
        self.min_update_time_of_performance_reviews_as("min_update_time_of_performance_reviews", crate::Q::performance_reviews().unlimited())
    }

    pub fn min_update_time_of_performance_reviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_performance_reviews(self) -> Self {
        self.max_update_time_of_performance_reviews_as("max_update_time_of_performance_reviews", crate::Q::performance_reviews().unlimited())
    }

    pub fn max_update_time_of_performance_reviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_performance_reviews_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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


    pub fn min_create_time_of_training_records(self) -> Self {
        self.min_create_time_of_training_records_as("min_create_time_of_training_records", crate::Q::training_records().unlimited())
    }

    pub fn min_create_time_of_training_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_training_records(self) -> Self {
        self.max_create_time_of_training_records_as("max_create_time_of_training_records", crate::Q::training_records().unlimited())
    }

    pub fn max_create_time_of_training_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_training_records(self) -> Self {
        self.min_update_time_of_training_records_as("min_update_time_of_training_records", crate::Q::training_records().unlimited())
    }

    pub fn min_update_time_of_training_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_training_records(self) -> Self {
        self.max_update_time_of_training_records_as("max_update_time_of_training_records", crate::Q::training_records().unlimited())
    }

    pub fn max_update_time_of_training_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_training_records_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_benefit_plans(self) -> Self {
        self.count_benefit_plans_as("count_benefit_plans")
    }

    pub fn count_benefit_plans_as(self, alias: impl Into<String>) -> Self {
        self.count_benefit_plans_with(alias, crate::Q::benefit_plans().unlimited())
    }

    pub fn count_benefit_plans_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "benefit_plan_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_benefit_plans(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_benefit_plans_as("refinements", request)
    }

    pub fn stats_from_benefit_plans_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "benefit_plan_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_benefit_plans_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_benefit_plans(request)
    }


    pub fn min_create_time_of_benefit_plans(self) -> Self {
        self.min_create_time_of_benefit_plans_as("min_create_time_of_benefit_plans", crate::Q::benefit_plans().unlimited())
    }

    pub fn min_create_time_of_benefit_plans_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_benefit_plans_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_benefit_plans(self) -> Self {
        self.max_create_time_of_benefit_plans_as("max_create_time_of_benefit_plans", crate::Q::benefit_plans().unlimited())
    }

    pub fn max_create_time_of_benefit_plans_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_benefit_plans_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_benefit_plans(self) -> Self {
        self.min_update_time_of_benefit_plans_as("min_update_time_of_benefit_plans", crate::Q::benefit_plans().unlimited())
    }

    pub fn min_update_time_of_benefit_plans_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_benefit_plans_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_benefit_plans(self) -> Self {
        self.max_update_time_of_benefit_plans_as("max_update_time_of_benefit_plans", crate::Q::benefit_plans().unlimited())
    }

    pub fn max_update_time_of_benefit_plans_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_benefit_plans_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_expense_claims(self) -> Self {
        self.count_expense_claims_as("count_expense_claims")
    }

    pub fn count_expense_claims_as(self, alias: impl Into<String>) -> Self {
        self.count_expense_claims_with(alias, crate::Q::expense_claims().unlimited())
    }

    pub fn count_expense_claims_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "expense_claim_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_expense_claims(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expense_claims_as("refinements", request)
    }

    pub fn stats_from_expense_claims_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "expense_claim_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_expense_claims_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expense_claims(request)
    }


    pub fn min_create_time_of_expense_claims(self) -> Self {
        self.min_create_time_of_expense_claims_as("min_create_time_of_expense_claims", crate::Q::expense_claims().unlimited())
    }

    pub fn min_create_time_of_expense_claims_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expense_claims_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_expense_claims(self) -> Self {
        self.max_create_time_of_expense_claims_as("max_create_time_of_expense_claims", crate::Q::expense_claims().unlimited())
    }

    pub fn max_create_time_of_expense_claims_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expense_claims_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_expense_claims(self) -> Self {
        self.min_update_time_of_expense_claims_as("min_update_time_of_expense_claims", crate::Q::expense_claims().unlimited())
    }

    pub fn min_update_time_of_expense_claims_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expense_claims_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_expense_claims(self) -> Self {
        self.max_update_time_of_expense_claims_as("max_update_time_of_expense_claims", crate::Q::expense_claims().unlimited())
    }

    pub fn max_update_time_of_expense_claims_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_expense_claims_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_tax_forms(self) -> Self {
        self.count_tax_forms_as("count_tax_forms")
    }

    pub fn count_tax_forms_as(self, alias: impl Into<String>) -> Self {
        self.count_tax_forms_with(alias, crate::Q::tax_forms().unlimited())
    }

    pub fn count_tax_forms_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tax_form_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_tax_forms(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_forms_as("refinements", request)
    }

    pub fn stats_from_tax_forms_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "tax_form_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_tax_forms_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_forms(request)
    }


    pub fn min_create_time_of_tax_forms(self) -> Self {
        self.min_create_time_of_tax_forms_as("min_create_time_of_tax_forms", crate::Q::tax_forms().unlimited())
    }

    pub fn min_create_time_of_tax_forms_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_forms_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_tax_forms(self) -> Self {
        self.max_create_time_of_tax_forms_as("max_create_time_of_tax_forms", crate::Q::tax_forms().unlimited())
    }

    pub fn max_create_time_of_tax_forms_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_forms_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_tax_forms(self) -> Self {
        self.min_update_time_of_tax_forms_as("min_update_time_of_tax_forms", crate::Q::tax_forms().unlimited())
    }

    pub fn min_update_time_of_tax_forms_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_forms_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_tax_forms(self) -> Self {
        self.max_update_time_of_tax_forms_as("max_update_time_of_tax_forms", crate::Q::tax_forms().unlimited())
    }

    pub fn max_update_time_of_tax_forms_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_tax_forms_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_contracts(self) -> Self {
        self.count_contracts_as("count_contracts")
    }

    pub fn count_contracts_as(self, alias: impl Into<String>) -> Self {
        self.count_contracts_with(alias, crate::Q::contracts().unlimited())
    }

    pub fn count_contracts_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "contract_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_contracts(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts_as("refinements", request)
    }

    pub fn stats_from_contracts_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "contract_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_contracts_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts(request)
    }


    pub fn min_create_time_of_contracts(self) -> Self {
        self.min_create_time_of_contracts_as("min_create_time_of_contracts", crate::Q::contracts().unlimited())
    }

    pub fn min_create_time_of_contracts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_contracts(self) -> Self {
        self.max_create_time_of_contracts_as("max_create_time_of_contracts", crate::Q::contracts().unlimited())
    }

    pub fn max_create_time_of_contracts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_contracts(self) -> Self {
        self.min_update_time_of_contracts_as("min_update_time_of_contracts", crate::Q::contracts().unlimited())
    }

    pub fn min_update_time_of_contracts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_contracts(self) -> Self {
        self.max_update_time_of_contracts_as("max_update_time_of_contracts", crate::Q::contracts().unlimited())
    }

    pub fn max_update_time_of_contracts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contracts_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_resignations(self) -> Self {
        self.count_resignations_as("count_resignations")
    }

    pub fn count_resignations_as(self, alias: impl Into<String>) -> Self {
        self.count_resignations_with(alias, crate::Q::resignations().unlimited())
    }

    pub fn count_resignations_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "resignation_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_resignations(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_resignations_as("refinements", request)
    }

    pub fn stats_from_resignations_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "resignation_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_resignations_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_resignations(request)
    }


    pub fn min_create_time_of_resignations(self) -> Self {
        self.min_create_time_of_resignations_as("min_create_time_of_resignations", crate::Q::resignations().unlimited())
    }

    pub fn min_create_time_of_resignations_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_resignations_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_resignations(self) -> Self {
        self.max_create_time_of_resignations_as("max_create_time_of_resignations", crate::Q::resignations().unlimited())
    }

    pub fn max_create_time_of_resignations_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_resignations_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_resignations(self) -> Self {
        self.min_update_time_of_resignations_as("min_update_time_of_resignations", crate::Q::resignations().unlimited())
    }

    pub fn min_update_time_of_resignations_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_resignations_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_resignations(self) -> Self {
        self.max_update_time_of_resignations_as("max_update_time_of_resignations", crate::Q::resignations().unlimited())
    }

    pub fn max_update_time_of_resignations_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_resignations_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_warning_letters(self) -> Self {
        self.count_warning_letters_as("count_warning_letters")
    }

    pub fn count_warning_letters_as(self, alias: impl Into<String>) -> Self {
        self.count_warning_letters_with(alias, crate::Q::warning_letters().unlimited())
    }

    pub fn count_warning_letters_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "warning_letter_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_warning_letters(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_warning_letters_as("refinements", request)
    }

    pub fn stats_from_warning_letters_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "warning_letter_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_warning_letters_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_warning_letters(request)
    }


    pub fn min_create_time_of_warning_letters(self) -> Self {
        self.min_create_time_of_warning_letters_as("min_create_time_of_warning_letters", crate::Q::warning_letters().unlimited())
    }

    pub fn min_create_time_of_warning_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_warning_letters_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_warning_letters(self) -> Self {
        self.max_create_time_of_warning_letters_as("max_create_time_of_warning_letters", crate::Q::warning_letters().unlimited())
    }

    pub fn max_create_time_of_warning_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_warning_letters_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_warning_letters(self) -> Self {
        self.min_update_time_of_warning_letters_as("min_update_time_of_warning_letters", crate::Q::warning_letters().unlimited())
    }

    pub fn min_update_time_of_warning_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_warning_letters_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_warning_letters(self) -> Self {
        self.max_update_time_of_warning_letters_as("max_update_time_of_warning_letters", crate::Q::warning_letters().unlimited())
    }

    pub fn max_update_time_of_warning_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_warning_letters_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_bonus_records(self) -> Self {
        self.count_bonus_records_as("count_bonus_records")
    }

    pub fn count_bonus_records_as(self, alias: impl Into<String>) -> Self {
        self.count_bonus_records_with(alias, crate::Q::bonus_records().unlimited())
    }

    pub fn count_bonus_records_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "bonus_record_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_bonus_records(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonus_records_as("refinements", request)
    }

    pub fn stats_from_bonus_records_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "bonus_record_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_bonus_records_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonus_records(request)
    }


    pub fn min_create_time_of_bonus_records(self) -> Self {
        self.min_create_time_of_bonus_records_as("min_create_time_of_bonus_records", crate::Q::bonus_records().unlimited())
    }

    pub fn min_create_time_of_bonus_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonus_records_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_bonus_records(self) -> Self {
        self.max_create_time_of_bonus_records_as("max_create_time_of_bonus_records", crate::Q::bonus_records().unlimited())
    }

    pub fn max_create_time_of_bonus_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonus_records_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_bonus_records(self) -> Self {
        self.min_update_time_of_bonus_records_as("min_update_time_of_bonus_records", crate::Q::bonus_records().unlimited())
    }

    pub fn min_update_time_of_bonus_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonus_records_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_bonus_records(self) -> Self {
        self.max_update_time_of_bonus_records_as("max_update_time_of_bonus_records", crate::Q::bonus_records().unlimited())
    }

    pub fn max_update_time_of_bonus_records_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_bonus_records_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_shift_schedules(self) -> Self {
        self.count_shift_schedules_as("count_shift_schedules")
    }

    pub fn count_shift_schedules_as(self, alias: impl Into<String>) -> Self {
        self.count_shift_schedules_with(alias, crate::Q::shift_schedules().unlimited())
    }

    pub fn count_shift_schedules_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "shift_schedule_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_shift_schedules(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_schedules_as("refinements", request)
    }

    pub fn stats_from_shift_schedules_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "shift_schedule_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_shift_schedules_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_schedules(request)
    }


    pub fn min_create_time_of_shift_schedules(self) -> Self {
        self.min_create_time_of_shift_schedules_as("min_create_time_of_shift_schedules", crate::Q::shift_schedules().unlimited())
    }

    pub fn min_create_time_of_shift_schedules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_schedules_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_shift_schedules(self) -> Self {
        self.max_create_time_of_shift_schedules_as("max_create_time_of_shift_schedules", crate::Q::shift_schedules().unlimited())
    }

    pub fn max_create_time_of_shift_schedules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_schedules_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_shift_schedules(self) -> Self {
        self.min_update_time_of_shift_schedules_as("min_update_time_of_shift_schedules", crate::Q::shift_schedules().unlimited())
    }

    pub fn min_update_time_of_shift_schedules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_schedules_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_shift_schedules(self) -> Self {
        self.max_update_time_of_shift_schedules_as("max_update_time_of_shift_schedules", crate::Q::shift_schedules().unlimited())
    }

    pub fn max_update_time_of_shift_schedules_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_shift_schedules_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_time_off_balances(self) -> Self {
        self.count_time_off_balances_as("count_time_off_balances")
    }

    pub fn count_time_off_balances_as(self, alias: impl Into<String>) -> Self {
        self.count_time_off_balances_with(alias, crate::Q::time_off_balances().unlimited())
    }

    pub fn count_time_off_balances_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "time_off_balance_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_time_off_balances(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_off_balances_as("refinements", request)
    }

    pub fn stats_from_time_off_balances_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "time_off_balance_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_time_off_balances_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_off_balances(request)
    }


    pub fn min_create_time_of_time_off_balances(self) -> Self {
        self.min_create_time_of_time_off_balances_as("min_create_time_of_time_off_balances", crate::Q::time_off_balances().unlimited())
    }

    pub fn min_create_time_of_time_off_balances_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_off_balances_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_time_off_balances(self) -> Self {
        self.max_create_time_of_time_off_balances_as("max_create_time_of_time_off_balances", crate::Q::time_off_balances().unlimited())
    }

    pub fn max_create_time_of_time_off_balances_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_off_balances_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_time_off_balances(self) -> Self {
        self.min_update_time_of_time_off_balances_as("min_update_time_of_time_off_balances", crate::Q::time_off_balances().unlimited())
    }

    pub fn min_update_time_of_time_off_balances_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_off_balances_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_time_off_balances(self) -> Self {
        self.max_update_time_of_time_off_balances_as("max_update_time_of_time_off_balances", crate::Q::time_off_balances().unlimited())
    }

    pub fn max_update_time_of_time_off_balances_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_time_off_balances_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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


    pub fn min_create_time_of_onboarding_checklists(self) -> Self {
        self.min_create_time_of_onboarding_checklists_as("min_create_time_of_onboarding_checklists", crate::Q::onboarding_checklists().unlimited())
    }

    pub fn min_create_time_of_onboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_onboarding_checklists(self) -> Self {
        self.max_create_time_of_onboarding_checklists_as("max_create_time_of_onboarding_checklists", crate::Q::onboarding_checklists().unlimited())
    }

    pub fn max_create_time_of_onboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_onboarding_checklists(self) -> Self {
        self.min_update_time_of_onboarding_checklists_as("min_update_time_of_onboarding_checklists", crate::Q::onboarding_checklists().unlimited())
    }

    pub fn min_update_time_of_onboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_onboarding_checklists(self) -> Self {
        self.max_update_time_of_onboarding_checklists_as("max_update_time_of_onboarding_checklists", crate::Q::onboarding_checklists().unlimited())
    }

    pub fn max_update_time_of_onboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_onboarding_checklists_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_offboarding_checklists(self) -> Self {
        self.count_offboarding_checklists_as("count_offboarding_checklists")
    }

    pub fn count_offboarding_checklists_as(self, alias: impl Into<String>) -> Self {
        self.count_offboarding_checklists_with(alias, crate::Q::offboarding_checklists().unlimited())
    }

    pub fn count_offboarding_checklists_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "offboarding_checklist_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_offboarding_checklists(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_checklists_as("refinements", request)
    }

    pub fn stats_from_offboarding_checklists_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "offboarding_checklist_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_offboarding_checklists_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_checklists(request)
    }


    pub fn min_create_time_of_offboarding_checklists(self) -> Self {
        self.min_create_time_of_offboarding_checklists_as("min_create_time_of_offboarding_checklists", crate::Q::offboarding_checklists().unlimited())
    }

    pub fn min_create_time_of_offboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_checklists_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_offboarding_checklists(self) -> Self {
        self.max_create_time_of_offboarding_checklists_as("max_create_time_of_offboarding_checklists", crate::Q::offboarding_checklists().unlimited())
    }

    pub fn max_create_time_of_offboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_checklists_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_offboarding_checklists(self) -> Self {
        self.min_update_time_of_offboarding_checklists_as("min_update_time_of_offboarding_checklists", crate::Q::offboarding_checklists().unlimited())
    }

    pub fn min_update_time_of_offboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_checklists_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_offboarding_checklists(self) -> Self {
        self.max_update_time_of_offboarding_checklists_as("max_update_time_of_offboarding_checklists", crate::Q::offboarding_checklists().unlimited())
    }

    pub fn max_update_time_of_offboarding_checklists_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offboarding_checklists_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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
