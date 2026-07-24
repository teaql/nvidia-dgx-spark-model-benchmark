#![allow(unused_imports)]
#![allow(async_fn_in_trait)]
use std::{collections::BTreeMap, future::Future, marker::PhantomData};

use serde_json::Value as JsonValue;
use teaql_core::{
    BinaryOp, Expr, Record,
    RelationAggregate as RuntimeRelationAggregate, SelectQuery, SmartList,
};
use teaql_runtime::{ContextError, GraphNode, EntityDataServiceBehavior, DataServiceError, RuntimeError, UserContext};

// Re-export query builder types from teaql_core::request
pub use teaql_core::request::{
    COUNT_ALIAS, TYPE_FIELD, TYPE_GROUP_FIELD,
    FieldOperator, DateRange, EntityReference,
    QuerySelection, RelationSelection, RelationFilter, QueryOptions,
    UnsafeRawSqlSegment, RawDynamicProperty, RawProjection,
    RelationAggregate, FacetRequest, ObjectGroupBy,
    apply_relation_selections, apply_runtime_metadata,
    field_operator_expr, field_operator_column_expr,
    required_value, required_text,
    remove_default_live_filter, remove_filter_expr,
    dynamic_json_value_to_teaql_value, dynamic_json_values,
    dynamic_json_operator, dynamic_json_filter_expr,
    dynamic_json_u64_field,
    runtime_relation_aggregates,
    merge_outer_filter_into_facet_aggregates, attach_facets,
};


pub trait TeaqlRecordRepository {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn fetch_all(&self, query: &SelectQuery) -> Result<Vec<Record>, DataServiceError<Self::Error>>;

    async fn fetch_smart_list(&self, query: &SelectQuery) -> Result<SmartList<Record>, DataServiceError<Self::Error>>;

    async fn fetch_smart_list_with_relation_aggregates(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<Record>, DataServiceError<Self::Error>>;

    async fn fetch_stream(&self, query: &SelectQuery) -> Result<Vec<teaql_data_service::StreamChunk>, DataServiceError<Self::Error>>;
}

pub trait TeaqlEntityRepository: TeaqlRecordRepository {
    async fn fetch_enhanced_entities<T>(&self, query: &SelectQuery) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn fetch_enhanced_entities_with_relation_aggregates<T>(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn save_entity_graph<T>(&self, entity: T) -> Result<GraphNode, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity;

    async fn save_entity_ledger(&self, root: teaql_runtime::EntityRoot) -> Result<(), DataServiceError<Self::Error>>;
}

impl<'a, E> TeaqlRecordRepository for teaql_runtime::EntityDataService<'a, E>
where
    E: teaql_data_service::QueryExecutor + teaql_data_service::MutationExecutor + teaql_data_service::StreamQueryExecutor + Send + Sync + 'static,
{
    type Error = E::Error;

    async fn fetch_all(&self, query: &SelectQuery) -> Result<Vec<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_all(self, query).await
    }

    async fn fetch_smart_list(&self, query: &SelectQuery) -> Result<SmartList<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_smart_list(self, query).await
    }

    async fn fetch_smart_list_with_relation_aggregates(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<Record>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_smart_list_with_relation_aggregates(
            self,
            query,
            relation_aggregates,
        ).await
    }

    async fn fetch_stream(&self, query: &SelectQuery) -> Result<Vec<teaql_data_service::StreamChunk>, DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::fetch_stream(self, query).await
    }
}

impl<'a, E> TeaqlEntityRepository for teaql_runtime::EntityDataService<'a, E>
where
    E: teaql_data_service::QueryExecutor + teaql_data_service::MutationExecutor + teaql_data_service::StreamQueryExecutor + Send + Sync + 'static,
{
    async fn fetch_enhanced_entities<T>(&self, query: &SelectQuery) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::fetch_enhanced_entities(self, query).await
    }

    async fn fetch_enhanced_entities_with_relation_aggregates<T>(
        &self,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
    ) -> Result<SmartList<T>, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::fetch_enhanced_entities_with_relation_aggregates(
            self,
            query,
            relation_aggregates,
        ).await
    }

    async fn save_entity_graph<T>(&self, entity: T) -> Result<GraphNode, DataServiceError<Self::Error>>
    where
        T: teaql_core::Entity,
    {
        teaql_runtime::EntityDataService::save_entity_graph(self, entity).await
    }

    async fn save_entity_ledger(&self, root: teaql_runtime::EntityRoot) -> Result<(), DataServiceError<Self::Error>> {
        teaql_runtime::EntityDataService::execute_ledger_plan(self, root).await
    }
}

