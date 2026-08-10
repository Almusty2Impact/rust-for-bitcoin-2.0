# Rust for Bitcoin 2.0 — Week 2, Session 4

Build a small lending library while practising structs, enums, traits,
ownership, borrowing, collections, and `Result`-based error handling. No
Bitcoin and no external crates — just Rust.

The crate is intentionally incomplete. Search for `TODO` and implement each
part; do not change the public type names or function signatures.

## Recommended workflow

1. Read [ASSIGNMENT.md](ASSIGNMENT.md).
2. Complete Part 2 in `error.rs`, then Part 3 in `library.rs`.
3. Remove `#[ignore]` from the relevant test and run it.
4. Complete the traits in Part 4 and the two operations in Parts 5–6.
5. Run the ownership experiments and record the errors.
6. Build the demo in `main.rs`.
7. Add the remaining required tests yourself.

```bash
cargo test
cargo test -- --ignored
cargo run
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
```

`cargo test` checks the starter project. Ignored tests intentionally exercise
unfinished code; enable them progressively rather than leaving them ignored in
the submission.

## Written answers

1. Why is `LoanStatus` an enum rather than a `bool` plus two `Option` fields?
A bool like on_loan: bool combined with Option<u32> fields for member
id and borrow day would allow invalid states — for example, on_loan: false
with Some(member_id) still set. The enum LoanStatus makes invalid
states unrepresentable: Available, OnLoan { member_id, day_borrowed },
and Lost are mutually exclusive, and the data each variant carries is
guaranteed to exist only when that variant is active. This is the
"make illegal states unrepresentable" principle.
2. What does `match` force you to do when a fourth `MediaKind` is added later?
Every match on MediaKind or LoanStatus must cover all variants. If a
new MediaKind::Magazine { issue: u32 } variant is added, the compiler will
produce an E0004 (non-exhaustive patterns) error at every match site that
doesn't handle it. This forces you to consciously decide how the new kind
behaves for loan length, daily fee, display formatting, and any other match
arm — you can't silently let a new variant fall through.
3. `Item::new` takes `String` rather than `&str`. Who owns the title afterwards?
The caller transfers ownership of the String into the Item. After
Item::new returns, the Item struct (and therefore its title field)
owns the heap-allocated string data. The original variable in the caller is
no longer usable — it was moved. This avoids an extra allocation or
lifetime parameter on the Item struct itself, keeping the type simpler.
4. Why does `add_item` take `self` by `&mut` but `item` by value?
&mut self is needed because the method mutates the library's internal
Vec<Item> by pushing a new entry. The item parameter is taken by value
(moved in) because the library becomes the sole owner of that Item. If
item were taken by reference (&Item), we would need to clone it to
store it, forcing an allocation the caller might not expect. Taking by
value makes the ownership transfer explicit and zero-cost.
5. When `add_item` returns `Err`, what happened to the `Item` the caller passed
   in? Was that a good design choice, and what is the alternative?
The Item is dropped. Because add_item takes item by value, ownership
transfers into the function body regardless of whether it returns Ok or
Err. If the title is empty or the id is a duplicate, the function returns
an error and the Item is destroyed. This is a deliberate choice: it
prevents the caller from accidentally reusing a partially-validated item.
The alternative is to take &Item and clone on success (which costs an
allocation) or return the item back in the error variant (e.g.
Err((LibraryError, Item))), allowing the caller to inspect or fix it.
6. Why does `find_item` return `Option<&Item>` rather than `Option<Item>`?
Returning &Item (a borrowed reference) avoids cloning the entire struct
including its heap-allocated String fields. The reference points directly
into the library's internal Vec<Item>, so the lookup is O(n) with zero
extra allocation. If it returned Option<Item>, every lookup would
deep-copy the title and author strings, which is wasteful when the caller
only wants to inspect the item. The borrow also ensures the item cannot be
modified outside the library's control while the reference is held.
7. What is the lifetime `'a` in `items_by_author` actually saying?
The signature fn items_by_author<'a>(&'a self, author: &str) -> Vec<&'a Item>
ties the output references to the lifetime of &self. It tells the
compiler: "the returned &Item references are valid for as long as the
caller holds the borrow on self (the Library), and no longer." This
prevents use-after-free — you can't drop the library while still holding
references to its items. The author parameter doesn't need the same
lifetime because it's only compared and not stored or returned.
8. Why can't `checkout` hold a `&mut Item` and a `&mut Member` from the same
   `Library` at once, and how did you structure the method around that?
