use rfb_labs_week_2_session_4::{
    Item, Library, LibraryError, LoanStatus, LoanTerms, MediaKind, Member,
};

fn library_with_items() -> Library {
    let mut library = Library::new();

    for (id, title, author, kind) in [
        (1, "Dune", "Frank Herbert", MediaKind::Book { pages: 320 }),
        (
            2,
            "Children of Dune",
            "Frank Herbert",
            MediaKind::Book { pages: 180 },
        ),
        (
            3,
            "Project Hail Mary",
            "Andy Weir",
            MediaKind::Audiobook { minutes: 540 },
        ),
        (
            4,
            "The Rust Programming Language",
            "Steve Klabnik",
            MediaKind::Ebook { size_kb: 1_200 },
        ),
    ] {
        library
            .add_item(Item::new(id, title.into(), author.into(), kind))
            .unwrap();
    }

    library
        .register_member(Member::new(100, "Ada".into()))
        .unwrap();

    library
}

// ── Starter tests (#[ignore] removed) ─────────────────────────────────

#[test]
fn checkout_updates_both_the_item_and_the_member() {
    let mut library = library_with_items();

    library.checkout(1, 100, 5).unwrap();

    assert_eq!(
        library.find_item(1).unwrap().status,
        LoanStatus::OnLoan {
            member_id: 100,
            day_borrowed: 5,
        }
    );
    assert_eq!(library.find_member(100).unwrap().borrowed_item_ids, vec![1]);
}

#[test]
fn a_member_cannot_exceed_the_borrow_limit() {
    let mut library = library_with_items();

    library.checkout(1, 100, 0).unwrap();
    library.checkout(2, 100, 0).unwrap();
    library.checkout(3, 100, 0).unwrap();

    assert_eq!(
        library.checkout(4, 100, 0),
        Err(LibraryError::BorrowLimitReached {
            member_id: 100,
            limit: 3,
        })
    );
}

#[test]
fn returning_a_book_late_charges_a_daily_fee() {
    let mut library = library_with_items();

    // A book may be kept 21 days. Held for 30, so 9 days are overdue.
    library.checkout(1, 100, 10).unwrap();

    assert_eq!(library.return_item(1, 40), Ok(9 * 25));
    assert_eq!(library.find_item(1).unwrap().status, LoanStatus::Available);
    assert!(
        library
            .find_member(100)
            .unwrap()
            .borrowed_item_ids
            .is_empty()
    );
}

#[test]
fn searching_by_author_borrows_rather_than_clones() {
    let library = library_with_items();

    let found = library.items_by_author("Frank Herbert");

    assert_eq!(found.len(), 2);
    assert_eq!(found[0].title, "Dune");
    // `found` holds references into `library`, so these are the same item.
    assert!(std::ptr::eq(found[0], library.find_item(1).unwrap()));
}

// ── Additional required tests ──────────────────────────────────────────

#[test]
fn item_cannot_be_lent_twice() {
    let mut library = library_with_items();

    library.checkout(1, 100, 0).unwrap();
    assert_eq!(
        library.checkout(1, 100, 5),
        Err(LibraryError::ItemAlreadyOnLoan {
            id: 1,
            member_id: 100,
        })
    );
}

#[test]
fn on_time_return_owes_nothing() {
    let mut library = library_with_items();

    // Book loaned for 21 days, returned on day 10 — 10 days held, not overdue.
    library.checkout(1, 100, 0).unwrap();
    assert_eq!(library.return_item(1, 10), Ok(0));
    assert_eq!(library.find_item(1).unwrap().status, LoanStatus::Available);
}

#[test]
fn ebook_returned_late_still_owes_nothing() {
    let mut library = library_with_items();

    // Ebook loaned for 7 days, returned on day 30 — 23 days overdue, but fee is 0.
    library.checkout(4, 100, 0).unwrap();
    assert_eq!(library.return_item(4, 30), Ok(0));
    assert_eq!(library.find_item(4).unwrap().status, LoanStatus::Available);
}

#[test]
fn author_search_returns_borrowed_items() {
    let mut library = library_with_items();

    library.checkout(1, 100, 0).unwrap();

    let herbert = library.items_by_author("Frank Herbert");
    assert_eq!(herbert.len(), 2);
    // Both items appear regardless of loan status.
    assert_eq!(herbert[0].id, 1);
    assert!(matches!(herbert[0].status, LoanStatus::OnLoan { .. }));
}

// ── Validation error tests ──────────────────────────────────────────────

#[test]
fn empty_title_rejected() {
    let mut library = Library::new();
    assert_eq!(
        library.add_item(Item::new(
            1,
            String::new(),
            "Author".into(),
            MediaKind::Book { pages: 100 }
        )),
        Err(LibraryError::EmptyTitle)
    );
}

