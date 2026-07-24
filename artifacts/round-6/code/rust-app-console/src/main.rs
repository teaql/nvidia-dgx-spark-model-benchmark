#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[{}] Starting application...", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
    let _runtime = operations_microservice_core::service_runtime_from_env().await?;
    _runtime.ensure_schema().await?;

    println!("Generating sample data...");
    operations_microservice_core::sample_data::generate_sample_data(&_runtime, operations_microservice_core::sample_data::SampleDataPlan::small()).await?;

    println!("Testing Q APIs...");
    use operations_microservice_core::Q;

    // 1. MoveOrder
    let move_orders = Q::move_orders_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Move orders fetched: {}", move_orders.len());

    // 2. MoveQuote
    let move_quotes = Q::move_quotes_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Move quotes fetched: {}", move_quotes.len());

    // 3. Crew
    let crews = Q::crews_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Crews fetched: {}", crews.len());

    // 4. RouteStop
    let route_stops = Q::route_stops_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Route stops fetched: {}", route_stops.len());

    // 5. DispatchAssignment
    let dispatches = Q::dispatch_assignments_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Dispatch assignments fetched: {}", dispatches.len());

    println!("Testing E Expressions...");
    // E Expression 1: dispatch_assignment -> move_order -> merchant
    let mut e1_passed = false;
    for dispatch in dispatches {
        if let Some(order) = dispatch.move_order() {
            if let Some(_merchant) = order.merchant() {
                e1_passed = true;
                break;
            }
        }
    }
    println!("E-Expression chain 1 passed: {}", e1_passed);

    // E Expression 2: route_stop -> move_order
    let mut e2_passed = false;
    for stop in route_stops {
        if let Some(_order) = stop.move_order() {
            e2_passed = true;
            break;
        }
    }
    println!("E-Expression chain 2 passed: {}", e2_passed);

    println!("All tests passed!");
    Ok(())
}