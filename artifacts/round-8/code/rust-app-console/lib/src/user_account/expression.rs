#[derive(Clone)]
pub struct UserAccountExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::UserAccount>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> UserAccountExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::UserAccount>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::UserAccount> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::UserAccount> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::UserAccount {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_account_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("account_id", |entity| entity.eval_account_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_status(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("status", |entity| entity.eval_status());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("employee_ref_id", |entity| entity.eval_employee_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_employee_ref(self) -> crate::EmployeeExpression<'a> {
        let next = self.result.and_then("employee_ref", |entity| entity.eval_employee_ref());
        crate::EmployeeExpression::new(next, self.root_desc.clone())
    }
    pub fn get_user_role_assignment_list(self) -> crate::UserRoleAssignmentListExpression<'a> {
        let next = self.result.and_then("user_role_assignment_list", |entity| entity.eval_user_role_assignment_list());
        crate::UserRoleAssignmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_magic_link_list(self) -> crate::MagicLinkListExpression<'a> {
        let next = self.result.and_then("magic_link_list", |entity| entity.eval_magic_link_list());
        crate::MagicLinkListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_user_session_list(self) -> crate::UserSessionListExpression<'a> {
        let next = self.result.and_then("user_session_list", |entity| entity.eval_user_session_list());
        crate::UserSessionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_access_token_list(self) -> crate::AccessTokenListExpression<'a> {
        let next = self.result.and_then("access_token_list", |entity| entity.eval_access_token_list());
        crate::AccessTokenListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_two_factor_auth_list(self) -> crate::TwoFactorAuthListExpression<'a> {
        let next = self.result.and_then("two_factor_auth_list", |entity| entity.eval_two_factor_auth_list());
        crate::TwoFactorAuthListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_login_attempt_list(self) -> crate::LoginAttemptListExpression<'a> {
        let next = self.result.and_then("login_attempt_list", |entity| entity.eval_login_attempt_list());
        crate::LoginAttemptListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_activity_log_list(self) -> crate::ActivityLogListExpression<'a> {
        let next = self.result.and_then("activity_log_list", |entity| entity.eval_activity_log_list());
        crate::ActivityLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_data_export_list(self) -> crate::DataExportListExpression<'a> {
        let next = self.result.and_then("data_export_list", |entity| entity.eval_data_export_list());
        crate::DataExportListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_notification_list(self) -> crate::NotificationListExpression<'a> {
        let next = self.result.and_then("notification_list", |entity| entity.eval_notification_list());
        crate::NotificationListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct UserAccountListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::UserAccount>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> UserAccountListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::UserAccount>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::UserAccount>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::UserAccount>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::UserAccount> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::UserAccountExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::UserAccountExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::UserAccountExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::UserAccountExpression::new(next, self.root_desc.clone())
    }
}