pub type TeaqlDataServiceError<R> = DataServiceError<<R as TeaqlRecordRepository>::Error>;

pub trait TeaqlRuntime {
    fn user_context(&self) -> &UserContext;

    fn fetch_facet_smart_list(
        &self,
        entity: &str,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
        trace_context: Vec<teaql_core::TraceNode>,
    ) -> impl std::future::Future<Output = Result<SmartList<Record>, RuntimeError>> + Send;
}

/// Internal trait for repository access. Application code should not use this trait directly.
#[doc(hidden)]
pub trait AuditedSave<'a, C>
where
    C: TeaqlRepositoryProvider + ?Sized + 'a,
{
    type Error;
    fn save(self, ctx: &'a C) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<teaql_runtime::GraphNode, Self::Error>> + '_>>;
}



pub trait TeaqlRepositoryProvider: TeaqlRuntime {
    type RouteStatusTypeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_status_type_repository(&self) -> Result<Self::RouteStatusTypeRepository<'_>, ContextError>;
    type InventoryConditionTypeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_condition_type_repository(&self) -> Result<Self::InventoryConditionTypeRepository<'_>, ContextError>;
    type ExceptionSeverityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn exception_severity_repository(&self) -> Result<Self::ExceptionSeverityRepository<'_>, ContextError>;
    type OrderStatusRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn order_status_repository(&self) -> Result<Self::OrderStatusRepository<'_>, ContextError>;
    type CrewRoleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_role_repository(&self) -> Result<Self::CrewRoleRepository<'_>, ContextError>;
    type PlatformRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn platform_repository(&self) -> Result<Self::PlatformRepository<'_>, ContextError>;
    type MerchantRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn merchant_repository(&self) -> Result<Self::MerchantRepository<'_>, ContextError>;
    type MoveQuoteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_quote_repository(&self) -> Result<Self::MoveQuoteRepository<'_>, ContextError>;
    type MoveOrderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError>;
    type RouteStopRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_stop_repository(&self) -> Result<Self::RouteStopRepository<'_>, ContextError>;
    type CrewRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_repository(&self) -> Result<Self::CrewRepository<'_>, ContextError>;
    type CrewMemberAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn crew_member_assignment_repository(&self) -> Result<Self::CrewMemberAssignmentRepository<'_>, ContextError>;
    type VehicleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError>;
    type VehicleAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_assignment_repository(&self) -> Result<Self::VehicleAssignmentRepository<'_>, ContextError>;
    type DispatchAssignmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dispatch_assignment_repository(&self) -> Result<Self::DispatchAssignmentRepository<'_>, ContextError>;
    type DamageReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn damage_report_repository(&self) -> Result<Self::DamageReportRepository<'_>, ContextError>;
    type ProofOfDeliveryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn proof_of_delivery_repository(&self) -> Result<Self::ProofOfDeliveryRepository<'_>, ContextError>;
    type OperationalExceptionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn operational_exception_repository(&self) -> Result<Self::OperationalExceptionRepository<'_>, ContextError>;
    type PickupInstructionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn pickup_instruction_repository(&self) -> Result<Self::PickupInstructionRepository<'_>, ContextError>;
    type DeliveryInstructionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn delivery_instruction_repository(&self) -> Result<Self::DeliveryInstructionRepository<'_>, ContextError>;
    type MoveInventoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_inventory_repository(&self) -> Result<Self::MoveInventoryRepository<'_>, ContextError>;
    type PackagingItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packaging_item_repository(&self) -> Result<Self::PackagingItemRepository<'_>, ContextError>;
    type LogisticsProviderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn logistics_provider_repository(&self) -> Result<Self::LogisticsProviderRepository<'_>, ContextError>;
    type ThirdPartyDispatchRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn third_party_dispatch_repository(&self) -> Result<Self::ThirdPartyDispatchRepository<'_>, ContextError>;
    type FuelLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_log_repository(&self) -> Result<Self::FuelLogRepository<'_>, ContextError>;
    type MaintenanceRecordRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_record_repository(&self) -> Result<Self::MaintenanceRecordRepository<'_>, ContextError>;
    type TollReceiptRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn toll_receipt_repository(&self) -> Result<Self::TollReceiptRepository<'_>, ContextError>;
    type ShiftLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn shift_log_repository(&self) -> Result<Self::ShiftLogRepository<'_>, ContextError>;
    type CustomerFeedbackRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_feedback_repository(&self) -> Result<Self::CustomerFeedbackRepository<'_>, ContextError>;
    type IncidentReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError>;
}

#[allow(async_fn_in_trait)]
pub trait TeaqlUserContextExt {
    async fn commit_data(&self) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>;

    async fn transaction_data<F, Fut>(&self, f: F) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>>;
}

impl TeaqlUserContextExt for teaql_runtime::UserContext {
    async fn commit_data(&self) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>> {
        self.commit_changes::<crate::runtime::DataServiceExecutor>().await
    }

    async fn transaction_data<F, Fut>(&self, f: F) -> Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), DataServiceError<<crate::runtime::DataServiceExecutor as teaql_data_service::DataServiceExecutor>::Error>>>,
    {
        let executor = self.require_resource::<crate::runtime::DataServiceExecutor>().map_err(|err| {
            DataServiceError::Runtime(RuntimeError::Graph(format!(
                "cannot start transaction without executor: {err}"
            )))
        })?;
        let root = self.entity_root();

        let tx = teaql_data_service::TransactionExecutor::begin(&*executor).await.map_err(DataServiceError::Executor)?;
        root.push_change_set();

        let result = f().await;
        match result {
            Ok(()) => {
                root.pop_change_set();
                teaql_data_service::Transaction::commit(tx).await.map_err(DataServiceError::Executor)?;
                Ok(())
            }
            Err(err) => {
                root.pop_change_set();
                teaql_data_service::Transaction::rollback(tx).await.map_err(DataServiceError::Executor)?;
                Err(err)
            }
        }
    }
}

