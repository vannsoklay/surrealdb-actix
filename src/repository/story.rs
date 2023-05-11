use actix_web::web::Data;
use surrealdb::sql::{Object, Value, thing, Array};
use crate::prelude::*;
use crate::SurrealDBRepo;
use crate::config::surrealdb::{Creatable, Patchable};
use crate::utils::macros::map;
use std::collections::BTreeMap;

pub struct StoryBMC;

impl StoryBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM todo;";

        let res = db.ds.execute(ast, &db.ses, None, true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn create<T: Creatable>(
        db: Data<SurrealDBRepo>,
        tb: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *";

        let data: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = map![
			"tb".into() => tb.into(),
			"data".into() => Value::from(data)];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        W(first_val.first()).try_into()
    }

    pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th";

        let tid = format!("todo:{}", tid);

        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn update<T: Patchable>(
        db: Data<SurrealDBRepo>,
        tid: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "UPDATE $th MERGE $data RETURN *";

        let tid = format!("todo:{}", tid);

        let vars = map![
			"th".into() => thing(&tid)?.into(),
			"data".into() => data.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        let result = first_res.result?;

        W(result.first()).try_into()
    }

    pub async fn delete(db: Data<SurrealDBRepo>, tid: &str) -> Result<String, Error> {
        let sql = "DELETE $th RETURN *";

        let tid = format!("todo:{}", tid);

        let vars = map!["th".into() => thing(&tid)?.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        first_res.result?;

        Ok(tid)
    }
}
