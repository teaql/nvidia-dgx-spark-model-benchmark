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
    type CustomerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError>;
    type AddressRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn address_repository(&self) -> Result<Self::AddressRepository<'_>, ContextError>;
    type TruckRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn truck_repository(&self) -> Result<Self::TruckRepository<'_>, ContextError>;
    type DriverRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_repository(&self) -> Result<Self::DriverRepository<'_>, ContextError>;
    type MoveOrderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError>;
    type InventoryItemRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError>;
    type PackingMaterialRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_material_repository(&self) -> Result<Self::PackingMaterialRepository<'_>, ContextError>;
    type RouteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_repository(&self) -> Result<Self::RouteRepository<'_>, ContextError>;
    type ScheduleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn schedule_repository(&self) -> Result<Self::ScheduleRepository<'_>, ContextError>;
    type LoadingUnloadingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loading_unloading_repository(&self) -> Result<Self::LoadingUnloadingRepository<'_>, ContextError>;
    type EquipmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError>;
    type ToolRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tool_repository(&self) -> Result<Self::ToolRepository<'_>, ContextError>;
    type StorageFacilityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn storage_facility_repository(&self) -> Result<Self::StorageFacilityRepository<'_>, ContextError>;
    type WarehouseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warehouse_repository(&self) -> Result<Self::WarehouseRepository<'_>, ContextError>;
    type ContainerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn container_repository(&self) -> Result<Self::ContainerRepository<'_>, ContextError>;
    type PalletRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn pallet_repository(&self) -> Result<Self::PalletRepository<'_>, ContextError>;
    type LabelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn label_repository(&self) -> Result<Self::LabelRepository<'_>, ContextError>;
    type BarcodeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn barcode_repository(&self) -> Result<Self::BarcodeRepository<'_>, ContextError>;
    type TrackingNumberRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tracking_number_repository(&self) -> Result<Self::TrackingNumberRepository<'_>, ContextError>;
    type NotificationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notification_repository(&self) -> Result<Self::NotificationRepository<'_>, ContextError>;
    type PaymentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError>;
    type InvoiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError>;
    type ClaimRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn claim_repository(&self) -> Result<Self::ClaimRepository<'_>, ContextError>;
    type FeedbackRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn feedback_repository(&self) -> Result<Self::FeedbackRepository<'_>, ContextError>;
    type EmployeeRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employee_repository(&self) -> Result<Self::EmployeeRepository<'_>, ContextError>;
    type BranchRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn branch_repository(&self) -> Result<Self::BranchRepository<'_>, ContextError>;
    type VehicleMaintenanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_maintenance_repository(&self) -> Result<Self::VehicleMaintenanceRepository<'_>, ContextError>;
    type FuelLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_log_repository(&self) -> Result<Self::FuelLogRepository<'_>, ContextError>;
    type InsurancePolicyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn insurance_policy_repository(&self) -> Result<Self::InsurancePolicyRepository<'_>, ContextError>;
    type LicenseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn license_repository(&self) -> Result<Self::LicenseRepository<'_>, ContextError>;
    type PermitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn permit_repository(&self) -> Result<Self::PermitRepository<'_>, ContextError>;
    type CustomsDocumentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customs_document_repository(&self) -> Result<Self::CustomsDocumentRepository<'_>, ContextError>;
    type CommunicationLogRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn communication_log_repository(&self) -> Result<Self::CommunicationLogRepository<'_>, ContextError>;
    type AuditTrailRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError>;
    type ReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn report_repository(&self) -> Result<Self::ReportRepository<'_>, ContextError>;
    type DashboardRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dashboard_repository(&self) -> Result<Self::DashboardRepository<'_>, ContextError>;
    type SettingsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn settings_repository(&self) -> Result<Self::SettingsRepository<'_>, ContextError>;
    type UserRoleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn user_role_repository(&self) -> Result<Self::UserRoleRepository<'_>, ContextError>;
    type PermissionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn permission_repository(&self) -> Result<Self::PermissionRepository<'_>, ContextError>;
    type ApiKeyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn api_key_repository(&self) -> Result<Self::ApiKeyRepository<'_>, ContextError>;
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
    type CustomerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Customer")
    }

    type AddressRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn address_repository(&self) -> Result<Self::AddressRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Address")
    }

    type TruckRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn truck_repository(&self) -> Result<Self::TruckRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Truck")
    }

    type DriverRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_repository(&self) -> Result<Self::DriverRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Driver")
    }

    type MoveOrderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn move_order_repository(&self) -> Result<Self::MoveOrderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MoveOrder")
    }

    type InventoryItemRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_item_repository(&self) -> Result<Self::InventoryItemRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InventoryItem")
    }

    type PackingMaterialRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_material_repository(&self) -> Result<Self::PackingMaterialRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PackingMaterial")
    }

    type RouteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_repository(&self) -> Result<Self::RouteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Route")
    }

    type ScheduleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn schedule_repository(&self) -> Result<Self::ScheduleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Schedule")
    }

    type LoadingUnloadingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loading_unloading_repository(&self) -> Result<Self::LoadingUnloadingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadingUnloading")
    }

    type EquipmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Equipment")
    }

    type ToolRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tool_repository(&self) -> Result<Self::ToolRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Tool")
    }

    type StorageFacilityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn storage_facility_repository(&self) -> Result<Self::StorageFacilityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StorageFacility")
    }

    type WarehouseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warehouse_repository(&self) -> Result<Self::WarehouseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Warehouse")
    }

    type ContainerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn container_repository(&self) -> Result<Self::ContainerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Container")
    }

    type PalletRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn pallet_repository(&self) -> Result<Self::PalletRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Pallet")
    }

    type LabelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn label_repository(&self) -> Result<Self::LabelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Label")
    }

    type BarcodeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn barcode_repository(&self) -> Result<Self::BarcodeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Barcode")
    }

    type TrackingNumberRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tracking_number_repository(&self) -> Result<Self::TrackingNumberRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("TrackingNumber")
    }

    type NotificationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notification_repository(&self) -> Result<Self::NotificationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Notification")
    }

    type PaymentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payment")
    }

    type InvoiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Invoice")
    }

    type ClaimRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn claim_repository(&self) -> Result<Self::ClaimRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Claim")
    }

    type FeedbackRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn feedback_repository(&self) -> Result<Self::FeedbackRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Feedback")
    }

    type EmployeeRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employee_repository(&self) -> Result<Self::EmployeeRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Employee")
    }

    type BranchRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn branch_repository(&self) -> Result<Self::BranchRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Branch")
    }

    type VehicleMaintenanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_maintenance_repository(&self) -> Result<Self::VehicleMaintenanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("VehicleMaintenance")
    }

    type FuelLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_log_repository(&self) -> Result<Self::FuelLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FuelLog")
    }

    type InsurancePolicyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn insurance_policy_repository(&self) -> Result<Self::InsurancePolicyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("InsurancePolicy")
    }

    type LicenseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn license_repository(&self) -> Result<Self::LicenseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("License")
    }

    type PermitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn permit_repository(&self) -> Result<Self::PermitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Permit")
    }

    type CustomsDocumentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customs_document_repository(&self) -> Result<Self::CustomsDocumentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CustomsDocument")
    }

    type CommunicationLogRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn communication_log_repository(&self) -> Result<Self::CommunicationLogRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CommunicationLog")
    }

    type AuditTrailRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_trail_repository(&self) -> Result<Self::AuditTrailRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("AuditTrail")
    }

    type ReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn report_repository(&self) -> Result<Self::ReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Report")
    }

    type DashboardRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dashboard_repository(&self) -> Result<Self::DashboardRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Dashboard")
    }

    type SettingsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn settings_repository(&self) -> Result<Self::SettingsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Settings")
    }

    type UserRoleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn user_role_repository(&self) -> Result<Self::UserRoleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UserRole")
    }

    type PermissionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn permission_repository(&self) -> Result<Self::PermissionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Permission")
    }

    type ApiKeyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn api_key_repository(&self) -> Result<Self::ApiKeyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ApiKey")
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
