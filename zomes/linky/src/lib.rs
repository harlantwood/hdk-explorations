// #![allow(warnings)]
#![warn(warnings)]

// use std::convert::{TryFrom, TryInto};

// use hdk::entry::EntryDefRegistration;
use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;

#[hdk_extern]
pub fn make_link(
    (base, target, link_tag_bytes): (EntryHashB64, EntryHashB64, Vec<u8>),
) -> ExternResult<()> {
    create_link(base.into(), target.into(), LinkTag(link_tag_bytes))?;

    Ok(())
}

#[hdk_extern]
pub fn fetch_links((base, link_tag_bytes): (EntryHashB64, Vec<u8>)) -> ExternResult<Vec<Link>> {
    let links = get_links(base.into(), Some(LinkTag(link_tag_bytes)))?;

    Ok(links)
}
