use futures::future;
use hdk::prelude::*;
use holochain::sweettest::*;
use holochain::test_utils::consistency_10s;

use nifty::*;

const DNA_FILEPATH: &str = "../../workdir/dna/nifty.dna";

#[tokio::test(flavor = "multi_thread")]
#[ignore]
pub async fn test_get_details_for_entry() {
    let (conductors, _agents, apps) = setup_conductors(2).await;

    let conductor_alice = &conductors[0];
    let conductor_bob = &conductors[1];

    let cells = apps.cells_flattened();
    let cell_alice = cells[0];
    let cell_bob = cells[1];

    let nifty_input = NiftyId {
        id: "abc123".into(),
    };

    let _: () = conductor_alice
        .call(&cell_alice.zome("nifty"), "create", nifty_input.clone())
        .await;
    let _: () = conductor_alice
        .call(&cell_alice.zome("nifty"), "create", nifty_input.clone())
        .await;
    let _: () = conductor_bob
        .call(&cell_bob.zome("nifty"), "create", nifty_input.clone())
        .await;

    consistency_10s(&[&cell_alice, &cell_bob]).await;

    let _details: Details = conductor_alice
        .call(
            &cell_alice.zome("nifty"),
            "get_details_for_entry",
            nifty_input.clone(),
        )
        .await;

    // println!("{:#?}", details);

    // println!("{}", details.headers[0].into_inner().1);
    // assert_eq!(details.entry, nifty);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_transfer() {
    let (conductors, _agents, apps) = setup_conductors(2).await;

    let conductor_alice = &conductors[0];
    let _conductor_bob = &conductors[1];

    let cells = apps.cells_flattened();
    let cell_alice = cells[0];
    let cell_bob = cells[1];

    let nifty_id = String::from("abc123");
    let nifty_input = NiftyId {
        id: nifty_id.clone(),
    };

    let transfer_input = TransferInput {
        nifty_id,
        recipient: cell_bob.agent_pubkey().clone(),
    };

    let _: () = conductor_alice
        .call(&cell_alice.zome("nifty"), "create", nifty_input.clone())
        .await;

    let _: () = conductor_alice
        .call(&cell_alice.zome("nifty"), "transfer", transfer_input)
        .await;

    let current_owner: AgentPubKey = conductor_alice
        .call(&cell_alice.zome("nifty"), "current_owner", nifty_input)
        .await;

    assert_eq!(current_owner, cell_bob.agent_pubkey().clone());

    // consistency_10s(&[&cell_alice, &cell_bob]).await;
}

#[tokio::test(flavor = "multi_thread")]
#[ignore]
pub async fn test_wasm_debugging() {
    let (conductors, _agents, apps) = setup_conductors(1).await;

    let conductor = &conductors[0];

    let cells = apps.cells_flattened();
    let cell_alice = cells[0];

    let _: () = conductor.call(&cell_alice.zome("nifty"), "debug", ()).await;
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
