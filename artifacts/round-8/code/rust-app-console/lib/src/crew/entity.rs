// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/crew
use std::collections::BTreeMap;

use teaql_core::SmartList;
use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Crew", table = "crew_data", data_service = "sqlite")]
pub struct Crew {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    crew_id: String,

// @source model.xml:2
    size: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "merchant_ref")]
    merchant_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Merchant", local_key = "merchant_ref_id", foreign_key = "id"))]
    merchant_ref: Option<crate::Merchant>,
#[teaql(relation(target = "DispatchAssignment", local_key = "id", foreign_key = "crew_ref_id", many))]
    dispatch_assignment_list: SmartList<crate::DispatchAssignment>,
#[teaql(relation(target = "VehicleAssignment", local_key = "id", foreign_key = "crew_ref_id", many))]
    vehicle_assignment_list: SmartList<crate::VehicleAssignment>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Crew {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            crew_id: String::new(),
            size: String::new(),
            version: 0_i64,
            merchant_ref_id: 0_u64,
            merchant_ref: None,
            dispatch_assignment_list: Default::default(),
            vehicle_assignment_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Crew", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.merchant_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.dispatch_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.vehicle_assignment_list {
            entity.attach_root_recursive(root.clone());
        }
    }

    pub fn is_loaded(&self, field_or_relation: &str) -> bool {
        self.__load_state.is_loaded(field_or_relation)
    }

    pub fn set_load_state(&mut self, state: teaql_core::eval::LoadState) {
        self.__load_state = state;
    }

    pub fn id(&self) -> u64 {
        self.changed_id().and_then(|value| value.try_u64()).unwrap_or(self.id)
    }

    pub fn update_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.id = value.try_u64().unwrap_or(self.id.clone());
        self.root.set(self.entity_key(), "id", value);
        self
    }

    pub fn changed_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "id")
    }

    pub fn eval_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "id".to_string(), attempted_path: "id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.id())
                }}

    pub fn crew_id(&self) -> String {
        self.changed_crew_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.crew_id.clone())
    }

    pub fn update_crew_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.crew_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.crew_id.clone());
        self.root.set(self.entity_key(), "crew_id", value);
        self
    }

    pub fn changed_crew_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "crew_id")
    }

    pub fn eval_crew_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("crew_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "crew_id".to_string(), attempted_path: "crew_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.crew_id())
                }}

    pub fn size(&self) -> String {
        self.changed_size().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.size.clone())
    }

    pub fn update_size(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.size = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.size.clone());
        self.root.set(self.entity_key(), "size", value);
        self
    }

    pub fn changed_size(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "size")
    }

    pub fn eval_size(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("size") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "size".to_string(), attempted_path: "size".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.size())
                }}

    pub fn version(&self) -> i64 {
        self.changed_version().and_then(|value| value.try_i64()).unwrap_or(self.version)
    }

    pub fn update_version(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.version = value.try_i64().unwrap_or(self.version.clone());
        self.root.set(self.entity_key(), "version", value);
        self
    }

    pub fn changed_version(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "version")
    }

    pub fn eval_version(&self) -> teaql_core::eval::EvalResult<i64> {
        if !self.is_loaded("version") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "version".to_string(), attempted_path: "version".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.version())
                }}
    pub fn merchant_ref_id(&self) -> u64 {
        self.changed_merchant_ref_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_ref_id)
    }

    pub fn update_merchant_ref_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_ref_id = value.try_u64().unwrap_or(self.merchant_ref_id.clone());
        self.root.set(self.entity_key(), "merchant_ref_id", value);
        self
    }

    pub fn changed_merchant_ref_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_ref_id")
    }

    pub fn eval_merchant_ref_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_ref_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref_id".to_string(), attempted_path: "merchant_ref_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_ref_id())
                }}
    pub fn merchant_ref(&self) -> Option<&crate::Merchant> {
        self.merchant_ref.as_ref()
    }

    pub fn eval_merchant_ref(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant_ref") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_ref".to_string(), attempted_path: "merchant_ref".to_string() }
        } else {
            match &self.merchant_ref {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }
    pub fn dispatch_assignment_list(&self) -> &SmartList<crate::DispatchAssignment> {
        &self.dispatch_assignment_list
    }

    pub fn dispatch_assignment_list_mut(&mut self) -> &mut SmartList<crate::DispatchAssignment> {
        &mut self.dispatch_assignment_list
    }

    pub fn eval_dispatch_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DispatchAssignment>> {
        if !self.is_loaded("dispatch_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "dispatch_assignment_list".to_string(), attempted_path: "dispatch_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.dispatch_assignment_list)
        }
    }

    pub fn vehicle_assignment_list(&self) -> &SmartList<crate::VehicleAssignment> {
        &self.vehicle_assignment_list
    }

    pub fn vehicle_assignment_list_mut(&mut self) -> &mut SmartList<crate::VehicleAssignment> {
        &mut self.vehicle_assignment_list
    }

    pub fn eval_vehicle_assignment_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::VehicleAssignment>> {
        if !self.is_loaded("vehicle_assignment_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "vehicle_assignment_list".to_string(), attempted_path: "vehicle_assignment_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.vehicle_assignment_list)
        }
    }

    pub fn mark_as_delete(&mut self) -> &mut Self {
        self.root.mark_as_delete(self.entity_key());
        self
    }

    pub fn set_comment(&mut self, comment: impl Into<String>) -> &mut Self {
        self.root.set_comment(comment);
        self
    }

    pub(crate) async fn save<'a, C>(
        &self,
        ctx: &'a C,
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::CrewRepository<'a>>>
    where
        C: crate::TeaqlRepositoryProvider + ?Sized,
    {
        let root = ctx.user_context().entity_root();
        let key = self.entity_key();
        let has_ledger_change = (self.id != 0)
            && (root.current_change_set().changes().contains_key(&key)
                || root.is_marked_as_delete(&key)
                || root.is_new(&key));
        let repository = ctx
            .crew_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Crew"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

