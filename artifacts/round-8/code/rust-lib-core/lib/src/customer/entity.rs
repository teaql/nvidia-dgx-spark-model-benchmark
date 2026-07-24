// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/customer
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
#[teaql(entity = "Customer", table = "customer_data", data_service = "sqlite")]
pub struct Customer {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    customer_id: String,

// @source model.xml:2
    status: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "merchant_ref")]
    merchant_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Merchant", local_key = "merchant_ref_id", foreign_key = "id"))]
    merchant_ref: Option<crate::Merchant>,
    #[teaql(boxed_relations)]
    pub _relations: Box<CustomerReverseRelations>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Customer {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            customer_id: String::new(),
            status: String::new(),
            version: 0_i64,
            merchant_ref_id: 0_u64,
            merchant_ref: None,
            _relations: Box::new(CustomerReverseRelations::new()),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Customer", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.merchant_ref {
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

    pub fn customer_id(&self) -> String {
        self.changed_customer_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.customer_id.clone())
    }

    pub fn update_customer_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.customer_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.customer_id.clone());
        self.root.set(self.entity_key(), "customer_id", value);
        self
    }

    pub fn changed_customer_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "customer_id")
    }

    pub fn eval_customer_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("customer_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_id".to_string(), attempted_path: "customer_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.customer_id())
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
    pub fn merchant_ref_id(&self) -> u64 {
        self.changed_merchant_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_ref_id)
    }

    pub fn update_merchant_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_ref_id = value.try_u64().unwrap_or(self.merchant_ref_id.clone());
        self.root.set(self.entity_key(), "merchant_ref_id", value);
        self
    }

    pub fn changed_merchant_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_ref_id")
    }

    pub fn eval_merchant_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref_id".to_string(), attempted_path: "merchant_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_ref_id())
                }}
    pub fn merchant_ref(&self) -> Option<&crate::Merchant> {
        self.merchant_ref.as_ref()
    }

    pub fn eval_merchant_ref(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref".to_string(), attempted_path: "merchant_ref".to_string() }
        } else {
            match &self.merchant_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn move_order_list(&self) -> &SmartList<crate::MoveOrder> {
        &self._relations.move_order_list
    }

    pub fn move_order_list_mut(&mut self) -> &mut SmartList<crate::MoveOrder> {
        &mut self._relations.move_order_list
    }

    pub fn eval_move_order_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MoveOrder>> {
        if !self.is_loaded("move_order_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "move_order_list".to_string(), attempted_path: "move_order_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.move_order_list)
        }
    }

    pub fn private_customer_profile_list(&self) -> &SmartList<crate::PrivateCustomerProfile> {
        &self._relations.private_customer_profile_list
    }

    pub fn private_customer_profile_list_mut(&mut self) -> &mut SmartList<crate::PrivateCustomerProfile> {
        &mut self._relations.private_customer_profile_list
    }

    pub fn eval_private_customer_profile_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::PrivateCustomerProfile>> {
        if !self.is_loaded("private_customer_profile_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "private_customer_profile_list".to_string(), attempted_path: "private_customer_profile_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.private_customer_profile_list)
        }
    }

    pub fn corporate_customer_profile_list(&self) -> &SmartList<crate::CorporateCustomerProfile> {
        &self._relations.corporate_customer_profile_list
    }

    pub fn corporate_customer_profile_list_mut(&mut self) -> &mut SmartList<crate::CorporateCustomerProfile> {
        &mut self._relations.corporate_customer_profile_list
    }

    pub fn eval_corporate_customer_profile_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CorporateCustomerProfile>> {
        if !self.is_loaded("corporate_customer_profile_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "corporate_customer_profile_list".to_string(), attempted_path: "corporate_customer_profile_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.corporate_customer_profile_list)
        }
    }

    pub fn customer_contact_list(&self) -> &SmartList<crate::CustomerContact> {
        &self._relations.customer_contact_list
    }

    pub fn customer_contact_list_mut(&mut self) -> &mut SmartList<crate::CustomerContact> {
        &mut self._relations.customer_contact_list
    }

    pub fn eval_customer_contact_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerContact>> {
        if !self.is_loaded("customer_contact_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_contact_list".to_string(), attempted_path: "customer_contact_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_contact_list)
        }
    }

    pub fn billing_profile_list(&self) -> &SmartList<crate::BillingProfile> {
        &self._relations.billing_profile_list
    }

    pub fn billing_profile_list_mut(&mut self) -> &mut SmartList<crate::BillingProfile> {
        &mut self._relations.billing_profile_list
    }

    pub fn eval_billing_profile_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::BillingProfile>> {
        if !self.is_loaded("billing_profile_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "billing_profile_list".to_string(), attempted_path: "billing_profile_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.billing_profile_list)
        }
    }

    pub fn customer_history_list(&self) -> &SmartList<crate::CustomerHistory> {
        &self._relations.customer_history_list
    }

    pub fn customer_history_list_mut(&mut self) -> &mut SmartList<crate::CustomerHistory> {
        &mut self._relations.customer_history_list
    }

    pub fn eval_customer_history_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerHistory>> {
        if !self.is_loaded("customer_history_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_history_list".to_string(), attempted_path: "customer_history_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_history_list)
        }
    }

    pub fn customer_preference_list(&self) -> &SmartList<crate::CustomerPreference> {
        &self._relations.customer_preference_list
    }

    pub fn customer_preference_list_mut(&mut self) -> &mut SmartList<crate::CustomerPreference> {
        &mut self._relations.customer_preference_list
    }

    pub fn eval_customer_preference_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerPreference>> {
        if !self.is_loaded("customer_preference_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_preference_list".to_string(), attempted_path: "customer_preference_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_preference_list)
        }
    }

    pub fn customer_consent_list(&self) -> &SmartList<crate::CustomerConsent> {
        &self._relations.customer_consent_list
    }

    pub fn customer_consent_list_mut(&mut self) -> &mut SmartList<crate::CustomerConsent> {
        &mut self._relations.customer_consent_list
    }

    pub fn eval_customer_consent_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerConsent>> {
        if !self.is_loaded("customer_consent_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_consent_list".to_string(), attempted_path: "customer_consent_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_consent_list)
        }
    }

    pub fn customer_feedback_list(&self) -> &SmartList<crate::CustomerFeedback> {
        &self._relations.customer_feedback_list
    }

    pub fn customer_feedback_list_mut(&mut self) -> &mut SmartList<crate::CustomerFeedback> {
        &mut self._relations.customer_feedback_list
    }

    pub fn eval_customer_feedback_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CustomerFeedback>> {
        if !self.is_loaded("customer_feedback_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "customer_feedback_list".to_string(), attempted_path: "customer_feedback_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.customer_feedback_list)
        }
    }

    pub fn loyalty_tier_list(&self) -> &SmartList<crate::LoyaltyTier> {
        &self._relations.loyalty_tier_list
    }

    pub fn loyalty_tier_list_mut(&mut self) -> &mut SmartList<crate::LoyaltyTier> {
        &mut self._relations.loyalty_tier_list
    }

    pub fn eval_loyalty_tier_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::LoyaltyTier>> {
        if !self.is_loaded("loyalty_tier_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "loyalty_tier_list".to_string(), attempted_path: "loyalty_tier_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.loyalty_tier_list)
        }
    }

    pub fn referral_code_list(&self) -> &SmartList<crate::ReferralCode> {
        &self._relations.referral_code_list
    }

    pub fn referral_code_list_mut(&mut self) -> &mut SmartList<crate::ReferralCode> {
        &mut self._relations.referral_code_list
    }

    pub fn eval_referral_code_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ReferralCode>> {
        if !self.is_loaded("referral_code_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "referral_code_list".to_string(), attempted_path: "referral_code_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.referral_code_list)
        }
    }

    pub fn communication_log_list(&self) -> &SmartList<crate::CommunicationLog> {
        &self._relations.communication_log_list
    }

    pub fn communication_log_list_mut(&mut self) -> &mut SmartList<crate::CommunicationLog> {
        &mut self._relations.communication_log_list
    }

    pub fn eval_communication_log_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CommunicationLog>> {
        if !self.is_loaded("communication_log_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "communication_log_list".to_string(), attempted_path: "communication_log_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.communication_log_list)
        }
    }

    pub fn service_rating_list(&self) -> &SmartList<crate::ServiceRating> {
        &self._relations.service_rating_list
    }

    pub fn service_rating_list_mut(&mut self) -> &mut SmartList<crate::ServiceRating> {
        &mut self._relations.service_rating_list
    }

    pub fn eval_service_rating_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServiceRating>> {
        if !self.is_loaded("service_rating_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_rating_list".to_string(), attempted_path: "service_rating_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.service_rating_list)
        }
    }

    pub fn account_status_list(&self) -> &SmartList<crate::AccountStatus> {
        &self._relations.account_status_list
    }

    pub fn account_status_list_mut(&mut self) -> &mut SmartList<crate::AccountStatus> {
        &mut self._relations.account_status_list
    }

    pub fn eval_account_status_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AccountStatus>> {
        if !self.is_loaded("account_status_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "account_status_list".to_string(), attempted_path: "account_status_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.account_status_list)
        }
    }

    pub fn contact_method_list(&self) -> &SmartList<crate::ContactMethod> {
        &self._relations.contact_method_list
    }

    pub fn contact_method_list_mut(&mut self) -> &mut SmartList<crate::ContactMethod> {
        &mut self._relations.contact_method_list
    }

    pub fn eval_contact_method_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ContactMethod>> {
        if !self.is_loaded("contact_method_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "contact_method_list".to_string(), attempted_path: "contact_method_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self._relations.contact_method_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CustomerRepository<'a>>>
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
            .customer_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Customer"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

#[derive(Clone, Debug, PartialEq, teaql_macros::TeaqlReverseRelations)]
pub struct CustomerReverseRelations {
#[teaql(relation(target = "MoveOrder", local_key = "id", foreign_key = "customer_ref_id", many))]
    move_order_list: SmartList<crate::MoveOrder>,
#[teaql(relation(target = "PrivateCustomerProfile", local_key = "id", foreign_key = "customer_ref_id", many))]
    private_customer_profile_list: SmartList<crate::PrivateCustomerProfile>,
#[teaql(relation(target = "CorporateCustomerProfile", local_key = "id", foreign_key = "customer_ref_id", many))]
    corporate_customer_profile_list: SmartList<crate::CorporateCustomerProfile>,
#[teaql(relation(target = "CustomerContact", local_key = "id", foreign_key = "customer_ref_id", many))]
    customer_contact_list: SmartList<crate::CustomerContact>,
#[teaql(relation(target = "BillingProfile", local_key = "id", foreign_key = "customer_ref_id", many))]
    billing_profile_list: SmartList<crate::BillingProfile>,
#[teaql(relation(target = "CustomerHistory", local_key = "id", foreign_key = "customer_ref_id", many))]
    customer_history_list: SmartList<crate::CustomerHistory>,
#[teaql(relation(target = "CustomerPreference", local_key = "id", foreign_key = "customer_ref_id", many))]
    customer_preference_list: SmartList<crate::CustomerPreference>,
#[teaql(relation(target = "CustomerConsent", local_key = "id", foreign_key = "customer_ref_id", many))]
    customer_consent_list: SmartList<crate::CustomerConsent>,
#[teaql(relation(target = "CustomerFeedback", local_key = "id", foreign_key = "customer_ref_id", many))]
    customer_feedback_list: SmartList<crate::CustomerFeedback>,
#[teaql(relation(target = "LoyaltyTier", local_key = "id", foreign_key = "customer_ref_id", many))]
    loyalty_tier_list: SmartList<crate::LoyaltyTier>,
#[teaql(relation(target = "ReferralCode", local_key = "id", foreign_key = "customer_ref_id", many))]
    referral_code_list: SmartList<crate::ReferralCode>,
#[teaql(relation(target = "CommunicationLog", local_key = "id", foreign_key = "customer_ref_id", many))]
    communication_log_list: SmartList<crate::CommunicationLog>,
#[teaql(relation(target = "ServiceRating", local_key = "id", foreign_key = "customer_ref_id", many))]
    service_rating_list: SmartList<crate::ServiceRating>,
#[teaql(relation(target = "AccountStatus", local_key = "id", foreign_key = "customer_ref_id", many))]
    account_status_list: SmartList<crate::AccountStatus>,
#[teaql(relation(target = "ContactMethod", local_key = "id", foreign_key = "customer_ref_id", many))]
    contact_method_list: SmartList<crate::ContactMethod>,
}

impl CustomerReverseRelations {
    pub fn new() -> Self {
        Self {
            move_order_list: Default::default(),
            private_customer_profile_list: Default::default(),
            corporate_customer_profile_list: Default::default(),
            customer_contact_list: Default::default(),
            billing_profile_list: Default::default(),
            customer_history_list: Default::default(),
            customer_preference_list: Default::default(),
            customer_consent_list: Default::default(),
            customer_feedback_list: Default::default(),
            loyalty_tier_list: Default::default(),
            referral_code_list: Default::default(),
            communication_log_list: Default::default(),
            service_rating_list: Default::default(),
            account_status_list: Default::default(),
            contact_method_list: Default::default(),
        }
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        for entity in &mut self.move_order_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.private_customer_profile_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.corporate_customer_profile_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_contact_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.billing_profile_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_history_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_preference_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_consent_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.customer_feedback_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.loyalty_tier_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.referral_code_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.communication_log_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_rating_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.account_status_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.contact_method_list {
            entity.attach_root_recursive(root.clone());
        }
    }
}
