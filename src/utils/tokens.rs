use dotenv::dotenv;

pub fn load_token(token_name: String) -> String {
    dotenv().ok();
    std::env::var(token_name).expect("\n\nVariable was not found inside the .env file!\n\n")
}
