use std::marker::PhantomData;

use serde_json::Value as JsonValue;
use teaql_core::{Aggregate, AggregateFunction, EntityDescriptor, Expr, Record, SelectQuery, SmartList};
use teaql_runtime::{DataServiceError, RuntimeError};

use crate::request_support::*;

impl EntityReference for crate::Customer {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(&self)
    }
}

impl EntityReference for &crate::Customer {
    fn entity_id_value(self) -> teaql_core::Value {
        teaql_core::IdentifiableEntity::id_value(self)
    }
}

// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/customer
#[derive(Debug)]
pub struct CustomerRequest<R = crate::Customer> {
    query: SelectQuery,
    relation_selections: Vec<RelationSelection>,
    relation_filters: Vec<RelationFilter>,
    child_enhancements: Vec<QuerySelection>,
    query_options: QueryOptions,
    marker: PhantomData<R>,
}

impl<R> Clone for CustomerRequest<R> {
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

impl<R> CustomerRequest<R> {
    pub(crate) fn new() -> Self {
        Self {
            query: SelectQuery::new("Customer")
                .project("id")
                .project("version"),
            relation_selections: Vec::new(),
            relation_filters: Vec::new(),
            child_enhancements: Vec::new(),
            query_options: QueryOptions::default(),
            marker: PhantomData,
        }
    }

