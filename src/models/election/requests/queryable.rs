use mysk_lib::query::{QueryParam, Queryable, SqlWhereClause};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableElection {
    pub ids: Option<Vec<Uuid>>,
    pub owner_id: Option<Uuid>,
    pub member_ids: Option<Vec<Uuid>>, // also count owner as a member
    pub project_ids: Option<Uuid>,
    pub name: Option<String>,
    pub header: Option<String>,
    pub details: Option<String>,
    pub label: Option<String>,
}

impl Queryable for QueryableElection {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.project_ids, |mut f, project_ids| {
            f.push_sql("project_id = ")
                .push_param(QueryParam::Uuid(project_ids));

            f
        })
        .push_if_some(self.owner_id, |mut f, owner_id| {
            f.push_sql("project_id = ANY(SELECT id FROM projects WHERE owner_id = ")
                .push_param(QueryParam::Uuid(owner_id))
                .push_sql(")");

            f
        })
        .push_if_some(self.member_ids, |mut f, member_ids| {
            f.push_sql(
                "project_id = ANY(SELECT project_id FROM project_members WHERE user_id = ANY(",
            )
            .push_param(QueryParam::ArrayUuid(member_ids.clone()))
            .push_sql("))");
            f
        })
        .push_if_some(self.name, |mut f, name| {
            f.push_sql("(name_th ILIKE ('%' || ")
                .push_param(QueryParam::String(name))
                .push_sql(" || '%') OR name_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.header, |mut f, header| {
            f.push_sql("(header_th ILIKE ('%' || ")
                .push_param(QueryParam::String(header))
                .push_sql(" || '%') OR header_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.details, |mut f, details| {
            f.push_sql("(details_th ILIKE ('%' || ")
                .push_param(QueryParam::String(details))
                .push_sql(" || '%') OR details_en ILIKE ('%' || ")
                .push_prev_param()
                .push_sql(" || '%'))");

            f
        })
        .push_if_some(self.label, |mut f, label| {
            f.push_sql("(label ILIKE ('%' || ")
                .push_param(QueryParam::String(label))
                .push_sql(" || '%'))");
            f
        });

        wc
    }
}
