#![warn(warnings)]

use futures::future;
use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;
use holochain::sweettest::*;
// use holochain::test_utils::consistency_10s;

// use linky::*;
use nifty::NiftyId;

const DNA_FILEPATH: &str = "../../workdir/dna/all.dna";

#[tokio::test(flavor = "multi_thread")]
pub async fn test_link_to_entries_from_another_zome() {
    let (conductors, _agents, apps) = setup_conductors(1).await;

    let conductor = &conductors[0];

    let cells = apps.cells_flattened();
    let cell_alice = cells[0];

    // NIFTY1 ENTRY
    let nifty1_input = NiftyId {
        id: "nifty1a".into(),
    };
    let nifty1_entry_hash: EntryHashB64 = conductor
        .call(&cell_alice.zome("nifty"), "create", nifty1_input.clone())
        .await;

    // NIFTY2 ENTRY
    let nifty2_input = NiftyId {
        id: "nifty2b".into(),
    };
    let nifty2_entry_hash: EntryHashB64 = conductor
        .call(&cell_alice.zome("nifty"), "create", nifty2_input.clone())
        .await;

    let link_tag_bytes: Vec<u8> = vec![88, 89, 90];

    // MAKE LINK: NIFTY1 -[XYZ]-> NIFTY2
    let _: () = conductor
        .call(
            &cell_alice.zome("linky"),
            "make_link",
            (
                nifty1_entry_hash.clone(),
                nifty2_entry_hash.clone(),
                link_tag_bytes.clone(),
            ),
        )
        .await;

    // FETCH LINKS: NIFTY1 -[XYZ]-> *
    let links: Vec<Link> = conductor
        .call(
            &cell_alice.zome("linky"),
            "fetch_links",
            (nifty1_entry_hash, link_tag_bytes.clone()),
        )
        .await;

    let link_target: EntryHashB64 = links[0].target.clone().into();
    assert_eq!(link_target, nifty2_entry_hash);
    println!("link target: {:#?}", links[0].target);
}

// UTILS:

async fn setup_conductors(n: usize) -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
    let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
        .await
        .unwrap();

    let mut conductors = SweetConductorBatch::from_standard_config(n).await;

    let all_agents: Vec<AgentPubKey> =
        future::join_all(conductors.iter().map(|c| SweetAgents::one(c.keystore()))).await;
    let apps = conductors
        .setup_app_for_zipped_agents("app", &all_agents, &[dna])
        .await
        .unwrap();

    conductors.exchange_peer_info().await;
    (conductors, all_agents, apps)
}
