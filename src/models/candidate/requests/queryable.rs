use mysk_lib::query::{QueryParam, Queryable, SqlWhereClause};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryableCandidate {
    pub ids: Option<Vec<Uuid>>,
    pub owner_id: Option<Uuid>,
    pub member_ids: Option<Vec<Uuid>>, // also count owner as a member
    pub project_ids: Option<Vec<Uuid>>,
    pub election_ids: Option<Vec<Uuid>>,
    pub question_ids: Option<Vec<Uuid>>,
}

impl Queryable for QueryableCandidate {
    fn to_where_clause<'sql>(self) -> SqlWhereClause<'sql> {
        let mut wc = SqlWhereClause::new();
        wc.push_if_some(self.ids, |mut f, ids| {
            f.push_sql("id = ANY(")
                .push_param(QueryParam::ArrayUuid(ids))
                .push_sql(")");

            f
        })
        .push_if_some(self.project_ids, |mut f, project_ids| {
            f.push_sql("question_id = ANY(SELECT id FROM questions WHERE election_id = ANY(SELECT id FROM elections WHERE project_id = ANY(")
                .push_param(QueryParam::ArrayUuid(project_ids));
            f.push_sql(")))");

            f
        })
        .push_if_some(self.owner_id, |mut f, owner_id| {
            f.push_sql("question_id = ANY(SELECT id FROM questions WHERE election_id = ANY(SELECT id FROM elections WHERE project_id = ANY(SELECT id FROM projects WHERE owner_id = ")
                .push_param(QueryParam::Uuid(owner_id))
                .push_sql(")))");

            f
        })
        .push_if_some(self.member_ids, |mut f, member_ids| {
            f.push_sql(
                "(question_id = ANY(SELECT id FROM questions WHERE election_id = ANY(SELECT id FROM elections WHERE project_id = ANY(SELECT project_id FROM project_members WHERE user_id = ANY(",
            )
            .push_param(QueryParam::ArrayUuid(member_ids.clone()))
            .push_sql(")))) OR question_id = ANY(SELECT id FROM questions WHERE election_id = ANY(SELECT id FROM elections WHERE project_id = ANY(SELECT id FROM projects WHERE owner_id = ANY(")
            .push_param(QueryParam::ArrayUuid(member_ids))
            .push_sql(")))))");
            f
        })
        .push_if_some(self.election_ids, |mut f, election_ids| {
            f.push_sql("question_id = ANY(SELECT id FROM questions WHERE election_id = ANY(")
                .push_param(QueryParam::ArrayUuid(election_ids))
                .push_sql(") )");

            f
        })
        .push_if_some(self.question_ids, |mut f, question_ids| {
            f.push_sql("question_id = ANY(")
                .push_param(QueryParam::ArrayUuid(question_ids))
                .push_sql(")");

            f
        });

        wc
    }
}
