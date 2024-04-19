use super::DatabaseError;
use bizkitdata::{Category, CategoryDatabaseInput, ID};
use postgres::Client;

/// SQL queries for the student_photos database table
pub struct CategoriesSql;

impl CategoriesSql {
    /// Initialize accounts table
    pub fn init(client: &mut Client, db_owner: &str) -> Result<(), DatabaseError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS public.categories (
	id serial NOT NULL UNIQUE,
	name character varying(80) NOT NULL,
	version smallint NOT NULL,
	CONSTRAINT id_is_primary_key PRIMARY KEY (id),
	CONSTRAINT cat_name_is_unique UNIQUE (name)
);
ALTER TABLE public.categories OWNER TO {db_owner};
"
        );
        client
            .batch_execute(&sql)
            .map_err(|_| DatabaseError::TableCreation)?;

        Ok(())
    }

    /// Create a new row
    pub fn create(
        client: &mut Client,
        category: CategoryDatabaseInput,
    ) -> Result<(), DatabaseError> {
        client
            .execute(
                "INSERT INTO public.categories (name, version) VALUES ($1, $2);",
                &[&category.name, &category.version],
            )
            .map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }

    pub fn read(client: &mut Client, id: ID) -> Result<Option<Category>, DatabaseError> {
        let row = client
            .query("SELECT * FROM public.categories WHERE id = $1", &[&id])
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Category {
                id: row[0].get("id"),
                name: row[0].get("name"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn read_all(client: &mut Client) -> Result<Vec<Category>, DatabaseError> {
        let row = client
            .query("SELECT * FROM public.categories", &[])
            .map_err(|_| DatabaseError::SQL)?;
        let mut categories: Vec<Category> = vec![];

        for r in &row {
            categories.push(Category {
                id: r.get("id"),
                name: r.get("name"),
                version: r.get("version"),
            })
        }

        Ok(categories)
    }

    pub fn read_by_name(
        client: &mut Client,
        name: &str,
    ) -> Result<Option<Category>, DatabaseError> {
        let row = client
            .query("SELECT * FROM public.categories WHERE name = $1", &[&name])
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Category {
                id: row[0].get("id"),
                name: row[0].get("name"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn update(
        client: &mut Client,
        id: ID,
        category: &CategoryDatabaseInput,
    ) -> Result<(), DatabaseError> {
        let mut transaction = client.transaction().map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE public.categories SET name = $1 WHERE id = $2",
                &[&category.name, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;
        transaction
            .execute(
                "UPDATE public.categories SET version = $1 WHERE id = $2",
                &[&category.version, &id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        transaction.commit().map_err(|_| DatabaseError::SQL)?;

        Ok(())
    }

    pub fn delete(client: &mut Client, id: ID) -> Result<(), DatabaseError> {
        client
            .execute(
                "DELETE FROM public.product_categories WHERE category = $1",
                &[&id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        client
            .execute("DELETE FROM public.categories WHERE id = $1", &[&id])
            .map_err(|_| DatabaseError::SQL)?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::config::Config;
    use crate::database::Database;

    #[test]
    fn test_create_read() {
        let config = Config::from_file("example-config.toml").unwrap();
        let db = Database::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();

        let category = CategoryDatabaseInput {
            name: "Computer Science".to_string(),
            version: 0,
        };

        if let Some(mut client) = db.client {
            super::CategoriesSql::create(&mut client, category.clone()).unwrap();
            let category1 =
                super::CategoriesSql::read_by_name(&mut client, &category.name).unwrap();

            if let Some(category1) = category1 {
                assert_eq!(category.name, category1.name);

                if let Some(category2) =
                    super::CategoriesSql::read(&mut client, category1.id).unwrap()
                {
                    assert_eq!(category.name, category2.name);
                    assert_eq!(category1, category2);
                }
            } else {
                panic!("wiip");
            }
        } else {
            panic!("woop");
        }
    }

    #[test]
    fn test_update_delete() {
        let config = Config::from_file("example-config.toml").unwrap();
        let db = Database::new(
            &config.database_name,
            &config.database_user,
            &config.database_password,
        )
        .unwrap();

        let category = CategoryDatabaseInput {
            name: "Physics".to_string(),
            version: 0,
        };

        let category_update = CategoryDatabaseInput {
            name: "Education".to_string(),
            version: 0,
        };

        if let Some(mut client) = db.client {
            super::CategoriesSql::create(&mut client, category.clone()).unwrap();
            super::CategoriesSql::update(&mut client, 1, &category_update.clone()).unwrap();
            if let Some(updated) =
                super::CategoriesSql::read_by_name(&mut client, &category_update.name).unwrap()
            {
                assert_eq!(category_update.name, updated.name);

                // delete category
                super::CategoriesSql::delete(&mut client, updated.id).unwrap();
                let deleted_category = super::CategoriesSql::read(&mut client, updated.id).unwrap();
                assert_eq!(deleted_category, None);
            } else {
                panic!("tring");
            }
        } else {
            panic!("woop");
        }
    }
}
