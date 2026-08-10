//! Small executable for Part 8 of the assignment.

use rfb_labs_week_2_session_4::{Item, Library, LibraryError, MediaKind, Member};

fn main() -> Result<(), LibraryError> {
    let mut library = Library::new();

    // Stock the library
    library.add_item(Item::new(
        1,
        "Dune".into(),
        "Frank Herbert".into(),
        MediaKind::Book { pages: 320 },
    ))?;
    library.add_item(Item::new(
        2,
        "Project Hail Mary".into(),
        "Andy Weir".into(),
        MediaKind::Audiobook { minutes: 540 },
    ))?;
    library.add_item(Item::new(
        3,
        "The Rust Programming Language".into(),
        "Steve Klabnik".into(),
        MediaKind::Ebook { size_kb: 1_200 },
    ))?;

    // Register a member
    library.register_member(Member::new(100, "Ada Lovelace".into()))?;

    // Checkout a book
    println!("=== Checking out Dune to Ada on day 10 ===");
    library.checkout(1, 100, 10)?;
    if let Some(item) = library.find_item(1) {
        println!("{item}");
    }

    // Return it late (day 40 — 19 days overdue for a 21-day book)
    println!();
    println!("=== Returning Dune on day 40 (late!) ===");
    let fee = library.return_item(1, 40)?;
    println!("Late fee: {fee} cents");
    if let Some(item) = library.find_item(1) {
        println!("{item}");
    }

    // Demonstrate a handled error: trying to check out a non-existent item
    println!();
    println!("=== Attempting to check out a non-existent item ===");
    let err = library.checkout(999, 100, 40).unwrap_err();
    println!("Error: {err}");

    Ok(())
}
