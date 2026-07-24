#[derive(Clone)]
pub struct PayrollCalculationExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::PayrollCalculation>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PayrollCalculationExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::PayrollCalculation>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::PayrollCalculation> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::PayrollCalculation> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::PayrollCalculation {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_calc_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("calc_id", |entity| entity.eval_calc_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_gross_pay(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("gross_pay", |entity| entity.eval_gross_pay());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_payroll_period_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("payroll_period_ref_id", |entity| entity.eval_payroll_period_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_payroll_period_ref(self) -> crate::PayrollPeriodExpression<'a> {
        let next = self.result.and_then("payroll_period_ref", |entity| entity.eval_payroll_period_ref());
        crate::PayrollPeriodExpression::new(next, self.root_desc.clone())
    }
    pub fn get_payslip_list(self) -> crate::PayslipListExpression<'a> {
        let next = self.result.and_then("payslip_list", |entity| entity.eval_payslip_list());
        crate::PayslipListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_deduction_list(self) -> crate::DeductionListExpression<'a> {
        let next = self.result.and_then("deduction_list", |entity| entity.eval_deduction_list());
        crate::DeductionListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_tax_withholding_list(self) -> crate::TaxWithholdingListExpression<'a> {
        let next = self.result.and_then("tax_withholding_list", |entity| entity.eval_tax_withholding_list());
        crate::TaxWithholdingListExpression::new(next, self.root_desc.clone())
    }

    pub fn get_payroll_adjustment_list(self) -> crate::PayrollAdjustmentListExpression<'a> {
        let next = self.result.and_then("payroll_adjustment_list", |entity| entity.eval_payroll_adjustment_list());
        crate::PayrollAdjustmentListExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct PayrollCalculationListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PayrollCalculation>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> PayrollCalculationListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::PayrollCalculation>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::PayrollCalculation>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::PayrollCalculation>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::PayrollCalculation> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::PayrollCalculationExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PayrollCalculationExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::PayrollCalculationExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::PayrollCalculationExpression::new(next, self.root_desc.clone())
    }
}