#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[{}] Starting application...", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
    let _runtime = hr_payroll_microservice_core::service_runtime_from_env().await?;
    _runtime.ensure_schema().await?;

    println!("Generating sample data...");
    hr_payroll_microservice_core::sample_data::generate_sample_data(&_runtime, hr_payroll_microservice_core::sample_data::SampleDataPlan::small()).await?;

    println!("Testing Q APIs...");
    use hr_payroll_microservice_core::Q;

    // 1. Employee
    let employees = Q::employees_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Employees fetched: {}", employees.len());

    // 2. LeaveRequest
    let leaves = Q::leave_requests_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Leave requests fetched: {}", leaves.len());

    // 3. SalaryRecord
    let salaries = Q::salary_records_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Salary records fetched: {}", salaries.len());

    // 4. JobApplication
    let apps = Q::job_applications_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Job applications fetched: {}", apps.len());

    // 5. Contract
    let contracts = Q::contracts_minimal()
        .select_all()
        .limit(20)
        .comment("what: testing")
        .purpose("why: testing")
        .execute_for_list(&_runtime).await?;
    println!("Contracts fetched: {}", contracts.len());

    println!("Testing E Expressions...");
    // E Expression 1: leave_request -> employee -> position -> department
    let mut e1_passed = false;
    for leave in leaves {
        if let Some(emp) = leave.employee() {
            if let Some(pos) = emp.position() {
                if let Some(_dept) = pos.department() {
                    e1_passed = true;
                    break;
                }
            }
        }
    }
    println!("E-Expression chain 1 passed: {}", e1_passed);

    // E Expression 2: contract -> employee -> merchant
    let mut e2_passed = false;
    for contract in contracts {
        if let Some(emp) = contract.employee() {
            if let Some(_merchant) = emp.merchant() {
                e2_passed = true;
                break;
            }
        }
    }
    println!("E-Expression chain 2 passed: {}", e2_passed);

    println!("All tests passed!");
    Ok(())
}