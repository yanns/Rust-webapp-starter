use actix_web::*;
use futures::future::Future;
use api::index::State;
use model::user::{SignupUser, SigninUser};

pub fn signup(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.clone().json()                     
       .from_err()
       .and_then(move |signup_user: SignupUser| {  
            req.state().db.send(SignupUser{ 
                username: signup_user.username,
                email: signup_user.email,
                password: signup_user.password,
                confirm_password: signup_user.confirm_password,
            })         
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(signup_msg) => Ok(httpcodes::HTTPOk.build().json(signup_msg)?),
                    Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                }
            })
        }).responder()
}

pub fn signin(req: HttpRequest<State>) -> Box<Future<Item=HttpResponse, Error=Error>> {
    req.clone().json()                   
       .from_err()
       .and_then(move |signin_user: SigninUser| {  
            req.state().db.send(SigninUser{ 
                username: signin_user.username,
                password: signin_user.password,
            })         
            .from_err()
            .and_then(|res| {
                match res {
                    Ok(signin_msg) => Ok(httpcodes::HTTPOk.build().json(signin_msg)?),
                    Err(_) => Ok(httpcodes::HTTPInternalServerError.into())
                }
            })
        }).responder()
}