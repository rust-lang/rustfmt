fn main() {
lazy_static!(
        static ref DYNAMODB_CLIENT: Option<aws_sdk_dynamodb::Client> = None;
            static ref CASCADE_IP: String = std::env::var("CASCADE_IP").unwrap_or("127.0.0.1".to_string());
                static ref CASCADE_PORT: String = std::env::var("CASCADE_PORT").unwrap_or("4000".to_string());
)     ;
}
