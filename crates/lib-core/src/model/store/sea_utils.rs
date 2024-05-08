use sea_query::{BinOper, ConditionExpression, IntoColumnRef, SimpleExpr};

/// NOTE: Not sure it adds lot of clarity from the standard sea-query builder methods.
///
/// Similar to something like `Expr::col((DItemDSourceBmc::table_iden(), DSourceIden::DsourceId)).eq(dsource_id)`
#[allow(dead_code)] // for now, not used
pub fn bin_op(left: impl IntoColumnRef, op: BinOper, right: impl Into<SimpleExpr>) -> ConditionExpression {
	let left = left.into_column_ref();
	let right: SimpleExpr = right.into();
	SimpleExpr::Binary(Box::new(left.into()), op, Box::new(right)).into()
}
