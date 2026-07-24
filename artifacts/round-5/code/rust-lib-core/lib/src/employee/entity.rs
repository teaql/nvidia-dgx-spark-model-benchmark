// ⛔ AI agents: DO NOT read this file for API discovery. Instead run: cargo teaql --input modeling/MODEL.xml rust-assist-query/employee
use std::collections::BTreeMap;

use teaql_macros::TeaqlEntity;

/// [TEAQL AI WARNING]
/// TeaQL was explicitly designed to PREVENT AI hallucinations and random guessing.
/// DO NOT GUESS METHOD NAMES!
/// The methods listed below are the ONLY valid ways to interact with this entity.
/// If you encounter compilation errors (e.g., method not found), DO NOT guess another method name.
/// Read the method signatures in this file before proceeding.
#[derive(Clone, Debug, PartialEq, TeaqlEntity)]
#[teaql(entity = "Employee", table = "employee_data", data_service = "sqlite")]
pub struct Employee {
#[teaql(id)]
    id: u64,

// @source prepared.xml:5
    employee_number: String,

// @source prepared.xml:5
    name: String,

// @source prepared.xml:5
    birth_date: chrono::NaiveDate,

// @source prepared.xml:5
    mobile_phone: String,

// @source prepared.xml:5
    email: String,

// @source prepared.xml:5
    job_title: String,

// @source prepared.xml:5
    hiring_date: chrono::NaiveDate,

// @source prepared.xml:5
    create_time: chrono::DateTime<chrono::Utc>,

// @source prepared.xml:5
    update_time: chrono::DateTime<chrono::Utc>,
#[teaql(version)]
    version: i64,
// @source prepared.xml:5
#[teaql(column = "gender")]
    gender_id: u64,

// @source prepared.xml:5
#[teaql(column = "merchant")]
    merchant_id: u64,
// @source prepared.xml:5
#[teaql(relation(target = "GenderType", local_key = "gender_id", foreign_key = "id"))]
    gender: Option<crate::GenderType>,

// @source prepared.xml:5
#[teaql(relation(target = "Merchant", local_key = "merchant_id", foreign_key = "id"))]
    merchant: Option<crate::Merchant>,
    #[teaql(dynamic)]
    dynamic: BTreeMap<String, teaql_core::Value>,
    #[teaql(skip)]
    root: teaql_runtime::EntityRoot,
    #[teaql(skip)]
    pub __load_state: teaql_core::eval::LoadState,
}

impl Employee {
    pub fn with_id(id: u64) -> teaql_core::Value {
        teaql_core::Value::U64(id)
    }

    pub(crate) fn runtime_new(root: teaql_runtime::EntityRoot) -> Self {
        Self {
            id: 0_u64,
            employee_number: String::new(),
            name: String::new(),
            birth_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            mobile_phone: String::new(),
            email: String::new(),
            job_title: String::new(),
            hiring_date: chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            create_time: chrono::Utc::now(),
            update_time: chrono::Utc::now(),
            version: 0_i64,
            gender_id: 0_u64,
            merchant_id: 0_u64,
            gender: None,
            merchant: None,
            dynamic: BTreeMap::new(),
            root,
            __load_state: teaql_core::eval::LoadState::FullyLoaded,
        }
    }

    pub fn entity_key(&self) -> teaql_runtime::EntityKey {
        teaql_runtime::EntityKey::new("Employee", self.id)
    }

    pub fn attach_root_recursive(&mut self, root: teaql_runtime::EntityRoot) {
        self.root = root.clone();
        if let Some(entity) = &mut self.gender {
            entity.attach_root_recursive(root.clone());
        }
        if let Some(entity) = &mut self.merchant {
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

    pub fn employee_number(&self) -> String {
        self.changed_employee_number().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.employee_number.clone())
    }

    pub fn update_employee_number(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.employee_number = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.employee_number.clone());
        self.root.set(self.entity_key(), "employee_number", value);
        self
    }

