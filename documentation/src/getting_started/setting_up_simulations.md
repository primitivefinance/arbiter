# Setting up Simulations
Simulation design can be a difficult task but we are developing more tooling to make this process easier.
We will be adding more documentation and tutorials to help you get started with Arbiter and will update this page as we do so.

## High Level
To begin using Arbiter for simulating your smart contracts, you will want to map out the mechanics of your system.
Some questions you should ask yourself are:
- What are the key mechanisms of my system?
- What agents can interact with this system?
- What are the key objectives of my simulation?

For example, if you are simulating a decentralized exchange (DEX), you will want to think about the following:
- What are the key mechanisms of my system?
    - Liquidity (de)allocations
    - Swaps
- What agents can interact with this system?
    - Liquidity providers
    - Arbitrage bots
    - Random swappers
- What are the key objectives of my simulation?
    - Identify whether the system is vulnerable to large price movements.
    - Identify whether the system is vulnerable to spamming allocations and deallocations over prolonged periods of time.
    - Does the DEX perform well financially?

After reviewing your objectives, consider what new agents you may need to create to simulate attacks or find weaknesses in your system.
Consider adding more objectives and being sure to analyze them scientifically.


## Implementation
For you to use Arbiter after mapping out your system, you will need to do the following:
- Identify necessary contracts.
- Create a repository that will hold your simulation code and references to your smart contracts.
- Generate bindings for those contracts. 
You can use [`arbiter bind`](../usage/arbiter_cli.md#Bindings) to do this or you can use [`forge bind`](https://book.getfoundry.sh/reference/forge/forge-bind). 
Arbiter CLI makes this process a bit easier, so we suggest using it.

The stage is now set for you to begin writing your simulation code.
This will consist of the following:
- Creating a [`Environment`](../usage/arbiter_core/environment.md) for your simulation.
- Creating agents. 
(TODO: More documentation here for [`arbiter-engine`](../usage/arbiter_engine/index.md))
- Creating a [`RevmMiddleware`](../usage/arbiter_core/middleware.md) for each agent in your simulation.
- Deploy contracts using the binding's `MyContract::deploy()` method which will need a client `Arc<RevmMiddleware>` and constructor arguments passed as a tuple. 
Or, if you want to use a forked state, use the binding's `MyContract::new()` method and pass it the relevant client and address.

Once you have deployed your contracts, you can begin interacting with them using the binding's methods.
Your agents can be free to do what they need to do with these contracts to achieve your goals.

