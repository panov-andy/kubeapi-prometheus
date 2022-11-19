use axum::{
    Router,
    routing::get,
};
use axum::response::Response;
use procfs::net::{tcp, tcp6, TcpState};

use build_timestamp::build_time;

// generates a `const BUILD_TIME: &str`
build_time!("%A %Y-%m-%d / %H:%M:%S");

#[tokio::main]
async fn main() {
    println!("starting kube-apiserver connection metrics");

    println!("This is {}, built on: {}", env!("CARGO_PKG_NAME"), BUILD_TIME);

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { format!("Hello from kube-apiserver connection counter, build on {}", BUILD_TIME) }))
        .route("/metrics", get(|| metrics_handler()));


    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn metrics_handler() -> Response<String> {
    let mut connection_counter = 0;

    for entry in tcp().unwrap().iter() {
        if entry.local_address.port() == "443".parse::<u16>().unwrap() {
            if entry.state == TcpState::Established {
                connection_counter = connection_counter + 1;
            }
        }
    }
    for entry in tcp6().unwrap().iter() {
        if entry.local_address.port() == "443".parse::<u16>().unwrap() {
            if entry.state == TcpState::Established {
                connection_counter = connection_counter + 1;
            }
        }
    }

    println!("GET /metrics -> {}", connection_counter);
    Response::new(format!("# TYPE apiserver_connections_total counter
apiserver_connections_total {}\n", connection_counter))
}




