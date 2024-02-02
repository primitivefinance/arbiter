// Any user of arbiter-engine has to build something like this on their own.
#[derive(Serialize, Deserialize)]
enum MyBehaviors {
    Arbitrageur(Arbitreageur),
    TokenAdmin(TokenAdmin),
    LiquidityProvider(LiquidityProvider),
}
