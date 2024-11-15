use std::str::FromStr;

use raydium::{
    CreatePersonalPositionEventEvent, DecreaseLiquidityEventEvent, IncreaseLiquidityEventEvent,
    LiquidityCalculateEventEvent, LiquidityChangeEventEvent, PoolStateAccount, SwapEventEvent,
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};

pub enum EventType {
    Swap,
    LiquidityChangeEvent,
    LiquidityCalculateEvent,
    DecreaseLiquidityEvent,
    IncreaseLiquidityEvent,
    CreatePersonalPositionEvent,
}

#[derive(Debug)]
pub enum Events {
    Swap(SwapEventEvent),
    LiquidityChangeEvent(LiquidityChangeEventEvent),
    LiquidityCalculateEvent(LiquidityCalculateEventEvent),
    DecreaseLiquidityEvent(DecreaseLiquidityEventEvent),
    IncreaseLiquidityEvent(IncreaseLiquidityEventEvent),
    CreatePersonalPositionEvent(CreatePersonalPositionEventEvent),
}

fn get_pool_state(
    rpc_client: &RpcClient,
    pool_state_address: Pubkey,
) -> Result<PoolStateAccount, Box<dyn std::error::Error>> {
    let account_data = rpc_client.get_account_data(&pool_state_address)?;
    let pool_state = PoolStateAccount::deserialize(&account_data)?;
    Ok(pool_state)
}

fn get_events(event_type: EventType, logs: Vec<u8>) -> Result<Events, Box<dyn std::error::Error>> {
    let mut buf = logs.as_slice();
    let event = match event_type {
        EventType::Swap => Events::Swap(SwapEventEvent::deserialize(&mut buf)?),
        EventType::LiquidityChangeEvent => {
            Events::LiquidityChangeEvent(LiquidityChangeEventEvent::deserialize(&mut buf)?)
        }
        EventType::LiquidityCalculateEvent => {
            Events::LiquidityCalculateEvent(LiquidityCalculateEventEvent::deserialize(&mut buf)?)
        }
        EventType::DecreaseLiquidityEvent => {
            Events::DecreaseLiquidityEvent(DecreaseLiquidityEventEvent::deserialize(&mut buf)?)
        }
        EventType::IncreaseLiquidityEvent => {
            Events::IncreaseLiquidityEvent(IncreaseLiquidityEventEvent::deserialize(&mut buf)?)
        }
        EventType::CreatePersonalPositionEvent => Events::CreatePersonalPositionEvent(
            CreatePersonalPositionEventEvent::deserialize(&mut buf)?,
        ),
    };
    Ok(event)
}

fn main() {
    let rpc_client = RpcClient::new_with_commitment(
        "http://api.mainnet-beta.solana.com",
        CommitmentConfig::processed(),
    );

    let pool_state_address =
        Pubkey::from_str("3SsRkGV9SuaT4NfgkB5jAQfoGXgFMCkNFEZC14uqoGJk").unwrap();

    let _pool_state = get_pool_state(&rpc_client, pool_state_address).unwrap();

    let logs = vec![];
    // Get events
    let events = get_events(EventType::Swap, logs).unwrap();
    println!("{:?}", events);
}
