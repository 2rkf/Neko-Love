use axum::{body::Body, http::Request, middleware::Next, response::Response};
use chrono::{DateTime, Local};
use colored::Colorize;
use std::time::{Instant, SystemTime};

pub async fn log_requests(req: Request<Body>, next: Next) -> Response {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    let now = SystemTime::now();
    let datetime: DateTime<Local> = now.into();
    let timestamp = datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

    let method_colored = match method.as_str() {
        "GET" => method.bright_purple(),
        "POST" => method.blue(),
        "PUT" | "PATCH" => method.yellow(),
        "DELETE" => method.red(),
        _ => method.normal(),
    };

    let response = next.run(req).await;
    let duration = start.elapsed();
    let status = response.status().as_u16();

    let status_colored = match status {
        200..=299 => status.to_string().green(),
        300..=399 => status.to_string().cyan(),
        400..=499 => status.to_string().yellow(),
        500..=599 => status.to_string().red(),
        _ => status.to_string().normal(),
    };

    println!(
        "[{}] {} {} {} {}ms",
        timestamp.dimmed(),
        method_colored,
        path.bright_cyan(),
        status_colored,
        duration.as_millis().to_string().purple()
    );

    response
}
