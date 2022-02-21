use crate::{
    error::Result,
    modules::schema::{GetSchemaResponse, SchemaCreateOptions},
};
use async_trait::async_trait;
use serde_json::json;

use crate::modules::schema::{Schema, SchemaModule};

use super::agent::CloudAgentPython;

#[async_trait]
impl SchemaModule for CloudAgentPython {
    async fn create(&self, options: SchemaCreateOptions) -> Result<String> {
        let url = self.cloud_agent.create_url(vec!["schemas"])?;

        let body = json!({
          "attributes": options.attributes,
          "schema_name": options.name,
          "schema_version": options.version
        });

        self.cloud_agent
            .post::<Schema>(url, None, Some(body))
            .await
            .map(|s| s.sent.schema_id)
    }

    async fn get_by_id(&self, id: String) -> Result<GetSchemaResponse> {
        let url = self.cloud_agent.create_url(vec!["schemas", &id])?;
        self.cloud_agent.get(url, None).await
    }

    async fn get_all(&self) -> Result<Schema> {
        let _url = self.cloud_agent.create_url(vec!["schemas", "created"])?;
        todo!()
    }
}