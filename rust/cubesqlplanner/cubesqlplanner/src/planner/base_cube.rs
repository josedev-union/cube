use super::query_tools::QueryTools;
use super::sql_evaluator::{default_evaluate, EvaluationNode, MemberEvaluator};
use crate::cube_bridge::cube_definition::CubeDefinition;
use crate::cube_bridge::evaluator::CubeEvaluator;
use cubenativeutils::CubeError;
use std::rc::Rc;
pub struct BaseCube {
    cube_name: String,
    member_evaluator: Rc<EvaluationNode>,
    query_tools: Rc<QueryTools>,
}
impl BaseCube {
    pub fn try_new(
        cube_name: String,
        query_tools: Rc<QueryTools>,
        member_evaluator: Rc<EvaluationNode>,
    ) -> Result<Rc<Self>, CubeError> {
        let definition = query_tools
            .cube_evaluator()
            .cube_from_path(cube_name.clone())?;
        Ok(Rc::new(Self {
            cube_name,
            member_evaluator,
            query_tools,
        }))
    }

    pub fn to_sql(&self) -> Result<String, CubeError> {
        let cube_sql = self.table_sql()?;
        let cube_alias = self.cube_alias()?;
        let as_syntax_join = "AS"; //FIXME should be from JS BaseQuery

        Ok(format!("{} {} {}", cube_sql, as_syntax_join, cube_alias))
    }

    pub fn table_sql(&self) -> Result<String, CubeError> {
        default_evaluate(&self.member_evaluator, self.query_tools.clone())
    }

    fn cube_alias(&self) -> Result<String, CubeError> {
        Ok(self.query_tools.cube_alias_name(&self.cube_name))
    }
}
