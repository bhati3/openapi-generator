/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use std::rc::Rc;
use std::borrow::Borrow;
use std::option::Option;

use reqwest;

use super::{Error, configuration};

/// struct for passing parameters to the method `create_user`
#[derive(Clone, Debug)]
pub struct CreateUserParams {
    /// Created user object
    pub body: crate::models::User
}

/// struct for passing parameters to the method `create_users_with_array_input`
#[derive(Clone, Debug)]
pub struct CreateUsersWithArrayInputParams {
    /// List of user object
    pub body: Vec<crate::models::User>
}

/// struct for passing parameters to the method `create_users_with_list_input`
#[derive(Clone, Debug)]
pub struct CreateUsersWithListInputParams {
    /// List of user object
    pub body: Vec<crate::models::User>
}

/// struct for passing parameters to the method `delete_user`
#[derive(Clone, Debug)]
pub struct DeleteUserParams {
    /// The name that needs to be deleted
    pub username: String
}

/// struct for passing parameters to the method `get_user_by_name`
#[derive(Clone, Debug)]
pub struct GetUserByNameParams {
    /// The name that needs to be fetched. Use user1 for testing.
    pub username: String
}

/// struct for passing parameters to the method `login_user`
#[derive(Clone, Debug)]
pub struct LoginUserParams {
    /// The user name for login
    pub username: String,
    /// The password for login in clear text
    pub password: String
}

/// struct for passing parameters to the method `update_user`
#[derive(Clone, Debug)]
pub struct UpdateUserParams {
    /// name that need to be deleted
    pub username: String,
    /// Updated user object
    pub body: crate::models::User
}


    pub async fn create_user(configuration: &configuration::Configuration, params: CreateUserParams) -> Result<(), Error> {
        // unbox the parameters
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/user", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn create_users_with_array_input(configuration: &configuration::Configuration, params: CreateUsersWithArrayInputParams) -> Result<(), Error> {
        // unbox the parameters
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/user/createWithArray", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn create_users_with_list_input(configuration: &configuration::Configuration, params: CreateUsersWithListInputParams) -> Result<(), Error> {
        // unbox the parameters
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/user/createWithList", configuration.base_path);
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn delete_user(configuration: &configuration::Configuration, params: DeleteUserParams) -> Result<(), Error> {
        // unbox the parameters
        let username = params.username;

        let client = &configuration.client;

        let uri_str = format!("{}/user/{username}", configuration.base_path, username=crate::apis::urlencode(username));
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn get_user_by_name(configuration: &configuration::Configuration, params: GetUserByNameParams) -> Result<crate::models::User, Error> {
        // unbox the parameters
        let username = params.username;

        let client = &configuration.client;

        let uri_str = format!("{}/user/{username}", configuration.base_path, username=crate::apis::urlencode(username));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<crate::models::User>().await?)
    }

    pub async fn login_user(configuration: &configuration::Configuration, params: LoginUserParams) -> Result<String, Error> {
        // unbox the parameters
        let username = params.username;
        let password = params.password;

        let client = &configuration.client;

        let uri_str = format!("{}/user/login", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("username", &username.to_string())]);
        req_builder = req_builder.query(&[("password", &password.to_string())]);
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        Ok(client.execute(req).await?.error_for_status()?.json::<String>().await?)
    }

    pub async fn logout_user(configuration: &configuration::Configuration) -> Result<(), Error> {
        // unbox the parameters

        let client = &configuration.client;

        let uri_str = format!("{}/user/logout", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }

    pub async fn update_user(configuration: &configuration::Configuration, params: UpdateUserParams) -> Result<(), Error> {
        // unbox the parameters
        let username = params.username;
        let body = params.body;

        let client = &configuration.client;

        let uri_str = format!("{}/user/{username}", configuration.base_path, username=crate::apis::urlencode(username));
        let mut req_builder = client.put(uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.json(&body);

        let req = req_builder.build()?;
        client.execute(req).await?.error_for_status()?;
        Ok(())
    }
