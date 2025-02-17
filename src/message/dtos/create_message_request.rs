pub struct CreateMessageRequestDto<'a> {
    pub user_id: &'a str,
    pub text: &'a str,
}
