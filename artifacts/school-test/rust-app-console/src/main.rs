#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("[{}] Starting application...", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"));
    let _runtime = school_management_service_core::service_runtime_from_env().await?;
    _runtime.ensure_schema().await?;

    // Generate sample data
    school_management_service_core::sample_data::generate_sample_data(&_runtime, school_management_service_core::sample_data::SampleDataPlan::small()).await?;

    // Import domain items
    use school_management_service_core::domain::School;
    use school_management_service_core::domain::school;

    // Test Q API
    println!("--- Q API Test ---");
    let schools = _runtime.query::<School>().await?;
    println!("Found {} schools via Q API", schools.len());
    for s in schools.iter().take(3) {
        println!("  School ID: {}, Name: {}", s.id(), s.name());
    }

    // Test E API
    println!("--- E API Test ---");
    use school_management_service_core::E;
    let e_api_schools = _runtime.expression::<School>().filter(E::like(school::name, "%School%")).await?;
    println!("Found {} schools matching '%School%' via E API", e_api_schools.len());
    for s in e_api_schools.iter().take(3) {
         println!("  E-School ID: {}, Name: {}", s.id(), s.name());
    }

    Ok(())
}