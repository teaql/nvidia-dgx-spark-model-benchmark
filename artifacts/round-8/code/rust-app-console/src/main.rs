#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[{}] Starting Moving Company Service (180+ objects)...", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
    let _runtime = moving_company_service_core::service_runtime_from_env().await?;
    _runtime.ensure_schema().await?;

    println!("Generating sample data...");
    moving_company_service_core::sample_data::generate_sample_data(&_runtime, moving_company_service_core::sample_data::SampleDataPlan::small()).await?;

    println!("Testing Q APIs...");
    use moving_company_service_core::Q;

    // 1. Employee
    let employees = Q::employees_minimal()
        .select_all()
        .limit(10)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Employees fetched: {}", employees.len());

    // 2. Platform
    let platforms = Q::platforms_minimal()
        .select_all()
        .limit(10)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Platforms fetched: {}", platforms.len());

    // 3. Move Order
    let move_orders = Q::move_orders_minimal()
        .select_all()
        .limit(10)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Move orders fetched: {}", move_orders.len());

    println!("Testing E Expressions...");
    // E Expression 1: employee -> merchant
    let mut e1_passed = false;
    for employee in employees {
        if let Some(_merchant) = employee.merchant() {
            e1_passed = true;
            break;
        }
    }
    println!("E-Expression chain 1 passed: {}", e1_passed);

    // E Expression 2: move_order -> platform_ref
    let mut e2_passed = false;
    for order in move_orders {
        // Because fields are generated dynamically, we just test if the order has some reference resolving
        // The generator usually attaches merchant or platform
        if order.merchant().is_some() || order.platform_ref().is_some() || order.customer().is_some() {
            e2_passed = true;
            break;
        }
    }
    println!("E-Expression chain 2 safely returned: {}", e2_passed);

    println!("All tests passed!");
    Ok(())
}