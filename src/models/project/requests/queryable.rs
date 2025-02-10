use mysk_lib::query::{QueryParam, Queryable, SqlWhereClause};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableProject {
    pub ids: Option<Vec<Uuid>>,
    pub owner_id: Option<Uuid>,
    pub member_ids: Option<Vec<Uuid>>, // also count owner as a member
    pub name: Option<String>,
}

impl Queryable for QueryableProject {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.owner_id, |mut f, owner_id| {
            f.push_sql("owner_id = ANY(")
                .push_param(QueryParam::Uuid(owner_id))
                .push_sql(")");

            f
        })
        .push_if_some(self.member_ids, |mut f, member_ids| {
            f.push_sql("(id IN (SELECT project_id FROM project_members WHERE user_id = ANY(")
                .push_param(QueryParam::ArrayUuid(member_ids.clone()))
                .push_sql(")) OR owner_id = ANY(");
            f.push_param(QueryParam::ArrayUuid(member_ids.clone()))
                .push_sql(")");

            f
        })
        .push_if_some(self.name, |mut f, name| {
            f.push_sql("name ILIKE ")
                .push_param(QueryParam::String(format!("%{}%", name)));

            f
        });

        wc
    }
}
