#[derive(Deserialize, Clone)]
pub struct RequestEvent {
    #[serde(rename = "clientId")]
    pub client_id: String,
}
