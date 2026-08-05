use rfb_labs_week_2::{select_utxos, OutPoint, TransactionError, Utxo};

fn utxo(txid: &str, value: u64) -> Utxo {
    Utxo {
        outpoint: OutPoint {
            txid: txid.into(),
            vout: 0,
        },
        value,
    }
}

#[test]
fn selection_borrows_enough_utxos_in_slice_order() {
    let available = vec![utxo("a", 70_000), utxo("b", 50_000)];
    let selected = select_utxos(&available, 90_000).unwrap();

    assert_eq!(selected.len(), 2);
    assert_eq!(selected.iter().map(|utxo| utxo.value).sum::<u64>(), 120_000);
}

#[test]
fn insufficient_funds_is_an_error() {
    let available = vec![utxo("a", 30_000), utxo("b", 20_000)];

    assert_eq!(
        select_utxos(&available, 60_000),
        Err(TransactionError::InsufficientFunds {
            available: 50_000,
            required: 60_000,
        })
    );
}

#[test]
fn single_utxo_suffices_when_enough() {
    let available = vec![utxo("a", 100_000), utxo("b", 50_000)];
    let selected = select_utxos(&available, 80_000).unwrap();

    assert_eq!(selected.len(), 1);
    assert_eq!(selected[0].value, 100_000);
}

#[test]
fn exact_target_returns_immediately() {
    let available = vec![utxo("a", 50_000), utxo("b", 30_000), utxo("c", 20_000)];
    let selected = select_utxos(&available, 50_000).unwrap();

    assert_eq!(selected.len(), 1);
}

#[test]
fn empty_utxo_pool_is_insufficient() {
    let available: Vec<Utxo> = vec![];

    assert_eq!(
        select_utxos(&available, 1_000),
        Err(TransactionError::InsufficientFunds {
            available: 0,
            required: 1_000,
        })
    );
}
