#[derive(Clone)]
pub struct DispatchAssignmentExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::DispatchAssignment>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DispatchAssignmentExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::DispatchAssignment>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::DispatchAssignment> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::DispatchAssignment> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::DispatchAssignment {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_assignment_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("assignment_id", |entity| entity.eval_assignment_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_priority(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("priority", |entity| entity.eval_priority());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_crew_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("crew_ref_id", |entity| entity.eval_crew_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("move_order_ref_id", |entity| entity.eval_move_order_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_crew_ref(self) -> crate::CrewExpression<'a> {
        let next = self.result.and_then("crew_ref", |entity| entity.eval_crew_ref());
        crate::CrewExpression::new(next, self.root_desc.clone())
    }

    pub fn get_move_order_ref(self) -> crate::MoveOrderExpression<'a> {
        let next = self.result.and_then("move_order_ref", |entity| entity.eval_move_order_ref());
        crate::MoveOrderExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct DispatchAssignmentListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DispatchAssignment>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> DispatchAssignmentListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::DispatchAssignment>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::DispatchAssignment>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::DispatchAssignment>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::DispatchAssignment> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::DispatchAssignmentExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DispatchAssignmentExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::DispatchAssignmentExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::DispatchAssignmentExpression::new(next, self.root_desc.clone())
    }
}