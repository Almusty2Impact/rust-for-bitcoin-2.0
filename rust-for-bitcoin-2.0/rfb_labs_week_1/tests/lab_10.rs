mod support;

use rfb_labs_week_1::labs::lab10_reorg::{
    build_reorg_report, disconnect_peer, get_chain_tip, reconnect_peer,
};
use rfb_labs_week_1::model::{ChainTip, ForkSnapshot, ReorgReport};
use support::MockRpc;

fn tip(height: u64, hash: &str, chainwork: &str) -> ChainTip {
    ChainTip {
        height,
        best_block_hash: hash.to_owned(),
        chainwork: chainwork.to_owned(),
    }
}

#[test]
fn reads_tip_and_accumulated_chainwork() {
    let rpc = MockRpc::new();
    rpc.expect_ok(
        None,
        "getblockchaininfo",
        &[],
        r#"{"blocks":107,"bestblockhash":"tip-a","chainwork":"000000d7"}"#,
    );

    assert_eq!(get_chain_tip(&rpc).unwrap(), tip(107, "tip-a", "000000d7"));
    rpc.assert_done();
}

#[test]
fn disconnects_peer_by_address() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "disconnectnode", &["node-b:18444"], "");

    disconnect_peer(&rpc, "node-b:18444").unwrap();
    rpc.assert_done();
}

#[test]
fn reconnects_peer_for_synchronization() {
    let rpc = MockRpc::new();
    rpc.expect_ok(None, "addnode", &["node-b:18444", "onetry"], "");

    reconnect_peer(&rpc, "node-b:18444").unwrap();
    rpc.assert_done();
}

#[test]
fn reports_convergence_on_the_stronger_branch() {
    let competing = ForkSnapshot {
        node_a: tip(109, "short-branch", "000000d9"),
        node_b: tip(111, "strong-branch", "000000db"),
    };
    let final_tips = ForkSnapshot {
        node_a: tip(111, "strong-branch", "000000db"),
        node_b: tip(111, "strong-branch", "000000db"),
    };

    assert_eq!(
        build_reorg_report("common-tip", competing.clone(), final_tips.clone()),
        ReorgReport {
            common_tip_before_split: "common-tip".to_owned(),
            competing_tips: competing,
            final_tips,
            converged: true,
        }
    );
}
