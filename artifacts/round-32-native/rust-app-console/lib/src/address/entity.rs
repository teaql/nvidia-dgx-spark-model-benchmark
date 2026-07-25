// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/address
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Address", table = "address_data", data_service = "sqlite")]
pub struct Address {
#[teaql(id)]
    id: u64,

// @source module_0.xml:15
    street_address: String,

// @source module_0.xml:15
    unit: String,

// @source module_0.xml:15
    city: String,

// @source module_0.xml:15
    state_province: String,

// @source module_0.xml:15
    postal_code: String,

// @source module_0.xml:15
    country: String,

// @source module_0.xml:15
    latitude: String,

// @source module_0.xml:15
    longitude: String,
#[teaql(version)]
    version: i64,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Address {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            street_address: String::new(),
            unit: String::new(),
            city: String::new(),
            state_province: String::new(),
            postal_code: String::new(),
            country: String::new(),
            latitude: String::new(),
            longitude: String::new(),
            version: 0_i64,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Address", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
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

    pub fn street_address(&self) -> String {
        self.changed_street_address().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.street_address.clone())
    }

    pub fn update_street_address(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.street_address = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.street_address.clone());
        self.root.set(self.entity_key(), "street_address", value);
        self
    }

    pub fn changed_street_address(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "street_address")
    }

    pub fn eval_street_address(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("street_address") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "street_address".to_string(), attempted_path: "street_address".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.street_address())
                }}

    pub fn unit(&self) -> String {
        self.changed_unit().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.unit.clone())
    }

    pub fn update_unit(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.unit = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.unit.clone());
        self.root.set(self.entity_key(), "unit", value);
        self
    }

    pub fn changed_unit(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "unit")
    }

    pub fn eval_unit(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("unit") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "unit".to_string(), attempted_path: "unit".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.unit())
                }}

    pub fn city(&self) -> String {
        self.changed_city().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.city.clone())
    }

    pub fn update_city(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.city = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.city.clone());
        self.root.set(self.entity_key(), "city", value);
        self
    }

    pub fn changed_city(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "city")
    }

    pub fn eval_city(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("city") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "city".to_string(), attempted_path: "city".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.city())
                }}

    pub fn state_province(&self) -> String {
        self.changed_state_province().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.state_province.clone())
    }

    pub fn update_state_province(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.state_province = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.state_province.clone());
        self.root.set(self.entity_key(), "state_province", value);
        self
    }

    pub fn changed_state_province(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "state_province")
    }

    pub fn eval_state_province(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("state_province") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "state_province".to_string(), attempted_path: "state_province".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.state_province())
                }}

    pub fn postal_code(&self) -> String {
        self.changed_postal_code().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.postal_code.clone())
    }

    pub fn update_postal_code(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.postal_code = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.postal_code.clone());
        self.root.set(self.entity_key(), "postal_code", value);
        self
    }

    pub fn changed_postal_code(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "postal_code")
    }

    pub fn eval_postal_code(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("postal_code") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "postal_code".to_string(), attempted_path: "postal_code".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.postal_code())
                }}

    pub fn country(&self) -> String {
        self.changed_country().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.country.clone())
    }

    pub fn update_country(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.country = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.country.clone());
        self.root.set(self.entity_key(), "country", value);
        self
    }

    pub fn changed_country(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "country")
    }

    pub fn eval_country(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("country") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "country".to_string(), attempted_path: "country".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.country())
                }}

    pub fn latitude(&self) -> String {
        self.changed_latitude().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.latitude.clone())
    }

    pub fn update_latitude(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.latitude = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.latitude.clone());
        self.root.set(self.entity_key(), "latitude", value);
        self
    }

    pub fn changed_latitude(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "latitude")
    }

    pub fn eval_latitude(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("latitude") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "latitude".to_string(), attempted_path: "latitude".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.latitude())
                }}

    pub fn longitude(&self) -> String {
        self.changed_longitude().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.longitude.clone())
    }

    pub fn update_longitude(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.longitude = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.longitude.clone());
        self.root.set(self.entity_key(), "longitude", value);
        self
    }

    pub fn changed_longitude(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "longitude")
    }

    pub fn eval_longitude(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("longitude") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "longitude".to_string(), attempted_path: "longitude".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.longitude())
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::AddressRepository<'a>>>
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
            .address_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Address"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

