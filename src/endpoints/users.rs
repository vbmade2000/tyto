use crate::core::traits::{Notifier, UserManager};
use crate::emailer::EmailNotifier;
use crate::error::Error;
use crate::types::{self, CreateUserRequest, LoginRequest, Response, Status};
use crate::user_management::TytoUserManager;
use crate::Config;
use actix_web::{
    http::StatusCode,
    web::{self},
    HttpResponse,
};
use serde_json::{self, json};
use validator::validate_email;

pub async fn create_user(
    new_user: web::Json<CreateUserRequest>,
    user_manager: web::Data<TytoUserManager>,
    cfg: web::Data<Config>,
) -> Result<HttpResponse, Error> {
    // Extract and validate email
    let email = new_user.email.clone();
    if !validate_email(&email) {
        // TODO: Log error here
        println!("Invalid email found");
        return Err(Error::InvalidEmail);
    }

    // Read configurations
    let sender = cfg.email.sender.to_owned();
    let activation_url = cfg.activation_url.to_owned();

    let (user_id, activation_code) = user_manager.create(new_user.into_inner()).await?;

    let output = json!({
        "id": user_id,
    });

    // Notify a user about her newly created account.
    // TODO: Use some template crate for email body.
    let subject = String::from("Welcome to Tyto!");
    // Endpoint: www.localhost:8442/api/v1/users/{code}/activate
    let mut body = String::from(
        r#"Hi there,
                
           Your accouant in Tyto is successfully created.
                  
           Please visit {activation_url}/{code}.
           
           Regards,
           Tyto Team"#,
    );

    // Replace placeholders with actual activation code
    body = body.replace("{code}", &activation_code);
    body = body.replace("{activation_url}", &activation_url);

    /* |--IMP--|: As an alternative to creating new instance of EmailNotifier every time here, we can
     * have a single instance wrapped in Arc<Mutex>. In current scenario where email sending is
     * not performed in separate task/thread (it is sent in separate thread but it still takes time
     * so it has to be sent in separate tokio task), it can slow down the whole process. But this is
     * inevitable in future when the system matures.
     */
    let emailer = EmailNotifier::new(cfg, sender, email, subject, body);

    // TODO: Use log here
    match emailer.send().await {
        Ok(_) => println!("Email sent sucessfully"),
        Err(e) => println!("Error: {:?}", e),
    }

    let response = Response {
        status: Status::Success,
        message: None,
        data: output,
    };

    Ok(HttpResponse::build(StatusCode::CREATED).json(response))
}

/// Activates the user account if the provided activation code is valid.
pub async fn activate(
    activation_code: web::Path<String>,
    user_manager: web::Data<TytoUserManager>,
) -> Result<HttpResponse, Error> {
    let activation_code = activation_code.into_inner();
    user_manager.activate(activation_code).await?;

    Ok(HttpResponse::build(StatusCode::OK).finish())
}

pub async fn get_all_users(
    user_manager: web::Data<TytoUserManager>,
) -> Result<HttpResponse, Error> {
    let users = user_manager.get_all().await?;
    // Prepare response
    let response = types::Response {
        status: types::Status::Success,
        message: None,
        data: serde_json::to_value(users).unwrap(),
    };

    Ok(HttpResponse::build(StatusCode::OK).json(response))
}

/// Delete a user
pub async fn delete_user(
    user_id: web::Path<i64>,
    user_manager: web::Data<TytoUserManager>,
) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();
    user_manager.delete(user_id).await?;
    Ok(HttpResponse::build(StatusCode::OK).finish())
}

/// User login
pub async fn login(
    login_request: web::Json<LoginRequest>,
    user_manager: web::Data<TytoUserManager>,
) -> Result<HttpResponse, Error> {
    // TODO: Return 404 code. Currently it returns 500 because it is difficult to have disctinction between different database errors.
    let login_request = login_request.into_inner();
    let token = user_manager.login(login_request).await?;

    let data = json!({
        "token": token,
    });

    // Prepare response
    let response = types::Response {
        status: types::Status::Success,
        message: None,
        data: serde_json::to_value(data).unwrap(),
    };

    Ok(HttpResponse::build(StatusCode::OK).json(response))
}
