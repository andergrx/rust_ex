use axum::{
    routing::get,
    Router,
    Json,
    http::Method,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tokio::net::TcpListener;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// ✅ JSON Struct
#[derive(Serialize, Clone)]
struct AppData {
    id: u32,
    name: String,
    title: String,
    count: i32,
}

impl AppData {

    fn new(id: u32, name: &str, title: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            title: title.to_string(),
            count: 0,
        }
    }

    fn update(&mut self) {
        self.count += 1;
    }

}

static GLOBAL_STATE: Lazy<Mutex<AppData>> = Lazy::new(|| Mutex::new(
    AppData::new(99, "Global State", "Global Title")
));

#[tokio::main]
async fn main() {
    // ✅ Define the CORS middleware correctly
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins
        .allow_methods([Method::GET, Method::POST]); // Allow GET and POST methods

    // ✅ Apply CORS middleware directly (No ServiceBuilder needed)
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/hello", get(hello_handler))
        .route("/data", get(data_handler))
        .layer(cors); // ✅ This now works!

    // ✅ Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ✅ Route Handlers
async fn root_handler() -> &'static str {
    "Welcome to the root!"
}

async fn hello_handler() -> &'static str {
    "Hello, world!"
}

async fn data_handler() -> Json<AppData> {

    let mut state = GLOBAL_STATE.lock().unwrap(); // Lock the mutex
    state.update();

    Json(state.clone())
}

