use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveValue, Set};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tc_user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
    pub state: Option<String>,
    #[sea_orm(column_name = "createdAt")]
    pub created_at: DateTimeWithTimeZone,
    #[sea_orm(column_name = "updatedAt")]
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now().fixed_offset();

        if insert {
            if matches!(self.role, ActiveValue::NotSet) {
                self.role = Set(Some("user".to_owned()));
            }

            if matches!(self.state, ActiveValue::NotSet) {
                self.state = Set(Some("active".to_owned()));
            }

            if matches!(self.created_at, ActiveValue::NotSet) {
                self.created_at = Set(now);
            }
        }

        self.updated_at = Set(now);

        Ok(self)
    }
}