impl TeaqlRuntime for teaql_runtime::UserContext {
    fn user_context(&self) -> &UserContext {
        self
    }

    async fn fetch_facet_smart_list(
        &self,
        entity: &str,
        query: &SelectQuery,
        relation_aggregates: &[RuntimeRelationAggregate],
        trace_context: Vec<teaql_core::TraceNode>,
    ) -> Result<SmartList<Record>, RuntimeError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>(entity)
            .map_err(|err| RuntimeError::Graph(err.to_string()))?
            .with_trace_context(trace_context)
            .fetch_smart_list_with_relation_aggregates(query, relation_aggregates)
            .await
            .map_err(|err| RuntimeError::Graph(err.to_string()))
    }
}

impl TeaqlRepositoryProvider for teaql_runtime::UserContext {
    type RouteStatusTypeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_status_type_repository(&self) -> Result<Self::RouteStatusTypeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RouteStatusType")
    }

    type InventoryConditionTypeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_condition_type_repository(&self) -> Result<Self::InventoryConditionTypeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventoryConditionType")
    }

    type ExceptionSeverityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn exception_severity_repository(&self) -> Result<Self::ExceptionSeverityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExceptionSeverity")
    }

    type OrderStatusRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn order_status_repository(&self) -> Result<Self::OrderStatusRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OrderStatus")
    }

    type CrewRoleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn crew_role_repository(&self) -> Result<Self::CrewRoleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CrewRole")
    }

    type PlatformRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn platform_repository(&self) -> Result<Self::PlatformRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Platform")
    }

    type MerchantRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn merchant_repository(&self) -> Result<Self::MerchantRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Merchant")
    }

    type MoveQuoteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_quote_repository(&self) -> Result<Self::MoveQuoteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveQuote")
    }

    type MoveOrderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveOrder")
    }

    type RouteStopRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_stop_repository(&self) -> Result<Self::RouteStopRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("RouteStop")
    }

    type CrewRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn crew_repository(&self) -> Result<Self::CrewRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Crew")
    }

    type CrewMemberAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn crew_member_assignment_repository(&self) -> Result<Self::CrewMemberAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CrewMemberAssignment")
    }

    type VehicleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Vehicle")
    }

    type VehicleAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_assignment_repository(&self) -> Result<Self::VehicleAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleAssignment")
    }

    type DispatchAssignmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dispatch_assignment_repository(&self) -> Result<Self::DispatchAssignmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DispatchAssignment")
    }

    type DamageReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn damage_report_repository(&self) -> Result<Self::DamageReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DamageReport")
    }

    type ProofOfDeliveryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn proof_of_delivery_repository(&self) -> Result<Self::ProofOfDeliveryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ProofOfDelivery")
    }

    type OperationalExceptionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn operational_exception_repository(&self) -> Result<Self::OperationalExceptionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("OperationalException")
    }

    type PickupInstructionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn pickup_instruction_repository(&self) -> Result<Self::PickupInstructionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PickupInstruction")
    }

    type DeliveryInstructionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn delivery_instruction_repository(&self) -> Result<Self::DeliveryInstructionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("DeliveryInstruction")
    }

    type MoveInventoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_inventory_repository(&self) -> Result<Self::MoveInventoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveInventory")
    }

    type PackagingItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packaging_item_repository(&self) -> Result<Self::PackagingItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackagingItem")
    }

    type LogisticsProviderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn logistics_provider_repository(&self) -> Result<Self::LogisticsProviderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LogisticsProvider")
    }

    type ThirdPartyDispatchRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn third_party_dispatch_repository(&self) -> Result<Self::ThirdPartyDispatchRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ThirdPartyDispatch")
    }

    type FuelLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_log_repository(&self) -> Result<Self::FuelLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FuelLog")
    }

    type MaintenanceRecordRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_record_repository(&self) -> Result<Self::MaintenanceRecordRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MaintenanceRecord")
    }

    type TollReceiptRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn toll_receipt_repository(&self) -> Result<Self::TollReceiptRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TollReceipt")
    }

    type ShiftLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn shift_log_repository(&self) -> Result<Self::ShiftLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ShiftLog")
    }

    type CustomerFeedbackRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_feedback_repository(&self) -> Result<Self::CustomerFeedbackRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomerFeedback")
    }

    type IncidentReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("IncidentReport")
    }
}

