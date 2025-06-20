/*
 * Pipedrive API v1
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::v1::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`add_goal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AddGoalError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_goal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteGoalError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_goal_result`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGoalResultError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_goals`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetGoalsError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_goal`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateGoalError {
    UnknownValue(serde_json::Value),
}


/// Adds a new goal. Along with adding a new goal, a report is created to track the progress of your goal.
pub async fn add_goal(configuration: &configuration::Configuration, add_goal_request: Option<models::AddGoalRequest>) -> Result<models::UpsertGoalResponse, Error<AddGoalError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_add_goal_request = add_goal_request;

    let uri_str = format!("{}/goals", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-token", value);
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_add_goal_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UpsertGoalResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UpsertGoalResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<AddGoalError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Marks a goal as deleted.
pub async fn delete_goal(configuration: &configuration::Configuration, id: &str) -> Result<models::DeleteGoalResponse, Error<DeleteGoalError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;

    let uri_str = format!("{}/goals/{id}", configuration.base_path, id=crate::v1::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::DELETE, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-token", value);
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DeleteGoalResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DeleteGoalResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DeleteGoalError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Gets the progress of a goal for the specified period.
pub async fn get_goal_result(configuration: &configuration::Configuration, id: &str, period_period_start: String, period_period_end: String) -> Result<models::GetGoalResultResponse, Error<GetGoalResultError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_period_period_start = period_period_start;
    let p_period_period_end = period_period_end;

    let uri_str = format!("{}/goals/{id}/results", configuration.base_path, id=crate::v1::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("period.start", &p_period_period_start.to_string())]);
    req_builder = req_builder.query(&[("period.end", &p_period_period_end.to_string())]);
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-token", value);
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetGoalResultResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetGoalResultResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGoalResultError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Returns data about goals based on criteria. For searching, append `{searchField}={searchValue}` to the URL, where `searchField` can be any one of the lowest-level fields in dot-notation (e.g. `type.params.pipeline_id`; `title`). `searchValue` should be the value you are looking for on that field. Additionally, `is_active=<true|false>` can be provided to search for only active/inactive goals. When providing `period.start`, `period.end` must also be provided and vice versa.
pub async fn get_goals(configuration: &configuration::Configuration, type_period_name: Option<&str>, title: Option<&str>, is_active: Option<bool>, assignee_period_id: Option<i32>, assignee_period_type: Option<&str>, expected_outcome_period_target: Option<f64>, expected_outcome_period_tracking_metric: Option<&str>, expected_outcome_period_currency_id: Option<i32>, type_period_params_period_pipeline_id: Option<Vec<i32>>, type_period_params_period_stage_id: Option<i32>, type_period_params_period_activity_type_id: Option<Vec<i32>>, period_period_start: Option<String>, period_period_end: Option<String>) -> Result<models::GetGoalsResponse, Error<GetGoalsError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_type_period_name = type_period_name;
    let p_title = title;
    let p_is_active = is_active;
    let p_assignee_period_id = assignee_period_id;
    let p_assignee_period_type = assignee_period_type;
    let p_expected_outcome_period_target = expected_outcome_period_target;
    let p_expected_outcome_period_tracking_metric = expected_outcome_period_tracking_metric;
    let p_expected_outcome_period_currency_id = expected_outcome_period_currency_id;
    let p_type_period_params_period_pipeline_id = type_period_params_period_pipeline_id;
    let p_type_period_params_period_stage_id = type_period_params_period_stage_id;
    let p_type_period_params_period_activity_type_id = type_period_params_period_activity_type_id;
    let p_period_period_start = period_period_start;
    let p_period_period_end = period_period_end;

    let uri_str = format!("{}/goals/find", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_type_period_name {
        req_builder = req_builder.query(&[("type.name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_title {
        req_builder = req_builder.query(&[("title", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_is_active {
        req_builder = req_builder.query(&[("is_active", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_assignee_period_id {
        req_builder = req_builder.query(&[("assignee.id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_assignee_period_type {
        req_builder = req_builder.query(&[("assignee.type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_expected_outcome_period_target {
        req_builder = req_builder.query(&[("expected_outcome.target", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_expected_outcome_period_tracking_metric {
        req_builder = req_builder.query(&[("expected_outcome.tracking_metric", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_expected_outcome_period_currency_id {
        req_builder = req_builder.query(&[("expected_outcome.currency_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_type_period_params_period_pipeline_id {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("type.params.pipeline_id".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("type.params.pipeline_id", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_type_period_params_period_stage_id {
        req_builder = req_builder.query(&[("type.params.stage_id", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_type_period_params_period_activity_type_id {
        req_builder = match "multi" {
            "multi" => req_builder.query(&param_value.into_iter().map(|p| ("type.params.activity_type_id".to_owned(), p.to_string())).collect::<Vec<(std::string::String, std::string::String)>>()),
            _ => req_builder.query(&[("type.params.activity_type_id", &param_value.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]),
        };
    }
    if let Some(ref param_value) = p_period_period_start {
        req_builder = req_builder.query(&[("period.start", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_period_period_end {
        req_builder = req_builder.query(&[("period.end", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-token", value);
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GetGoalsResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GetGoalsResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<GetGoalsError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Updates an existing goal.
pub async fn update_goal(configuration: &configuration::Configuration, id: &str, basic_goal_request: Option<models::BasicGoalRequest>) -> Result<models::UpsertGoalResponse, Error<UpdateGoalError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_basic_goal_request = basic_goal_request;

    let uri_str = format!("{}/goals/{id}", configuration.base_path, id=crate::v1::apis::urlencode(p_id));
    let mut req_builder = configuration.client.request(reqwest::Method::PUT, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("x-api-token", value);
    };
    if let Some(ref token) = configuration.oauth_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    };
    req_builder = req_builder.json(&p_basic_goal_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::UpsertGoalResponse`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::UpsertGoalResponse`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UpdateGoalError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

