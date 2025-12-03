use serde::{Deserialize, Serialize};

/// Represents the "Coal" (Compute) resource.
/// Scarcity Model:
/// - Local Inference (Gemma): Low cost (burns local battery/heat).
/// - Cloud Inference (Gemini): High cost (burns API credits/quota).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Coal(pub f64);

impl Coal {
    /// Cost for local Gemma inference per token
    pub const GEMMA_COST_PER_TOKEN: f64 = 0.01;

    /// Cost for cloud Gemini inference per request (heuristic)
    pub const GEMINI_COST_PER_REQUEST: f64 = 5.0;

    /// Calculate cost for a given number of tokens (Local)
    pub fn cost_local(tokens: usize) -> Self {
        Coal((tokens as f64) * Self::GEMMA_COST_PER_TOKEN)
    }

    /// Calculate cost for a cloud request
    pub fn cost_cloud() -> Self {
        Coal(Self::GEMINI_COST_PER_REQUEST)
    }
}

/// Represents "Steam" (Mastery/Progress).
/// Generated when "Coal" is burned effectively (i.e., learning happens).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Steam(pub f64);

impl Steam {
    /// Conversion rate: How much Steam is generated per unit of Coal burned?
    /// This is the "Efficiency" of the engine.
    /// Higher mastery = Higher efficiency.
    pub fn generate(coal: Coal, efficiency: f64) -> Self {
        Steam(coal.0 * efficiency)
    }
}

/// Log of Coal usage for billing/telemetry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoalUsageLog {
    pub student_id: String,
    pub timestamp: i64, // Unix timestamp
    pub coal_burned: Coal,
    pub context: String, // e.g., "Essay Generation", "Quiz"
}

impl CoalUsageLog {
    /// Calculate the recharge cost for a set of logs.
    /// Returns the total cost in USD (mock currency).
    pub fn calculate_recharge_cost(logs: &[CoalUsageLog]) -> f64 {
        // Heuristic: 1 Coal = $0.001 (Internal Recharge Rate)
        const RECHARGE_RATE: f64 = 0.001;
        logs.iter()
            .map(|log| log.coal_burned.0 * RECHARGE_RATE)
            .sum()
    }
}
