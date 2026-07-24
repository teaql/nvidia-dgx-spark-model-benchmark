#[derive(Clone)]
pub struct AvailabilityCalendarExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::AvailabilityCalendar>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> AvailabilityCalendarExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::AvailabilityCalendar>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::AvailabilityCalendar> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::AvailabilityCalendar> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::AvailabilityCalendar {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_calendar_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("calendar_id", |entity| entity.eval_calendar_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_month(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("month", |entity| entity.eval_month());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_service_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("service_ref_id", |entity| entity.eval_service_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_service_ref(self) -> crate::ServiceExpression<'a> {
        let next = self.result.and_then("service_ref", |entity| entity.eval_service_ref());
        crate::ServiceExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct AvailabilityCalendarListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::AvailabilityCalendar>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> AvailabilityCalendarListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::AvailabilityCalendar>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::AvailabilityCalendar>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::AvailabilityCalendar>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::AvailabilityCalendar> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::AvailabilityCalendarExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::AvailabilityCalendarExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::AvailabilityCalendarExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::AvailabilityCalendarExpression::new(next, self.root_desc.clone())
    }
}