#[derive(Clone)]
pub struct PromotionalOfferExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::PromotionalOffer>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PromotionalOfferExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::PromotionalOffer>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::PromotionalOffer> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::PromotionalOffer> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::PromotionalOffer {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_offer_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("offer_id", |entity| entity.eval_offer_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_description(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("description", |entity| entity.eval_description());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_campaign_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("campaign_ref_id", |entity| entity.eval_campaign_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_campaign_ref(self) -> crate::CampaignExpression<'a> {
        let next = self.result.and_then("campaign_ref", |entity| entity.eval_campaign_ref());
        crate::CampaignExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PromotionalOfferListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PromotionalOffer>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PromotionalOfferListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PromotionalOffer>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::PromotionalOffer>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::PromotionalOffer>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::PromotionalOffer> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PromotionalOfferExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PromotionalOfferExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PromotionalOfferExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PromotionalOfferExpression::new(next, self.root_desc.clone())
    }
}