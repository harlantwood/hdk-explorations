use std::convert::{TryFrom, TryInto};

use hdk::entry::EntryDefRegistration;
use hdk::prelude::HasHash;
use hdk::prelude::*;
use hdk::prelude::{entry_defs, hdk_extern, map_extern, ExternResult};

entry_defs![Nifty::entry_def(), NiftyInput::entry_def()];

#[hdk_entry(id = "nifty_id", visibility = "public")]
#[derive(Clone)]
pub struct NiftyInput {
    pub id: String,
}

#[hdk_entry(id = "nifty", visibility = "public")]
#[derive(Clone)]
pub struct Nifty {
    pub id: String,
    pub owner: AgentPubKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferInput {
    pub nifty_id: String,
    pub recipient: AgentPubKey,
}

#[hdk_extern]
pub fn create(nifty_input: NiftyInput) -> ExternResult<()> {
    create_entry(nifty_input.clone())?;

    let owner = agent_info()?.agent_latest_pubkey;

    let nifty = Nifty {
        id: nifty_input.id,
        owner,
    };

    create_entry(nifty)?;

    Ok(())
}

#[hdk_extern]
pub fn get_details_for_entry(nifty_input: NiftyInput) -> ExternResult<Details> {
    let entry_hash = hash_entry(nifty_input)?;
    let details = get_details(entry_hash.clone(), GetOptions::default())?
        .ok_or_else(|| WasmError::Guest(format!("No entry was found for hash {}", entry_hash)))?;
    // debug!("{:#?}", details);
    Ok(details)
}

#[hdk_extern]
pub fn transfer(_transfer_input: TransferInput) -> ExternResult<()> {
    Ok(())
}

#[hdk_extern]
pub fn current_owner(_nifty_input: NiftyInput) -> ExternResult<AgentPubKey> {
    Ok(agent_info()?.agent_latest_pubkey)
}

#[hdk_extern]
pub fn debug(_: ()) -> ExternResult<()> {
    // source: https://holochain-open-dev.github.io/blog/recent-changes-for-happ-devs/
    debug!("debug works");
    info!("info works");
    warn!("warn works");
    error!("error works");
    debug!(foo = "fields", bar = "work", "too");

    trace!("tracing {}", "works!"); // this is a logging level

    Ok(())
}
