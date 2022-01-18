// #![allow(warnings)]
#![warn(warnings)]
// #![warn(unused_variables)]  # does not work

// use std::convert::{TryFrom, TryInto};

// use hdk::entry::EntryDefRegistration;
use hdk::prelude::*;

#[hdk_extern]
pub fn make_link(
    (base, target, link_tag_bytes): (EntryHash, EntryHash, Vec<u8>),
) -> ExternResult<()> {
    create_link(base, target, LinkTag(link_tag_bytes))?;

    Ok(())
}

#[hdk_extern]
pub fn fetch_links((base, link_tag_bytes): (EntryHash, Vec<u8>)) -> ExternResult<Vec<Link>> {
    let links = get_links(base, Some(LinkTag(link_tag_bytes)))?;

    Ok(links)
}
