use teaql_runtime::{CheckObjectStatus, CheckResults, ObjectLocation, TypedChecker, UserContext};

pub trait ExtraOperationsLogistics2CheckerLogic: Send + Sync {
    fn check_and_fix_extra_operations_logistics_2(
        &self,
        _ctx: &UserContext,
        _entity: &mut crate::ExtraOperationsLogistics2,
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
pub struct NoopExtraOperationsLogistics2Checker;

impl ExtraOperationsLogistics2CheckerLogic for NoopExtraOperationsLogistics2Checker {}

#[derive(Clone, Debug)]
pub struct ExtraOperationsLogistics2Checker<L = NoopExtraOperationsLogistics2Checker> {
    logic: L,
}

impl Default for ExtraOperationsLogistics2Checker<NoopExtraOperationsLogistics2Checker> {
    fn default() -> Self {
        Self {
            logic: NoopExtraOperationsLogistics2Checker,
        }
    }
}

impl<L> ExtraOperationsLogistics2Checker<L>
where
    L: ExtraOperationsLogistics2CheckerLogic,
{
    pub fn new(logic: L) -> Self {
        Self { logic }
    }
}

impl<L> TypedChecker<crate::ExtraOperationsLogistics2> for ExtraOperationsLogistics2Checker<L>
where
    L: ExtraOperationsLogistics2CheckerLogic,
{
    fn check_and_fix_typed(
        &self,
        ctx: &UserContext,
        entity: &mut crate::ExtraOperationsLogistics2,
        status: CheckObjectStatus,
        location: &ObjectLocation,
        results: &mut CheckResults,
    ) {
        self.logic
            .check_and_fix_extra_operations_logistics_2(ctx, entity, status, location, results);
    }
}