use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UNAgencyState {
    pub agency_id: u32,
    pub agency_name: String,
    pub project_name: String,
    pub portfolio: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MultilateralAuditRecord {
    pub audit_id: String,
    pub timestamp: u64,
    pub merkle_root_hash: String,
    pub verified_by_pqc: bool,
}

pub fn get_metadata() -> UNAgencyState {
    UNAgencyState {
        agency_id: 6,
        agency_name: "World Bank Group (WBG)".to_string(),
        project_name: "WorldBank BondAI".to_string(),
        portfolio: "Multilateral Development & Reconstruction".to_string(),
        strategy: "Programmatic Utility ($WBG) -- Green Infrastructure Bond and Multilateral Concession Clearing".to_string(),
        target_chain: "Internet Computer (ICP) + Algorand Box".to_string(),
        is_active: true,
    }
}
