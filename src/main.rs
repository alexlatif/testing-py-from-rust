mod utils;
use tracing::{error, info};
use tracing_subscriber::{filter::EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(std::io::stdout))
        .with(EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".into()),
        ))
        .init();

    pyo3::prepare_freethreaded_python();

    run_tests!(test_run_python_script, test_call_python_function);
}

fn test_run_python_script() -> Result<(), Box<dyn std::error::Error>> {
    let script_path = "./testing/example.py";
    let args = ["service_b", r#"{"smoothing": 1}"#];
    utils::run_python_script(script_path, &args)?;

    Ok(())
}

fn test_call_python_function() -> Result<(), Box<dyn std::error::Error>> {
    let result: String =
        utils::call_python_function("./testing/example.py", "string_function", "test_input")?;

    info!("Function result: {}", result);
    assert_eq!(result, "test_input_processed");

    let result: i32 =
        utils::call_python_function("./testing/example.py", "sum_function", (42, 42))?;

    info!("Function result: {}", result);
    assert_eq!(result, 84);

    Ok(())
}
