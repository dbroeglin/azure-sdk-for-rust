#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod redis {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn get(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        subscription_id: &str,
    ) -> std::result::Result<RedisResource, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/Redis/{}",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: RedisResource = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create_or_update(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        parameters: &RedisCreateOrUpdateParameters,
        subscription_id: &str,
    ) -> std::result::Result<create_or_update::Response, create_or_update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/Redis/{}",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(create_or_update::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(create_or_update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create_or_update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::CREATED => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: RedisResourceWithAccessKey =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Created201(rsp_value))
            }
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                let rsp_value: RedisResourceWithAccessKey =
                    serde_json::from_slice(&body).context(create_or_update::DeserializeError { body })?;
                Ok(create_or_update::Response::Ok200(rsp_value))
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create_or_update::ResponseBytesError)?;
                create_or_update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create_or_update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Created201(RedisResourceWithAccessKey),
            Ok200(RedisResourceWithAccessKey),
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        subscription_id: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/Redis/{}",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(delete::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::ACCEPTED => Ok(delete::Response::Accepted202),
            StatusCode::NOT_FOUND => delete::NotFound404 {}.fail(),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                delete::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        subscription_id: &str,
    ) -> std::result::Result<RedisListResult, list_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/Redis/",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_by_resource_group::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: RedisListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                list_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<RedisListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.Cache/Redis/",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: RedisListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_keys(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        subscription_id: &str,
    ) -> std::result::Result<RedisListKeysResult, list_keys::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/Redis/{}/listKeys",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_keys::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.header(reqwest::header::CONTENT_LENGTH, 0);
        let req = req_builder.build().context(list_keys::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_keys::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_keys::ResponseBytesError)?;
                let rsp_value: RedisListKeysResult = serde_json::from_slice(&body).context(list_keys::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_keys::ResponseBytesError)?;
                list_keys::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_keys {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn regenerate_key(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        parameters: &RedisRegenerateKeyParameters,
        subscription_id: &str,
    ) -> std::result::Result<RedisListKeysResult, regenerate_key::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/Redis/{}/regenerateKey",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(regenerate_key::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(regenerate_key::BuildRequestError)?;
        let rsp = client.execute(req).await.context(regenerate_key::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(regenerate_key::ResponseBytesError)?;
                let rsp_value: RedisListKeysResult = serde_json::from_slice(&body).context(regenerate_key::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(regenerate_key::ResponseBytesError)?;
                regenerate_key::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod regenerate_key {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn force_reboot(
        operation_config: &crate::OperationConfig,
        resource_group_name: &str,
        name: &str,
        parameters: &RedisRebootParameters,
        subscription_id: &str,
    ) -> std::result::Result<force_reboot::Response, force_reboot::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.Cache/Redis/{}/forceReboot",
            &operation_config.base_path, subscription_id, resource_group_name, name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(force_reboot::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(parameters);
        let req = req_builder.build().context(force_reboot::BuildRequestError)?;
        let rsp = client.execute(req).await.context(force_reboot::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(force_reboot::Response::Ok200),
            StatusCode::ACCEPTED => Ok(force_reboot::Response::Accepted202),
            StatusCode::NOT_FOUND => force_reboot::NotFound404 {}.fail(),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(force_reboot::ResponseBytesError)?;
                force_reboot::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod force_reboot {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            NotFound404 {},
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
