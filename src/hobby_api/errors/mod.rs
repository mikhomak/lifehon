pub struct CreateUserErrors;
pub struct LoginErrors;

impl CreateUserErrors {
    pub const NAME_TAKEN: &'static str = "[CREATE_USER_001] - the user name is already taken";
    pub const EMAIL_TAKEN: &'static str = "[CREATE_USER_002] - the email is already taken";
    pub const EMPTY_NAME: &'static str = "[CREATE_USER_003] - the name is empty";
    pub const EMPTY_EMAIL: &'static str = "[CREATE_USER_004] - the email is empty";
    pub const EMPTY_PASSWORD: &'static str = "[CREATE_USER_005] - the password is empty";
    pub const CONSENT_NOT_AGREED: &'static str = "[CREATE_USER_006] - the consent is not agreed";
}

impl LoginErrors {}
