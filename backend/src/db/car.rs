use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{prelude::*, dsl::*};

use crate::api::model::QueryRequest;

use super::schema::{car, car::dsl::*};
use diesel::result::Error;

#[derive(Debug, PartialEq, Eq, Queryable)]
pub struct Car {
    pub id: i32,
    pub vin: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub color: String,
    pub price: BigDecimal,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = car)]
pub struct NewCar<'a> {
    pub vin: &'a str,
    pub make: &'a str,
    pub model: &'a str,
    pub year: i32,
    pub color: &'a str,
    pub price: BigDecimal,
}

pub struct CarResource<'a> {
    conn: &'a mut PgConnection,
}

impl<'a> CarResource<'a> {
    pub fn with(conn: &'a mut PgConnection) -> Self {
        Self { conn }
    }

    pub fn get_single(&mut self, i: i32) -> Result<Car, Error> {
        car.find(i).first(self.conn)
    }

    pub fn get_manufacturers(&mut self) -> Result<Vec<String>, Error> {
        car.select(make)
            .distinct()
            .order(make.asc())
            .load::<String>(self.conn)
    }

    #[allow(dead_code)]
    pub fn create(&mut self, new_car: NewCar) -> Result<Car, Error> {
        diesel::insert_into(car::table)
            .values(&new_car)
            .get_result(self.conn)
    }

    #[allow(dead_code)]
    pub fn update(&mut self, update_car: Car) -> Result<Car, Error> {
        diesel::update(car.find(update_car.id))
            .set((
                vin.eq(update_car.vin),
                make.eq(update_car.make),
                model.eq(update_car.model),
                year.eq(update_car.year),
                color.eq(update_car.color),
                price.eq(update_car.price),
            ))
            .get_result::<Car>(self.conn)
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, i: i32) -> Result<usize, Error> {
        diesel::delete(car.find(i)).execute(self.conn)
    }

    // TODO: Add filters and sorts
    pub fn list(&mut self, q: QueryRequest) -> Result<(Vec<Car>, i64), Error> {
        let query = car;
        let cnt = query.select(count(id)).first::<i64>(self.conn)?;
        let res = query.order(make.asc()).limit(q.rows.unwrap()).offset(q.first.unwrap()).load::<Car>(self.conn)?;
        Ok((res, cnt))
    }
}
