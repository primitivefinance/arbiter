use super::*;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub(crate) struct TokenAdmin {
    /// The identifier of the token admin.
    pub token_data: HashMap<String, TokenData>,
    #[serde(skip)]
    pub tokens: Option<HashMap<String, ArbiterToken<RevmMiddleware>>>,
    #[serde(skip)]
    pub client: Option<Arc<RevmMiddleware>>,
    #[serde(skip)]
    pub messager: Option<Messager>,
    pub count: u64,
    pub max_count: Option<u64>,
    startup_message: Option<String>,
}

impl TokenAdmin {
    pub fn new(max_count: Option<u64>) -> Self {
        Self {
            token_data: HashMap::new(),
            tokens: None,
            client: None,
            messager: None,
            count: 0,
            max_count,
            startup_message: None,
        }
    }

    /// Adds a token to the token admin.
    pub fn add_token(&mut self, token_data: TokenData) {
        self.token_data.insert(token_data.name.clone(), token_data);
    }
}
