use std::convert::{TryFrom, TryInto};

use hdk::entry::EntryDefRegistration;
use hdk::prelude::HasHash;
use hdk::prelude::*;
use hdk::prelude::{entry_defs, hdk_extern, map_extern, ExternResult};

entry_defs![Nifty::entry_def(), NiftyId::entry_def()];

#[hdk_entry(id = "nifty_id", visibility = "public")]
#[derive(Clone)]
pub struct NiftyId {
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
pub fn create(nifty_id: NiftyId) -> ExternResult<()> {
    let _nifty_id_entry = create_entry(nifty_id.clone())?;

    let owner = agent_info()?.agent_latest_pubkey;

    let nifty = Nifty {
        id: nifty_id.clone().id,
        owner,
    };

    let _nifty_entry = create_entry(nifty.clone())?;

    link_id_to_nifty(nifty_id.clone(), nifty.clone())?;

    Ok(())
}

fn link_id_to_nifty(source: NiftyId, target: Nifty) -> ExternResult<()> {
    create_link(
        hash_entry(source)?,
        hash_entry(target)?,
        link_tag("id -> nifty")?,
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_details_for_entry(nifty_id: NiftyId) -> ExternResult<Details> {
    let entry_hash = hash_entry(nifty_id)?;
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
pub fn current_owner(nifty_id: NiftyId) -> ExternResult<AgentPubKey> {
    let latest_nifty = latest_nifty(nifty_id)?;
    let owner = latest_nifty.owner;
    Ok(owner)
}

fn latest_nifty(nifty_id: NiftyId) -> ExternResult<Nifty> {
    // TODO walk the chain to find the latest update
    // We currently return the first entry!

    let nifty_id_hash = hash_entry(&nifty_id)?;
    let links = get_links(nifty_id_hash, None)?; // TODO pass in link tag

    if links.len() > 1 {
        // TODO: filter by nifty creator
        // error if still > 1; only one link expected
    }

    let link = links[0].clone();

    let element: Element = get(link.target, GetOptions::default())?
        .ok_or_else(|| WasmError::Guest(String::from("Entry not found")))?;

    let entry_option = element.entry().to_app_option()?;

    let nifty =
        entry_option.ok_or_else(|| WasmError::Guest("The targeted entry is empty :(".into()))?;

    Ok(nifty)
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);
fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let serialized_bytes: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(serialized_bytes.bytes().clone()))
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
