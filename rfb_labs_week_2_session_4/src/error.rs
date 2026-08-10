use std::fmt;

/// Every expected failure in the lending library.
///
/// This is the only file whose types are written for you. Nothing here should
/// ever be produced by a `panic!`, an `unwrap`, or an `expect` — these are
/// ordinary outcomes a caller is expected to handle.
#[derive(Debug, PartialEq, Eq)]
pub enum LibraryError {
    EmptyTitle,
    DuplicateItemId {
        id: u32,
    },
    DuplicateMemberId {
        id: u32,
    },
    ItemNotFound {
        id: u32,
    },
    MemberNotFound {
        id: u32,
    },
    ItemAlreadyOnLoan {
        id: u32,
        member_id: u32,
    },
    ItemNotOnLoan {
        id: u32,
    },
    ItemIsLost {
        id: u32,
    },
    BorrowLimitReached {
        member_id: u32,
        limit: usize,
    },
    InvalidReturnDay {
        day_borrowed: u32,
        day_returned: u32,
    },
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyTitle => write!(f, "item title must not be empty"),
            Self::DuplicateItemId { id } => {
                write!(f, "item with id {id} already exists")
            }
            Self::DuplicateMemberId { id } => {
                write!(f, "member with id {id} already exists")
            }
            Self::ItemNotFound { id } => {
                write!(f, "item with id {id} not found")
            }
            Self::MemberNotFound { id } => {
                write!(f, "member with id {id} not found")
            }
            Self::ItemAlreadyOnLoan { id, member_id } => {
                write!(f, "item {id} is already on loan to member {member_id}")
            }
            Self::ItemNotOnLoan { id } => {
                write!(f, "item {id} is not currently on loan")
            }
            Self::ItemIsLost { id } => {
                write!(f, "item {id} is reported as lost")
            }
            Self::BorrowLimitReached { member_id, limit } => write!(
                f,
                "member {member_id} has reached the borrow limit of {limit} items"
            ),
            Self::InvalidReturnDay {
                day_borrowed,
                day_returned,
            } => {
                write!(
                    f,
                    "return day {day_returned} is earlier than borrow day {day_borrowed}"
                )
            }
        }
    }
}

impl std::error::Error for LibraryError {}
