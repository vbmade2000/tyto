use crate::core::traits::{Notifier, UserManager};
use crate::emailer::EmailNotifier;
use crate::error::Error;
use crate::types::{CreateUserRequest, Response, Status};
use crate::user_management::TytoUserManager;
use crate::Config;
use actix_web::{
    http::StatusCode,
    web::{self, HttpResponse},
};
use serde_json::json;
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

    let sender = cfg.email.sender.to_owned();
    let user_id = user_manager.create(new_user.into_inner()).await?;
    let output = json!({
        "id": user_id,
    });

    /* IMP: Not sure if this is a good idea to create new instance of EmailNotifier
     *  every time we create new user but for now it is OK to use in that way. Even if
     *  we pass it as shared data then also we need to change "To" field of email for
     *  every user which is not possible for now. For production, this functionality
     *  should be handed over to either external provider like Sendgrid or a separate
     *  service as it can introduce delay in returning response and affect speed.
     */

    // Notify a user about her newly created account.
    // TODO: Use some template crate for email body.
    let subject = String::from("Welcome to Tyto!");
    let body = String::from("Hi there, \n Your account has been created successfully with Tyto");
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
