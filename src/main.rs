use authenticator::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run(false).await?.await
}