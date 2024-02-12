use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use check_for_update::check_for_update;
use clokwerk::{Scheduler, TimeUnits};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

mod check_for_update;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

fn start_scheduler(rt: Arc<Mutex<Runtime>>) {
    let mut scheduler = Scheduler::new();
    scheduler.every(5.seconds()).run(move || {
        let rt = Arc::clone(&rt);
        let rt = rt.lock().unwrap();
        rt.spawn(async {
            println!("Scheduling");
            check_for_update().await;
        });
    });

    let _thread_handle = thread::spawn(move || loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let rt = Arc::new(Mutex::new(Runtime::new().unwrap()));
    start_scheduler(rt.clone());

    HttpServer::new(move || App::new().route("/hello", web::get().to(hello)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
