use anyhow::Result;
use common::counter::Counter;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{self, EnvFilter};
mod common;
/// npx @modelcontextprotocol/inspector cargo run -p mcp-server-examples --example std_io
#[tokio::main]
//#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    // Create an instance of our counter router
    let service = Counter::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}

// Custom pow implementation without using exp, ln, or other std math functions
#[unsafe(no_mangle)]
pub extern "Rust" fn pow(mut x: f64, mut y: f64) -> f64 {
    // Handle special cases
    if y == 0.0 {
        return 1.0;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return 1.0;
    }

    // Handle negative base with integer exponent
    if x < 0.0 && y.fract() == 0.0 {
        let abs_result = pow_positive(-x, y);
        if y as i64 % 2 == 0 {
            return abs_result; // Even exponent
        } else {
            return -abs_result; // Odd exponent
        }
    }

    // For positive base and integer exponent, use pow_positive
    if x > 0.0 && y.fract() == 0.0 {
        return pow_positive(x, y);
    }

    // For other cases (fractional exponents, negative base with non-integer exponent), return NaN
    f64::NAN
}

// Fast exponentiation by squaring for integer exponents
fn pow_positive(mut base: f64, mut exp: f64) -> f64 {
    if base <= 0.0 {
        return f64::NAN;
    }
    let mut result = 1.0;
    let mut n = exp.abs() as u64;
    let mut base_power = base;

    while n > 0 {
        if n % 2 == 1 {
            result *= base_power;
        }
        base_power *= base_power;
        n /= 2;
    }

    if exp < 0.0 { 1.0 / result } else { result }
}
