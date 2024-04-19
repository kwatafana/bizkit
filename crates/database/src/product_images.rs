use super::DatabaseError;
use bizkitdata::{ProductImages, ProductImagesDatabaseInput, ID};
use postgres::Client;

/// SQL queries for the product_images database table
pub struct ProductImagesSql;

impl ProductImagesSql {
    /// Initialize accounts table
    pub fn init(client: &mut Client, db_owner: &str) -> Result<(), DatabaseError> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS public.product_images (
	id serial NOT NULL UNIQUE,
	product integer NOT NULL,
	raw bytea[] NOT NULL,
	mimes varchar(128)[] NOT NULL,
	version smallint NOT NULL,
	CONSTRAINT \"Product Images id is PRIMARY KEY\" PRIMARY KEY (id),
	CONSTRAINT \"Product images id is UNIQUE\" UNIQUE (id)
);
ALTER TABLE public.product_images OWNER TO {db_owner};

-- object: \"Product images product is FOREIGN KEY\" | type: CONSTRAINT --
ALTER TABLE public.product_images DROP CONSTRAINT IF EXISTS \"Product images product is FOREIGN KEY\" CASCADE;
ALTER TABLE public.product_images ADD CONSTRAINT \"Product images product is FOREIGN KEY\" FOREIGN KEY (id)
REFERENCES public.products (id) MATCH SIMPLE
ON DELETE NO ACTION ON UPDATE NO ACTION;
-- ddl-end --
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
        product_images: ProductImagesDatabaseInput,
    ) -> Result<(), DatabaseError> {
        client
            .execute(
                "INSERT INTO public.product_images (product, raw, mimes, version) VALUES ($1, $2, $3, $4);",
                &[
                    &product_images.product,
                    &product_images.raw,
                    &product_images.mimes,
                    &product_images.version,
                ],
            )
        .map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }

    pub fn read(client: &mut Client, id: ID) -> Result<Option<ProductImages>, DatabaseError> {
        let row = client
            .query("SELECT * FROM public.product_images WHERE id = $1", &[&id])
            .map_err(|_| DatabaseError::SQL)?;

        if row.is_empty() {
            Ok(None)
        } else {
            Ok(Some(ProductImages {
                raw: row[0].get("raw"),
                mimes: row[0].get("mimes"),
                version: row[0].get("version"),
            }))
        }
    }

    pub fn read_all(client: &mut Client) -> Result<Vec<ProductImages>, DatabaseError> {
        let row = client
            .query("SELECT * FROM public.product_images", &[])
            .map_err(|_| DatabaseError::SQL)?;
        let mut product_images: Vec<ProductImages> = vec![];

        for r in &row {
            product_images.push(ProductImages {
                raw: r.get("raw"),
                mimes: r.get("mimes"),
                version: r.get("version"),
            })
        }

        Ok(product_images)
    }

    pub fn delete(client: &mut Client, id: ID) -> Result<(), DatabaseError> {
        client
            .execute("DELETE FROM public.product_images WHERE id = $1", &[&id])
            .map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }

    pub fn delete_by_product(client: &mut Client, product_id: ID) -> Result<(), DatabaseError> {
        client
            .execute(
                "DELETE FROM public.product_images WHERE product = $1",
                &[&product_id],
            )
            .map_err(|_| DatabaseError::SQL)?;
        Ok(())
    }
}
