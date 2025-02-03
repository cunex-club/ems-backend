use crate::models::{candidate::db::DbCandidate, question::Question};
use async_trait::async_trait;
use mysk_lib::{
    common::{requests::FetchLevel, string::MultiLangString},
    models::traits::{FetchLevelVariant, TopLevelGetById},
    permissions::Authorizer,
    prelude::*,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultCandidate {
    pub id: Uuid,
    pub question: Question,
    pub choice_label: MultiLangString,
    pub title: String,
    pub info_line_1: String,
    pub info_line_2: String,
    pub info_line_3: String,
    pub info_line_4: String,
    pub info_line_5: String,
    pub body_title_1: String,
    pub body_1: String,
    pub body_title_2: String,
    pub body_2: String,
    pub image_file: String,
}

#[async_trait]
impl FetchLevelVariant<DbCandidate> for DefaultCandidate {
    async fn from_table(
        pool: &PgPool,
        table: DbCandidate,
        descendant_fetch_level: Option<&FetchLevel>,
        authorizer: &dyn Authorizer,
    ) -> Result<Self> {
        Ok(Self {
            id: table.id,
            choice_label: MultiLangString {
                th: table.choice_label_th,
                en: Some(table.choice_label_en),
            },
            question: Question::get_by_id(
                pool,
                table.question_id,
                descendant_fetch_level,
                Some(&FetchLevel::IdOnly),
                authorizer,
            )
            .await?,
            title: table.title,
            info_line_1: table.info_line_1,
            info_line_2: table.info_line_2,
            info_line_3: table.info_line_3,
            info_line_4: table.info_line_4,
            info_line_5: table.info_line_5,
            body_title_1: table.body_title_1,
            body_1: table.body_1,
            body_title_2: table.body_title_2,
            body_2: table.body_2,
            image_file: table.image_file,
        })
    }
}
