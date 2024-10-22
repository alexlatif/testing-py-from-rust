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

    run_tests!(test_run_python_script, test_call_python_function);
}

fn test_run_python_script() -> Result<(), Box<dyn std::error::Error>> {
    let script_path = "../src/bar.py";
    let args = ["service_b", r#"{"smoothing": 1}"#];
    utils::run_python_script(script_path, &args)?;

    Ok(())
}

fn test_call_python_function() -> Result<(), Box<dyn std::error::Error>> {
    pyo3::prepare_freethreaded_python();
    let script_path = "../src/bar.py";
    let function_name = "my_test_function";
    let input_data = "test_input";

    let result = utils::call_python_function(script_path, function_name, input_data)?;

    info!("Function result: {}", result);
    assert_eq!(result, "test_input_processed");

    Ok(())
}
