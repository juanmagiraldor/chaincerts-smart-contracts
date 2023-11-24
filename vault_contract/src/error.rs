use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    EmptyDIDs = 3,
    IssuerNotFound = 4,
    DidRevoked = 5,
    DidNotFound = 6,
    IssuerRevoked = 7,
    VCNotFound = 8,
    DuplicatedDID = 9,
}
