use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait ExtraCustomerManagement3CheckerLogic: Send + Sync {
    fn check_and_fix_extra_customer_management_3(
        &self,
        _ctx: &UserContext,
        _entity: &mut crate::ExtraCustomerManagement3,
        _status: CheckObjectStatus,
        _location: &ObjectLocation,
        _results: &mut CheckResults,
    ) {
    }

    fn required(
        &self,
        value: bool,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if !value {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn required_option<V>(
        &self,
        value: Option<&V>,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.is_none() {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn required_text(
        &self,
        value: &str,
        field: &str,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.trim().is_empty() {
            results.push(teaql_runtime::CheckResult::required(location.clone().member(field)));
        }
    }

    fn min_string_length(
        &self,
        value: &str,
        field: &str,
        min_len: usize,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.chars().count() < min_len {
            results.push(teaql_runtime::CheckResult::min_str(
                location.clone().member(field),
                min_len as u64,
                value.to_owned(),
            ));
        }
    }

    fn max_string_length(
        &self,
        value: &str,
        field: &str,
        max_len: usize,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        if value.chars().count() > max_len {
            results.push(teaql_runtime::CheckResult::max_str(
                location.clone().member(field),
                max_len as u64,
                value.to_owned(),
            ));
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NoopExtraCustomerManagement3Checker;

impl ExtraCustomerManagement3CheckerLogic for NoopExtraCustomerManagement3Checker {}

#[derive(Clone, Debug)]
pub struct ExtraCustomerManagement3Checker<L = NoopExtraCustomerManagement3Checker> {
    logic: L,
}

impl Default for ExtraCustomerManagement3Checker<NoopExtraCustomerManagement3Checker> {
    fn default() -> Self {
        Self {
            logic: NoopExtraCustomerManagement3Checker,
        }
    }
}

impl<L> ExtraCustomerManagement3Checker<L>
where
    L: ExtraCustomerManagement3CheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::ExtraCustomerManagement3> for ExtraCustomerManagement3Checker<L>
where
    L: ExtraCustomerManagement3CheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        ctx: &UserContext,
        entity: &mut crate::ExtraCustomerManagement3,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        self.logic
            .check_and_fix_extra_customer_management_3(ctx, entity, status, location, results);
    }
}