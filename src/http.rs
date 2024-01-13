use std::{
    io::{BufReader, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
};

use crate::{prelude::*, thread_pool::ThreadPool, config::Config};

pub struct HttpServer<T>
where
    T: HttpApplication + 'static,
{
    listener: TcpListener,
    pool: ThreadPool,
    application: Arc<T>,
}

impl<T> HttpServer<T>
where
    T: HttpApplication + 'static,
{
    pub fn new(config: &Config, application: T) -> Self {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).unwrap();
        let pool = ThreadPool::new(8);

        Self {
            listener,
            pool,
            application: Arc::new(application),
        }
    }

    pub fn run(self) {
        for stream in self.listener.incoming() {
            if let Ok(stream) = stream {
                let app_clone = Arc::clone(&self.application);
                self.pool.execute(move || {
                    HttpServer::handle_connection(&app_clone, stream);
                });
            }
        }
    }

    fn handle_connection(app: &Arc<T>, mut stream: TcpStream) {
        if let Ok(req) = Request::parse(BufReader::new(&mut stream)) {
            let response = app.handle_request(req);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        } else {
            let response = serve_file("res/bad_request.html", StatusCode::BadRequest);
            stream.write_all(response.to_string().as_bytes()).unwrap();
        }
    }
}

pub trait HttpApplication: Send + Sync {
    fn handle_request(&self, req: Request) -> Response;
}

pub fn run<T>(application: T)
where
    T: HttpApplication + 'static,
{
    let config = Config::load();
    let server = HttpServer::new(&config, application);
    server.run();
}
