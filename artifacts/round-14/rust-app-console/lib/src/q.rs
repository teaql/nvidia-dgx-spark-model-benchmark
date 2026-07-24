use teaql_core::Expr;

use crate::*;

pub struct PurposedQuery<T> {
    pub inner: T,
    pub purpose: String,
}

impl<T> PurposedQuery<T> {
    pub fn new(inner: T, purpose: impl Into<String>) -> Self {
        Self { inner, purpose: purpose.into() }
    }
}

pub struct Q;

impl Q {
    pub fn platforms() -> PlatformRequest {
        PlatformRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_minimal() -> PlatformRequest {
        PlatformRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn platforms_with_children() -> PlatformRequest {
        PlatformRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn merchants() -> MerchantRequest {
        MerchantRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchants_minimal() -> MerchantRequest {
        MerchantRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn merchants_with_children() -> MerchantRequest {
        MerchantRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn move_orders() -> MoveOrderRequest {
        MoveOrderRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_orders_minimal() -> MoveOrderRequest {
        MoveOrderRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn move_orders_with_children() -> MoveOrderRequest {
        MoveOrderRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn employees() -> EmployeeRequest {
        EmployeeRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_minimal() -> EmployeeRequest {
        EmployeeRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn employees_with_children() -> EmployeeRequest {
        EmployeeRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn customers() -> CustomerRequest {
        CustomerRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customers_minimal() -> CustomerRequest {
        CustomerRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn customers_with_children() -> CustomerRequest {
        CustomerRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn products() -> ProductRequest {
        ProductRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn products_minimal() -> ProductRequest {
        ProductRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn products_with_children() -> ProductRequest {
        ProductRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn campaigns() -> CampaignRequest {
        CampaignRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaigns_minimal() -> CampaignRequest {
        CampaignRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn campaigns_with_children() -> CampaignRequest {
        CampaignRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn payments() -> PaymentRequest {
        PaymentRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payments_minimal() -> PaymentRequest {
        PaymentRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn payments_with_children() -> PaymentRequest {
        PaymentRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn vehicles() -> VehicleRequest {
        VehicleRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicles_minimal() -> VehicleRequest {
        VehicleRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn vehicles_with_children() -> VehicleRequest {
        VehicleRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn contracts() -> ContractRequest {
        ContractRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contracts_minimal() -> ContractRequest {
        ContractRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn contracts_with_children() -> ContractRequest {
        ContractRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn user_accounts() -> UserAccountRequest {
        UserAccountRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_accounts_minimal() -> UserAccountRequest {
        UserAccountRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn user_accounts_with_children() -> UserAccountRequest {
        UserAccountRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn activity_logs() -> ActivityLogRequest {
        ActivityLogRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn activity_logs_minimal() -> ActivityLogRequest {
        ActivityLogRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn activity_logs_with_children() -> ActivityLogRequest {
        ActivityLogRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn notifications() -> NotificationRequest {
        NotificationRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notifications_minimal() -> NotificationRequest {
        NotificationRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn notifications_with_children() -> NotificationRequest {
        NotificationRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }

    pub fn api_clients() -> ApiClientRequest {
        ApiClientRequest::new()
            .select_self()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_clients_minimal() -> ApiClientRequest {
        ApiClientRequest::new()
            .and_filter(Expr::gt("version", 0_i64))
    }

    pub fn api_clients_with_children() -> ApiClientRequest {
        ApiClientRequest::new()
            .unlimited()
            .select_self_fields()
            .enhance_children_if_needed()
    }
}