use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Merchant {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Merchant {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/merchant
#[derive(Debug)]
pub struct MerchantRequest<R = crate::Merchant> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for MerchantRequest<R> {
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

impl<R> MerchantRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Merchant")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> MerchantRequest<T> {
        MerchantRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .merchant_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Merchant is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .merchant_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::MerchantRepository<'a>>>
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
            "name" => Some("name"),
            "tax_number" => Some("tax_number"),
            "address" => Some("address"),
            "external_id" => Some("external_id"),
            "create_time" => Some("create_time"),
            "update_time" => Some("update_time"),
            "version" => Some("version"),
            "platform" | "platform_id" => Some("platform_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "platform" => {
                self.with_platform_matching(
                    crate::Q::platforms_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "department_list" => {
                self.with_department_list_matching(
                    crate::Q::departments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "position_list" => {
                self.with_position_list_matching(
                    crate::Q::positions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "employee_list" => {
                self.with_employee_list_matching(
                    crate::Q::employees_minimal()
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
            "payroll_run_list" => {
                self.with_payroll_run_list_matching(
                    crate::Q::payroll_runs_minimal()
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
            "recruitment_post_list" => {
                self.with_recruitment_post_list_matching(
                    crate::Q::recruitment_posts_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "job_application_list" => {
                self.with_job_application_list_matching(
                    crate::Q::job_applications_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "interview_list" => {
                self.with_interview_list_matching(
                    crate::Q::interviews_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "offer_letter_list" => {
                self.with_offer_letter_list_matching(
                    crate::Q::offer_letters_minimal()
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
        self.query = self.query.project("name");
        self.query = self.query.project("tax_number");
        self.query = self.query.project("address");
        self.query = self.query.project("external_id");
        self.query = self.query.project("create_time");
        self.query = self.query.project("update_time");
        self.query = self.query.project("version");
        self.query = self.query.project("platform_id");
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
        request = request.select_platform();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_department_list();
        request = request.select_position_list();
        request = request.select_employee_list();
        request = request.select_salary_record_list();
        request = request.select_attendance_log_list();
        request = request.select_leave_request_list();
        request = request.select_performance_review_list();
        request = request.select_training_record_list();
        request = request.select_benefit_plan_list();
        request = request.select_expense_claim_list();
        request = request.select_payroll_run_list();
        request = request.select_tax_form_list();
        request = request.select_contract_list();
        request = request.select_resignation_list();
        request = request.select_warning_letter_list();
        request = request.select_bonus_record_list();
        request = request.select_shift_schedule_list();
        request = request.select_time_off_balance_list();
        request = request.select_recruitment_post_list();
        request = request.select_job_application_list();
        request = request.select_interview_list();
        request = request.select_offer_letter_list();
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


    pub fn select_tax_number(mut self) -> Self {
        self.query = self.query.project("tax_number");
        self
    }

    pub fn project_tax_number(self) -> Self {
        self.select_tax_number()
    }

    pub fn select_tax_number_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_tax_number_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_tax_number_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("tax_number", raw_sql_segment));
        self
    }

    pub fn group_by_tax_number(self) -> Self {
        self.group_by("tax_number")
    }

    pub fn group_by_tax_number_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("tax_number");
        request.query = request
            .query
            .project_expr(alias, Expr::column("tax_number"));
        request
    }

    pub fn group_by_tax_number_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("tax_number")
            .aggregate_with_function("tax_number", alias, function)
    }

    pub fn count_tax_number(self) -> Self {
        self.count_tax_number_as("tax_number_count")
    }

    pub fn count_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("tax_number", alias)
    }

    pub fn sum_tax_number(self) -> Self {
        self.sum_tax_number_as("sum_tax_number")
    }

    pub fn sum_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("tax_number", alias)
    }

    pub fn avg_tax_number(self) -> Self {
        self.avg_tax_number_as("avg_tax_number")
    }

    pub fn avg_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("tax_number", alias)
    }

    pub fn min_tax_number(self) -> Self {
        self.min_tax_number_as("min_tax_number")
    }

    pub fn min_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("tax_number", alias)
    }

    pub fn max_tax_number(self) -> Self {
        self.max_tax_number_as("max_tax_number")
    }

    pub fn max_tax_number_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("tax_number", alias)
    }

    pub fn unselect_tax_number(mut self) -> Self {
        self.query.projection.retain(|field| field != "tax_number");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "tax_number");
        self
    }


    pub fn with_tax_number(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "tax_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_tax_number_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "tax_number",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_tax_number_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("tax_number", value));
        self
    }



    pub fn with_tax_number_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("tax_number", value));
        self
    }

    pub fn with_tax_number_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tax_number", value));
        self
    }

    pub fn with_tax_number_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("tax_number", value));
        self
    }

    pub fn with_tax_number_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tax_number", value));
        self
    }

    pub fn with_tax_number_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("tax_number", value));
        self
    }

    pub fn with_tax_number_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("tax_number", lower, upper));
        self
    }

    pub fn with_tax_number_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "tax_number",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_tax_number_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "tax_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tax_number_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "tax_number",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_tax_number_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("tax_number", value));
        self
    }

    pub fn with_tax_number_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("tax_number", value));
        self
    }

    pub fn with_tax_number_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("tax_number", value));
        self
    }

    pub fn with_tax_number_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("tax_number", value));
        self
    }

    pub fn with_tax_number_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("tax_number", value));
        self
    }

    pub fn with_tax_number_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("tax_number", value));
        self
    }

    pub fn with_tax_number_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("tax_number", value));
        self
    }
    pub fn with_tax_number_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("tax_number", value));
        self
    }

    pub fn with_tax_number_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("tax_number", value));
        self
    }

    pub fn with_tax_number_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("tax_number"));
        self
    }



    pub fn with_tax_number_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("tax_number"));
        self
    }


    pub fn order_by_tax_number_asc(mut self) -> Self {
        self.query = self.query.order_asc("tax_number");
        self
    }

    pub fn order_by_tax_number_desc(mut self) -> Self {
        self.query = self.query.order_desc("tax_number");
        self
    }

    pub fn order_by_tax_number_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("tax_number");
        self
    }

    pub fn order_by_tax_number_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("tax_number");
        self
    }


    pub fn select_address(mut self) -> Self {
        self.query = self.query.project("address");
        self
    }

    pub fn project_address(self) -> Self {
        self.select_address()
    }

    pub fn select_address_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_address_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_address_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("address", raw_sql_segment));
        self
    }

    pub fn group_by_address(self) -> Self {
        self.group_by("address")
    }

    pub fn group_by_address_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("address");
        request.query = request
            .query
            .project_expr(alias, Expr::column("address"));
        request
    }

    pub fn group_by_address_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("address")
            .aggregate_with_function("address", alias, function)
    }

    pub fn count_address(self) -> Self {
        self.count_address_as("address_count")
    }

    pub fn count_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("address", alias)
    }

    pub fn sum_address(self) -> Self {
        self.sum_address_as("sum_address")
    }

    pub fn sum_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("address", alias)
    }

    pub fn avg_address(self) -> Self {
        self.avg_address_as("avg_address")
    }

    pub fn avg_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("address", alias)
    }

    pub fn min_address(self) -> Self {
        self.min_address_as("min_address")
    }

    pub fn min_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("address", alias)
    }

    pub fn max_address(self) -> Self {
        self.max_address_as("max_address")
    }

    pub fn max_address_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("address", alias)
    }

    pub fn unselect_address(mut self) -> Self {
        self.query.projection.retain(|field| field != "address");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "address");
        self
    }


    pub fn with_address(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "address",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_address_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "address",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_address_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("address", value));
        self
    }



    pub fn with_address_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("address", value));
        self
    }

    pub fn with_address_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("address", value));
        self
    }

    pub fn with_address_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("address", value));
        self
    }

    pub fn with_address_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("address", value));
        self
    }

    pub fn with_address_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("address", value));
        self
    }

    pub fn with_address_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("address", lower, upper));
        self
    }

    pub fn with_address_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "address",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_address_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "address",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_address_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "address",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_address_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("address", value));
        self
    }

    pub fn with_address_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("address", value));
        self
    }

    pub fn with_address_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("address", value));
        self
    }

    pub fn with_address_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("address", value));
        self
    }

    pub fn with_address_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("address", value));
        self
    }

    pub fn with_address_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("address", value));
        self
    }

    pub fn with_address_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("address", value));
        self
    }
    pub fn with_address_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("address", value));
        self
    }

    pub fn with_address_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("address", value));
        self
    }

    pub fn with_address_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("address"));
        self
    }



    pub fn with_address_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("address"));
        self
    }


    pub fn order_by_address_asc(mut self) -> Self {
        self.query = self.query.order_asc("address");
        self
    }

    pub fn order_by_address_desc(mut self) -> Self {
        self.query = self.query.order_desc("address");
        self
    }

    pub fn order_by_address_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("address");
        self
    }

    pub fn order_by_address_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("address");
        self
    }


    pub fn select_external_id(mut self) -> Self {
        self.query = self.query.project("external_id");
        self
    }

    pub fn project_external_id(self) -> Self {
        self.select_external_id()
    }

    pub fn select_external_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_external_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_external_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("external_id", raw_sql_segment));
        self
    }

    pub fn group_by_external_id(self) -> Self {
        self.group_by("external_id")
    }

    pub fn group_by_external_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("external_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("external_id"));
        request
    }

    pub fn group_by_external_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("external_id")
            .aggregate_with_function("external_id", alias, function)
    }

    pub fn count_external_id(self) -> Self {
        self.count_external_id_as("external_id_count")
    }

    pub fn count_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("external_id", alias)
    }

    pub fn sum_external_id(self) -> Self {
        self.sum_external_id_as("sum_external_id")
    }

    pub fn sum_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("external_id", alias)
    }

    pub fn avg_external_id(self) -> Self {
        self.avg_external_id_as("avg_external_id")
    }

    pub fn avg_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("external_id", alias)
    }

    pub fn min_external_id(self) -> Self {
        self.min_external_id_as("min_external_id")
    }

    pub fn min_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("external_id", alias)
    }

    pub fn max_external_id(self) -> Self {
        self.max_external_id_as("max_external_id")
    }

    pub fn max_external_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("external_id", alias)
    }

    pub fn unselect_external_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "external_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "external_id");
        self
    }


    pub fn with_external_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "external_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_external_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "external_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_external_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("external_id", value));
        self
    }



    pub fn with_external_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("external_id", value));
        self
    }

    pub fn with_external_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("external_id", value));
        self
    }

    pub fn with_external_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("external_id", value));
        self
    }

    pub fn with_external_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("external_id", value));
        self
    }

    pub fn with_external_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("external_id", value));
        self
    }

    pub fn with_external_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("external_id", lower, upper));
        self
    }

    pub fn with_external_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "external_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_external_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "external_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_external_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "external_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_external_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("external_id", value));
        self
    }

    pub fn with_external_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("external_id", value));
        self
    }

    pub fn with_external_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("external_id", value));
        self
    }

    pub fn with_external_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("external_id", value));
        self
    }

    pub fn with_external_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("external_id", value));
        self
    }

    pub fn with_external_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("external_id", value));
        self
    }

    pub fn with_external_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("external_id", value));
        self
    }
    pub fn with_external_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("external_id", value));
        self
    }

    pub fn with_external_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("external_id", value));
        self
    }

    pub fn with_external_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("external_id"));
        self
    }



    pub fn with_external_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("external_id"));
        self
    }


    pub fn order_by_external_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("external_id");
        self
    }

    pub fn order_by_external_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("external_id");
        self
    }

    pub fn order_by_external_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("external_id");
        self
    }

    pub fn order_by_external_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("external_id");
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
    pub fn filter_by_platform(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("platform_id", value.entity_id_value()));
        self
    }

    pub fn with_platform_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "platform_id",
            <crate::Platform as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("platform", selection));
        self
    }


    pub fn without_platform_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "platform_id",
            <crate::Platform as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("platform", selection));
        self
    }


    pub fn have_platform(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("platform_id"));
        self
    }

    pub fn have_no_platform(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("platform_id"));
        self
    }


    pub fn group_by_platform(self) -> Self {
        self.group_by("platform_id")
    }

    pub fn group_by_platform_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("platform_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("platform_id"));
        request
    }

    pub fn group_by_platform_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("platform_id")
            .aggregate_with_function("platform_id", alias, function)
    }

    pub fn group_by_platform_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("platform_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "platform",
            "platform_id",
            request,
        ));
        self
    }

    pub fn group_by_platform_with_details(self) -> Self {
        self.group_by_platform_with_details_from(crate::Q::platforms().unlimited())
    }

    pub fn group_by_platform_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_platform_with(request)
    }


    pub fn roll_up_to_platform(self) -> Self {
        self.roll_up_to_platform_with(crate::Q::platforms().unlimited())
    }

    pub fn roll_up_to_platform_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_platform_matching(selection.clone())
            .group_by_platform_with(selection)
    }

    pub fn count_platform(self) -> Self {
        self.count_platform_as("platform_count")
    }

    pub fn count_platform_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("platform_id", alias)
    }

    pub fn unselect_platform(mut self) -> Self {
        self.query.projection.retain(|field| field != "platform_id");
        self.query.relations.retain(|relation| relation.name != "platform");
        self
    }
    pub fn select_platform(mut self) -> Self {
        self.query = self.query.relation("platform");
        self
    }

    pub fn select_platform_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("platform", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("platform", selection));
        self
}

    pub fn facet_by_platform_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_platform_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_platform_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "platform",
            request,
            include_all_facets,
        ));
        self
    }
    pub fn have_departments(self) -> Self {
        self.with_department_list_matching(SelectQuery::new("Department"))
    }

    pub fn have_no_departments(self) -> Self {
        self.without_department_list_matching(SelectQuery::new("Department"))
    }

    pub fn with_department_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Department as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("department_list", selection));
        self
    }

    pub fn without_department_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Department as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("department_list", selection));
        self
    }

    pub fn select_department_list(mut self) -> Self {
        self.query = self.query.relation("department_list");
        self
    }

    pub fn select_department_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("department_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("department_list", selection));
        self
}

    pub fn have_positions(self) -> Self {
        self.with_position_list_matching(SelectQuery::new("Position"))
    }

    pub fn have_no_positions(self) -> Self {
        self.without_position_list_matching(SelectQuery::new("Position"))
    }

    pub fn with_position_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Position as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("position_list", selection));
        self
    }

    pub fn without_position_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Position as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("position_list", selection));
        self
    }

    pub fn select_position_list(mut self) -> Self {
        self.query = self.query.relation("position_list");
        self
    }

    pub fn select_position_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("position_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("position_list", selection));
        self
}

    pub fn have_employees(self) -> Self {
        self.with_employee_list_matching(SelectQuery::new("Employee"))
    }

    pub fn have_no_employees(self) -> Self {
        self.without_employee_list_matching(SelectQuery::new("Employee"))
    }

    pub fn with_employee_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Employee as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_list", selection));
        self
    }

    pub fn without_employee_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Employee as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_list", selection));
        self
    }

    pub fn select_employee_list(mut self) -> Self {
        self.query = self.query.relation("employee_list");
        self
    }

    pub fn select_employee_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("employee_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("employee_list", selection));
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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

    pub fn have_payroll_runs(self) -> Self {
        self.with_payroll_run_list_matching(SelectQuery::new("PayrollRun"))
    }

    pub fn have_no_payroll_runs(self) -> Self {
        self.without_payroll_run_list_matching(SelectQuery::new("PayrollRun"))
    }

    pub fn with_payroll_run_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PayrollRun as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_run_list", selection));
        self
    }

    pub fn without_payroll_run_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PayrollRun as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("payroll_run_list", selection));
        self
    }

    pub fn select_payroll_run_list(mut self) -> Self {
        self.query = self.query.relation("payroll_run_list");
        self
    }

    pub fn select_payroll_run_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("payroll_run_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("payroll_run_list", selection));
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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

    pub fn have_recruitment_posts(self) -> Self {
        self.with_recruitment_post_list_matching(SelectQuery::new("RecruitmentPost"))
    }

    pub fn have_no_recruitment_posts(self) -> Self {
        self.without_recruitment_post_list_matching(SelectQuery::new("RecruitmentPost"))
    }

    pub fn with_recruitment_post_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::RecruitmentPost as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("recruitment_post_list", selection));
        self
    }

    pub fn without_recruitment_post_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::RecruitmentPost as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("recruitment_post_list", selection));
        self
    }

    pub fn select_recruitment_post_list(mut self) -> Self {
        self.query = self.query.relation("recruitment_post_list");
        self
    }

    pub fn select_recruitment_post_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("recruitment_post_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("recruitment_post_list", selection));
        self
}

    pub fn have_job_applications(self) -> Self {
        self.with_job_application_list_matching(SelectQuery::new("JobApplication"))
    }

    pub fn have_no_job_applications(self) -> Self {
        self.without_job_application_list_matching(SelectQuery::new("JobApplication"))
    }

    pub fn with_job_application_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::JobApplication as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("job_application_list", selection));
        self
    }

    pub fn without_job_application_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::JobApplication as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("job_application_list", selection));
        self
    }

    pub fn select_job_application_list(mut self) -> Self {
        self.query = self.query.relation("job_application_list");
        self
    }

    pub fn select_job_application_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("job_application_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("job_application_list", selection));
        self
}

    pub fn have_interviews(self) -> Self {
        self.with_interview_list_matching(SelectQuery::new("Interview"))
    }

    pub fn have_no_interviews(self) -> Self {
        self.without_interview_list_matching(SelectQuery::new("Interview"))
    }

    pub fn with_interview_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Interview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("interview_list", selection));
        self
    }

    pub fn without_interview_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Interview as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("interview_list", selection));
        self
    }

    pub fn select_interview_list(mut self) -> Self {
        self.query = self.query.relation("interview_list");
        self
    }

    pub fn select_interview_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("interview_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("interview_list", selection));
        self
}

    pub fn have_offer_letters(self) -> Self {
        self.with_offer_letter_list_matching(SelectQuery::new("OfferLetter"))
    }

    pub fn have_no_offer_letters(self) -> Self {
        self.without_offer_letter_list_matching(SelectQuery::new("OfferLetter"))
    }

    pub fn with_offer_letter_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::OfferLetter as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("offer_letter_list", selection));
        self
    }

    pub fn without_offer_letter_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::OfferLetter as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "merchant_id",
        ));
        self.relation_filters.push(RelationFilter::new("offer_letter_list", selection));
        self
    }

    pub fn select_offer_letter_list(mut self) -> Self {
        self.query = self.query.relation("offer_letter_list");
        self
    }

    pub fn select_offer_letter_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("offer_letter_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("offer_letter_list", selection));
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
            "merchant_id",
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
    pub fn count_departments(self) -> Self {
        self.count_departments_as("count_departments")
    }

    pub fn count_departments_as(self, alias: impl Into<String>) -> Self {
        self.count_departments_with(alias, crate::Q::departments().unlimited())
    }

    pub fn count_departments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "department_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_departments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments_as("refinements", request)
    }

    pub fn stats_from_departments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "department_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_departments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments(request)
    }


    pub fn min_create_time_of_departments(self) -> Self {
        self.min_create_time_of_departments_as("min_create_time_of_departments", crate::Q::departments().unlimited())
    }

    pub fn min_create_time_of_departments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_departments(self) -> Self {
        self.max_create_time_of_departments_as("max_create_time_of_departments", crate::Q::departments().unlimited())
    }

    pub fn max_create_time_of_departments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_departments(self) -> Self {
        self.min_update_time_of_departments_as("min_update_time_of_departments", crate::Q::departments().unlimited())
    }

    pub fn min_update_time_of_departments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_departments(self) -> Self {
        self.max_update_time_of_departments_as("max_update_time_of_departments", crate::Q::departments().unlimited())
    }

    pub fn max_update_time_of_departments_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_departments_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_positions(self) -> Self {
        self.count_positions_as("count_positions")
    }

    pub fn count_positions_as(self, alias: impl Into<String>) -> Self {
        self.count_positions_with(alias, crate::Q::positions().unlimited())
    }

    pub fn count_positions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "position_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_positions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_positions_as("refinements", request)
    }

    pub fn stats_from_positions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "position_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_positions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_positions(request)
    }


    pub fn min_create_time_of_positions(self) -> Self {
        self.min_create_time_of_positions_as("min_create_time_of_positions", crate::Q::positions().unlimited())
    }

    pub fn min_create_time_of_positions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_positions_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_positions(self) -> Self {
        self.max_create_time_of_positions_as("max_create_time_of_positions", crate::Q::positions().unlimited())
    }

    pub fn max_create_time_of_positions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_positions_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_positions(self) -> Self {
        self.min_update_time_of_positions_as("min_update_time_of_positions", crate::Q::positions().unlimited())
    }

    pub fn min_update_time_of_positions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_positions_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_positions(self) -> Self {
        self.max_update_time_of_positions_as("max_update_time_of_positions", crate::Q::positions().unlimited())
    }

    pub fn max_update_time_of_positions_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_positions_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_employees(self) -> Self {
        self.count_employees_as("count_employees")
    }

    pub fn count_employees_as(self, alias: impl Into<String>) -> Self {
        self.count_employees_with(alias, crate::Q::employees().unlimited())
    }

    pub fn count_employees_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "employee_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_employees(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees_as("refinements", request)
    }

    pub fn stats_from_employees_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "employee_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_employees_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees(request)
    }


    pub fn min_create_time_of_employees(self) -> Self {
        self.min_create_time_of_employees_as("min_create_time_of_employees", crate::Q::employees().unlimited())
    }

    pub fn min_create_time_of_employees_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_employees(self) -> Self {
        self.max_create_time_of_employees_as("max_create_time_of_employees", crate::Q::employees().unlimited())
    }

    pub fn max_create_time_of_employees_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_employees(self) -> Self {
        self.min_update_time_of_employees_as("min_update_time_of_employees", crate::Q::employees().unlimited())
    }

    pub fn min_update_time_of_employees_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_employees(self) -> Self {
        self.max_update_time_of_employees_as("max_update_time_of_employees", crate::Q::employees().unlimited())
    }

    pub fn max_update_time_of_employees_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_employees_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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

    pub fn count_payroll_runs(self) -> Self {
        self.count_payroll_runs_as("count_payroll_runs")
    }

    pub fn count_payroll_runs_as(self, alias: impl Into<String>) -> Self {
        self.count_payroll_runs_with(alias, crate::Q::payroll_runs().unlimited())
    }

    pub fn count_payroll_runs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payroll_run_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_payroll_runs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_runs_as("refinements", request)
    }

    pub fn stats_from_payroll_runs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "payroll_run_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_payroll_runs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_runs(request)
    }


    pub fn min_create_time_of_payroll_runs(self) -> Self {
        self.min_create_time_of_payroll_runs_as("min_create_time_of_payroll_runs", crate::Q::payroll_runs().unlimited())
    }

    pub fn min_create_time_of_payroll_runs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_runs_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_payroll_runs(self) -> Self {
        self.max_create_time_of_payroll_runs_as("max_create_time_of_payroll_runs", crate::Q::payroll_runs().unlimited())
    }

    pub fn max_create_time_of_payroll_runs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_runs_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_payroll_runs(self) -> Self {
        self.min_update_time_of_payroll_runs_as("min_update_time_of_payroll_runs", crate::Q::payroll_runs().unlimited())
    }

    pub fn min_update_time_of_payroll_runs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_runs_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_payroll_runs(self) -> Self {
        self.max_update_time_of_payroll_runs_as("max_update_time_of_payroll_runs", crate::Q::payroll_runs().unlimited())
    }

    pub fn max_update_time_of_payroll_runs_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_payroll_runs_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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

    pub fn count_recruitment_posts(self) -> Self {
        self.count_recruitment_posts_as("count_recruitment_posts")
    }

    pub fn count_recruitment_posts_as(self, alias: impl Into<String>) -> Self {
        self.count_recruitment_posts_with(alias, crate::Q::recruitment_posts().unlimited())
    }

    pub fn count_recruitment_posts_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "recruitment_post_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_recruitment_posts(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_recruitment_posts_as("refinements", request)
    }

    pub fn stats_from_recruitment_posts_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "recruitment_post_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_recruitment_posts_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_recruitment_posts(request)
    }


    pub fn min_create_time_of_recruitment_posts(self) -> Self {
        self.min_create_time_of_recruitment_posts_as("min_create_time_of_recruitment_posts", crate::Q::recruitment_posts().unlimited())
    }

    pub fn min_create_time_of_recruitment_posts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_recruitment_posts_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_recruitment_posts(self) -> Self {
        self.max_create_time_of_recruitment_posts_as("max_create_time_of_recruitment_posts", crate::Q::recruitment_posts().unlimited())
    }

    pub fn max_create_time_of_recruitment_posts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_recruitment_posts_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_recruitment_posts(self) -> Self {
        self.min_update_time_of_recruitment_posts_as("min_update_time_of_recruitment_posts", crate::Q::recruitment_posts().unlimited())
    }

    pub fn min_update_time_of_recruitment_posts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_recruitment_posts_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_recruitment_posts(self) -> Self {
        self.max_update_time_of_recruitment_posts_as("max_update_time_of_recruitment_posts", crate::Q::recruitment_posts().unlimited())
    }

    pub fn max_update_time_of_recruitment_posts_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_recruitment_posts_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_job_applications(self) -> Self {
        self.count_job_applications_as("count_job_applications")
    }

    pub fn count_job_applications_as(self, alias: impl Into<String>) -> Self {
        self.count_job_applications_with(alias, crate::Q::job_applications().unlimited())
    }

    pub fn count_job_applications_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "job_application_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_job_applications(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_applications_as("refinements", request)
    }

    pub fn stats_from_job_applications_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "job_application_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_job_applications_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_applications(request)
    }


    pub fn min_create_time_of_job_applications(self) -> Self {
        self.min_create_time_of_job_applications_as("min_create_time_of_job_applications", crate::Q::job_applications().unlimited())
    }

    pub fn min_create_time_of_job_applications_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_applications_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_job_applications(self) -> Self {
        self.max_create_time_of_job_applications_as("max_create_time_of_job_applications", crate::Q::job_applications().unlimited())
    }

    pub fn max_create_time_of_job_applications_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_applications_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_job_applications(self) -> Self {
        self.min_update_time_of_job_applications_as("min_update_time_of_job_applications", crate::Q::job_applications().unlimited())
    }

    pub fn min_update_time_of_job_applications_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_applications_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_job_applications(self) -> Self {
        self.max_update_time_of_job_applications_as("max_update_time_of_job_applications", crate::Q::job_applications().unlimited())
    }

    pub fn max_update_time_of_job_applications_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_job_applications_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_interviews(self) -> Self {
        self.count_interviews_as("count_interviews")
    }

    pub fn count_interviews_as(self, alias: impl Into<String>) -> Self {
        self.count_interviews_with(alias, crate::Q::interviews().unlimited())
    }

    pub fn count_interviews_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "interview_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_interviews(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_interviews_as("refinements", request)
    }

    pub fn stats_from_interviews_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "interview_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_interviews_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_interviews(request)
    }


    pub fn min_create_time_of_interviews(self) -> Self {
        self.min_create_time_of_interviews_as("min_create_time_of_interviews", crate::Q::interviews().unlimited())
    }

    pub fn min_create_time_of_interviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_interviews_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_interviews(self) -> Self {
        self.max_create_time_of_interviews_as("max_create_time_of_interviews", crate::Q::interviews().unlimited())
    }

    pub fn max_create_time_of_interviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_interviews_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_interviews(self) -> Self {
        self.min_update_time_of_interviews_as("min_update_time_of_interviews", crate::Q::interviews().unlimited())
    }

    pub fn min_update_time_of_interviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_interviews_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_interviews(self) -> Self {
        self.max_update_time_of_interviews_as("max_update_time_of_interviews", crate::Q::interviews().unlimited())
    }

    pub fn max_update_time_of_interviews_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_interviews_as(alias, request.into().into_query().max("update_time", "max_update_time"))
    }

    pub fn count_offer_letters(self) -> Self {
        self.count_offer_letters_as("count_offer_letters")
    }

    pub fn count_offer_letters_as(self, alias: impl Into<String>) -> Self {
        self.count_offer_letters_with(alias, crate::Q::offer_letters().unlimited())
    }

    pub fn count_offer_letters_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "offer_letter_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_offer_letters(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offer_letters_as("refinements", request)
    }

    pub fn stats_from_offer_letters_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "offer_letter_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_offer_letters_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offer_letters(request)
    }


    pub fn min_create_time_of_offer_letters(self) -> Self {
        self.min_create_time_of_offer_letters_as("min_create_time_of_offer_letters", crate::Q::offer_letters().unlimited())
    }

    pub fn min_create_time_of_offer_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offer_letters_as(alias, request.into().into_query().min("create_time", "min_create_time"))
    }
    pub fn max_create_time_of_offer_letters(self) -> Self {
        self.max_create_time_of_offer_letters_as("max_create_time_of_offer_letters", crate::Q::offer_letters().unlimited())
    }

    pub fn max_create_time_of_offer_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offer_letters_as(alias, request.into().into_query().max("create_time", "max_create_time"))
    }
    pub fn min_update_time_of_offer_letters(self) -> Self {
        self.min_update_time_of_offer_letters_as("min_update_time_of_offer_letters", crate::Q::offer_letters().unlimited())
    }

    pub fn min_update_time_of_offer_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offer_letters_as(alias, request.into().into_query().min("update_time", "min_update_time"))
    }
    pub fn max_update_time_of_offer_letters(self) -> Self {
        self.max_update_time_of_offer_letters_as("max_update_time_of_offer_letters", crate::Q::offer_letters().unlimited())
    }

    pub fn max_update_time_of_offer_letters_as(self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_offer_letters_as(alias, request.into().into_query().max("update_time", "max_update_time"))
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

impl<R> Default for MerchantRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< MerchantRequest<R> > for SelectQuery {
    fn from(request: MerchantRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< MerchantRequest<R> > for QuerySelection {
    fn from(request: MerchantRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Merchant> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::MerchantRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<MerchantRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Merchant
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Merchant::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> MerchantRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::MerchantRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