    pub fn return_type<T>(self) -> CustomerRequest<T> {
        CustomerRequest {
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
        R: teaql_core::Entity,
    {
        let repository = ctx
            .customer_repository()
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
    ) -> Result<Vec<teaql_data_service::StreamChunk>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
    ) -> Result<Option<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
    ) -> Result<SmartList<R>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
    ) -> Result<u64, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
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
            .ok_or_else(|| DataServiceError::Runtime(RuntimeError::Graph(format!("count result for Customer is missing or not numeric"))))
    }

    pub(crate) async fn _execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
            .map_err(|err| DataServiceError::Runtime(RuntimeError::Graph(err.to_string())))?;
        let mut query = self.query.limit(1);
        query.relations.clear();
        let rows = repository.fetch_all(&query).await?;
        Ok(!rows.is_empty())
    }

    pub(crate) async fn _execute_for_records<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<SmartList<Record>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: TeaqlRepositoryProvider + ?Sized,
    {
        let repository = ctx
            .customer_repository()
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
    ) -> Result<Option<Record>, TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
            "customer_id" => Some("customer_id"),
            "status" => Some("status"),
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
            "move_order_list" => {
                self.with_move_order_list_matching(
                    crate::Q::move_orders_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "private_customer_profile_list" => {
                self.with_private_customer_profile_list_matching(
                    crate::Q::private_customer_profiles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "corporate_customer_profile_list" => {
                self.with_corporate_customer_profile_list_matching(
                    crate::Q::corporate_customer_profiles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_contact_list" => {
                self.with_customer_contact_list_matching(
                    crate::Q::customer_contacts_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "billing_profile_list" => {
                self.with_billing_profile_list_matching(
                    crate::Q::billing_profiles_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_history_list" => {
                self.with_customer_history_list_matching(
                    crate::Q::customer_histories_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_preference_list" => {
                self.with_customer_preference_list_matching(
                    crate::Q::customer_preferences_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_consent_list" => {
                self.with_customer_consent_list_matching(
                    crate::Q::customer_consents_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "customer_feedback_list" => {
                self.with_customer_feedback_list_matching(
                    crate::Q::customer_feedback_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "loyalty_tier_list" => {
                self.with_loyalty_tier_list_matching(
                    crate::Q::loyalty_tiers_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "referral_code_list" => {
                self.with_referral_code_list_matching(
                    crate::Q::referral_codes_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "communication_log_list" => {
                self.with_communication_log_list_matching(
                    crate::Q::communication_logs_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "service_rating_list" => {
                self.with_service_rating_list_matching(
                    crate::Q::service_ratings_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "account_status_list" => {
                self.with_account_status_list_matching(
                    crate::Q::account_statuses_minimal()
                        .apply_dynamic_json_filter(tail, value),
                )
            }
            "contact_method_list" => {
                self.with_contact_method_list_matching(
                    crate::Q::contact_methods_minimal()
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
        self.query = self.query.project("customer_id");
        self.query = self.query.project("status");
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
        request = request.select_move_order_list();
        request = request.select_private_customer_profile_list();
        request = request.select_corporate_customer_profile_list();
        request = request.select_customer_contact_list();
        request = request.select_billing_profile_list();
        request = request.select_customer_history_list();
        request = request.select_customer_preference_list();
        request = request.select_customer_consent_list();
        request = request.select_customer_feedback_list();
        request = request.select_loyalty_tier_list();
        request = request.select_referral_code_list();
        request = request.select_communication_log_list();
        request = request.select_service_rating_list();
        request = request.select_account_status_list();
        request = request.select_contact_method_list();
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


    pub fn select_customer_id(mut self) -> Self {
        self.query = self.query.project("customer_id");
        self
    }

    pub fn project_customer_id(self) -> Self {
        self.select_customer_id()
    }

    pub fn select_customer_id_raw(self, raw_sql_segment: impl Into<String>) -> Self {
        self.select_customer_id_unsafe_raw(UnsafeRawSqlSegment::trusted(raw_sql_segment))
    }

    pub fn select_customer_id_unsafe_raw(mut self, raw_sql_segment: UnsafeRawSqlSegment) -> Self {
        self.query_options
            .raw_projections
            .push(RawProjection::new("customer_id", raw_sql_segment));
        self
    }

    pub fn group_by_customer_id(self) -> Self {
        self.group_by("customer_id")
    }

    pub fn group_by_customer_id_as(self, alias: impl Into<String>) -> Self {
        let alias = alias.into();
        let mut request = self.group_by("customer_id");
        request.query = request
            .query
            .project_expr(alias, Expr::column("customer_id"));
        request
    }

    pub fn group_by_customer_id_with_function(
        self,
        alias: impl Into<String>,
        function: AggregateFunction,
    ) -> Self {
        self.group_by("customer_id")
            .aggregate_with_function("customer_id", alias, function)
    }

    pub fn count_customer_id(self) -> Self {
        self.count_customer_id_as("customer_id_count")
    }

    pub fn count_customer_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_count_field("customer_id", alias)
    }

    pub fn sum_customer_id(self) -> Self {
        self.sum_customer_id_as("sum_customer_id")
    }

    pub fn sum_customer_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_sum("customer_id", alias)
    }

    pub fn avg_customer_id(self) -> Self {
        self.avg_customer_id_as("avg_customer_id")
    }

    pub fn avg_customer_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_avg("customer_id", alias)
    }

    pub fn min_customer_id(self) -> Self {
        self.min_customer_id_as("min_customer_id")
    }

    pub fn min_customer_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_min("customer_id", alias)
    }

    pub fn max_customer_id(self) -> Self {
        self.max_customer_id_as("max_customer_id")
    }

    pub fn max_customer_id_as(self, alias: impl Into<String>) -> Self {
        self.aggregate_max("customer_id", alias)
    }

    pub fn unselect_customer_id(mut self) -> Self {
        self.query.projection.retain(|field| field != "customer_id");
        self.query_options.raw_projections.retain(|projection| projection.property_name != "customer_id");
        self
    }


    pub fn with_customer_id(
        mut self,
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(field_operator_expr(
            "customer_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        ));
        self
    }

    pub fn create_customer_id_criteria(
        operator: FieldOperator,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Expr {
        field_operator_expr(
            "customer_id",
            operator,
            values.into_iter().map(Into::into).collect(),
        )
    }

    pub fn with_customer_id_is(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::eq("customer_id", value));
        self
    }



    pub fn with_customer_id_is_not(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::ne("customer_id", value));
        self
    }

    pub fn with_customer_id_greater_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("customer_id", value));
        self
    }

    pub fn with_customer_id_greater_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gte("customer_id", value));
        self
    }

    pub fn with_customer_id_less_than(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("customer_id", value));
        self
    }

    pub fn with_customer_id_less_than_or_equal_to(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lte("customer_id", value));
        self
    }

    pub fn with_customer_id_between(
        mut self,
        lower: impl Into<teaql_core::Value>,
        upper: impl Into<teaql_core::Value>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::between("customer_id", lower, upper));
        self
    }

    pub fn with_customer_id_between_range<T>(mut self, range: DateRange<T>) -> Self
    where
        T: Into<teaql_core::Value>,
    {
        self.query = self.query.and_filter(Expr::between(
            "customer_id",
            range.start,
            range.end,
        ));
        self
    }

    pub fn with_customer_id_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::in_list(
            "customer_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_customer_id_not_in(
        mut self,
        values: impl IntoIterator<Item = impl Into<teaql_core::Value>>,
    ) -> Self {
        self.query = self.query.and_filter(Expr::not_in_list(
            "customer_id",
            values.into_iter().map(Into::into),
        ));
        self
    }

    pub fn with_customer_id_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::contain("customer_id", value));
        self
    }

    pub fn with_customer_id_not_containing(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_contain("customer_id", value));
        self
    }

    pub fn with_customer_id_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::begin_with("customer_id", value));
        self
    }

    pub fn with_customer_id_not_starting_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_begin_with("customer_id", value));
        self
    }

    pub fn with_customer_id_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::end_with("customer_id", value));
        self
    }

    pub fn with_customer_id_not_ending_with(mut self, value: impl Into<String>) -> Self {
        self.query = self.query.and_filter(Expr::not_end_with("customer_id", value));
        self
    }

    pub fn with_customer_id_sounding_like(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::sound_like("customer_id", value));
        self
    }
    pub fn with_customer_id_before(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::lt("customer_id", value));
        self
    }

    pub fn with_customer_id_after(mut self, value: impl Into<teaql_core::Value>) -> Self {
        self.query = self.query.and_filter(Expr::gt("customer_id", value));
        self
    }

    pub fn with_customer_id_is_unknown(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_null("customer_id"));
        self
    }



    pub fn with_customer_id_is_known(mut self) -> Self {
        self.query = self.query.and_filter(Expr::is_not_null("customer_id"));
        self
    }


    pub fn order_by_customer_id_asc(mut self) -> Self {
        self.query = self.query.order_asc("customer_id");
        self
    }

    pub fn order_by_customer_id_desc(mut self) -> Self {
        self.query = self.query.order_desc("customer_id");
        self
    }

    pub fn order_by_customer_id_asc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_asc("customer_id");
        self
    }

    pub fn order_by_customer_id_desc_using_gbk(mut self) -> Self {
        self.query = self.query.order_gbk_desc("customer_id");
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
    pub fn have_move_orders(self) -> Self {
        self.with_move_order_list_matching(SelectQuery::new("MoveOrder"))
    }

    pub fn have_no_move_orders(self) -> Self {
        self.without_move_order_list_matching(SelectQuery::new("MoveOrder"))
    }

    pub fn with_move_order_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::MoveOrder as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_order_list", selection));
        self
    }

    pub fn without_move_order_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::MoveOrder as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("move_order_list", selection));
        self
    }

    pub fn select_move_order_list(mut self) -> Self {
        self.query = self.query.relation("move_order_list");
        self
    }

    pub fn select_move_order_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("move_order_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("move_order_list", selection));
        self
}

    pub fn have_private_customer_profiles(self) -> Self {
        self.with_private_customer_profile_list_matching(SelectQuery::new("PrivateCustomerProfile"))
    }

    pub fn have_no_private_customer_profiles(self) -> Self {
        self.without_private_customer_profile_list_matching(SelectQuery::new("PrivateCustomerProfile"))
    }

    pub fn with_private_customer_profile_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::PrivateCustomerProfile as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("private_customer_profile_list", selection));
        self
    }

    pub fn without_private_customer_profile_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::PrivateCustomerProfile as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("private_customer_profile_list", selection));
        self
    }

    pub fn select_private_customer_profile_list(mut self) -> Self {
        self.query = self.query.relation("private_customer_profile_list");
        self
    }

    pub fn select_private_customer_profile_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("private_customer_profile_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("private_customer_profile_list", selection));
        self
}

    pub fn have_corporate_customer_profiles(self) -> Self {
        self.with_corporate_customer_profile_list_matching(SelectQuery::new("CorporateCustomerProfile"))
    }

    pub fn have_no_corporate_customer_profiles(self) -> Self {
        self.without_corporate_customer_profile_list_matching(SelectQuery::new("CorporateCustomerProfile"))
    }

    pub fn with_corporate_customer_profile_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CorporateCustomerProfile as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("corporate_customer_profile_list", selection));
        self
    }

    pub fn without_corporate_customer_profile_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CorporateCustomerProfile as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("corporate_customer_profile_list", selection));
        self
    }

    pub fn select_corporate_customer_profile_list(mut self) -> Self {
        self.query = self.query.relation("corporate_customer_profile_list");
        self
    }

    pub fn select_corporate_customer_profile_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("corporate_customer_profile_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("corporate_customer_profile_list", selection));
        self
}

    pub fn have_customer_contacts(self) -> Self {
        self.with_customer_contact_list_matching(SelectQuery::new("CustomerContact"))
    }

    pub fn have_no_customer_contacts(self) -> Self {
        self.without_customer_contact_list_matching(SelectQuery::new("CustomerContact"))
    }

    pub fn with_customer_contact_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CustomerContact as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_contact_list", selection));
        self
    }

    pub fn without_customer_contact_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CustomerContact as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_contact_list", selection));
        self
    }

    pub fn select_customer_contact_list(mut self) -> Self {
        self.query = self.query.relation("customer_contact_list");
        self
    }

    pub fn select_customer_contact_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_contact_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_contact_list", selection));
        self
}

    pub fn have_billing_profiles(self) -> Self {
        self.with_billing_profile_list_matching(SelectQuery::new("BillingProfile"))
    }

    pub fn have_no_billing_profiles(self) -> Self {
        self.without_billing_profile_list_matching(SelectQuery::new("BillingProfile"))
    }

    pub fn with_billing_profile_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::BillingProfile as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("billing_profile_list", selection));
        self
    }

    pub fn without_billing_profile_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::BillingProfile as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("billing_profile_list", selection));
        self
    }

    pub fn select_billing_profile_list(mut self) -> Self {
        self.query = self.query.relation("billing_profile_list");
        self
    }

    pub fn select_billing_profile_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("billing_profile_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("billing_profile_list", selection));
        self
}

    pub fn have_customer_histories(self) -> Self {
        self.with_customer_history_list_matching(SelectQuery::new("CustomerHistory"))
    }

    pub fn have_no_customer_histories(self) -> Self {
        self.without_customer_history_list_matching(SelectQuery::new("CustomerHistory"))
    }

    pub fn with_customer_history_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CustomerHistory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_history_list", selection));
        self
    }

    pub fn without_customer_history_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CustomerHistory as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_history_list", selection));
        self
    }

    pub fn select_customer_history_list(mut self) -> Self {
        self.query = self.query.relation("customer_history_list");
        self
    }

    pub fn select_customer_history_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_history_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_history_list", selection));
        self
}

    pub fn have_customer_preferences(self) -> Self {
        self.with_customer_preference_list_matching(SelectQuery::new("CustomerPreference"))
    }

    pub fn have_no_customer_preferences(self) -> Self {
        self.without_customer_preference_list_matching(SelectQuery::new("CustomerPreference"))
    }

    pub fn with_customer_preference_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CustomerPreference as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_preference_list", selection));
        self
    }

    pub fn without_customer_preference_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CustomerPreference as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_preference_list", selection));
        self
    }

    pub fn select_customer_preference_list(mut self) -> Self {
        self.query = self.query.relation("customer_preference_list");
        self
    }

    pub fn select_customer_preference_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_preference_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_preference_list", selection));
        self
}

    pub fn have_customer_consents(self) -> Self {
        self.with_customer_consent_list_matching(SelectQuery::new("CustomerConsent"))
    }

    pub fn have_no_customer_consents(self) -> Self {
        self.without_customer_consent_list_matching(SelectQuery::new("CustomerConsent"))
    }

    pub fn with_customer_consent_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CustomerConsent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_consent_list", selection));
        self
    }

    pub fn without_customer_consent_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CustomerConsent as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_consent_list", selection));
        self
    }

    pub fn select_customer_consent_list(mut self) -> Self {
        self.query = self.query.relation("customer_consent_list");
        self
    }

    pub fn select_customer_consent_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_consent_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_consent_list", selection));
        self
}

    pub fn have_customer_feedback(self) -> Self {
        self.with_customer_feedback_list_matching(SelectQuery::new("CustomerFeedback"))
    }

    pub fn have_no_customer_feedback(self) -> Self {
        self.without_customer_feedback_list_matching(SelectQuery::new("CustomerFeedback"))
    }

    pub fn with_customer_feedback_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CustomerFeedback as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_feedback_list", selection));
        self
    }

    pub fn without_customer_feedback_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CustomerFeedback as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("customer_feedback_list", selection));
        self
    }

    pub fn select_customer_feedback_list(mut self) -> Self {
        self.query = self.query.relation("customer_feedback_list");
        self
    }

    pub fn select_customer_feedback_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("customer_feedback_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("customer_feedback_list", selection));
        self
}

    pub fn have_loyalty_tiers(self) -> Self {
        self.with_loyalty_tier_list_matching(SelectQuery::new("LoyaltyTier"))
    }

    pub fn have_no_loyalty_tiers(self) -> Self {
        self.without_loyalty_tier_list_matching(SelectQuery::new("LoyaltyTier"))
    }

    pub fn with_loyalty_tier_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::LoyaltyTier as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("loyalty_tier_list", selection));
        self
    }

    pub fn without_loyalty_tier_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::LoyaltyTier as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("loyalty_tier_list", selection));
        self
    }

    pub fn select_loyalty_tier_list(mut self) -> Self {
        self.query = self.query.relation("loyalty_tier_list");
        self
    }

    pub fn select_loyalty_tier_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("loyalty_tier_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("loyalty_tier_list", selection));
        self
}

    pub fn have_referral_codes(self) -> Self {
        self.with_referral_code_list_matching(SelectQuery::new("ReferralCode"))
    }

    pub fn have_no_referral_codes(self) -> Self {
        self.without_referral_code_list_matching(SelectQuery::new("ReferralCode"))
    }

    pub fn with_referral_code_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ReferralCode as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("referral_code_list", selection));
        self
    }

    pub fn without_referral_code_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ReferralCode as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("referral_code_list", selection));
        self
    }

    pub fn select_referral_code_list(mut self) -> Self {
        self.query = self.query.relation("referral_code_list");
        self
    }

    pub fn select_referral_code_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("referral_code_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("referral_code_list", selection));
        self
}

    pub fn have_communication_logs(self) -> Self {
        self.with_communication_log_list_matching(SelectQuery::new("CommunicationLog"))
    }

    pub fn have_no_communication_logs(self) -> Self {
        self.without_communication_log_list_matching(SelectQuery::new("CommunicationLog"))
    }

    pub fn with_communication_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::CommunicationLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("communication_log_list", selection));
        self
    }

    pub fn without_communication_log_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::CommunicationLog as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("communication_log_list", selection));
        self
    }

    pub fn select_communication_log_list(mut self) -> Self {
        self.query = self.query.relation("communication_log_list");
        self
    }

    pub fn select_communication_log_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("communication_log_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("communication_log_list", selection));
        self
}

    pub fn have_service_ratings(self) -> Self {
        self.with_service_rating_list_matching(SelectQuery::new("ServiceRating"))
    }

    pub fn have_no_service_ratings(self) -> Self {
        self.without_service_rating_list_matching(SelectQuery::new("ServiceRating"))
    }

    pub fn with_service_rating_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ServiceRating as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_rating_list", selection));
        self
    }

    pub fn without_service_rating_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ServiceRating as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("service_rating_list", selection));
        self
    }

    pub fn select_service_rating_list(mut self) -> Self {
        self.query = self.query.relation("service_rating_list");
        self
    }

    pub fn select_service_rating_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("service_rating_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("service_rating_list", selection));
        self
}

    pub fn have_account_statuses(self) -> Self {
        self.with_account_status_list_matching(SelectQuery::new("AccountStatus"))
    }

    pub fn have_no_account_statuses(self) -> Self {
        self.without_account_status_list_matching(SelectQuery::new("AccountStatus"))
    }

    pub fn with_account_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::AccountStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("account_status_list", selection));
        self
    }

    pub fn without_account_status_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::AccountStatus as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("account_status_list", selection));
        self
    }

    pub fn select_account_status_list(mut self) -> Self {
        self.query = self.query.relation("account_status_list");
        self
    }

    pub fn select_account_status_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("account_status_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("account_status_list", selection));
        self
}

    pub fn have_contact_methods(self) -> Self {
        self.with_contact_method_list_matching(SelectQuery::new("ContactMethod"))
    }

    pub fn have_no_contact_methods(self) -> Self {
        self.without_contact_method_list_matching(SelectQuery::new("ContactMethod"))
    }

    pub fn with_contact_method_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::in_subquery(
            "id",
            <crate::ContactMethod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("contact_method_list", selection));
        self
    }

    pub fn without_contact_method_list_matching(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.and_filter(Expr::not_in_subquery(
            "id",
            <crate::ContactMethod as teaql_core::TeaqlEntity>::entity_descriptor(),
            selection.query.clone(),
            "customer_ref_id",
        ));
        self.relation_filters.push(RelationFilter::new("contact_method_list", selection));
        self
    }

    pub fn select_contact_method_list(mut self) -> Self {
        self.query = self.query.relation("contact_method_list");
        self
    }

    pub fn select_contact_method_list_with(mut self, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query = self.query.relation_query("contact_method_list", selection.clone().into_query());
        self.relation_selections.push(RelationSelection::new("contact_method_list", selection));
        self
}
    pub fn count_move_orders(self) -> Self {
        self.count_move_orders_as("count_move_orders")
    }

    pub fn count_move_orders_as(self, alias: impl Into<String>) -> Self {
        self.count_move_orders_with(alias, crate::Q::move_orders().unlimited())
    }

    pub fn count_move_orders_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_order_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_move_orders(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders_as("refinements", request)
    }

    pub fn stats_from_move_orders_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "move_order_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_move_orders_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_move_orders(request)
    }




    pub fn count_private_customer_profiles(self) -> Self {
        self.count_private_customer_profiles_as("count_private_customer_profiles")
    }

    pub fn count_private_customer_profiles_as(self, alias: impl Into<String>) -> Self {
        self.count_private_customer_profiles_with(alias, crate::Q::private_customer_profiles().unlimited())
    }

    pub fn count_private_customer_profiles_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "private_customer_profile_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_private_customer_profiles(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_private_customer_profiles_as("refinements", request)
    }

    pub fn stats_from_private_customer_profiles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "private_customer_profile_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_private_customer_profiles_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_private_customer_profiles(request)
    }




    pub fn count_corporate_customer_profiles(self) -> Self {
        self.count_corporate_customer_profiles_as("count_corporate_customer_profiles")
    }

    pub fn count_corporate_customer_profiles_as(self, alias: impl Into<String>) -> Self {
        self.count_corporate_customer_profiles_with(alias, crate::Q::corporate_customer_profiles().unlimited())
    }

    pub fn count_corporate_customer_profiles_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "corporate_customer_profile_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_corporate_customer_profiles(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_corporate_customer_profiles_as("refinements", request)
    }

    pub fn stats_from_corporate_customer_profiles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "corporate_customer_profile_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_corporate_customer_profiles_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_corporate_customer_profiles(request)
    }




    pub fn count_customer_contacts(self) -> Self {
        self.count_customer_contacts_as("count_customer_contacts")
    }

    pub fn count_customer_contacts_as(self, alias: impl Into<String>) -> Self {
        self.count_customer_contacts_with(alias, crate::Q::customer_contacts().unlimited())
    }

    pub fn count_customer_contacts_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_contact_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customer_contacts(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_contacts_as("refinements", request)
    }

    pub fn stats_from_customer_contacts_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_contact_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customer_contacts_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_contacts(request)
    }




    pub fn count_billing_profiles(self) -> Self {
        self.count_billing_profiles_as("count_billing_profiles")
    }

    pub fn count_billing_profiles_as(self, alias: impl Into<String>) -> Self {
        self.count_billing_profiles_with(alias, crate::Q::billing_profiles().unlimited())
    }

    pub fn count_billing_profiles_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "billing_profile_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_billing_profiles(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_billing_profiles_as("refinements", request)
    }

    pub fn stats_from_billing_profiles_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "billing_profile_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_billing_profiles_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_billing_profiles(request)
    }




    pub fn count_customer_histories(self) -> Self {
        self.count_customer_histories_as("count_customer_histories")
    }

    pub fn count_customer_histories_as(self, alias: impl Into<String>) -> Self {
        self.count_customer_histories_with(alias, crate::Q::customer_histories().unlimited())
    }

    pub fn count_customer_histories_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_history_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customer_histories(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_histories_as("refinements", request)
    }

    pub fn stats_from_customer_histories_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_history_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customer_histories_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_histories(request)
    }




    pub fn count_customer_preferences(self) -> Self {
        self.count_customer_preferences_as("count_customer_preferences")
    }

    pub fn count_customer_preferences_as(self, alias: impl Into<String>) -> Self {
        self.count_customer_preferences_with(alias, crate::Q::customer_preferences().unlimited())
    }

    pub fn count_customer_preferences_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_preference_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customer_preferences(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_preferences_as("refinements", request)
    }

    pub fn stats_from_customer_preferences_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_preference_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customer_preferences_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_preferences(request)
    }




    pub fn count_customer_consents(self) -> Self {
        self.count_customer_consents_as("count_customer_consents")
    }

    pub fn count_customer_consents_as(self, alias: impl Into<String>) -> Self {
        self.count_customer_consents_with(alias, crate::Q::customer_consents().unlimited())
    }

    pub fn count_customer_consents_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_consent_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customer_consents(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_consents_as("refinements", request)
    }

    pub fn stats_from_customer_consents_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_consent_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customer_consents_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_consents(request)
    }




    pub fn count_customer_feedback(self) -> Self {
        self.count_customer_feedback_as("count_customer_feedback")
    }

    pub fn count_customer_feedback_as(self, alias: impl Into<String>) -> Self {
        self.count_customer_feedback_with(alias, crate::Q::customer_feedback().unlimited())
    }

    pub fn count_customer_feedback_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_feedback_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_customer_feedback(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback_as("refinements", request)
    }

    pub fn stats_from_customer_feedback_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "customer_feedback_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_customer_feedback_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_customer_feedback(request)
    }




    pub fn count_loyalty_tiers(self) -> Self {
        self.count_loyalty_tiers_as("count_loyalty_tiers")
    }

    pub fn count_loyalty_tiers_as(self, alias: impl Into<String>) -> Self {
        self.count_loyalty_tiers_with(alias, crate::Q::loyalty_tiers().unlimited())
    }

    pub fn count_loyalty_tiers_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "loyalty_tier_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_loyalty_tiers(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_loyalty_tiers_as("refinements", request)
    }

    pub fn stats_from_loyalty_tiers_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "loyalty_tier_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_loyalty_tiers_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_loyalty_tiers(request)
    }




    pub fn count_referral_codes(self) -> Self {
        self.count_referral_codes_as("count_referral_codes")
    }

    pub fn count_referral_codes_as(self, alias: impl Into<String>) -> Self {
        self.count_referral_codes_with(alias, crate::Q::referral_codes().unlimited())
    }

    pub fn count_referral_codes_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "referral_code_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_referral_codes(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_referral_codes_as("refinements", request)
    }

    pub fn stats_from_referral_codes_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "referral_code_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_referral_codes_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_referral_codes(request)
    }




    pub fn count_communication_logs(self) -> Self {
        self.count_communication_logs_as("count_communication_logs")
    }

    pub fn count_communication_logs_as(self, alias: impl Into<String>) -> Self {
        self.count_communication_logs_with(alias, crate::Q::communication_logs().unlimited())
    }

    pub fn count_communication_logs_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "communication_log_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_communication_logs(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_communication_logs_as("refinements", request)
    }

    pub fn stats_from_communication_logs_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "communication_log_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_communication_logs_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_communication_logs(request)
    }




    pub fn count_service_ratings(self) -> Self {
        self.count_service_ratings_as("count_service_ratings")
    }

    pub fn count_service_ratings_as(self, alias: impl Into<String>) -> Self {
        self.count_service_ratings_with(alias, crate::Q::service_ratings().unlimited())
    }

    pub fn count_service_ratings_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_rating_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_service_ratings(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_ratings_as("refinements", request)
    }

    pub fn stats_from_service_ratings_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "service_rating_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_service_ratings_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_service_ratings(request)
    }




    pub fn count_account_statuses(self) -> Self {
        self.count_account_statuses_as("count_account_statuses")
    }

    pub fn count_account_statuses_as(self, alias: impl Into<String>) -> Self {
        self.count_account_statuses_with(alias, crate::Q::account_statuses().unlimited())
    }

    pub fn count_account_statuses_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "account_status_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_account_statuses(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_account_statuses_as("refinements", request)
    }

    pub fn stats_from_account_statuses_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "account_status_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_account_statuses_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_account_statuses(request)
    }




    pub fn count_contact_methods(self) -> Self {
        self.count_contact_methods_as("count_contact_methods")
    }

    pub fn count_contact_methods_as(self, alias: impl Into<String>) -> Self {
        self.count_contact_methods_with(alias, crate::Q::contact_methods().unlimited())
    }

    pub fn count_contact_methods_with(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "contact_method_list",
            alias,
            selection,
            true,
        ));
        self
    }

    pub fn stats_from_contact_methods(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contact_methods_as("refinements", request)
    }

    pub fn stats_from_contact_methods_as(mut self, alias: impl Into<String>, request: impl Into<QuerySelection>) -> Self {
        let selection = request.into();
        self.query_options.relation_aggregates.push(RelationAggregate::new(
            "contact_method_list",
            alias,
            selection,
            false,
        ));
        self
    }

    pub fn group_by_contact_methods_with_details(self, request: impl Into<QuerySelection>) -> Self {
        self.stats_from_contact_methods(request)
    }



}

