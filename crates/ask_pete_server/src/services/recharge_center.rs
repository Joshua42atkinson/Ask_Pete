use pete_core::economy::{Coal, CoalUsageLog};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalVoucher {
    pub voucher_id: String,
    pub department: String,
    pub amount: f64,
    pub description: String,
    pub timestamp: i64,
}

pub struct RechargeCenter {
    // In-memory storage for MVP. In production, this would be a DB.
    logs: Vec<CoalUsageLog>,
}

impl RechargeCenter {
    pub fn new() -> Self {
        Self { logs: Vec::new() }
    }

    pub fn log_usage(&mut self, log: CoalUsageLog) {
        self.logs.push(log);
    }

    pub fn generate_monthly_voucher(&self, department: &str) -> JournalVoucher {
        // Filter logs for the department (mocking department association via student_id for now)
        let dept_logs: Vec<CoalUsageLog> = self
            .logs
            .iter()
            .filter(|l| l.context.contains(department)) // Simple mock filter
            .cloned()
            .collect();

        let total_cost = CoalUsageLog::calculate_recharge_cost(&dept_logs);

        JournalVoucher {
            voucher_id: format!("JV-{}-{}", department, chrono::Utc::now().timestamp()),
            department: department.to_string(),
            amount: total_cost,
            description: format!("Internal Recharge for {} Coal Usage", department),
            timestamp: chrono::Utc::now().timestamp(),
        }
    }

    pub fn get_total_usage(&self) -> f64 {
        CoalUsageLog::calculate_recharge_cost(&self.logs)
    }
}
