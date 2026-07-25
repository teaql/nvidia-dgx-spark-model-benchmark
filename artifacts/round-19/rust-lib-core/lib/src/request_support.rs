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
    type TrucksRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn trucks_repository(&self) -> Result<Self::TrucksRepository<'_>, ContextError>;
    type VehiclesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicles_repository(&self) -> Result<Self::VehiclesRepository<'_>, ContextError>;
    type DriversRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn drivers_repository(&self) -> Result<Self::DriversRepository<'_>, ContextError>;
    type RoutesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn routes_repository(&self) -> Result<Self::RoutesRepository<'_>, ContextError>;
    type LocationsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn locations_repository(&self) -> Result<Self::LocationsRepository<'_>, ContextError>;
    type AddressesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn addresses_repository(&self) -> Result<Self::AddressesRepository<'_>, ContextError>;
    type DispatchesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dispatches_repository(&self) -> Result<Self::DispatchesRepository<'_>, ContextError>;
    type JobsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn jobs_repository(&self) -> Result<Self::JobsRepository<'_>, ContextError>;
    type SchedulesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn schedules_repository(&self) -> Result<Self::SchedulesRepository<'_>, ContextError>;
    type ShiftsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn shifts_repository(&self) -> Result<Self::ShiftsRepository<'_>, ContextError>;
    type TimesheetsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn timesheets_repository(&self) -> Result<Self::TimesheetsRepository<'_>, ContextError>;
    type TrackingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tracking_repository(&self) -> Result<Self::TrackingRepository<'_>, ContextError>;
    type GeofenceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn geofence_repository(&self) -> Result<Self::GeofenceRepository<'_>, ContextError>;
    type FuelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_repository(&self) -> Result<Self::FuelRepository<'_>, ContextError>;
    type MaintenanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_repository(&self) -> Result<Self::MaintenanceRepository<'_>, ContextError>;
    type RepairsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn repairs_repository(&self) -> Result<Self::RepairsRepository<'_>, ContextError>;
    type InspectionsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inspections_repository(&self) -> Result<Self::InspectionsRepository<'_>, ContextError>;
    type EquipmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError>;
    type WarehouseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warehouse_repository(&self) -> Result<Self::WarehouseRepository<'_>, ContextError>;
    type InventoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inventory_repository(&self) -> Result<Self::InventoryRepository<'_>, ContextError>;
    type InvoicesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoices_repository(&self) -> Result<Self::InvoicesRepository<'_>, ContextError>;
    type PaymentsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payments_repository(&self) -> Result<Self::PaymentsRepository<'_>, ContextError>;
    type ExpensesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expenses_repository(&self) -> Result<Self::ExpensesRepository<'_>, ContextError>;
    type AccountsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn accounts_repository(&self) -> Result<Self::AccountsRepository<'_>, ContextError>;
    type LedgersRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn ledgers_repository(&self) -> Result<Self::LedgersRepository<'_>, ContextError>;
    type TaxesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn taxes_repository(&self) -> Result<Self::TaxesRepository<'_>, ContextError>;
    type QuotesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn quotes_repository(&self) -> Result<Self::QuotesRepository<'_>, ContextError>;
    type EstimatesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn estimates_repository(&self) -> Result<Self::EstimatesRepository<'_>, ContextError>;
    type AuditRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_repository(&self) -> Result<Self::AuditRepository<'_>, ContextError>;
    type SecurityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn security_repository(&self) -> Result<Self::SecurityRepository<'_>, ContextError>;
    type BudgetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn budget_repository(&self) -> Result<Self::BudgetRepository<'_>, ContextError>;
    type PayrollRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_repository(&self) -> Result<Self::PayrollRepository<'_>, ContextError>;
    type ReimbursementsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn reimbursements_repository(&self) -> Result<Self::ReimbursementsRepository<'_>, ContextError>;
    type FinancialReportsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_reports_repository(&self) -> Result<Self::FinancialReportsRepository<'_>, ContextError>;
    type CashFlowRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cash_flow_repository(&self) -> Result<Self::CashFlowRepository<'_>, ContextError>;
    type CustomersRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customers_repository(&self) -> Result<Self::CustomersRepository<'_>, ContextError>;
    type EmployeesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn employees_repository(&self) -> Result<Self::EmployeesRepository<'_>, ContextError>;
    type ContactsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contacts_repository(&self) -> Result<Self::ContactsRepository<'_>, ContextError>;
    type DocumentsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn documents_repository(&self) -> Result<Self::DocumentsRepository<'_>, ContextError>;
    type ContractsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contracts_repository(&self) -> Result<Self::ContractsRepository<'_>, ContextError>;
    type SignaturesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn signatures_repository(&self) -> Result<Self::SignaturesRepository<'_>, ContextError>;
    type FeedbackRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn feedback_repository(&self) -> Result<Self::FeedbackRepository<'_>, ContextError>;
    type ReviewsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn reviews_repository(&self) -> Result<Self::ReviewsRepository<'_>, ContextError>;
    type RatingsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn ratings_repository(&self) -> Result<Self::RatingsRepository<'_>, ContextError>;
    type NotificationsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn notifications_repository(&self) -> Result<Self::NotificationsRepository<'_>, ContextError>;
    type AlertsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn alerts_repository(&self) -> Result<Self::AlertsRepository<'_>, ContextError>;
    type CalendarsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn calendars_repository(&self) -> Result<Self::CalendarsRepository<'_>, ContextError>;
    type UsersRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn users_repository(&self) -> Result<Self::UsersRepository<'_>, ContextError>;
    type RolesRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn roles_repository(&self) -> Result<Self::RolesRepository<'_>, ContextError>;
    type PermissionsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn permissions_repository(&self) -> Result<Self::PermissionsRepository<'_>, ContextError>;
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
    type TrucksRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn trucks_repository(&self) -> Result<Self::TrucksRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Trucks")
    }

    type VehiclesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicles_repository(&self) -> Result<Self::VehiclesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Vehicles")
    }

    type DriversRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn drivers_repository(&self) -> Result<Self::DriversRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Drivers")
    }

    type RoutesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn routes_repository(&self) -> Result<Self::RoutesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Routes")
    }

    type LocationsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn locations_repository(&self) -> Result<Self::LocationsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Locations")
    }

    type AddressesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn addresses_repository(&self) -> Result<Self::AddressesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Addresses")
    }

    type DispatchesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dispatches_repository(&self) -> Result<Self::DispatchesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Dispatches")
    }

    type JobsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn jobs_repository(&self) -> Result<Self::JobsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Jobs")
    }

    type SchedulesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn schedules_repository(&self) -> Result<Self::SchedulesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Schedules")
    }

    type ShiftsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn shifts_repository(&self) -> Result<Self::ShiftsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Shifts")
    }

    type TimesheetsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn timesheets_repository(&self) -> Result<Self::TimesheetsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Timesheets")
    }

    type TrackingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tracking_repository(&self) -> Result<Self::TrackingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Tracking")
    }

    type GeofenceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn geofence_repository(&self) -> Result<Self::GeofenceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Geofence")
    }

    type FuelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_repository(&self) -> Result<Self::FuelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Fuel")
    }

    type MaintenanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_repository(&self) -> Result<Self::MaintenanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Maintenance")
    }

    type RepairsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn repairs_repository(&self) -> Result<Self::RepairsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Repairs")
    }

    type InspectionsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inspections_repository(&self) -> Result<Self::InspectionsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Inspections")
    }

    type EquipmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equipment_repository(&self) -> Result<Self::EquipmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Equipment")
    }

    type WarehouseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warehouse_repository(&self) -> Result<Self::WarehouseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Warehouse")
    }

    type InventoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inventory_repository(&self) -> Result<Self::InventoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Inventory")
    }

    type InvoicesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoices_repository(&self) -> Result<Self::InvoicesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Invoices")
    }

    type PaymentsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payments_repository(&self) -> Result<Self::PaymentsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payments")
    }

    type ExpensesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expenses_repository(&self) -> Result<Self::ExpensesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Expenses")
    }

    type AccountsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn accounts_repository(&self) -> Result<Self::AccountsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Accounts")
    }

    type LedgersRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn ledgers_repository(&self) -> Result<Self::LedgersRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Ledgers")
    }

    type TaxesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn taxes_repository(&self) -> Result<Self::TaxesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Taxes")
    }

    type QuotesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn quotes_repository(&self) -> Result<Self::QuotesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Quotes")
    }

    type EstimatesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn estimates_repository(&self) -> Result<Self::EstimatesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Estimates")
    }

    type AuditRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_repository(&self) -> Result<Self::AuditRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Audit")
    }

    type SecurityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn security_repository(&self) -> Result<Self::SecurityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Security")
    }

    type BudgetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn budget_repository(&self) -> Result<Self::BudgetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Budget")
    }

    type PayrollRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_repository(&self) -> Result<Self::PayrollRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payroll")
    }

    type ReimbursementsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn reimbursements_repository(&self) -> Result<Self::ReimbursementsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Reimbursements")
    }

    type FinancialReportsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_reports_repository(&self) -> Result<Self::FinancialReportsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialReports")
    }

    type CashFlowRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cash_flow_repository(&self) -> Result<Self::CashFlowRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CashFlow")
    }

    type CustomersRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customers_repository(&self) -> Result<Self::CustomersRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Customers")
    }

    type EmployeesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn employees_repository(&self) -> Result<Self::EmployeesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Employees")
    }

    type ContactsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contacts_repository(&self) -> Result<Self::ContactsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Contacts")
    }

    type DocumentsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn documents_repository(&self) -> Result<Self::DocumentsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Documents")
    }

    type ContractsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contracts_repository(&self) -> Result<Self::ContractsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Contracts")
    }

    type SignaturesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn signatures_repository(&self) -> Result<Self::SignaturesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Signatures")
    }

    type FeedbackRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn feedback_repository(&self) -> Result<Self::FeedbackRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Feedback")
    }

    type ReviewsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn reviews_repository(&self) -> Result<Self::ReviewsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Reviews")
    }

    type RatingsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn ratings_repository(&self) -> Result<Self::RatingsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Ratings")
    }

    type NotificationsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn notifications_repository(&self) -> Result<Self::NotificationsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Notifications")
    }

    type AlertsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn alerts_repository(&self) -> Result<Self::AlertsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Alerts")
    }

    type CalendarsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn calendars_repository(&self) -> Result<Self::CalendarsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Calendars")
    }

    type UsersRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn users_repository(&self) -> Result<Self::UsersRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Users")
    }

    type RolesRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn roles_repository(&self) -> Result<Self::RolesRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Roles")
    }

    type PermissionsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn permissions_repository(&self) -> Result<Self::PermissionsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Permissions")
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
