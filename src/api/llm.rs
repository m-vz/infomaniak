use serde::{Deserialize, Serialize};

use crate::{client::Client, error::Error};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Product {
    #[serde(rename = "product_name")]
    pub name: String,
    #[serde(rename = "product_id")]
    pub id: u64,
    pub account_name: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductResponse {
    #[serde(rename = "data")]
    pub products: Vec<Product>,
}

pub async fn products(client: &Client) -> Result<ProductResponse, Error> {
    client.get("/1/llm").await
}

#[cfg(test)]
mod tests {
    use httpmock::{Method, MockServer};
    use serde_json::json;

    use crate::{
        api::{self, llm::Product},
        client::Client,
    };

    #[tokio::test]
    async fn test_products() {
        let server = MockServer::start_async().await;
        let mock = server.mock(|when, then| {
            when.method(Method::GET)
                .path("/1/llm")
                .header("Authorization", "Bearer TOKEN")
                .header("Content-Type", "application/json");
            then.status(200).json_body(json!({
               "result": "success",
               "data": [
                   {
                       "product_name": "LLM API",
                       "product_id": 47054,
                       "account_name": "My organization",
                       "status": "ok"
                   }
               ]
            }));
        });
        let client =
            Client::with_url(&server.base_url(), "TOKEN").expect("Could not create client");

        let result = api::llm::products(&client).await;

        mock.assert();
        assert_eq!(
            result.unwrap().products,
            vec![Product {
                name: "LLM API".into(),
                id: 47054,
                account_name: "My organization".into(),
                status: "ok".into(),
            }]
        )
    }
}
