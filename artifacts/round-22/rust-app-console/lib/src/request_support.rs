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
    type InvoiceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError>;
    type BillRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn bill_repository(&self) -> Result<Self::BillRepository<'_>, ContextError>;
    type PaymentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError>;
    type ExpenseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_repository(&self) -> Result<Self::ExpenseRepository<'_>, ContextError>;
    type RevenueRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn revenue_repository(&self) -> Result<Self::RevenueRepository<'_>, ContextError>;
    type LedgerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn ledger_repository(&self) -> Result<Self::LedgerRepository<'_>, ContextError>;
    type AuditRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn audit_repository(&self) -> Result<Self::AuditRepository<'_>, ContextError>;
    type TaxRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tax_repository(&self) -> Result<Self::TaxRepository<'_>, ContextError>;
    type BudgetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn budget_repository(&self) -> Result<Self::BudgetRepository<'_>, ContextError>;
    type ForecastRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn forecast_repository(&self) -> Result<Self::ForecastRepository<'_>, ContextError>;
    type PayrollRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn payroll_repository(&self) -> Result<Self::PayrollRepository<'_>, ContextError>;
    type ExpenseReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn expense_report_repository(&self) -> Result<Self::ExpenseReportRepository<'_>, ContextError>;
    type CreditRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn credit_repository(&self) -> Result<Self::CreditRepository<'_>, ContextError>;
    type DebitRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn debit_repository(&self) -> Result<Self::DebitRepository<'_>, ContextError>;
    type BalanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn balance_repository(&self) -> Result<Self::BalanceRepository<'_>, ContextError>;
    type AssetRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn asset_repository(&self) -> Result<Self::AssetRepository<'_>, ContextError>;
    type LiabilityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn liability_repository(&self) -> Result<Self::LiabilityRepository<'_>, ContextError>;
    type EquityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn equity_repository(&self) -> Result<Self::EquityRepository<'_>, ContextError>;
    type CashFlowRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn cash_flow_repository(&self) -> Result<Self::CashFlowRepository<'_>, ContextError>;
    type FinancialStatementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn financial_statement_repository(&self) -> Result<Self::FinancialStatementRepository<'_>, ContextError>;
    type ShipmentRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn shipment_repository(&self) -> Result<Self::ShipmentRepository<'_>, ContextError>;
    type RouteRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn route_repository(&self) -> Result<Self::RouteRepository<'_>, ContextError>;
    type VehicleRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError>;
    type DriverRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn driver_repository(&self) -> Result<Self::DriverRepository<'_>, ContextError>;
    type LoadRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn load_repository(&self) -> Result<Self::LoadRepository<'_>, ContextError>;
    type UnloadRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn unload_repository(&self) -> Result<Self::UnloadRepository<'_>, ContextError>;
    type CapacityRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn capacity_repository(&self) -> Result<Self::CapacityRepository<'_>, ContextError>;
    type ManifestRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn manifest_repository(&self) -> Result<Self::ManifestRepository<'_>, ContextError>;
    type TrackingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn tracking_repository(&self) -> Result<Self::TrackingRepository<'_>, ContextError>;
    type DispatchRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn dispatch_repository(&self) -> Result<Self::DispatchRepository<'_>, ContextError>;
    type FreightRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn freight_repository(&self) -> Result<Self::FreightRepository<'_>, ContextError>;
    type CarrierRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn carrier_repository(&self) -> Result<Self::CarrierRepository<'_>, ContextError>;
    type WarehouseRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warehouse_repository(&self) -> Result<Self::WarehouseRepository<'_>, ContextError>;
    type LoadingDockRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loading_dock_repository(&self) -> Result<Self::LoadingDockRepository<'_>, ContextError>;
    type UnloadingDockRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn unloading_dock_repository(&self) -> Result<Self::UnloadingDockRepository<'_>, ContextError>;
    type FreightForwarderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn freight_forwarder_repository(&self) -> Result<Self::FreightForwarderRepository<'_>, ContextError>;
    type CustomsRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customs_repository(&self) -> Result<Self::CustomsRepository<'_>, ContextError>;
    type DocumentationRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn documentation_repository(&self) -> Result<Self::DocumentationRepository<'_>, ContextError>;
    type TollRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn toll_repository(&self) -> Result<Self::TollRepository<'_>, ContextError>;
    type FuelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn fuel_repository(&self) -> Result<Self::FuelRepository<'_>, ContextError>;
    type CustomerRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError>;
    type ClientRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn client_repository(&self) -> Result<Self::ClientRepository<'_>, ContextError>;
    type ContactRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contact_repository(&self) -> Result<Self::ContactRepository<'_>, ContextError>;
    type LeadRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn lead_repository(&self) -> Result<Self::LeadRepository<'_>, ContextError>;
    type ProspectRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn prospect_repository(&self) -> Result<Self::ProspectRepository<'_>, ContextError>;
    type AccountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn account_repository(&self) -> Result<Self::AccountRepository<'_>, ContextError>;
    type ServiceAgreementRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn service_agreement_repository(&self) -> Result<Self::ServiceAgreementRepository<'_>, ContextError>;
    type ContractRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn contract_repository(&self) -> Result<Self::ContractRepository<'_>, ContextError>;
    type WarrantyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn warranty_repository(&self) -> Result<Self::WarrantyRepository<'_>, ContextError>;
    type SupportTicketRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn support_ticket_repository(&self) -> Result<Self::SupportTicketRepository<'_>, ContextError>;
    type FeedbackRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn feedback_repository(&self) -> Result<Self::FeedbackRepository<'_>, ContextError>;
    type SurveyRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn survey_repository(&self) -> Result<Self::SurveyRepository<'_>, ContextError>;
    type LoyaltyProgramRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn loyalty_program_repository(&self) -> Result<Self::LoyaltyProgramRepository<'_>, ContextError>;
    type ReferralRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn referral_repository(&self) -> Result<Self::ReferralRepository<'_>, ContextError>;
    type DiscountRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn discount_repository(&self) -> Result<Self::DiscountRepository<'_>, ContextError>;
    type PromotionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn promotion_repository(&self) -> Result<Self::PromotionRepository<'_>, ContextError>;
    type MarketingCampaignRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn marketing_campaign_repository(&self) -> Result<Self::MarketingCampaignRepository<'_>, ContextError>;
    type NewsletterRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn newsletter_repository(&self) -> Result<Self::NewsletterRepository<'_>, ContextError>;
    type CommunicationPreferenceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn communication_preference_repository(&self) -> Result<Self::CommunicationPreferenceRepository<'_>, ContextError>;
    type ProfileRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn profile_repository(&self) -> Result<Self::ProfileRepository<'_>, ContextError>;
    type MaintenanceRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn maintenance_repository(&self) -> Result<Self::MaintenanceRepository<'_>, ContextError>;
    type RepairRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn repair_repository(&self) -> Result<Self::RepairRepository<'_>, ContextError>;
    type InspectionRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn inspection_repository(&self) -> Result<Self::InspectionRepository<'_>, ContextError>;
    type SafetyCheckRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn safety_check_repository(&self) -> Result<Self::SafetyCheckRepository<'_>, ContextError>;
    type IncidentReportRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError>;
    type ClaimRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn claim_repository(&self) -> Result<Self::ClaimRepository<'_>, ContextError>;
    type PartsInventoryRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn parts_inventory_repository(&self) -> Result<Self::PartsInventoryRepository<'_>, ContextError>;
    type StockLevelRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn stock_level_repository(&self) -> Result<Self::StockLevelRepository<'_>, ContextError>;
    type ReorderPointRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn reorder_point_repository(&self) -> Result<Self::ReorderPointRepository<'_>, ContextError>;
    type SupplierRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn supplier_repository(&self) -> Result<Self::SupplierRepository<'_>, ContextError>;
    type VendorRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn vendor_repository(&self) -> Result<Self::VendorRepository<'_>, ContextError>;
    type PurchaseOrderRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn purchase_order_repository(&self) -> Result<Self::PurchaseOrderRepository<'_>, ContextError>;
    type ReceivingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn receiving_repository(&self) -> Result<Self::ReceivingRepository<'_>, ContextError>;
    type PutawayRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn putaway_repository(&self) -> Result<Self::PutawayRepository<'_>, ContextError>;
    type PickingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn picking_repository(&self) -> Result<Self::PickingRepository<'_>, ContextError>;
    type PackingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn packing_repository(&self) -> Result<Self::PackingRepository<'_>, ContextError>;
    type ShippingRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn shipping_repository(&self) -> Result<Self::ShippingRepository<'_>, ContextError>;
    type ReturnsProcessRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn returns_process_repository(&self) -> Result<Self::ReturnsProcessRepository<'_>, ContextError>;
    type QualityControlRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn quality_control_repository(&self) -> Result<Self::QualityControlRepository<'_>, ContextError>;
    type PerformanceMetricRepository<'a>: TeaqlEntityRepository + 'a
    where
        Self: 'a;

    fn performance_metric_repository(&self) -> Result<Self::PerformanceMetricRepository<'_>, ContextError>;
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
    type InvoiceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn invoice_repository(&self) -> Result<Self::InvoiceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Invoice")
    }

    type BillRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn bill_repository(&self) -> Result<Self::BillRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Bill")
    }

    type PaymentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payment_repository(&self) -> Result<Self::PaymentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payment")
    }

    type ExpenseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_repository(&self) -> Result<Self::ExpenseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Expense")
    }

    type RevenueRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn revenue_repository(&self) -> Result<Self::RevenueRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Revenue")
    }

    type LedgerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn ledger_repository(&self) -> Result<Self::LedgerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Ledger")
    }

    type AuditRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn audit_repository(&self) -> Result<Self::AuditRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Audit")
    }

    type TaxRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tax_repository(&self) -> Result<Self::TaxRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Tax")
    }

    type BudgetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn budget_repository(&self) -> Result<Self::BudgetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Budget")
    }

    type ForecastRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn forecast_repository(&self) -> Result<Self::ForecastRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Forecast")
    }

    type PayrollRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn payroll_repository(&self) -> Result<Self::PayrollRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Payroll")
    }

    type ExpenseReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn expense_report_repository(&self) -> Result<Self::ExpenseReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ExpenseReport")
    }

    type CreditRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn credit_repository(&self) -> Result<Self::CreditRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Credit")
    }

    type DebitRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn debit_repository(&self) -> Result<Self::DebitRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Debit")
    }

    type BalanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn balance_repository(&self) -> Result<Self::BalanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Balance")
    }

    type AssetRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn asset_repository(&self) -> Result<Self::AssetRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Asset")
    }

    type LiabilityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn liability_repository(&self) -> Result<Self::LiabilityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Liability")
    }

    type EquityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn equity_repository(&self) -> Result<Self::EquityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Equity")
    }

    type CashFlowRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn cash_flow_repository(&self) -> Result<Self::CashFlowRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CashFlow")
    }

    type FinancialStatementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn financial_statement_repository(&self) -> Result<Self::FinancialStatementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FinancialStatement")
    }

    type ShipmentRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn shipment_repository(&self) -> Result<Self::ShipmentRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Shipment")
    }

    type RouteRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn route_repository(&self) -> Result<Self::RouteRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Route")
    }

    type VehicleRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vehicle_repository(&self) -> Result<Self::VehicleRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Vehicle")
    }

    type DriverRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn driver_repository(&self) -> Result<Self::DriverRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Driver")
    }

    type LoadRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn load_repository(&self) -> Result<Self::LoadRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Load")
    }

    type UnloadRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn unload_repository(&self) -> Result<Self::UnloadRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Unload")
    }

    type CapacityRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn capacity_repository(&self) -> Result<Self::CapacityRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Capacity")
    }

    type ManifestRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn manifest_repository(&self) -> Result<Self::ManifestRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Manifest")
    }

    type TrackingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn tracking_repository(&self) -> Result<Self::TrackingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Tracking")
    }

    type DispatchRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn dispatch_repository(&self) -> Result<Self::DispatchRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Dispatch")
    }

    type FreightRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn freight_repository(&self) -> Result<Self::FreightRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Freight")
    }

    type CarrierRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn carrier_repository(&self) -> Result<Self::CarrierRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Carrier")
    }

    type WarehouseRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warehouse_repository(&self) -> Result<Self::WarehouseRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Warehouse")
    }

    type LoadingDockRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loading_dock_repository(&self) -> Result<Self::LoadingDockRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoadingDock")
    }

    type UnloadingDockRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn unloading_dock_repository(&self) -> Result<Self::UnloadingDockRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("UnloadingDock")
    }

    type FreightForwarderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn freight_forwarder_repository(&self) -> Result<Self::FreightForwarderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("FreightForwarder")
    }

    type CustomsRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customs_repository(&self) -> Result<Self::CustomsRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Customs")
    }

    type DocumentationRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn documentation_repository(&self) -> Result<Self::DocumentationRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Documentation")
    }

    type TollRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn toll_repository(&self) -> Result<Self::TollRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Toll")
    }

    type FuelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn fuel_repository(&self) -> Result<Self::FuelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Fuel")
    }

    type CustomerRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn customer_repository(&self) -> Result<Self::CustomerRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Customer")
    }

    type ClientRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn client_repository(&self) -> Result<Self::ClientRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Client")
    }

    type ContactRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contact_repository(&self) -> Result<Self::ContactRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Contact")
    }

    type LeadRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn lead_repository(&self) -> Result<Self::LeadRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Lead")
    }

    type ProspectRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn prospect_repository(&self) -> Result<Self::ProspectRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Prospect")
    }

    type AccountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn account_repository(&self) -> Result<Self::AccountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Account")
    }

    type ServiceAgreementRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn service_agreement_repository(&self) -> Result<Self::ServiceAgreementRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ServiceAgreement")
    }

    type ContractRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn contract_repository(&self) -> Result<Self::ContractRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Contract")
    }

    type WarrantyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn warranty_repository(&self) -> Result<Self::WarrantyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Warranty")
    }

    type SupportTicketRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn support_ticket_repository(&self) -> Result<Self::SupportTicketRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SupportTicket")
    }

    type FeedbackRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn feedback_repository(&self) -> Result<Self::FeedbackRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Feedback")
    }

    type SurveyRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn survey_repository(&self) -> Result<Self::SurveyRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Survey")
    }

    type LoyaltyProgramRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn loyalty_program_repository(&self) -> Result<Self::LoyaltyProgramRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("LoyaltyProgram")
    }

    type ReferralRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn referral_repository(&self) -> Result<Self::ReferralRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Referral")
    }

    type DiscountRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn discount_repository(&self) -> Result<Self::DiscountRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Discount")
    }

    type PromotionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn promotion_repository(&self) -> Result<Self::PromotionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Promotion")
    }

    type MarketingCampaignRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn marketing_campaign_repository(&self) -> Result<Self::MarketingCampaignRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("MarketingCampaign")
    }

    type NewsletterRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn newsletter_repository(&self) -> Result<Self::NewsletterRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Newsletter")
    }

    type CommunicationPreferenceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn communication_preference_repository(&self) -> Result<Self::CommunicationPreferenceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("CommunicationPreference")
    }

    type ProfileRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn profile_repository(&self) -> Result<Self::ProfileRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Profile")
    }

    type MaintenanceRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn maintenance_repository(&self) -> Result<Self::MaintenanceRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Maintenance")
    }

    type RepairRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn repair_repository(&self) -> Result<Self::RepairRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Repair")
    }

    type InspectionRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn inspection_repository(&self) -> Result<Self::InspectionRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Inspection")
    }

    type SafetyCheckRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn safety_check_repository(&self) -> Result<Self::SafetyCheckRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("SafetyCheck")
    }

    type IncidentReportRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn incident_report_repository(&self) -> Result<Self::IncidentReportRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("IncidentReport")
    }

    type ClaimRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn claim_repository(&self) -> Result<Self::ClaimRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Claim")
    }

    type PartsInventoryRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn parts_inventory_repository(&self) -> Result<Self::PartsInventoryRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PartsInventory")
    }

    type StockLevelRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn stock_level_repository(&self) -> Result<Self::StockLevelRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("StockLevel")
    }

    type ReorderPointRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn reorder_point_repository(&self) -> Result<Self::ReorderPointRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ReorderPoint")
    }

    type SupplierRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn supplier_repository(&self) -> Result<Self::SupplierRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Supplier")
    }

    type VendorRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn vendor_repository(&self) -> Result<Self::VendorRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Vendor")
    }

    type PurchaseOrderRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn purchase_order_repository(&self) -> Result<Self::PurchaseOrderRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PurchaseOrder")
    }

    type ReceivingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn receiving_repository(&self) -> Result<Self::ReceivingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Receiving")
    }

    type PutawayRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn putaway_repository(&self) -> Result<Self::PutawayRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Putaway")
    }

    type PickingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn picking_repository(&self) -> Result<Self::PickingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Picking")
    }

    type PackingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn packing_repository(&self) -> Result<Self::PackingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Packing")
    }

    type ShippingRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn shipping_repository(&self) -> Result<Self::ShippingRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("Shipping")
    }

    type ReturnsProcessRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn returns_process_repository(&self) -> Result<Self::ReturnsProcessRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("ReturnsProcess")
    }

    type QualityControlRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn quality_control_repository(&self) -> Result<Self::QualityControlRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("QualityControl")
    }

    type PerformanceMetricRepository<'a> = teaql_runtime::EntityDataService<'a, crate::runtime::DataServiceExecutor>
    where
        Self: 'a;

    fn performance_metric_repository(&self) -> Result<Self::PerformanceMetricRepository<'_>, ContextError> {
        self.entity_data_service::<crate::runtime::DataServiceExecutor>("PerformanceMetric")
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
