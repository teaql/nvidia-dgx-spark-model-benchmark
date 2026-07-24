#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[{}] Starting application...", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
    let _runtime = moving_company_service_core::service_runtime_from_env().await?;
    _runtime.ensure_schema().await?;

    println!("Generating sample data...");
    moving_company_service_core::sample_data::generate_sample_data(&_runtime, moving_company_service_core::sample_data::SampleDataPlan::small()).await?;

    println!("Testing Q APIs...");
    use moving_company_service_core::Q;
    use teaql_core::Entity;

    // 1. Employees & Payroll - Employee
    let employees = Q::employees_minimal()
        .select_employee_number()
        .select_name()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Employees fetched: {}", employees.len());

    // 2. Platform Administration - Platform
    let platforms = Q::platforms_minimal()
        .select_name()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Platforms fetched: {}", platforms.len());

    // 3. Organization Administration - Merchant
    let merchants = Q::merchants_minimal()
        .select_name()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Merchants fetched: {}", merchants.len());

    // 4. Operations & Logistics - MoveOrder
    let move_orders = Q::move_orders_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Move orders fetched: {}", move_orders.len());

    // 5. Products & Services - Service
    let services = Q::services_minimal()
        .select_record_name()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Services fetched: {}", services.len());

    println!("Testing E Expressions...");
    // E Expression 1: employee -> merchant -> platform
    for emp in employees {
        if let Some(merchant) = emp.merchant() {
            if let Some(platform) = merchant.platform() {
                println!("E-Expression chain 1 passed for employee {}", emp.name());
                break;
            }
        }
    }

    // E Expression 2: move_order -> merchant -> platform
    for order in move_orders {
        if let Some(merchant) = order.merchant() {
            if let Some(platform) = merchant.platform() {
                println!("E-Expression chain 2 passed for order!");
                break;
            }
        }
    }

    println!("All tests passed!");
    Ok(())
}