    pub fn changed_employee_number(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "employee_number")
    }

    pub fn eval_employee_number(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("employee_number") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "employee_number".to_string(), attempted_path: "employee_number".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.employee_number())
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

    pub fn birth_date(&self) -> chrono::NaiveDate {
        self.changed_birth_date().and_then(|value| value.try_date()).unwrap_or(self.birth_date)
    }

    pub fn update_birth_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.birth_date = value.try_date().unwrap_or(self.birth_date.clone());
        self.root.set(self.entity_key(), "birth_date", value);
        self
    }

    pub fn changed_birth_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "birth_date")
    }

    pub fn eval_birth_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("birth_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "birth_date".to_string(), attempted_path: "birth_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.birth_date())
                }}

    pub fn mobile_phone(&self) -> String {
        self.changed_mobile_phone().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.mobile_phone.clone())
    }

    pub fn update_mobile_phone(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.mobile_phone = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.mobile_phone.clone());
        self.root.set(self.entity_key(), "mobile_phone", value);
        self
    }

    pub fn changed_mobile_phone(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "mobile_phone")
    }

    pub fn eval_mobile_phone(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("mobile_phone") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "mobile_phone".to_string(), attempted_path: "mobile_phone".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.mobile_phone())
                }}

    pub fn email(&self) -> String {
        self.changed_email().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.email.clone())
    }

    pub fn update_email(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.email = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.email.clone());
        self.root.set(self.entity_key(), "email", value);
        self
    }

    pub fn changed_email(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "email")
    }

    pub fn eval_email(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("email") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "email".to_string(), attempted_path: "email".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.email())
                }}

    pub fn job_title(&self) -> String {
        self.changed_job_title().and_then(|value| value.try_text().map(|value| value.to_owned())).unwrap_or_else(|| self.job_title.clone())
    }

    pub fn update_job_title(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.job_title = value.try_text().map(|value| value.trim().to_owned()).unwrap_or_else(|| self.job_title.clone());
        self.root.set(self.entity_key(), "job_title", value);
        self
    }

    pub fn changed_job_title(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "job_title")
    }

    pub fn eval_job_title(&self) -> teaql_core::eval::EvalResult<String> {
        if !self.is_loaded("job_title") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "job_title".to_string(), attempted_path: "job_title".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.job_title())
                }}

    pub fn hiring_date(&self) -> chrono::NaiveDate {
        self.changed_hiring_date().and_then(|value| value.try_date()).unwrap_or(self.hiring_date)
    }

    pub fn update_hiring_date(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.hiring_date = value.try_date().unwrap_or(self.hiring_date.clone());
        self.root.set(self.entity_key(), "hiring_date", value);
        self
    }

    pub fn changed_hiring_date(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "hiring_date")
    }

    pub fn eval_hiring_date(&self) -> teaql_core::eval::EvalResult<chrono::NaiveDate> {
        if !self.is_loaded("hiring_date") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "hiring_date".to_string(), attempted_path: "hiring_date".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.hiring_date())
                }}

    pub fn create_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_create_time().and_then(|value| value.try_timestamp()).unwrap_or(self.create_time)
    }

    pub fn update_create_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.create_time = value.try_timestamp().unwrap_or(self.create_time.clone());
        self.root.set(self.entity_key(), "create_time", value);
        self
    }

    pub fn changed_create_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "create_time")
    }

    pub fn eval_create_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("create_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "create_time".to_string(), attempted_path: "create_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.create_time())
                }}

    pub fn update_time(&self) -> chrono::DateTime<chrono::Utc> {
        self.changed_update_time().and_then(|value| value.try_timestamp()).unwrap_or(self.update_time)
    }

    pub fn update_update_time(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.update_time = value.try_timestamp().unwrap_or(self.update_time.clone());
        self.root.set(self.entity_key(), "update_time", value);
        self
    }

    pub fn changed_update_time(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "update_time")
    }

    pub fn eval_update_time(&self) -> teaql_core::eval::EvalResult<chrono::DateTime<chrono::Utc>> {
        if !self.is_loaded("update_time") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "update_time".to_string(), attempted_path: "update_time".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.update_time())
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
    pub fn gender_id(&self) -> u64 {
        self.changed_gender_id().and_then(|value| value.try_u64()).unwrap_or(self.gender_id)
    }

    pub(crate) fn update_gender_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.gender_id = value.try_u64().unwrap_or(self.gender_id.clone());
        self.root.set(self.entity_key(), "gender_id", value);
        self
    }

    pub fn changed_gender_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "gender_id")
    }

    pub fn eval_gender_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("gender_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "gender_id".to_string(), attempted_path: "gender_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.gender_id())
                }}

    pub fn merchant_id(&self) -> u64 {
        self.changed_merchant_id().and_then(|value| value.try_u64()).unwrap_or(self.merchant_id)
    }

    pub fn update_merchant_id(&mut self, value: impl Into<teaql_core::Value>) -> &mut Self {
        let value = value.into();
        self.merchant_id = value.try_u64().unwrap_or(self.merchant_id.clone());
        self.root.set(self.entity_key(), "merchant_id", value);
        self
    }

    pub fn changed_merchant_id(&self) -> Option<teaql_core::Value> {
        self.root.get(&self.entity_key(), "merchant_id")
    }

    pub fn eval_merchant_id(&self) -> teaql_core::eval::EvalResult<u64> {
        if !self.is_loaded("merchant_id") {
                    teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant_id".to_string(), attempted_path: "merchant_id".to_string() }
                } else {
                    teaql_core::eval::EvalResult::Value(self.merchant_id())
                }}
    pub fn update_gender_to_male(&mut self) -> &mut Self {
        self.update_gender_id(1001_u64)
    }

    pub fn gender_is_male(&self) -> bool {
        self.gender_id() == 1001_u64
    }
    pub fn update_gender_to_female(&mut self) -> &mut Self {
        self.update_gender_id(1002_u64)
    }

    pub fn gender_is_female(&self) -> bool {
        self.gender_id() == 1002_u64
    }
    pub fn gender(&self) -> Option<&crate::GenderType> {
        self.gender.as_ref()
    }

    pub fn eval_gender(&self) -> teaql_core::eval::EvalResult<&crate::GenderType> {
        if !self.is_loaded("gender") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "gender".to_string(), attempted_path: "gender".to_string() }
        } else {
            match &self.gender {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
        }
    }

    pub fn merchant(&self) -> Option<&crate::Merchant> {
        self.merchant.as_ref()
    }

    pub fn eval_merchant(&self) -> teaql_core::eval::EvalResult<&crate::Merchant> {
        if !self.is_loaded("merchant") {
            teaql_core::eval::EvalResult::NotLoaded { failed_node: "merchant".to_string(), attempted_path: "merchant".to_string() }
        } else {
            match &self.merchant {
                Some(v) => teaql_core::eval::EvalResult::Value(v),
                None => teaql_core::eval::EvalResult::Null,
            }
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
    ) -> Result<teaql_runtime::GraphNode, crate::TeaqlDataServiceError<C::EmployeeRepository<'a>>>
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
            .employee_repository()
            .map_err(|err| teaql_runtime::DataServiceError::Runtime(teaql_runtime::RuntimeError::Graph(err.to_string())))?;
        if has_ledger_change {
            crate::TeaqlEntityRepository::save_entity_ledger(&repository, root.clone()).await?;
            return Ok(teaql_runtime::GraphNode::new("Employee"));
        }
        crate::TeaqlEntityRepository::save_entity_graph(&repository, self.clone()).await
    }
}

