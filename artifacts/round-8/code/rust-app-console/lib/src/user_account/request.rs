use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::UserAccount {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::UserAccount {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/user_account
#[derive(Debug)]
pub struct UserAccountRequest<R = crate::UserAccount> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for UserAccountRequest<R> {
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

impl<R> UserAccountRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("UserAccount")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> UserAccountRequest<T> {
        UserAccountRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .user_account_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .user_account_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .user_account_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for UserAccount is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .user_account_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .user_account_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::UserAccountRepository<'a>>>
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
            "account_id" => Some("account_id"),
            "status" => Some("status"),
            "version" => Some("version"),
            "employee_ref" | "employee_ref_id" => Some("employee_ref_id"),
            _ => None,
        }
    }

    fn apply_dynamic_json_chain_filter(self, head: &str, tail: &str, value: &JsonValue) -> Self {
        let _ = (tail, value);
        match head {
            "employee_ref" => {
                self.with_employee_ref_matching(
                    crate::Q::employees_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "user_role_assignment_list" => {
                self.with_user_role_assignment_list_matching(
                    crate::Q::user_role_assignments_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "magic_link_list" => {
                self.with_magic_link_list_matching(
                    crate::Q::magic_links_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "user_session_list" => {
                self.with_user_session_list_matching(
                    crate::Q::user_sessions_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "access_token_list" => {
                self.with_access_token_list_matching(
                    crate::Q::access_tokens_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "two_factor_auth_list" => {
                self.with_two_factor_auth_list_matching(
                    crate::Q::two_factor_auths_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "login_attempt_list" => {
                self.with_login_attempt_list_matching(
                    crate::Q::login_attempts_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "activity_log_list" => {
                self.with_activity_log_list_matching(
                    crate::Q::activity_logs_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "data_export_list" => {
                self.with_data_export_list_matching(
                    crate::Q::data_exports_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "notification_list" => {
                self.with_notification_list_matching(
                    crate::Q::notifications_minimal()
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
        self.query = self.query.project("account_id");
        self.query = self.query.project("status");
        self.query = self.query.project("version");
        self.query = self.query.project("employee_ref_id");
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
        request = request.select_employee_ref();
        request
    }

    pub fn select_children(self) -> Self {
        let mut request = self.select_all();
        request = request.select_user_role_assignment_list();
        request = request.select_magic_link_list();
        request = request.select_user_session_list();
        request = request.select_access_token_list();
        request = request.select_two_factor_auth_list();
        request = request.select_login_attempt_list();
        request = request.select_activity_log_list();
        request = request.select_data_export_list();
        request = request.select_notification_list();
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


    pub fn select_account_id(mut self) -> Self {
        self.query = self.query.project("account_id");
        self
    }

    pub fn project_account_id(self) -> Self {
        self.select_account_id()
    }

    pub fn select_account_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_account_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_account_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("account_id", raw_sql_segment));
        self
    }

    pub fn group_by_account_id(self) -> Self {
        self.group_by("account_id")
    }

    pub fn group_by_account_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("account_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("account_id"));
        request
    }

    pub fn group_by_account_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("account_id")
            .aggregate_with_function("account_id", alias, function)
    }

    pub fn count_account_id(self) -> Self {
        self.count_account_id_as("account_id_count")
    }

    pub fn count_account_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("account_id", alias)
    }

    pub fn sum_account_id(self) -> Self {
        self.sum_account_id_as("sum_account_id")
    }

    pub fn sum_account_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("account_id", alias)
    }

    pub fn avg_account_id(self) -> Self {
        self.avg_account_id_as("avg_account_id")
    }

    pub fn avg_account_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("account_id", alias)
    }

    pub fn min_account_id(self) -> Self {
        self.min_account_id_as("min_account_id")
    }

    pub fn min_account_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("account_id", alias)
    }

    pub fn max_account_id(self) -> Self {
        self.max_account_id_as("max_account_id")
    }

    pub fn max_account_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("account_id", alias)
    }

    pub fn unselect_account_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "account_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "account_id");
        self
    }


    pub fn with_account_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "account_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_account_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "account_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_account_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("account_id", value));
        self
    }



    pub fn with_account_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("account_id", value));
        self
    }

    pub fn with_account_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("account_id", value));
        self
    }

    pub fn with_account_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("account_id", value));
        self
    }

    pub fn with_account_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("account_id", value));
        self
    }

    pub fn with_account_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("account_id", value));
        self
    }

    pub fn with_account_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("account_id", lower, upper));
        self
    }

    pub fn with_account_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "account_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_account_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "account_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_account_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "account_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_account_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("account_id", value));
        self
    }

    pub fn with_account_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("account_id", value));
        self
    }

    pub fn with_account_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("account_id", value));
        self
    }

    pub fn with_account_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("account_id", value));
        self
    }

    pub fn with_account_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("account_id", value));
        self
    }

    pub fn with_account_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("account_id", value));
        self
    }

    pub fn with_account_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("account_id", value));
        self
    }
    pub fn with_account_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("account_id", value));
        self
    }

    pub fn with_account_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("account_id", value));
        self
    }

    pub fn with_account_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("account_id"));
        self
    }



    pub fn with_account_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("account_id"));
        self
    }


    pub fn order_by_account_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("account_id");
        self
    }

    pub fn order_by_account_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("account_id");
        self
    }

    pub fn order_by_account_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("account_id");
        self
    }

    pub fn order_by_account_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("account_id");
        self
    }


    pub fn select_status(mut self) -> Self {
        self.query = self.query.project("status");
        self
    }

    pub fn project_status(self) -> Self {
        self.select_status()
    }

    pub fn select_status_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_status_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_status_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("status", raw_sql_segment));
        self
    }

    pub fn group_by_status(self) -> Self {
        self.group_by("status")
    }

    pub fn group_by_status_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("status");
        request.query = request
            .query
            .project_expr(alias, Expr::column("status"));
        request
    }

    pub fn group_by_status_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("status")
            .aggregate_with_function("status", alias, function)
    }

    pub fn count_status(self) -> Self {
        self.count_status_as("status_count")
    }

    pub fn count_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("status", alias)
    }

    pub fn sum_status(self) -> Self {
        self.sum_status_as("sum_status")
    }

    pub fn sum_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("status", alias)
    }

    pub fn avg_status(self) -> Self {
        self.avg_status_as("avg_status")
    }

    pub fn avg_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("status", alias)
    }

    pub fn min_status(self) -> Self {
        self.min_status_as("min_status")
    }

    pub fn min_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("status", alias)
    }

    pub fn max_status(self) -> Self {
        self.max_status_as("max_status")
    }

    pub fn max_status_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("status", alias)
    }

    pub fn unselect_status(mut self) -> Self {
        self.query.projection.retain(|field| field != "status");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "status");
        self
    }


    pub fn with_status(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "status",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_status_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "status",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_status_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("status", value));
        self
    }



    pub fn with_status_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("status", value));
        self
    }

    pub fn with_status_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("status", value));
        self
    }

    pub fn with_status_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("status", value));
        self
    }

    pub fn with_status_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("status", value));
        self
    }

    pub fn with_status_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("status", value));
        self
    }

    pub fn with_status_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("status", lower, upper));
        self
    }

    pub fn with_status_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "status",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_status_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "status",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_status_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "status",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_status_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("status", value));
        self
    }

    pub fn with_status_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("status", value));
        self
    }

    pub fn with_status_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("status", value));
        self
    }

    pub fn with_status_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("status", value));
        self
    }

    pub fn with_status_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("status", value));
        self
    }

    pub fn with_status_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("status", value));
        self
    }

    pub fn with_status_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("status", value));
        self
    }
    pub fn with_status_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("status", value));
        self
    }

    pub fn with_status_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("status", value));
        self
    }

    pub fn with_status_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("status"));
        self
    }



    pub fn with_status_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("status"));
        self
    }


    pub fn order_by_status_asc(mut self) -> Self {
        self.query = self.query.order_asc("status");
        self
    }

    pub fn order_by_status_desc(mut self) -> Self {
        self.query = self.query.order_desc("status");
        self
    }

    pub fn order_by_status_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("status");
        self
    }

    pub fn order_by_status_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("status");
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
    pub fn filter_by_employee_ref(mut self, value: impl EntityReference) -> Self {
        self.query = self.query.and_filter(Expr::eq("employee_ref_id", value.entity_id_value()));
        self
    }

    pub fn with_employee_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "employee_ref_id",
            <crate::Employee as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_ref", selection));
        self
    }


    pub fn without_employee_ref_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "employee_ref_id",
            <crate::Employee as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "id",
        ));
        self.relation_filters.push(RelationFilter::new("employee_ref", selection));
        self
    }


    pub fn have_employee_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("employee_ref_id"));
        self
    }

    pub fn have_no_employee_ref(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("employee_ref_id"));
        self
    }


    pub fn group_by_employee_ref(self) -> Self {
        self.group_by("employee_ref_id")
    }

    pub fn group_by_employee_ref_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("employee_ref_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("employee_ref_id"));
        request
    }

    pub fn group_by_employee_ref_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("employee_ref_id")
            .aggregate_with_function("employee_ref_id", alias, function)
    }

    pub fn group_by_employee_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        self.query = self.query.group_by("employee_ref_id");
        self.query_options.object_group_bys.push(ObjectGroupBy::new(
            "employee_ref",
            "employee_ref_id",
            request,
        ));
        self
    }

    pub fn group_by_employee_ref_with_details(self) -> Self {
        self.group_by_employee_ref_with_details_from(crate::Q::employees().unlimited())
    }

    pub fn group_by_employee_ref_with_details_from(self, request: impl Into<QuerySelection>) -> Self {
        self.group_by_employee_ref_with(request)
    }


    pub fn roll_up_to_employee_ref(self) -> Self {
        self.roll_up_to_employee_ref_with(crate::Q::employees().unlimited())
    }

    pub fn roll_up_to_employee_ref_with(self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.with_employee_ref_matching(selection.clone())
            .group_by_employee_ref_with(selection)
    }

    pub fn count_employee_ref(self) -> Self {
        self.count_employee_ref_as("employee_ref_count")
    }

    pub fn count_employee_ref_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("employee_ref_id", alias)
    }

    pub fn unselect_employee_ref(mut self) -> Self {
        self.query.projection.retain(|field| field != "employee_ref_id");
        self.query.relations.retain(|relation| relation.name != "employee_ref");
        self
    }
    pub fn select_employee_ref(mut self) -> Self {
        self.query = self.query.relation("employee_ref");
        self
    }

    pub fn select_employee_ref_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("employee_ref", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("employee_ref", selection));
        self
}

    pub fn facet_by_employee_ref_as(self, facet_name: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        self.facet_by_employee_ref_as_with_options(facet_name, request, true)
    }

    pub fn facet_by_employee_ref_as_with_options(
        mut self,
        facet_name: impl Into<String>,
        request: impl Into<QuerySelection>,
        include_all_facets: bool,
    ) -> Self {
        self.query_options.facets.push(FacetRequest::new(
            facet_name,
            "employee_ref",
            request,
            include_all_facets,
        ));
        self
    }
    pub fn have_user_role_assignments(self) -> Self {
        self.with_user_role_assignment_list_matching(SelectQuery::new("UserRoleAssignment"))
    }

    pub fn have_no_user_role_assignments(self) -> Self {
        self.without_user_role_assignment_list_matching(SelectQuery::new("UserRoleAssignment"))
    }

    pub fn with_user_role_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::UserRoleAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("user_role_assignment_list", selection));
        self
    }

    pub fn without_user_role_assignment_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::UserRoleAssignment as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("user_role_assignment_list", selection));
        self
    }

    pub fn select_user_role_assignment_list(mut self) -> Self {
        self.query = self.query.relation("user_role_assignment_list");
        self
    }

    pub fn select_user_role_assignment_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("user_role_assignment_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("user_role_assignment_list", selection));
        self
}

    pub fn have_magic_links(self) -> Self {
        self.with_magic_link_list_matching(SelectQuery::new("MagicLink"))
    }

    pub fn have_no_magic_links(self) -> Self {
        self.without_magic_link_list_matching(SelectQuery::new("MagicLink"))
    }

    pub fn with_magic_link_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MagicLink as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("magic_link_list", selection));
        self
    }

    pub fn without_magic_link_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MagicLink as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("magic_link_list", selection));
        self
    }

    pub fn select_magic_link_list(mut self) -> Self {
        self.query = self.query.relation("magic_link_list");
        self
    }

    pub fn select_magic_link_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("magic_link_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("magic_link_list", selection));
        self
}

    pub fn have_user_sessions(self) -> Self {
        self.with_user_session_list_matching(SelectQuery::new("UserSession"))
    }

    pub fn have_no_user_sessions(self) -> Self {
        self.without_user_session_list_matching(SelectQuery::new("UserSession"))
    }

    pub fn with_user_session_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::UserSession as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("user_session_list", selection));
        self
    }

    pub fn without_user_session_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::UserSession as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("user_session_list", selection));
        self
    }

    pub fn select_user_session_list(mut self) -> Self {
        self.query = self.query.relation("user_session_list");
        self
    }

    pub fn select_user_session_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("user_session_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("user_session_list", selection));
        self
}

    pub fn have_access_tokens(self) -> Self {
        self.with_access_token_list_matching(SelectQuery::new("AccessToken"))
    }

    pub fn have_no_access_tokens(self) -> Self {
        self.without_access_token_list_matching(SelectQuery::new("AccessToken"))
    }

    pub fn with_access_token_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AccessToken as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("access_token_list", selection));
        self
    }

    pub fn without_access_token_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AccessToken as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("access_token_list", selection));
        self
    }

    pub fn select_access_token_list(mut self) -> Self {
        self.query = self.query.relation("access_token_list");
        self
    }

    pub fn select_access_token_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("access_token_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("access_token_list", selection));
        self
}

    pub fn have_two_factor_auths(self) -> Self {
        self.with_two_factor_auth_list_matching(SelectQuery::new("TwoFactorAuth"))
    }

    pub fn have_no_two_factor_auths(self) -> Self {
        self.without_two_factor_auth_list_matching(SelectQuery::new("TwoFactorAuth"))
    }

    pub fn with_two_factor_auth_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::TwoFactorAuth as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("two_factor_auth_list", selection));
        self
    }

    pub fn without_two_factor_auth_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::TwoFactorAuth as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("two_factor_auth_list", selection));
        self
    }

    pub fn select_two_factor_auth_list(mut self) -> Self {
        self.query = self.query.relation("two_factor_auth_list");
        self
    }

    pub fn select_two_factor_auth_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("two_factor_auth_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("two_factor_auth_list", selection));
        self
}

    pub fn have_login_attempts(self) -> Self {
        self.with_login_attempt_list_matching(SelectQuery::new("LoginAttempt"))
    }

    pub fn have_no_login_attempts(self) -> Self {
        self.without_login_attempt_list_matching(SelectQuery::new("LoginAttempt"))
    }

    pub fn with_login_attempt_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::LoginAttempt as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("login_attempt_list", selection));
        self
    }

    pub fn without_login_attempt_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::LoginAttempt as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("login_attempt_list", selection));
        self
    }

    pub fn select_login_attempt_list(mut self) -> Self {
        self.query = self.query.relation("login_attempt_list");
        self
    }

    pub fn select_login_attempt_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("login_attempt_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("login_attempt_list", selection));
        self
}

    pub fn have_activity_logs(self) -> Self {
        self.with_activity_log_list_matching(SelectQuery::new("ActivityLog"))
    }

    pub fn have_no_activity_logs(self) -> Self {
        self.without_activity_log_list_matching(SelectQuery::new("ActivityLog"))
    }

    pub fn with_activity_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ActivityLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("activity_log_list", selection));
        self
    }

    pub fn without_activity_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ActivityLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("activity_log_list", selection));
        self
    }

    pub fn select_activity_log_list(mut self) -> Self {
        self.query = self.query.relation("activity_log_list");
        self
    }

    pub fn select_activity_log_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("activity_log_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("activity_log_list", selection));
        self
}

    pub fn have_data_exports(self) -> Self {
        self.with_data_export_list_matching(SelectQuery::new("DataExport"))
    }

    pub fn have_no_data_exports(self) -> Self {
        self.without_data_export_list_matching(SelectQuery::new("DataExport"))
    }

    pub fn with_data_export_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::DataExport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("data_export_list", selection));
        self
    }

    pub fn without_data_export_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::DataExport as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("data_export_list", selection));
        self
    }

    pub fn select_data_export_list(mut self) -> Self {
        self.query = self.query.relation("data_export_list");
        self
    }

    pub fn select_data_export_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("data_export_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("data_export_list", selection));
        self
}

    pub fn have_notifications(self) -> Self {
        self.with_notification_list_matching(SelectQuery::new("Notification"))
    }

    pub fn have_no_notifications(self) -> Self {
        self.without_notification_list_matching(SelectQuery::new("Notification"))
    }

    pub fn with_notification_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::Notification as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("notification_list", selection));
        self
    }

    pub fn without_notification_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::Notification as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "user_account_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("notification_list", selection));
        self
    }

    pub fn select_notification_list(mut self) -> Self {
        self.query = self.query.relation("notification_list");
        self
    }

    pub fn select_notification_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("notification_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("notification_list", selection));
        self
}
    pub fn count_user_role_assignments(self) -> Self {
        self.count_user_role_assignments_as("count_user_role_assignments")
    }

    pub fn count_user_role_assignments_as(self, alias: impl Into<String>) -> Self {
        self.count_user_role_assignments_with(alias, crate::Q::user_role_assignments().unlimited())
    }

    pub fn count_user_role_assignments_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "user_role_assignment_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_user_role_assignments(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_user_role_assignments_as("refinements", request)
    }

    pub fn stats_from_user_role_assignments_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "user_role_assignment_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_user_role_assignments_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_user_role_assignments(request)
    }




    pub fn count_magic_links(self) -> Self {
        self.count_magic_links_as("count_magic_links")
    }

    pub fn count_magic_links_as(self, alias: impl Into<String>) -> Self {
        self.count_magic_links_with(alias, crate::Q::magic_links().unlimited())
    }

    pub fn count_magic_links_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "magic_link_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_magic_links(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_magic_links_as("refinements", request)
    }

    pub fn stats_from_magic_links_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "magic_link_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_magic_links_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_magic_links(request)
    }




    pub fn count_user_sessions(self) -> Self {
        self.count_user_sessions_as("count_user_sessions")
    }

    pub fn count_user_sessions_as(self, alias: impl Into<String>) -> Self {
        self.count_user_sessions_with(alias, crate::Q::user_sessions().unlimited())
    }

    pub fn count_user_sessions_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "user_session_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_user_sessions(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_user_sessions_as("refinements", request)
    }

    pub fn stats_from_user_sessions_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "user_session_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_user_sessions_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_user_sessions(request)
    }




    pub fn count_access_tokens(self) -> Self {
        self.count_access_tokens_as("count_access_tokens")
    }

    pub fn count_access_tokens_as(self, alias: impl Into<String>) -> Self {
        self.count_access_tokens_with(alias, crate::Q::access_tokens().unlimited())
    }

    pub fn count_access_tokens_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "access_token_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_access_tokens(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_access_tokens_as("refinements", request)
    }

    pub fn stats_from_access_tokens_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "access_token_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_access_tokens_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_access_tokens(request)
    }




    pub fn count_two_factor_auths(self) -> Self {
        self.count_two_factor_auths_as("count_two_factor_auths")
    }

    pub fn count_two_factor_auths_as(self, alias: impl Into<String>) -> Self {
        self.count_two_factor_auths_with(alias, crate::Q::two_factor_auths().unlimited())
    }

    pub fn count_two_factor_auths_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "two_factor_auth_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_two_factor_auths(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_two_factor_auths_as("refinements", request)
    }

    pub fn stats_from_two_factor_auths_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "two_factor_auth_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_two_factor_auths_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_two_factor_auths(request)
    }




    pub fn count_login_attempts(self) -> Self {
        self.count_login_attempts_as("count_login_attempts")
    }

    pub fn count_login_attempts_as(self, alias: impl Into<String>) -> Self {
        self.count_login_attempts_with(alias, crate::Q::login_attempts().unlimited())
    }

    pub fn count_login_attempts_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "login_attempt_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_login_attempts(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_login_attempts_as("refinements", request)
    }

    pub fn stats_from_login_attempts_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "login_attempt_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_login_attempts_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_login_attempts(request)
    }




    pub fn count_activity_logs(self) -> Self {
        self.count_activity_logs_as("count_activity_logs")
    }

    pub fn count_activity_logs_as(self, alias: impl Into<String>) -> Self {
        self.count_activity_logs_with(alias, crate::Q::activity_logs().unlimited())
    }

    pub fn count_activity_logs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "activity_log_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_activity_logs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_activity_logs_as("refinements", request)
    }

    pub fn stats_from_activity_logs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "activity_log_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_activity_logs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_activity_logs(request)
    }




    pub fn count_data_exports(self) -> Self {
        self.count_data_exports_as("count_data_exports")
    }

    pub fn count_data_exports_as(self, alias: impl Into<String>) -> Self {
        self.count_data_exports_with(alias, crate::Q::data_exports().unlimited())
    }

    pub fn count_data_exports_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "data_export_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_data_exports(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_data_exports_as("refinements", request)
    }

    pub fn stats_from_data_exports_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "data_export_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_data_exports_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_data_exports(request)
    }




    pub fn count_notifications(self) -> Self {
        self.count_notifications_as("count_notifications")
    }

    pub fn count_notifications_as(self, alias: impl Into<String>) -> Self {
        self.count_notifications_with(alias, crate::Q::notifications().unlimited())
    }

    pub fn count_notifications_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "notification_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_notifications(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_notifications_as("refinements", request)
    }

    pub fn stats_from_notifications_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "notification_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_notifications_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_notifications(request)
    }



}

impl<R> Default for UserAccountRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< UserAccountRequest<R> > for SelectQuery {
    fn from(request: UserAccountRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< UserAccountRequest<R> > for QuerySelection {
    fn from(request: UserAccountRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::UserAccount> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::UserAccountRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<UserAccountRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::UserAccount
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::UserAccount::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> UserAccountRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
