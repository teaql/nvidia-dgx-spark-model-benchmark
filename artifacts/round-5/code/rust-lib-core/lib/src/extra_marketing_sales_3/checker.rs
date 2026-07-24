use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait ExtraMarketingSales3CheckerLogic: Send + Sync {
    fn check_and_fix_extra_marketing_sales_3(
        &self,
        _ctx: &UserContext,
        _entity: &mut crate::ExtraMarketingSales3,
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
pub struct NoopExtraMarketingSales3Checker;

impl ExtraMarketingSales3CheckerLogic for NoopExtraMarketingSales3Checker {}

#[derive(Clone, Debug)]
pub struct ExtraMarketingSales3Checker<L = NoopExtraMarketingSales3Checker> {
    logic: L,
}

impl Default for ExtraMarketingSales3Checker<NoopExtraMarketingSales3Checker> {
    fn default() -> Self {
        Self {
            logic: NoopExtraMarketingSales3Checker,
        }
    }
}

impl<L> ExtraMarketingSales3Checker<L>
where
    L: ExtraMarketingSales3CheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::ExtraMarketingSales3> for ExtraMarketingSales3Checker<L>
where
    L: ExtraMarketingSales3CheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        ctx: &UserContext,
        entity: &mut crate::ExtraMarketingSales3,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        self.logic
            .check_and_fix_extra_marketing_sales_3(ctx, entity, status, location, results);
    }
}