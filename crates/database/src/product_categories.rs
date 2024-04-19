use super::DatabaseError;
use bizkitdata::{ProductCategories, ProductCategoriesDatabaseInput, ID};
use postgres::Client;

/// SQL queries for the student_photos database table
pub struct ProductCategoriesSql;

impl ProductCategoriesSql {
    pub fn init(client: &mut Client, db_owner: &str) -> Result<(), DatabaseError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS public.product_categories (
	id serial NOT NULL UNIQUE,
	product integer NOT NULL,
	category integer NOT NULL,
	version smallint NOT NULL,
	CONSTRAINT id_is_primayr_key PRIMARY KEY (id)
);

ALTER TABLE public.product_categories OWNER TO {db_owner};
-- object: product_is_foreign_key | type: CONSTRAINT --
ALTER TABLE public.product_categories DROP CONSTRAINT IF EXISTS product_is_foreign_key CASCADE;
ALTER TABLE public.product_categories ADD CONSTRAINT product_is_foreign_key FOREIGN KEY (product)
REFERENCES public.products (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --

-- object: category_is_foreign_key | type: CONSTRAINT --
ALTER TABLE public.product_categories DROP CONSTRAINT IF EXISTS category_is_foreign_key CASCADE;
ALTER TABLE public.product_categories ADD CONSTRAINT category_is_foreign_key FOREIGN KEY (category)
REFERENCES public.categories (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --
"
        );
        client.batch_execute(&sql).unwrap();
        //.map_err(|_| DatabaseError::TableCreation)?;

        Ok(())
    }

    /// Create a new row
    pub fn create(
        client: &mut Client,
        product_category: ProductCategoriesDatabaseInput,
    ) -> Result<(), DatabaseError> {
        client
            .execute(
                "INSERT INTO public.product_categories (product, category, version) VALUES ($1, $2, $3);",
                &[&product_category.product, &product_category.category, &product_category.version],
            ).unwrap();
        //.map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }

    pub fn read(client: &mut Client, id: ID) -> Result<Option<ProductCategories>, DatabaseError> {
        let row = client
            .query(
                "SELECT * FROM public.product_categories WHERE id = $1",
                &[&id],
            )
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ProductCategories {
                id: row[0].get("id"),
                product: row[0].get("product"),
                category: row[0].get("category"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn read_all(client: &mut Client) -> Result<Vec<ProductCategories>, DatabaseError> {
        let row = client
            .query("SELECT * FROM public.product_categories", &[])
            .map_err(|_| DatabaseError::SQL)?;
        let mut product_categories: Vec<ProductCategories> = vec![];

        for r in &row {
            product_categories.push(ProductCategories {
                id: r.get("id"),
                product: r.get("product"),
                category: r.get("category"),
                version: r.get("version"),
            })
        }

        Ok(product_categories)
    }

    pub fn delete(client: &mut Client, id: ID) -> Result<(), DatabaseError> {
        client
            .execute(
                "DELETE FROM public.product_categories WHERE id = $1",
                &[&id],
            )
            .map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }
}