#[test]
fn duplicate_item_id_rejected() {
    let mut library = Library::new();
    library
        .add_item(Item::new(
            1,
            "Dune".into(),
            "Frank Herbert".into(),
            MediaKind::Book { pages: 320 },
        ))
        .unwrap();
    assert_eq!(
        library.add_item(Item::new(
            1,
            "Other".into(),
            "Someone".into(),
            MediaKind::Ebook { size_kb: 500 },
        )),
        Err(LibraryError::DuplicateItemId { id: 1 })
    );
}

#[test]
fn duplicate_member_id_rejected() {
    let mut library = Library::new();
    library
        .register_member(Member::new(1, "Ada".into()))
        .unwrap();
    assert_eq!(
        library.register_member(Member::new(1, "Bob".into())),
        Err(LibraryError::DuplicateMemberId { id: 1 })
    );
}

#[test]
fn checkout_unknown_item() {
    let mut library = library_with_items();
    assert_eq!(
        library.checkout(999, 100, 0),
        Err(LibraryError::ItemNotFound { id: 999 })
    );
}

#[test]
fn checkout_unknown_member() {
    let mut library = library_with_items();
    assert_eq!(
        library.checkout(1, 999, 0),
        Err(LibraryError::MemberNotFound { id: 999 })
    );
}

#[test]
fn return_unknown_item() {
    let mut library = library_with_items();
    assert_eq!(
        library.return_item(999, 10),
        Err(LibraryError::ItemNotFound { id: 999 })
    );
}

#[test]
fn return_item_not_on_loan() {
    let mut library = library_with_items();
    assert_eq!(
        library.return_item(1, 10),
        Err(LibraryError::ItemNotOnLoan { id: 1 })
    );
}

#[test]
fn return_day_earlier_than_borrow_day() {
    let mut library = library_with_items();
    library.checkout(1, 100, 10).unwrap();
    assert_eq!(
        library.return_item(1, 5),
        Err(LibraryError::InvalidReturnDay {
            day_borrowed: 10,
            day_returned: 5,
        })
    );
}

// ── Display / trait tests ───────────────────────────────────────────────

#[test]
fn media_kind_display() {
    assert_eq!(
        format!("{}", MediaKind::Book { pages: 320 }),
        "Book (320 pages)"
    );
    assert_eq!(
        format!("{}", MediaKind::Audiobook { minutes: 540 }),
        "Audiobook (540 min)"
    );
    assert_eq!(
        format!("{}", MediaKind::Ebook { size_kb: 1_200 }),
        "Ebook (1200 KB)"
    );
}

#[test]
fn loan_status_display() {
    assert_eq!(format!("{}", LoanStatus::Available), "Available");
    assert_eq!(format!("{}", LoanStatus::Lost), "Lost");
    assert_eq!(
        format!(
            "{}",
            LoanStatus::OnLoan {
                member_id: 42,
                day_borrowed: 7
            }
        ),
        "On loan to member 42 since day 7"
    );
}

#[test]
fn item_display() {
    let item = Item::new(
        1,
        "Dune".into(),
        "Frank Herbert".into(),
        MediaKind::Book { pages: 320 },
    );
    let s = format!("{item}");
    assert!(s.contains("[1]"));
    assert!(s.contains("Dune"));
    assert!(s.contains("Frank Herbert"));
    assert!(s.contains("Book (320 pages)"));
    assert!(s.contains("Available"));
}

#[test]
fn loan_terms_book() {
    let kind = MediaKind::Book { pages: 100 };
    assert_eq!(kind.loan_days(), 21);
    assert_eq!(kind.daily_late_fee_cents(), 25);
    assert_eq!(kind.late_fee_cents(30), 9 * 25); // 30 - 21 = 9 overdue
    assert_eq!(kind.late_fee_cents(21), 0); // on time
    assert_eq!(kind.late_fee_cents(10), 0); // early
}

#[test]
fn loan_terms_audiobook() {
    let kind = MediaKind::Audiobook { minutes: 300 };
    assert_eq!(kind.loan_days(), 14);
    assert_eq!(kind.daily_late_fee_cents(), 25);
    assert_eq!(kind.late_fee_cents(20), 6 * 25);
}

#[test]
fn loan_terms_ebook_never_late() {
    let kind = MediaKind::Ebook { size_kb: 500 };
    assert_eq!(kind.loan_days(), 7);
    assert_eq!(kind.daily_late_fee_cents(), 0);
    assert_eq!(kind.late_fee_cents(100), 0);
}

#[test]
fn longest_loan_item_returns_book_over_audiobook() {
    let library = library_with_items();
    let longest = library.longest_loan_item().unwrap();
    // Book (21 days) > Audiobook (14) > Ebook (7)
    assert_eq!(longest.loan_days(), 21);
}

#[test]
fn available_items_excludes_on_loan() {
    let mut library = library_with_items();
    library.checkout(1, 100, 0).unwrap();
    let available = library.available_items();
    assert_eq!(available.len(), 3);
    assert!(available.iter().all(|i| i.id != 1));
}

#[test]
fn filter_items_generic_search() {
    let library = library_with_items();
    let books = library.filter_items(|i| matches!(i.kind, MediaKind::Book { .. }));
    assert_eq!(books.len(), 2);
}
