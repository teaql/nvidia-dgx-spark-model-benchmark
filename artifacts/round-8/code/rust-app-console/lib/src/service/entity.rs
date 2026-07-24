// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/service
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
#[teaql(entity = "Service", table = "service_data", data_service = "sqlite")]
pub struct Service {
#[teaql(id)]
    id: u64,

// @source model.xml:2
    service_id: String,

// @source model.xml:2
    name: String,
#[teaql(version)]
    version: i64,
// @source model.xml:2
#[teaql(column = "merchant_ref")]
    merchant_ref_id: u64,
// @source model.xml:2
#[teaql(relation(target = "Merchant", local_key = "merchant_ref_id", foreign_key = "id"))]
    merchant_ref: Option<crate::Merchant>,
#[teaql(relation(target = "MovingService", local_key = "id", foreign_key = "service_ref_id", many))]
    moving_service_list: SmartList<crate::MovingService>,
#[teaql(relation(target = "CleaningService", local_key = "id", foreign_key = "service_ref_id", many))]
    cleaning_service_list: SmartList<crate::CleaningService>,
#[teaql(relation(target = "ServiceConfiguration", local_key = "id", foreign_key = "service_ref_id", many))]
    service_configuration_list: SmartList<crate::ServiceConfiguration>,
#[teaql(relation(target = "DisposalService", local_key = "id", foreign_key = "service_ref_id", many))]
    disposal_service_list: SmartList<crate::DisposalService>,
#[teaql(relation(target = "AvailabilityCalendar", local_key = "id", foreign_key = "service_ref_id", many))]
    availability_calendar_list: SmartList<crate::AvailabilityCalendar>,
#[teaql(relation(target = "ServiceLevelAgreement", local_key = "id", foreign_key = "service_ref_id", many))]
    service_level_agreement_list: SmartList<crate::ServiceLevelAgreement>,
#[teaql(relation(target = "AddOnService", local_key = "id", foreign_key = "service_ref_id", many))]
    add_on_service_list: SmartList<crate::AddOnService>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Service {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            service_id: String::new(),
            name: String::new(),
            version: 0_i64,
            merchant_ref_id: 0_u64,
            merchant_ref: None,
            moving_service_list: Default::default(),
            cleaning_service_list: Default::default(),
            service_configuration_list: Default::default(),
            disposal_service_list: Default::default(),
            availability_calendar_list: Default::default(),
            service_level_agreement_list: Default::default(),
            add_on_service_list: Default::default(),
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Service", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.merchant_ref {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.moving_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.cleaning_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_configuration_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.disposal_service_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.availability_calendar_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.service_level_agreement_list {
            entity.attach_root_recursive(root.clone());
        }
        for entity in &mut self.add_on_service_list {
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

    pub fn service_id(&self) -> String {
        self.changed_service_id().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.service_id.clone())
    }

    pub fn update_service_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.service_id = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.service_id.clone());
        self.root.set(self.entity_key(), "service_id", value);
        self
    }

    pub fn changed_service_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "service_id")
    }

    pub fn eval_service_id(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("service_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_id".to_string(), attempted_path: "service_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.service_id())
                }}

    pub fn name(&self) -> String {
        self.changed_name().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.name.clone())
    }

    pub fn update_name(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.name = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.name.clone());
        self.root.set(self.entity_key(), "name", value);
        self
    }

    pub fn changed_name(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "name")
    }

    pub fn eval_name(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("name") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "name".to_string(), attempted_path: "name".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.name())
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
    pub fn moving_service_list(&self) -> &SmartList<crate::MovingService> {
        &self.moving_service_list
    }

    pub fn moving_service_list_mut(&mut self) -> &mut SmartList<crate::MovingService> {
        &mut self.moving_service_list
    }

    pub fn eval_moving_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::MovingService>> {
        if !self.is_loaded("moving_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "moving_service_list".to_string(), attempted_path: "moving_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.moving_service_list)
        }
    }

    pub fn cleaning_service_list(&self) -> &SmartList<crate::CleaningService> {
        &self.cleaning_service_list
    }

    pub fn cleaning_service_list_mut(&mut self) -> &mut SmartList<crate::CleaningService> {
        &mut self.cleaning_service_list
    }

    pub fn eval_cleaning_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::CleaningService>> {
        if !self.is_loaded("cleaning_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "cleaning_service_list".to_string(), attempted_path: "cleaning_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.cleaning_service_list)
        }
    }

    pub fn service_configuration_list(&self) -> &SmartList<crate::ServiceConfiguration> {
        &self.service_configuration_list
    }

    pub fn service_configuration_list_mut(&mut self) -> &mut SmartList<crate::ServiceConfiguration> {
        &mut self.service_configuration_list
    }

    pub fn eval_service_configuration_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServiceConfiguration>> {
        if !self.is_loaded("service_configuration_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_configuration_list".to_string(), attempted_path: "service_configuration_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.service_configuration_list)
        }
    }

    pub fn disposal_service_list(&self) -> &SmartList<crate::DisposalService> {
        &self.disposal_service_list
    }

    pub fn disposal_service_list_mut(&mut self) -> &mut SmartList<crate::DisposalService> {
        &mut self.disposal_service_list
    }

    pub fn eval_disposal_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::DisposalService>> {
        if !self.is_loaded("disposal_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "disposal_service_list".to_string(), attempted_path: "disposal_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.disposal_service_list)
        }
    }

    pub fn availability_calendar_list(&self) -> &SmartList<crate::AvailabilityCalendar> {
        &self.availability_calendar_list
    }

    pub fn availability_calendar_list_mut(&mut self) -> &mut SmartList<crate::AvailabilityCalendar> {
        &mut self.availability_calendar_list
    }

    pub fn eval_availability_calendar_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AvailabilityCalendar>> {
        if !self.is_loaded("availability_calendar_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "availability_calendar_list".to_string(), attempted_path: "availability_calendar_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.availability_calendar_list)
        }
    }

    pub fn service_level_agreement_list(&self) -> &SmartList<crate::ServiceLevelAgreement> {
        &self.service_level_agreement_list
    }

    pub fn service_level_agreement_list_mut(&mut self) -> &mut SmartList<crate::ServiceLevelAgreement> {
        &mut self.service_level_agreement_list
    }

    pub fn eval_service_level_agreement_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::ServiceLevelAgreement>> {
        if !self.is_loaded("service_level_agreement_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "service_level_agreement_list".to_string(), attempted_path: "service_level_agreement_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.service_level_agreement_list)
        }
    }

    pub fn add_on_service_list(&self) -> &SmartList<crate::AddOnService> {
        &self.add_on_service_list
    }

    pub fn add_on_service_list_mut(&mut self) -> &mut SmartList<crate::AddOnService> {
        &mut self.add_on_service_list
    }

    pub fn eval_add_on_service_list(&self) -> teaql_core::eval::EvalResult<&SmartList<crate::AddOnService>> {
        if !self.is_loaded("add_on_service_list") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "add_on_service_list".to_string(), attempted_path: "add_on_service_list".to_string() }
        } else {
            teaql_core::eval::EvalResult::Value(&self.add_on_service_list)
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::ServiceRepository<'a>>>
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
            .service_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Service"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

