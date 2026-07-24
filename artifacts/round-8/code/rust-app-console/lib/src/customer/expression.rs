#[derive(Clone)]
pub struct CustomerExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Customer>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CustomerExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Customer>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Customer> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Customer> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Customer {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("customer_id", |entity| entity.eval_customer_id());
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
    pub fn get_merchant_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("merchant_ref_id", |entity| entity.eval_merchant_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_merchant_ref(self) -> crate::MerchantExpression<'a> {
        let next = self.result.and_then("merchant_ref", |entity| entity.eval_merchant_ref());
        crate::MerchantExpression::new(next, self.root_desc.clone())
    }
    pub fn get_move_order_list(self) -> crate::MoveOrderListExpression<'a> {
        let next = self.result.and_then("move_order_list", |entity| entity.eval_move_order_list());
        crate::MoveOrderListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_private_customer_profile_list(self) -> crate::PrivateCustomerProfileListExpression<'a> {
        let next = self.result.and_then("private_customer_profile_list", |entity| entity.eval_private_customer_profile_list());
        crate::PrivateCustomerProfileListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_corporate_customer_profile_list(self) -> crate::CorporateCustomerProfileListExpression<'a> {
        let next = self.result.and_then("corporate_customer_profile_list", |entity| entity.eval_corporate_customer_profile_list());
        crate::CorporateCustomerProfileListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_contact_list(self) -> crate::CustomerContactListExpression<'a> {
        let next = self.result.and_then("customer_contact_list", |entity| entity.eval_customer_contact_list());
        crate::CustomerContactListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_billing_profile_list(self) -> crate::BillingProfileListExpression<'a> {
        let next = self.result.and_then("billing_profile_list", |entity| entity.eval_billing_profile_list());
        crate::BillingProfileListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_history_list(self) -> crate::CustomerHistoryListExpression<'a> {
        let next = self.result.and_then("customer_history_list", |entity| entity.eval_customer_history_list());
        crate::CustomerHistoryListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_preference_list(self) -> crate::CustomerPreferenceListExpression<'a> {
        let next = self.result.and_then("customer_preference_list", |entity| entity.eval_customer_preference_list());
        crate::CustomerPreferenceListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_consent_list(self) -> crate::CustomerConsentListExpression<'a> {
        let next = self.result.and_then("customer_consent_list", |entity| entity.eval_customer_consent_list());
        crate::CustomerConsentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_customer_feedback_list(self) -> crate::CustomerFeedbackListExpression<'a> {
        let next = self.result.and_then("customer_feedback_list", |entity| entity.eval_customer_feedback_list());
        crate::CustomerFeedbackListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_loyalty_tier_list(self) -> crate::LoyaltyTierListExpression<'a> {
        let next = self.result.and_then("loyalty_tier_list", |entity| entity.eval_loyalty_tier_list());
        crate::LoyaltyTierListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_referral_code_list(self) -> crate::ReferralCodeListExpression<'a> {
        let next = self.result.and_then("referral_code_list", |entity| entity.eval_referral_code_list());
        crate::ReferralCodeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_communication_log_list(self) -> crate::CommunicationLogListExpression<'a> {
        let next = self.result.and_then("communication_log_list", |entity| entity.eval_communication_log_list());
        crate::CommunicationLogListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_service_rating_list(self) -> crate::ServiceRatingListExpression<'a> {
        let next = self.result.and_then("service_rating_list", |entity| entity.eval_service_rating_list());
        crate::ServiceRatingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_account_status_list(self) -> crate::AccountStatusListExpression<'a> {
        let next = self.result.and_then("account_status_list", |entity| entity.eval_account_status_list());
        crate::AccountStatusListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_contact_method_list(self) -> crate::ContactMethodListExpression<'a> {
        let next = self.result.and_then("contact_method_list", |entity| entity.eval_contact_method_list());
        crate::ContactMethodListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct CustomerListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Customer>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CustomerListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Customer>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Customer>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Customer>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Customer> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::CustomerExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CustomerExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::CustomerExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CustomerExpression::new(next, self.root_desc.clone())
    }
}