// #![allow(warnings)]
#![warn(warnings)]
// #![warn(unused_variables)]  # does not work

use std::convert::{TryFrom, TryInto};

use hdk::entry::EntryDefRegistration;
use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;

entry_defs![
    Nifty::entry_def(),
    NiftyId::entry_def(),
    EzNifty::entry_def()
];

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

#[hdk_entry(id = "eznifty", visibility = "public")]
#[derive(Clone)]
pub struct EzNifty {
    pub id: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferInput {
    pub nifty_id: String,
    pub recipient: AgentPubKey,
}

#[hdk_extern]
pub fn create(nifty_id: NiftyId) -> ExternResult<EntryHashB64> {
    let _nifty_id_entry = create_entry(nifty_id.clone())?;

    let owner = agent_info()?.agent_latest_pubkey;

    let nifty = Nifty {
        id: nifty_id.clone().id,
        owner,
    };

    let _nifty_entry = create_entry(nifty.clone())?;

    link_id_to_nifty(nifty_id.clone(), nifty.clone())?;

    let nifty_entry_hash = hash_entry(nifty)?;
    Ok(nifty_entry_hash.into())
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
pub fn transfer(_transfer_input: TransferInput) -> ExternResult<()> {
    // let nifty_id = transfer_input.nifty_id.clone();
    // let latest_nifty_element = latest_nifty_element(NiftyId {
    //     id: nifty_id.clone(),
    // })?;

    // // TODO: validate I own this thing

    // // update owner via entry update
    // update_entry(
    //     latest_nifty_element.header_address().clone(),
    //     Nifty {
    //         id: nifty_id,
    //         owner: agent_info()?.agent_latest_pubkey,
    //     },
    // )?;

    Ok(())
}

#[hdk_extern]
pub fn current_owner(nifty_id: NiftyId) -> ExternResult<AgentPubKey> {
    let latest_nifty = latest_nifty(nifty_id)?;
    let owner = latest_nifty.owner;
    Ok(owner)
}

fn latest_nifty(_nifty_id: NiftyId) -> ExternResult<Nifty> {
    // let element = latest_nifty_element(nifty_id)?;

    // let entry_option = element.entry().to_app_option()?;

    // let nifty =
    //     entry_option.ok_or_else(|| WasmError::Guest("The targeted entry is empty :(".into()))?;

    // Ok(nifty)

    Ok(Nifty {
        id: "foobar".into(),
        owner: agent_info()?.agent_latest_pubkey,
    })
}

// fn latest_nifty_element(nifty_id: NiftyId) -> ExternResult<Element> {
//     // TODO walk the chain to find the latest update
//     // We currently return the first entry!

//     let nifty_id_hash = hash_entry(&nifty_id)?;
//     let links = get_links(nifty_id_hash, None)?; // TODO pass in link tag

//     if links.len() > 1 {
//         // TODO: filter by nifty creator
//         // error if still > 1; only one link expected
//     }

//     let link = links[0].clone();

//     let details = get_details(link.target, GetOptions::default())?;
//     // debug!("{:#?}", details);
//     // get_details(hash(element));

//     let maybe_latest = match details {
//         Some(Details::Entry(EntryDetails { updates, .. })) => Some(updates.clone()),
//         _ => None,
//     };

//     debug!("maybe_latest: {:#?}", maybe_latest);

//     // return pair
//     // header hash
//     // entry

//     // unimplemented!();
// }

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);
fn link_tag(tag: &str) -> ExternResult<LinkTag> {
    let serialized_bytes: SerializedBytes = StringLinkTag(tag.into()).try_into()?;
    Ok(LinkTag(serialized_bytes.bytes().clone()))
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
pub fn get_details_for_entry_with_multiple_updates(nifty_id: NiftyId) -> ExternResult<()> {
    let nifty_amy = EzNifty {
        id: nifty_id.id.clone(),
        owner: "Amy".into(),
    };
    let create_header_hash = create_entry(nifty_amy.clone())?;

    let nifty_beatrix = EzNifty {
        id: nifty_id.id.clone(),
        owner: "Beatrix".into(),
    };
    update_entry(create_header_hash.clone(), nifty_beatrix)?;

    let nifty_camille = EzNifty {
        id: nifty_id.id,
        owner: "Camille".into(),
    };
    let update_header_hash = update_entry(create_header_hash.clone(), nifty_camille)?;

    let details =
        get_details(create_header_hash.clone(), GetOptions::default())?.ok_or_else(|| {
            WasmError::Guest(format!(
                "No entry was found for hash {}",
                create_header_hash.clone()
            ))
        })?;

    debug!("CREATE HEADER DETAILS: {:#?}", details);

    let details =
        get_details(update_header_hash.clone(), GetOptions::default())?.ok_or_else(|| {
            WasmError::Guest(format!(
                "No entry was found for hash {}",
                update_header_hash.clone()
            ))
        })?;

    debug!("LAST UPDATE HEADER DETAILS: {:#?}", details);

    // unimplemented!()
    Ok(())
}

// fn details(header_hash: HoloHash<Header>) -> ExternResult<()> {
//     get_details(header_hash.clone(), GetOptions::default())?.ok_or_else(|| {
//         WasmError::Guest(format!(
//             "No entry was found for hash {}",
//             header_hash.clone()
//         ))
//     });
// }

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

#[hdk_extern]
pub fn create_link_with_tag(link_tag_bytes: Vec<u8>) -> ExternResult<()> {
    let base = NiftyId { id: "base".into() };
    let target = NiftyId {
        id: "target".into(),
    };

    create_entry(base.clone())?;
    create_entry(target.clone())?;

    let _link = create_link(
        hash_entry(base)?,
        hash_entry(target)?,
        LinkTag(link_tag_bytes),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_links_by_tag(link_tag_bytes: Vec<u8>) -> ExternResult<LinkTag> {
    let base = NiftyId { id: "base".into() };

    let links = get_links(hash_entry(base)?, Some(LinkTag(link_tag_bytes)))?;

    let link = links[0].clone(); // TODO: handle multiple links

    let tag = link.tag.clone();

    debug!("tag length in bytes: {:#?}", tag.clone().into_inner().len());

    Ok(tag)
}

#[hdk_extern]
pub fn update_entry_to_different_type(_: ()) -> ExternResult<()> {
    let base = NiftyId { id: "base".into() };
    let new = Nifty {
        id: "base".into(),
        owner: agent_info()?.agent_latest_pubkey,
    };

    let header_hash_create = create_entry(base.clone())?;
    update_entry(header_hash_create.clone(), new.clone())?;
    Ok(())
}
