use serial_test::serial;
use tasks_authenticated::{
    AppConfig, AppEnvironment,
    context::JwtState,
    models::auth::{LoginUser, RegisterUser},
    repositories::users::User,
};

async fn seed_data(config: &AppConfig) {
    config.db().recreate().await.unwrap();

    let users = [
        RegisterUser {
            username: "user1".into(),
            email: "user1@mail.com".into(),
            password: "Password".into(),
            confirm_password: "Password".into(),
        },
        RegisterUser {
            username: "user2".into(),
            email: "user2@mail.com".into(),
            password: "Password".into(),
            confirm_password: "Password".into(),
        },
    ];

    for user in users {
        let registered = User::create_with_password(&config.db().connection_pool().unwrap(), &user)
            .await
            .unwrap();
        tracing::info!("Registered success: {}", registered.username.as_str());
    }
}

#[tokio::test]
#[serial]
async fn can_create_new_user() {
    let config = AppConfig::from_env(&AppEnvironment::Development).unwrap();
    config.db().recreate().await.unwrap();

    let params = RegisterUser {
        username: "example".into(),
        email: "example@mail.com".into(),
        password: "Password".into(),
        confirm_password: "Password".into(),
    };

    let result = User::create_with_password(&config.db().connection_pool().unwrap(), &params).await;

    assert!(result.is_ok());
}

#[tokio::test]
#[serial]
async fn can_handle_redundant_email() {
    let config = AppConfig::from_env(&AppEnvironment::Development).unwrap();
    seed_data(&config).await;

    let params = RegisterUser {
        username: "testOne".into(),
        email: "user1@mail.com".into(),
        password: "Password".into(),
        confirm_password: "Password".into(),
    };

    let result = User::create_with_password(&config.db().connection_pool().unwrap(), &params).await;

    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn can_handle_redundant_username() {
    let config = AppConfig::from_env(&AppEnvironment::Development).unwrap();
    seed_data(&config).await;

    let params = RegisterUser {
        username: "user1".into(),
        email: "example@mail.com".into(),
        password: "Password".into(),
        confirm_password: "Password".into(),
    };

    let result = User::create_with_password(&config.db().connection_pool().unwrap(), &params).await;

    assert!(result.is_err());
}

#[tokio::test]
#[serial]
async fn can_login_user() {
    let config = AppConfig::from_env(&AppEnvironment::Development).unwrap();
    seed_data(&config).await;

    let params = LoginUser {
        email: "user1@mail.com".into(),
        password: "Password".into(),
    };

    let auth = JwtState::new(
        &config.auth().access.private_key,
        &config.auth().access.public_key,
        config.auth().access.expiration,
    )
    .unwrap();
    let result = User::login_user(&config.db().connection_pool().unwrap(), &params, &auth).await;

    assert!(result.is_ok());
}
