use actix_files::NamedFile;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::{
    cookie::Key, get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::path::PathBuf;

#[get("/")]
async fn index() -> impl Responder {
    let path = "index/index.html";

    NamedFile::open_async(SITE_PATH.parse::<PathBuf>().unwrap().join(path)).await
}

#[post("/api")]
async fn graphql_api(
    schema: web::Data<AppGraphQLSchema>,
    session: Session,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let auth_token = session.get::<String>("auth_token").unwrap();
    let session = Shared::new(session);

    if auth_token.is_none() {
        schema.execute(req.into_inner().data(session)).await.into()
    } else {
        schema
            .execute(req.into_inner().data(auth_token.unwrap()).data(session))
            .await
            .into()
    }
}

const SITE_PATH: &'static str = "sites/";

async fn app_service(
    req: HttpRequest,
    site_configuration: web::Data<SiteConfiguration>,
    session: Session,
) -> AppResponse<Redirect, HttpResponse> {
    let site_path = SITE_PATH.parse::<std::path::PathBuf>().unwrap();
    let path = req.match_info().query("path").to_owned();
    let path_as_vec = path.as_paths();

    if path_as_vec.last().unwrap().is_file() {
        return AppResponse::File(NamedFile::open(site_path.join(path)).ok());
    }

    let category = site_configuration.determine_category(path_as_vec.first().unwrap());
    let category = match category {
        Some(category) => category,
        None => return AppResponse::Status(HttpResponse::NotFound().body("Not Found")),
    };

    match category {
        actix_server_backend::configs::SiteCategory::SinglePageApplication(config) => {
            let auth_token = session.get::<String>("auth_token").unwrap();
            let corresponding_rule = config.rules.iter().filter_one(path.clone());

            let app_name = path_as_vec.first().unwrap();
            let index_path = format!("{app_name}/index.html");
            let index_file = NamedFile::open(site_path.join(index_path));

            if corresponding_rule.is_none() {
                return AppResponse::File(index_file.ok());
            }

            let redirect =
                AppResponse::Redirect(Redirect::to(corresponding_rule.as_ref().unwrap().redirect));
            let secret = dotenv::var("SECRET").unwrap();

            println!("session: {:?}", session.get::<String>("auth_token"));

            if !corresponding_rule.as_ref().unwrap().on_fail {
                if auth_token.is_none() {
                    return AppResponse::File(index_file.ok());
                }

                let token = jsonwebtoken::decode::<AuthClaims>(
                    auth_token.as_ref().unwrap(),
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                );
                match token {
                    Ok(_) => {}
                    Err(_) => {
                        session.remove("auth_token");
                        return AppResponse::File(index_file.ok());
                    }
                }

                return redirect;
            }

            if auth_token.is_none() {
                return redirect;
            }

            let token = jsonwebtoken::decode::<AuthClaims>(
                auth_token.as_ref().unwrap(),
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            );
            let (is_admin, _id) = match token {
                Ok(token) => (token.claims.admin, token.claims.id),
                Err(_) => {
                    session.remove("auth_token");
                    return redirect;
                }
            };

            if corresponding_rule.unwrap().admin_only && !is_admin {
                return redirect;
            }

            return AppResponse::File(index_file.ok());
        }
        actix_server_backend::configs::SiteCategory::StaticSite(_config) => {
            return AppResponse::File(
                NamedFile::open(site_path.join(format!("{}/index.html", path_as_vec.join("/"))))
                    .ok(),
            );
        }
    };
}

use actix_server_backend::{
    configs::{SPAConfig, SiteConfiguration, StaticConfig},
    extensions::{FilterOneExt, Shared, VecPathExt},
    graphql_resolvers::{AppGraphQLSchema, Mutation, Query},
    jwt_claims::AuthClaims,
    responders::responses::AppResponse,
    ConnectionPool,
};
use actix_web_lab::web::Redirect;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use jsonwebtoken::{DecodingKey, Validation};

fn get_site_config() -> SiteConfiguration {
    let mut site_configuration = SiteConfiguration::new();

    let mut snake_config = SPAConfig::new("snake");
    snake_config.restrict_route("life", "/snake");
    snake_config.restrict_route_to_admin("admin", "/snake");

    let qwik_test_config = StaticConfig::new("qwik-test");

    site_configuration.add_static(qwik_test_config);
    site_configuration.add_spa(snake_config);

    site_configuration
}

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file!");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let connection_pool: ConnectionPool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool!");

    use diesel_migrations::MigrationHarness;

    let harness = &mut connection_pool.get().unwrap() as &mut PgConnection;
    harness.run_pending_migrations(MIGRATIONS).unwrap();

    let site_configuration = web::Data::new(get_site_config());

    let schema: AppGraphQLSchema = Schema::build(Query, Mutation, EmptySubscription)
        .data(connection_pool.clone())
        .finish();
    let schema = web::Data::new(schema);
    let connection_pool = web::Data::new(connection_pool);

    HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    CookieSessionStore::default(),
                    Key::from(dotenv::var("SECRET").unwrap().as_bytes()),
                )
                .cookie_secure(false)
                .build(),
            )
            .app_data(site_configuration.clone())
            .app_data(schema.clone())
            .app_data(connection_pool.clone())
            .service(index)
            .service(graphql_api)
            .route("/{path}*", web::get().to(app_service))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
