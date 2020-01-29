#[derive(Deserialize, Clone)]
pub struct RequestEvent {
    #[serde(rename = "day")]
    pub day: String,
}
