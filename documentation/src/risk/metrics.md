# Metrics
Data plays a crucial role in quantifying risk and modeling systems. It provides the foundation for statistical analysis and predictive modeling, enabling us to measure and understand the behavior of systems under various conditions. We can identify patterns, trends, and correlations by analyzing data to help us predict future events or outcomes. This is particularly important in economic risk, where accurate predictions can help mitigate potential losses and optimize returns.

The particular metrics we have been interested in (by no means exhaustive or representative of the entire field) are:

## Arbitrage Profit
Arbitrage profit is the profit made by taking advantage of the price differences of a particular asset across different markets or platforms. In DeFi, these opportunities can arise due to inefficiencies in asset pricing. If related to a decentralized exchange, such as an automated market maker(AMM), mathematical metrics can be derived to compute the cost and revenue of these arbitrage opportunities exactly. 

There are generally two types of arbitrage opportunities in DeFi:

>Atomic arbitrage opportunities in DeFi are transactions that are either fully executed or not executed at all. This is possible due to the atomicity of the Ethereum Virtual Machine (EVM), which ensures that all operations within a transaction are treated as a single, indivisible unit. The entire transaction is reverted if any operation fails, ensuring no partial state changes occur. This characteristic of the EVM allows for risk-free arbitrage opportunities, as the arbitrageur is not exposed to the risk of one part of the trade executing while the other does not.

>Non-atomic arbitrage opportunities in DeFi are transactions that are partially executed. This is possible due to the lack of atomicity in the EVM, allowing partial state changes to occur. If one part of the trade fails, the other can still be executed, resulting in a partial state change. This characteristic of the EVM allows for riskier arbitrage opportunities, as the arbitrageur is exposed to the risk of one part of the trade executing while the other is not.

Non-atomic arbitrage is much more challenging to measure and model, requiring a more complex understanding of the EVM and its execution model. However, atomic arbitrage is [easy to measure](https://explore.flashbots.net/), as it only requires a basic understanding of the EVM and its execution model.

## Liquidity Provider Portfolio Value
Liquidity Provider Portfolio Value refers to the payoff that an LP assumes when providing liquidity to a pool[Replicating Market Makers](https://arxiv.org/abs/2103.14769)[Replicating Monotonic Payoffs Without Oracles](https://arxiv.org/abs/2111.13740). 

 The has been shown to have two components path dependent and path independent components, which have been introduced in this [paper](https://arxiv.org/abs/2208.06046) as loss vs. holding(LVH) and loss vs. rebalancing (LVR), respectively.

## Fee Growth

Fee Growth in Automated Market Makers (AMMs) refers to the fees collected by the liquidity providers over time. These fees are generated from the trading activity in the liquidity pool and are directly proportional to the volume of trades. The more the trading activity (turnover), the higher the fees collected, leading to a growth in the fees. This fee growth can be a significant source of income for liquidity providers, in addition to the potential price appreciation of the assets in the pool.

## Model Parameters

## Geometric Brownian Motion (GBM)

Geometric Brownian Motion (GBM) is a standard method to model price paths in financial markets. Two parameters characterize it:

1. **Drift (μ)**: This represents the asset's expected return. It is the direction that we expect our asset to move in the future.

2. **Volatility (σ)**: This represents the standard deviation of the asset's returns. It is a measure of the asset's risk or uncertainty.

The GBM model assumes that the logarithmic returns of the asset prices are normally distributed and that the following stochastic differential equation can model them:

$$
dS_t = μS_t dt + σS_t dW_t
$$

Where:

- $S_t$ is the asset price at time t
- $μ$ is the drift
- $σ$ is the volatility
- $W_t$ is a Wiener process

This equation describes the change in the asset price over an infinitesimally small period. The first term on the right-hand side represents the deterministic trend (drift), and the second term represents the random fluctuation (volatility).