Rust's borrow checker enforces the rule that you can have either any number
of immutable references or exactly one mutable reference, but not both.
Getting &mut self.items[idx] and &mut self.members[idx] from the same
self requires two simultaneous mutable borrows of self, which the
compiler rejects. I structured checkout around this by using usize
indices (item_idx, member_idx) found via Iterator::position, then
accessing self.items[item_idx] and self.members[member_idx] through
direct indexing on the struct fields. Because each index access is a
separate statement and the compiler can verify they don't alias, this
works without needing to split the struct into separate borrows.
9. Why are `Library`'s fields private?
Private fields enforce the invariant that an item's LoanStatus::OnLoan
always matches the corresponding member's borrowed_item_ids entry. If
callers could directly push to borrowed_item_ids or change an item's
status, these two pieces of state could drift apart — e.g. an item could
be marked Available while the member's list still contains its id.
By routing all mutations through checkout and return_item, the library
guarantees consistency. This is encapsulation protecting an invariant.
10. What duplication does the provided `late_fee_cents` remove, and what would
    you lose by making it a free function instead?
Without the default method on LoanTerms, both impl LoanTerms for MediaKind and impl LoanTerms for Item would need to duplicate the
formula (days_held - loan_days) * daily_fee. A free function
fn late_fee(terms: &impl LoanTerms, days_held: u32) -> u32 would
remove the duplication, but callers would need to remember to call the
free function instead of the method, and it wouldn't be discoverable
through method autocomplete on the trait. As a default trait method, the
formula is inherited automatically by every implementor and can be
called with dot syntax — item.late_fee_cents(30) — which is more
ergonomic and idiomatic.
11. Why is `Result` preferable to `panic!` for validation failures? Name a
    place in this crate where a panic would be defensible.
Result forces the caller to handle the failure explicitly (with ?,
match, or .unwrap()). Validation failures like duplicate IDs or empty
titles are expected outcomes — they depend on caller-supplied data and can
be recovered from by the caller fixing their input and trying again.
A panic! would crash the entire process, which is inappropriate for
recoverable business logic. A defensible panic in this crate would be
calling .unwrap() on the member_idx lookup inside checkout after we
have already confirmed the member exists — this is an internal invariant
that can never fail if the earlier validation logic is correct, so a panic
there signals a programmer bug, not a data problem.
12. Which derive did you deliberately leave off a type, and why?
I left Clone off Member. Although Member only contains owned data
(u32, String, Vec<u32>), automatically deriving Clone would make
it trivial to copy member structs around, potentially allowing two copies
of a member's borrowed_item_ids to exist independently. If the library
and some external code both held a clone and modified their respective
lists, the borrowed-count invariant could break. By not deriving Clone,
the type signals that members should be accessed through the library's
borrowing API, not copied wholesale.

## Design notes

I kept an item's status and a member's borrowed list from drifting apart by
routing all state changes through exactly two methods: checkout and
return_item. Both methods validate first (returning Err early), then
mutate the item's status and the member's borrowed_item_ids in the same
logical operation. There is no public API that can change one without the
other. The find_item and find_member methods return shared references,
so callers can read but not mutate, further protecting the invariant.

For the optional generic search (Part 9), I added filter_items taking
Fn(&Item) -> bool. The existing items_by_author and available_items
could be reimplemented as library.filter_items(|i| i.author == author) and
library.filter_items(|i| i.status == LoanStatus::Available). I kept the
named methods because they're more readable and self-documenting, but the
generic version is useful for ad-hoc queries that don't deserve their own
method.

## Example output

almusty@DESKTOP-OG1A0K3:/mnt/c/Users/USER/Desktop/TheBuidl_Rust-for-bitcoin-2.0/rust-for-bitcoin-2.0/rfb_labs_week_2_session_4$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/rfb_labs_week_2_session_4`
=== Checking out Dune to Ada on day 10 ===
[1] "Dune" by Frank Herbert (Book (320 pages)) — On loan to member 100 since day 10

=== Returning Dune on day 40 (late!) ===
Late fee: 225 cents
[1] "Dune" by Frank Herbert (Book (320 pages)) — Available

=== Attempting to check out a non-existent item ===
Error: item with id 999 not found
