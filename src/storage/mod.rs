use std::sync::Arc;

use arrow_array::{types::Float32Type, FixedSizeListArray, RecordBatch, StringArray};

use lancedb::query::{ExecutableQuery, QueryBase};

use futures::TryStreamExt;
use lancedb::{
    arrow::arrow_schema::{DataType, Field, Schema},
    Table,
};
use tracing::warn;

pub struct VecDB {
    default_table: Table,
}

impl VecDB {
    pub async fn connect(db_path: &str, default_table: &str) -> anyhow::Result<Self> {
        let connection = lancedb::connect(db_path).execute().await?;
        let table_exists = connection
            .table_names()
            .execute()
            .await?
            .contains(&default_table.to_string());
        if !table_exists {
            warn!("Table {} does not exist, creating it", default_table);
            let schema = Self::get_default_schema();
            connection
                .create_empty_table(default_table, schema)
                .execute()
                .await?;
        }
        let table = connection.open_table(default_table).execute().await?;
        Ok(Self {
            default_table: table,
        })
    }

    pub async fn find_similar(&self, vector: Vec<f32>, n: usize) -> anyhow::Result<RecordBatch> {
        let results = self
            .default_table
            .query()
            .nearest_to(vector)?
            .limit(n)
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        results
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("No results found in the vector table"))
    }

    /// Get the default schema for the VecDB
    pub fn get_default_schema() -> Arc<Schema> {
        Arc::new(Schema::new(vec![
            Field::new("filename", DataType::Utf8, false),
            Field::new(
                "vector",
                DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), 384),
                true,
            ),
        ]))
    }

    pub async fn add_vector(
        &self,
        filenames: &[&str],
        vectors: Vec<Vec<f32>>,
        vec_dim: i32,
    ) -> anyhow::Result<()> {
        let schema = self.default_table.schema().await?;
        let key_array = StringArray::from_iter_values(filenames);
        let vectors_array = FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
            vectors
                .into_iter()
                .map(|v| Some(v.into_iter().map(|i| Some(i)))),
            vec_dim,
        );
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![Arc::new(key_array), Arc::new(vectors_array)],
        )?;
        self.default_table.add(batch).execute().await?;
        Ok(())
    }
}