pub(crate) async fn execute_facets<C>(
    ctx: &C,
    outer_query: &SelectQuery,
    options: &QueryOptions,
) -> Result<BTreeMap<String, SmartList<Record>>, RuntimeError>
where
    C: TeaqlRuntime + ?Sized,
{
    let mut facets = BTreeMap::new();
    for facet in &options.facets {
        let mut selection = facet.query.clone();
        merge_outer_filter_into_facet_aggregates(&mut selection, outer_query);
        if !facet.include_all_facets {
            selection = restrict_facet_to_outer_query(ctx, selection, outer_query, &facet.relation_name)?;
        }
        let relation_aggregates = runtime_relation_aggregates(&selection.query_options);
        let query = apply_runtime_metadata(
            selection.query,
            &selection.query_options,
            &selection.child_enhancements,
        );
        let mut chain = outer_query.trace_chain.clone();
        chain.push(teaql_core::TraceNode::new(
            query.entity.clone(),
            None,
            facet.facet_name.clone(),
        ));

        let facet_rows = ctx.fetch_facet_smart_list(&query.entity, &query, &relation_aggregates, chain).await?;
        facets.insert(facet.facet_name.clone(), facet_rows);
    }
    Ok(facets)
}

pub(crate) fn restrict_facet_to_outer_query<C>(
    ctx: &C,
    mut selection: QuerySelection,
    outer_query: &SelectQuery,
    relation_name: &str,
) -> Result<QuerySelection, RuntimeError>
where
    C: TeaqlRuntime + ?Sized,
{
    let descriptor = ctx
        .user_context()
        .entity(&outer_query.entity)
        .cloned()
        .ok_or_else(|| RuntimeError::Graph(format!("missing entity: {}", outer_query.entity)))?;
    let relation = descriptor
        .relation_by_name(relation_name)
        .cloned()
        .ok_or_else(|| RuntimeError::MissingRelation {
            entity: outer_query.entity.clone(),
            relation: relation_name.to_owned(),
        })?;
    let mut subquery = outer_query.clone();
    subquery.projection.clear();
    subquery.expr_projection.clear();
    subquery.order_by.clear();
    subquery.slice = None;
    subquery.aggregates.clear();
    subquery.group_by.clear();
    subquery.relations.clear();
    selection.query = selection.query.and_filter(Expr::in_subquery(
        relation.foreign_key,
        descriptor,
        subquery,
        relation.local_key,
    ));
    Ok(selection)
}
