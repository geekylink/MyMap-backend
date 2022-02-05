use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_session::CookieSession;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;

mod api;
mod upload;
mod user;

// Defaults
const DEFAULT_WWW_PATH: &str = "./www/build/";
const DEFAULT_INDEX: &str = "index.html";
const DEFAULT_ADDRESS: &str = "0.0.0.0";
const DEFAULT_PORT: u32 = 8080;

pub struct APIServer {
    pub full_address: String,
    pub use_auth_api: bool,
}

impl APIServer {
    pub fn new_from_full_address(full_address: &String) -> APIServer {
        let api = APIServer {
            full_address: full_address.to_string(),
            use_auth_api: true,
        };

        api
    }

    pub fn new(address: &str, port: i32) -> APIServer {
        let mut address_s = &address;
        let mut port_s = format!("{}", port);

        if address == "" {
            address_s = &DEFAULT_ADDRESS;
        }

        if port == -1 {
            port_s = format!("{}", DEFAULT_PORT);
        }

        let full_address = format!("{}:{}", address_s, port_s);
        APIServer::new_from_full_address(&full_address)
    }

    pub fn disable_auth_api(&mut self) {
        // Disables all authenticated api calls, user login, etc
        self.use_auth_api = false;
        println!("Disabled auth api, no writes will be possible.");
    }

    pub async fn launch_server(&self) -> std::io::Result<()> {
        println!("Launching server on: http://{}", self.full_address);

        // Enable logging
        env_logger::init_from_env(Env::default().default_filter_or("info"));

        let use_auth_api = self.use_auth_api;

        HttpServer::new(move || {
            let app = App::new()
                .wrap(Logger::default()) // Logging
                .wrap(Logger::new("%a %{User-Agent}i"))
                .wrap(CookieSession::signed(&[0; 32]).secure(false))
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&[0; 32]) // <- create cookie identity policy
                        .name("auth-cookie")
                        .secure(false),
                ));

            let app = match use_auth_api {
                true => app
                    .service(web::scope("/upload").service(upload::save_file::photo))
                    .service(
                        web::scope("/user")
                            .service(user::login::index)
                            .service(user::login::login)
                            .service(user::login::logout),
                    ),
                false => app,
            };

            // General non-authenticated API calls
            let scope = web::scope("/api").service(api::locations::get_location_files);

            // Authenticated API calls
            let scope = match use_auth_api {
                true => scope.service(api::locations::save_location),
                false => scope,
            };

            app.service(scope)
                .service(fs::Files::new("/", DEFAULT_WWW_PATH).index_file(DEFAULT_INDEX))
            // Root webapp
        })
        .bind(&self.full_address)?
        .run()
        .await
    }
}
