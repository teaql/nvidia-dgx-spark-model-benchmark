#[derive(Clone)]
pub struct EquipmentSerialExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a crate::EquipmentSerial>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EquipmentSerialExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a crate::EquipmentSerial>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a crate::EquipmentSerial> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a crate::EquipmentSerial> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a crate::EquipmentSerial {
        self.resolve().expect("Relation was legitimately null in database!")
    }

    pub fn get_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("id", |entity| entity.eval_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_serial_id(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("serial_id", |entity| entity.eval_serial_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_code(self) -> crate::ValueExpression<'a, String> {
        let next = self.result.and_then("code", |entity| entity.eval_code());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn get_version(self) -> crate::ValueExpression<'a, i64> {
        let next = self.result.and_then("version", |entity| entity.eval_version());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_equipment_ref_id(self) -> crate::ValueExpression<'a, u64> {
        let next = self.result.and_then("equipment_ref_id", |entity| entity.eval_equipment_ref_id());
        crate::ValueExpression::new(next, self.root_desc.clone())
    }
    pub fn get_equipment_ref(self) -> crate::EquipmentExpression<'a> {
        let next = self.result.and_then("equipment_ref", |entity| entity.eval_equipment_ref());
        crate::EquipmentExpression::new(next, self.root_desc.clone())
    }
}

#[derive(Clone)]
pub struct EquipmentSerialListExpression<'a> {
    result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::EquipmentSerial>>,
    root_desc: std::sync::Arc<String>,
}

impl<'a> EquipmentSerialListExpression<'a> {
    pub fn new(result: teaql_core::eval::EvalResult<&'a teaql_core::SmartList<crate::EquipmentSerial>>, root_desc: std::sync::Arc<String>) -> Self {
        Self { result, root_desc }
    }

    fn resolve(&self) -> Option<&'a teaql_core::SmartList<crate::EquipmentSerial>> {
        match &self.result {
            teaql_core::eval::EvalResult::Value(v) => Some(*v),
            teaql_core::eval::EvalResult::Null => None,
            teaql_core::eval::EvalResult::NotLoaded { failed_node, attempted_path } => {
                crate::trigger_logic_bug_panic(&self.root_desc, &failed_node, &attempted_path)
            }
        }
    }

    pub fn eval(&self) -> Option<&'a teaql_core::SmartList<crate::EquipmentSerial>> {
        self.resolve()
    }

    pub fn unwrap(&self) -> &'a teaql_core::SmartList<crate::EquipmentSerial> {
        self.resolve().expect("List relation was legitimately null in database!")
    }

    pub fn size(&self) -> crate::ValueExpression<'a, usize> {
        let next = self.result.clone().and_then("size", |list| teaql_core::eval::EvalResult::Value(list.len()));
        crate::ValueExpression::new(next, self.root_desc.clone())
    }

    pub fn first(&self) -> crate::EquipmentSerialExpression<'a> {
        let next = self.result.clone().and_then("first", |list| {
            if let Some(item) = list.first() {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EquipmentSerialExpression::new(next, self.root_desc.clone())
    }

    pub fn get(&self, index: usize) -> crate::EquipmentSerialExpression<'a> {
        let next = self.result.clone().and_then("get", |list| {
            if let Some(item) = list.get(index) {
                teaql_core::eval::EvalResult::Value(item)
            } else {
                teaql_core::eval::EvalResult::Null
            }
        });
        crate::EquipmentSerialExpression::new(next, self.root_desc.clone())
    }
}