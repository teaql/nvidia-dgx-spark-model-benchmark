// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/user_account
use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "UserAccount", table = "user_account_data", data_service = "sqlite")]
pub struct UserAccount {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    account_id: String,

// @source model.xml:2
    status: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "employee_ref")]
    employee_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Employee", local_key = "employee_ref_id", foreign_key = "id"))]
    employee_ref: Option<crate::Employee>,
    #[teaql(boxed_relations)]
    pub _relations: Box<UserAccountReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl UserAccount {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            account_id: String::new(),
            status: String::new(),
            version: 0_i64,
            employee_ref_id: 0_u64,
            employee_ref: None,
            _relations: Box::new(UserAccountReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("UserAccount", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.employee_ref {
            entity.attach_root_recursive(root.clone());
        }
        self._relations.attach_root_recursive(root.clone());
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn account_id(&self) -> String {
        self.changed_account_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.account_id.clone())
    }

    pub fn update_account_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.account_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.account_id.clone());
        self.root.set(self.entity_key(), "account_id", value);
        self
    }

    pub fn changed_account_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "account_id")
    }

    pub fn eval_account_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("account_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "account_id".to_string(), attempted_path: "account_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.account_id())
                }}

    pub fn status(&self) -> String {
        self.changed_status().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.status.clone())
    }

    pub fn update_status(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.status = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.status.clone());
        self.root.set(self.entity_key(), "status", value);
        self
    }

    pub fn changed_status(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "status")
    }

    pub fn eval_status(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("status") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "status".to_string(), attempted_path: "status".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.status())
                }}

    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn employee_ref_id(&self) -> u64 {
        self.changed_employee_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.employee_ref_id)
    }

    pub fn update_employee_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employee_ref_id = value.try_u64().unwrap_or(self.employee_ref_id.clone());
        self.root.set(self.entity_key(), "employee_ref_id", value);
        self
    }

    pub fn changed_employee_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employee_ref_id")
    }

    pub fn eval_employee_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("employee_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_ref_id".to_string(), attempted_path: "employee_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employee_ref_id())
                }}
    pub fn employee_ref(&self) -> Option<&crate::Employee> {
        self.employee_ref.as_ref()
    }

    pub fn eval_employee_ref(&self) -> teaql_core::eval::EvalResult<&crate::Employee> {
        if !self.is_loaded("employee_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_ref".to_string(), attempted_path: "employee_ref".to_string() }
        } else {
            match &self.employee_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn user_role_assignment_list(&self) -> &SmartList<crate::UserRoleAssignment> {
        &self._relations.user_role_assignment_list
    }

    pub fn user_role_assignment_list_mut(&mut self) -> &mut SmartList<crate::UserRoleAssignment> {
        &mut self._relations.user_role_assignment_list
    }

    pub fn eval_user_role_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UserRoleAssignment>> {
        if !self.is_loaded("user_role_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_role_assignment_list".to_string(), attempted_path: "user_role_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.user_role_assignment_list)
        }
    }

    pub fn magic_link_list(&self) -> &SmartList<crate::MagicLink> {
        &self._relations.magic_link_list
    }

    pub fn magic_link_list_mut(&mut self) -> &mut SmartList<crate::MagicLink> {
        &mut self._relations.magic_link_list
    }

    pub fn eval_magic_link_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MagicLink>> {
        if !self.is_loaded("magic_link_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "magic_link_list".to_string(), attempted_path: "magic_link_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.magic_link_list)
        }
    }

    pub fn user_session_list(&self) -> &SmartList<crate::UserSession> {
        &self._relations.user_session_list
    }

    pub fn user_session_list_mut(&mut self) -> &mut SmartList<crate::UserSession> {
        &mut self._relations.user_session_list
    }

    pub fn eval_user_session_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::UserSession>> {
        if !self.is_loaded("user_session_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "user_session_list".to_string(), attempted_path: "user_session_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.user_session_list)
        }
    }

    pub fn access_token_list(&self) -> &SmartList<crate::AccessToken> {
        &self._relations.access_token_list
    }

    pub fn access_token_list_mut(&mut self) -> &mut SmartList<crate::AccessToken> {
        &mut self._relations.access_token_list
    }

    pub fn eval_access_token_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AccessToken>> {
        if !self.is_loaded("access_token_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "access_token_list".to_string(), attempted_path: "access_token_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.access_token_list)
        }
    }

    pub fn two_factor_auth_list(&self) -> &SmartList<crate::TwoFactorAuth> {
        &self._relations.two_factor_auth_list
    }

    pub fn two_factor_auth_list_mut(&mut self) -> &mut SmartList<crate::TwoFactorAuth> {
        &mut self._relations.two_factor_auth_list
    }

    pub fn eval_two_factor_auth_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::TwoFactorAuth>> {
        if !self.is_loaded("two_factor_auth_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "two_factor_auth_list".to_string(), attempted_path: "two_factor_auth_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.two_factor_auth_list)
        }
    }

    pub fn login_attempt_list(&self) -> &SmartList<crate::LoginAttempt> {
        &self._relations.login_attempt_list
    }

    pub fn login_attempt_list_mut(&mut self) -> &mut SmartList<crate::LoginAttempt> {
        &mut self._relations.login_attempt_list
    }

    pub fn eval_login_attempt_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LoginAttempt>> {
        if !self.is_loaded("login_attempt_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "login_attempt_list".to_string(), attempted_path: "login_attempt_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.login_attempt_list)
        }
    }

    pub fn activity_log_list(&self) -> &SmartList<crate::ActivityLog> {
        &self._relations.activity_log_list
    }

    pub fn activity_log_list_mut(&mut self) -> &mut SmartList<crate::ActivityLog> {
        &mut self._relations.activity_log_list
    }

    pub fn eval_activity_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ActivityLog>> {
        if !self.is_loaded("activity_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "activity_log_list".to_string(), attempted_path: "activity_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.activity_log_list)
        }
    }

    pub fn data_export_list(&self) -> &SmartList<crate::DataExport> {
        &self._relations.data_export_list
    }

    pub fn data_export_list_mut(&mut self) -> &mut SmartList<crate::DataExport> {
        &mut self._relations.data_export_list
    }

    pub fn eval_data_export_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DataExport>> {
        if !self.is_loaded("data_export_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "data_export_list".to_string(), attempted_path: "data_export_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.data_export_list)
        }
    }

    pub fn notification_list(&self) -> &SmartList<crate::Notification> {
        &self._relations.notification_list
    }

    pub fn notification_list_mut(&mut self) -> &mut SmartList<crate::Notification> {
        &mut self._relations.notification_list
    }

    pub fn eval_notification_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::Notification>> {
        if !self.is_loaded("notification_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "notification_list".to_string(), attempted_path: "notification_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.notification_list)
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }

    pub(crate) async fn save<'a, C>(
        &self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::UserAccountRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let root = ctx.user_context().entity_root();
        let key = self.entity_key();
        let has_ledger_change = (self.id != 0)
            && (root.current_change_set().changes().contains_key(&key)
                || root.is_marked_as_delete(&key)
                || root.is_new(&key));
        let repository = ctx
            .user_account_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("UserAccount"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct UserAccountReverseRelations {
#[teaql(relation(target = "UserRoleAssignment", local_key = "id", foreign_key = "user_account_ref_id", many))]
    user_role_assignment_list: SmartList<crate::UserRoleAssignment>,
#[teaql(relation(target = "MagicLink", local_key = "id", foreign_key = "user_account_ref_id", many))]
    magic_link_list: SmartList<crate::MagicLink>,
#[teaql(relation(target = "UserSession", local_key = "id", foreign_key = "user_account_ref_id", many))]
    user_session_list: SmartList<crate::UserSession>,
#[teaql(relation(target = "AccessToken", local_key = "id", foreign_key = "user_account_ref_id", many))]
    access_token_list: SmartList<crate::AccessToken>,
#[teaql(relation(target = "TwoFactorAuth", local_key = "id", foreign_key = "user_account_ref_id", many))]
    two_factor_auth_list: SmartList<crate::TwoFactorAuth>,
#[teaql(relation(target = "LoginAttempt", local_key = "id", foreign_key = "user_account_ref_id", many))]
    login_attempt_list: SmartList<crate::LoginAttempt>,
#[teaql(relation(target = "ActivityLog", local_key = "id", foreign_key = "user_account_ref_id", many))]
    activity_log_list: SmartList<crate::ActivityLog>,
#[teaql(relation(target = "DataExport", local_key = "id", foreign_key = "user_account_ref_id", many))]
    data_export_list: SmartList<crate::DataExport>,
#[teaql(relation(target = "Notification", local_key = "id", foreign_key = "user_account_ref_id", many))]
    notification_list: SmartList<crate::Notification>,
}

impl UserAccountReverseRelations {
    pub fn new() -> Self {
        Self {
            user_role_assignment_list: Default::default(),
            magic_link_list: Default::default(),
            user_session_list: Default::default(),
            access_token_list: Default::default(),
            two_factor_auth_list: Default::default(),
            login_attempt_list: Default::default(),
            activity_log_list: Default::default(),
            data_export_list: Default::default(),
            notification_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.user_role_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.magic_link_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.user_session_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.access_token_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.two_factor_auth_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.login_attempt_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.activity_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.data_export_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.notification_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
