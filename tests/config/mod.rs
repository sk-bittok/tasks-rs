use tasks_authenticated::config::{AppConfig, AppEnvironment};

#[test]
fn can_build_from_env() {
    let env = AppEnvironment::Development;
    let result = AppConfig::from_env(&env);

    assert!(result.is_ok());
}

#[test]
fn can_build_server_config() {
    let env = AppEnvironment::Development;
    let result = AppConfig::from_env(&env).unwrap();

    let server = result.server().to_string();

    assert_eq!(server, "http://localhost:5150".to_string());
}

#[test]
fn cannot_build_from_missing_file() {
    let env = AppEnvironment::Other("base".into());
    let result = AppConfig::from_env(&env);

    assert!(result.is_err());
}
