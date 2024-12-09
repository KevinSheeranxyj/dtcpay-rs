

#[derive(Debug, Deserialize, ToSchema, Serialize)]
pub struct CreateApiKeyRequest {
    #[schema(max_length0 = 64, example = "Sandbox integration key")]
    pub name: String,
    #[schema(
        max_length = 256,
        example = "Key used by our developers to integrate with the sandbox environment"
    )]
    pub description: Option<String>,

    #[schema(example = "2022-01-10T10:12:12Z")]
    pub expiration: ApiKeyExpiration,
}

#[derive(Debug, Deserialize, ToSchema, Serialize)]
pub struct CreateApiKeyResponse {
    #[schema(max_length = 64, example = "Sandbox integration key")]
    pub key_id: common_utils::id_type::ApiKeyId,
    #[schema(max_length = 256, example = "Sandbox integration key")]
    pub merchant_id: common_utils::id_type::MerchantId,
    pub name: String,
    pub description: Option<String>,
    pub api_key: StrongSecret<String>,
    pub created: PrimitiveDateTime,
    pub expiration: ApiKeyExpiration,

}