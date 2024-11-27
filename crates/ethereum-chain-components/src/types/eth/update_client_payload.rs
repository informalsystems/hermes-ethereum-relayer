use unionlabs::ethereum::config::{
    BYTES_PER_LOGS_BLOOM, MAX_EXTRA_DATA_BYTES, SYNC_COMMITTEE_SIZE,
};
use unionlabs::ibc::lightclients::ethereum::header::Header;
use unionlabs::ibc::lightclients::ethereum::trusted_sync_committee::TrustedSyncCommittee;

pub struct EthUpdateClientPayload<Preset>
where
    Preset: SYNC_COMMITTEE_SIZE + BYTES_PER_LOGS_BLOOM + MAX_EXTRA_DATA_BYTES,
{
    pub headers: Vec<Header<Preset>>,
    pub trusted_committee: TrustedSyncCommittee<Preset>,
}
