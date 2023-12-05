use crate::inflector::Inflector;
use anyhow::Result;
use indoc::indoc;

// todo add models geenration for prisma

#[allow(dead_code)]
pub struct Model {
    pub config: ModelConfig,
    pub file_contents: String,
}

#[allow(dead_code)]
pub struct ModelConfig {
    pub model_name: String,
    pub table_name: String,
    pub file_name: String,
}

#[allow(dead_code)]
pub fn create(resource_name: &str) -> Result<Model> {
    let resource = generate(resource_name);

    crate::fs::add_rust_file(
        "backend/models",
        resource.config.file_name.as_str(),
        resource.file_contents.as_str(),
    )?;

    Ok(resource)
}

#[allow(dead_code)]
fn config(resource_name: &str) -> ModelConfig {
    let model_name = resource_name.to_pascal_case();
    let file_name = model_name.to_snake_case();
    let table_name = model_name.to_table_case();

    ModelConfig {
        model_name,
        file_name,
        table_name,
    }
}

// struct-field-names-as-array = "0.3.0"
// use this crate to convert the struct into array to pass
// prisma updates
#[allow(dead_code)] //todo add models ge
// enration for prisma
fn generate(resource_name: &str) -> Model {
    let config = config(resource_name);

    let contents_template: &str = indoc! {"
        use crate::schema::*;
        use crate::diesel::*;
        
        use diesel::QueryResult;
        use serde::{Serialize, Deserialize};
        use crate::models::*;
        use crate::DB;
        
        #[tsync::tsync]
        #[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, Identifiable, AsChangeset)]
        #[diesel(table_name=$TABLE_NAME)]
        pub struct $MODEL_NAME {
          /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
             Add columns here in the same order as the schema
             (because #[derive(Queryable)] expects this)
             -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- */
          // pub id: ID,
          // pub created_at: UTC,
          // pub updated_at: UTC,
        }
        
        #[tsync::tsync]
        #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
        #[diesel(table_name=$TABLE_NAME)]
        pub struct $MODEL_NAMEChangeset {
          /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
             Don't include non-mutable columns
             (ex: id, created_at/updated_at)
             -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- */
        }
        
        impl $MODEL_NAME {
          pub fn create(db: &DB, item: &$MODEL_NAMEChangeset) -> QueryResult<Self> {
              use crate::schema::$TABLE_NAME::dsl::*;
              
              insert_into($TABLE_NAME)
              .values(item)
              .get_result::<$MODEL_NAME>(db)
          }
          
          pub fn read(db: &DB, item_id: ID) -> QueryResult<Self> {
              use crate::schema::$TABLE_NAME::dsl::*;
              
              $TABLE_NAME.filter(id.eq(item_id)).first::<$MODEL_NAME>(db)
          }
          
          pub fn read_all(db: &DB, pagination: &PaginationParams) -> QueryResult<Vec<Self>> {
              use crate::schema::$TABLE_NAME::dsl::*;
          
              $TABLE_NAME
                  .order(created_at)
                  .limit(pagination.page_size)
                  .offset(pagination.page * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64))
                  .load::<$MODEL_NAME>(db)
          }
          
          pub fn update(db: &DB, item_id: ID, item: &$MODEL_NAMEChangeset) -> QueryResult<Self> {
              use crate::schema::$TABLE_NAME::dsl::*;
          
              diesel::update($TABLE_NAME.filter(id.eq(item_id)))
                  .set(item)
                  .get_result(db)
          }
          
          pub fn delete(db: &DB, item_id: ID) -> QueryResult<usize> {
              use crate::schema::$TABLE_NAME::dsl::*;
          
              diesel::delete($TABLE_NAME.filter(id.eq(item_id))).execute(db)
          }
        }
    "};
    let contents_prisma_template: &str = indoc! {"
        use crate::prisma_schema::*;
        use crate::prisma::*;

        use diesel::QueryResult;
        use serde::{Serialize, Deserialize};
        use crate::models::*;
        use crate::DB;

        #[tsync::tsync]
        #[derive(Debug, Serialize, Deserialize, Clone, Queryable, Insertable, Identifiable, AsChangeset)]
        #[diesel(table_name=$TABLE_NAME)]
        pub struct $MODEL_NAME {
          /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
             Add columns here in the same order as the schema
             (because #[derive(Queryable)] expects this)
             -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- */
          // pub id: ID,
          // pub created_at: UTC,
          // pub updated_at: UTC,
        }

        #[tsync::tsync]
        #[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset)]
        #[diesel(table_name=$TABLE_NAME)]
        pub struct $MODEL_NAMEChangeset {
          /* -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
             Don't include non-mutable columns
             (ex: id, created_at/updated_at)
             -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=- */
        }

        impl $MODEL_NAME {
          pub async fn create(db: &DB, item: &$MODEL_NAMEChangeset) -> QueryResult<Self> {
              use crate::prisma_model::$TABLE_NAME::dsl::*;

// use  create unchecked feature
client.$TABLE_NAME.create(
item.to_vec_required()
,vec![todo!(\"convert items to vec\")]).exec().await //todo add other fields with item.to_vec()
              // insert_into($TABLE_NAME)
              // .values(item)
              // .get_result::<$MODEL_NAME>(db)
          }

           pub async  fn read(db: &DB, item_id: ID) -> Result<Self::Data> {
              use crate::prisma_schema::$TABLE_NAME;

                client.$TABLE_NAME.find_first(vec![$TABLE_NAME::id::equals(item_id.to_string())])
                .exec()
    .await
    // .unwrap()


          }

          pub fn read_all(db: &DB, pagination: &PaginationParams) -> Result<Vec<Self::Data>> {
              use crate::prisma_schema::$TABLE_NAME;


              client.$TABLE_NAME
                  .find_many(vec![])
                  .order_by($TABLE_NAME::created_at::order(Direction::Desc))
.take(pagination.page_size).
                    .skip(pagination.page * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64))
                    .exec()
                    .await
                    // .unwrap()

                  // .offset(pagination.page * std::cmp::max(pagination.page_size, PaginationParams::MAX_PAGE_SIZE as i64))
                  .load::<$MODEL_NAME>(db)
          }

          pub fn update(db: &DB, item_id: ID, item: &$MODEL_NAMEChangeset) -> QueryResult<Self::Data> {
              use crate::prisma_schema::$TABLE_NAME;

    client.$TABLE_NAME.update($TABLE_NAME::id::equals(item_id.to_string()), vec![todo!(\"convert items to vec\")]).exec().await

          }

          pub fn delete(db: &DB, item_id: ID) -> QueryResult<usize> {
              use crate::prisma_schema::$TABLE_NAME;

client.$TABLE_NAME.delete($TABLE_NAME::id::equals(item_id.to_string())).exec().await
          }
        }
    "};

    let contents = String::from(contents_template)
        .replace("$MODEL_NAME", config.model_name.as_str())
        .replace("$TABLE_NAME", config.table_name.as_str());

    Model {
        config,
        file_contents: contents,
    }
}
