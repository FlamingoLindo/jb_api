pub fn load_env() {
    let env = std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string());
    let filename = format!(".env.{}", env);
    dotenv::from_filename(&filename)
        .unwrap_or_else(|_| panic!("Could not load env file: {}", filename));
    println!("Loaded env file: {}", filename);
}
