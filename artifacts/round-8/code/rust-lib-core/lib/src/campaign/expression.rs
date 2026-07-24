#[derive(Clone)]
pub struct CampaignExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::Campaign>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CampaignExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::Campaign>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::Campaign> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::Campaign> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::Campaign {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_campaign_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("campaign_id", |entity| entity.eval_campaign_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_name(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("name", |entity| entity.eval_name());
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
    pub fn get_discount_code_list(self) -> crate::DiscountCodeListExpression<'a> {
        let next = self.result.and_then("discount_code_list", |entity| entity.eval_discount_code_list());
        crate::DiscountCodeListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_lead_list(self) -> crate::LeadListExpression<'a> {
        let next = self.result.and_then("lead_list", |entity| entity.eval_lead_list());
        crate::LeadListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_conversion_metric_list(self) -> crate::ConversionMetricListExpression<'a> {
        let next = self.result.and_then("conversion_metric_list", |entity| entity.eval_conversion_metric_list());
        crate::ConversionMetricListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_audience_segment_list(self) -> crate::AudienceSegmentListExpression<'a> {
        let next = self.result.and_then("audience_segment_list", |entity| entity.eval_audience_segment_list());
        crate::AudienceSegmentListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_promotional_offer_list(self) -> crate::PromotionalOfferListExpression<'a> {
        let next = self.result.and_then("promotional_offer_list", |entity| entity.eval_promotional_offer_list());
        crate::PromotionalOfferListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_attribution_model_list(self) -> crate::AttributionModelListExpression<'a> {
        let next = self.result.and_then("attribution_model_list", |entity| entity.eval_attribution_model_list());
        crate::AttributionModelListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_campaign_budget_list(self) -> crate::CampaignBudgetListExpression<'a> {
        let next = self.result.and_then("campaign_budget_list", |entity| entity.eval_campaign_budget_list());
        crate::CampaignBudgetListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_conversion_report_list(self) -> crate::ConversionReportListExpression<'a> {
        let next = self.result.and_then("conversion_report_list", |entity| entity.eval_conversion_report_list());
        crate::ConversionReportListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct CampaignListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Campaign>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> CampaignListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::Campaign>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::Campaign>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::Campaign>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::Campaign> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::CampaignExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CampaignExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::CampaignExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::CampaignExpression::new(next, self.root_desc.clone())
    }
}