impl<R> Default for CustomerRequest<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R> From< CustomerRequest<R> > for SelectQuery {
    fn from(request: CustomerRequest<R>) -> Self {
        QuerySelection::from(request).into_query()
    }
}

impl<R> From< CustomerRequest<R> > for QuerySelection {
    fn from(request: CustomerRequest<R>) -> Self {
        Self {
            query: request.query,
            relation_selections: request.relation_selections,
            relation_filters: request.relation_filters,
            child_enhancements: request.child_enhancements,
            query_options: request.query_options,
        }
    }
}


impl<'a, C> crate::request_support::AuditedSave<'a, C> for teaql_core::Audited<crate::Customer> 
where C: crate::request_support::TeaqlRepositoryProvider + ?Sized + 'a
{
    type Error = crate::TeaqlDataServiceError<C::CustomerRepository<'a>>;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>> {
        Box::pin(async move { self.into_entity().save(ctx).await })
    }
}

impl<R: teaql_core::Entity> crate::PurposedQuery<CustomerRequest<R>> {
    pub fn new_entity<C>(&self, ctx: &C) -> crate::Customer
    where
        C: crate::TeaqlRuntime + ?Sized,
    {
        crate::Customer::runtime_new(ctx.user_context().entity_root())
    }

    fn into_inner_with_trace(mut self) -> CustomerRequest<R> {
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
    ) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_page(ctx, offset, limit).await
    }

    pub async fn execute_for_exists<'a, C>(
        self,
        ctx: &'a C,
    ) -> Result<bool, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_exists(ctx).await
    }

    pub async fn execute_for_list<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_list(ctx).await
    }

    /// Execute query in streaming mode (chunked).
    /// Returns a Vec of StreamChunk, each containing up to chunk_size rows.
    /// Set chunk size via .stream(chunk_size) or .stream_default() on the query.
    pub async fn execute_for_stream<'a, C>(self, ctx: &'a C) -> Result<Vec<teaql_data_service::StreamChunk>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_stream(ctx).await
    }

    pub async fn execute_for_first<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_first(ctx).await
    }

    pub async fn execute_for_one<'a, C>(self, ctx: &'a C) -> Result<Option<R>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_one(ctx).await
    }


    pub async fn execute_for_records<'a, C>(self, ctx: &'a C) -> Result<teaql_core::SmartList<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_records(ctx).await
    }

    pub async fn execute_for_record<'a, C>(self, ctx: &'a C) -> Result<Option<teaql_core::Record>, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_record(ctx).await
    }

    pub async fn execute_for_count<'a, C>(self, ctx: &'a C) -> Result<u64, crate::request_support::TeaqlDataServiceError<C::CustomerRepository<'a>>>
    where
        C: crate::request_support::TeaqlRepositoryProvider + ?Sized,
    {
        self.into_inner_with_trace()._execute_for_count(ctx).await
    }
}
