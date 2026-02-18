#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamStatus {
    Active = 0,
    Paused = 1,
    Completed = 2,
    Cancelled = 3,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Stream {
    pub stream_id: u64,
    pub sender: Address,
    pub recipient: Address,
    pub deposit_amount: i128,
    pub rate_per_second: i128,
    pub start_time: u64,
    pub cliff_time: u64,
    pub end_time: u64,
    pub withdrawn_amount: i128,
    pub status: StreamStatus,
}

#[contract]
pub struct FluxoraStream;

#[contractimpl]
impl FluxoraStream {
    /// Initialize the stream contract (e.g. set token and admin).
    pub fn init(env: Env, _token: Address, _admin: Address) -> () {
        // Store token and admin for create_stream, pause, cancel, etc.
        ()
    }

    /// Create a new stream. Treasury locks USDC and sets rate, duration, cliff.
    pub fn create_stream(
        _env: Env,
        _sender: Address,
        _recipient: Address,
        _deposit_amount: i128,
        _rate_per_second: i128,
        _start_time: u64,
        _cliff_time: u64,
        _end_time: u64,
    ) -> u64 {
        // TODO: transfer USDC from sender to contract, persist Stream, emit event
        0
    }

    /// Pause an active stream (callable by sender/admin).
    pub fn pause_stream(_env: Env, _stream_id: u64) -> () {
        ()
    }

    /// Resume a paused stream.
    pub fn resume_stream(_env: Env, _stream_id: u64) -> () {
        ()
    }

    /// Cancel stream (callable by sender/admin). Unstreamed amount returns to sender.
    pub fn cancel_stream(_env: Env, _stream_id: u64) -> () {
        ()
    }

    /// Recipient withdraws accrued USDC up to (accrued - withdrawn).
    pub fn withdraw(_env: Env, _stream_id: u64) -> i128 {
        0
    }

    /// Calculate accrued amount: min((now - start_time) * rate_per_second, deposit_amount).
    /// With cliff: if now < cliff_time then 0.
    pub fn calculate_accrued(_env: Env, _stream_id: u64) -> i128 {
        0
    }

    /// Return current stream state for a given stream_id.
    pub fn get_stream_state(env: Env, _stream_id: u64) -> Stream {
        // Placeholder until storage is implemented (use contract address as dummy)
        let placeholder = env.current_contract_address();
        Stream {
            stream_id: 0,
            sender: placeholder.clone(),
            recipient: placeholder,
            deposit_amount: 0,
            rate_per_second: 0,
            start_time: 0,
            cliff_time: 0,
            end_time: 0,
            withdrawn_amount: 0,
            status: StreamStatus::Active,
        }
    }
}

// Add tests with `soroban-sdk = { version = "20.0.0", features = ["testutils"] }` when needed.
