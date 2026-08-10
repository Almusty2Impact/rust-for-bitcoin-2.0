use crate::catalogue::{Item, LoanStatus, LoanTerms};
use crate::error::LibraryError;
use crate::member::Member;

pub const MAX_ITEMS_PER_MEMBER: usize = 3;

/// Owns every item and every member.
///
/// The fields are private because the library is responsible for keeping an
/// item's `LoanStatus` and a member's borrowed-id list in agreement. Callers
/// reach the data through the borrowing lookups below.
#[derive(Debug, Default)]
pub struct Library {
    items: Vec<Item>,
    members: Vec<Member>,
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), LibraryError> {
        if item.title.is_empty() {
            return Err(LibraryError::EmptyTitle);
        }
        if self.items.iter().any(|i| i.id == item.id) {
            return Err(LibraryError::DuplicateItemId { id: item.id });
        }
        self.items.push(item);
        Ok(())
    }

    pub fn register_member(&mut self, member: Member) -> Result<(), LibraryError> {
        if self.members.iter().any(|m| m.id == member.id) {
            return Err(LibraryError::DuplicateMemberId { id: member.id });
        }
        self.members.push(member);
        Ok(())
    }

    pub fn find_item(&self, id: u32) -> Option<&Item> {
        self.items.iter().find(|i| i.id == id)
    }

    pub fn find_member(&self, id: u32) -> Option<&Member> {
        self.members.iter().find(|m| m.id == id)
    }

    pub fn items_by_author<'a>(&'a self, author: &str) -> Vec<&'a Item> {
        self.items.iter().filter(|i| i.author == author).collect()
    }

    pub fn available_items(&self) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|i| i.status == LoanStatus::Available)
            .collect()
    }

    pub fn longest_loan_item(&self) -> Option<&Item> {
        self.items
            .iter()
            .filter(|i| i.status == LoanStatus::Available)
            .max_by_key(|i| i.loan_days())
    }

    pub fn checkout(&mut self, item_id: u32, member_id: u32, day: u32) -> Result<(), LibraryError> {
        // Validate order: unknown item, unknown member, lost, already on loan, borrow limit
        let item_idx = self
            .items
            .iter()
            .position(|i| i.id == item_id)
            .ok_or(LibraryError::ItemNotFound { id: item_id })?;

        if !self.members.iter().any(|m| m.id == member_id) {
            return Err(LibraryError::MemberNotFound { id: member_id });
        }

        match self.items[item_idx].status {
            LoanStatus::Lost => return Err(LibraryError::ItemIsLost { id: item_id }),
            LoanStatus::OnLoan {
                member_id: mid,
                day_borrowed: _,
            } => {
                return Err(LibraryError::ItemAlreadyOnLoan {
                    id: item_id,
                    member_id: mid,
                });
            }
            LoanStatus::Available => {}
        }

        let member_idx = self.members.iter().position(|m| m.id == member_id).unwrap();

        if self.members[member_idx].borrowed_item_ids.len() >= MAX_ITEMS_PER_MEMBER {
            return Err(LibraryError::BorrowLimitReached {
                member_id,
                limit: MAX_ITEMS_PER_MEMBER,
            });
        }

        // Mutate both
        self.items[item_idx].status = LoanStatus::OnLoan {
            member_id,
            day_borrowed: day,
        };
        self.members[member_idx].borrowed_item_ids.push(item_id);

        Ok(())
    }

    /// Returns the late fee owed, in cents.
    pub fn return_item(&mut self, item_id: u32, day: u32) -> Result<u32, LibraryError> {
        let item_idx = self
            .items
            .iter()
            .position(|i| i.id == item_id)
            .ok_or(LibraryError::ItemNotFound { id: item_id })?;

        match self.items[item_idx].status {
            LoanStatus::Lost => Err(LibraryError::ItemIsLost { id: item_id }),
            LoanStatus::Available => Err(LibraryError::ItemNotOnLoan { id: item_id }),
            LoanStatus::OnLoan {
                member_id,
                day_borrowed,
            } => {
                let days_held =
                    day.checked_sub(day_borrowed)
                        .ok_or(LibraryError::InvalidReturnDay {
                            day_borrowed,
                            day_returned: day,
                        })?;

                let fee = self.items[item_idx].late_fee_cents(days_held);

                self.items[item_idx].status = LoanStatus::Available;

                let member_idx = self.members.iter().position(|m| m.id == member_id).unwrap();
                self.members[member_idx]
                    .borrowed_item_ids
                    .retain(|&id| id != item_id);

                Ok(fee)
            }
        }
    }

    pub fn filter_items<F>(&self, predicate: F) -> Vec<&Item>
    where
        F: Fn(&Item) -> bool,
    {
        self.items.iter().filter(|item| predicate(item)).collect()
    }
}
