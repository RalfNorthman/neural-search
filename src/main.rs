use anyhow::Result;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{CreateCollection, SearchPoints, VectorParams, VectorsConfig};
use std::collections::HashMap;
use std::env;

// Example of top level client
// You may also use tonic-generated client from `src/qdrant.rs`
async fn client() -> Result<QdrantClient> {
    let mut config = QdrantClientConfig::from_url(env::var("QDRANT_CLUSTER_URL")?.as_ref());
    config.set_api_key(env::var("QDRANT_API_KEY")?.as_ref());
    QdrantClient::new(Some(config)).await
}

trait Helper {
    fn with_name_dim(name: &str, n_dim: u64) -> Self;
}

impl Helper for CreateCollection {
    fn with_name_dim(name: &str, n_dim: u64) -> Self {
        CreateCollection {
            collection_name: name.into(),
            vectors_config: Some(VectorsConfig {
                config: Some(Config::Params(VectorParams {
                    size: n_dim,
                    distance: Distance::Cosine.into(),
                })),
            }),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let client = client().await?;

    let collections_list = client.list_collections().await?;
    dbg!(collections_list);
    // collections_list = ListCollectionsResponse {
    //     collections: [
    //         CollectionDescription {
    //             name: "test",
    //         },
    //     ],
    //     time: 1.78e-6,
    // }

    let collection_name = "test";
    client.delete_collection(collection_name).await?;

    client
        .create_collection(&CreateCollection::with_name_dim(collection_name.into(), 10))
        .await?;

    let collection_info = client.collection_info(collection_name).await?;
    dbg!(collection_info);

    let payload: Payload = vec![("foo", "Bar".into()), ("bar", 12.into())]
        .into_iter()
        .collect::<HashMap<_, Value>>()
        .into();

    let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
    client
        .upsert_points_blocking(collection_name, points, None)
        .await?;

    let search_result = client
        .search_points(&SearchPoints {
            collection_name: collection_name.into(),
            vector: vec![11.; 10],
            filter: None,
            limit: 10,
            with_vectors: None,
            with_payload: None,
            params: None,
            score_threshold: None,
            offset: None,
            ..Default::default()
        })
        .await?;
    dbg!(search_result);
    // search_result = SearchResponse {
    //     result: [
    //         ScoredPoint {
    //             id: Some(
    //                 PointId {
    //                     point_id_options: Some(
    //                         Num(
    //                             0,
    //                         ),
    //                     ),
    //                 },
    //             ),
    //             payload: {},
    //             score: 1.0000001,
    //             version: 0,
    //             vectors: None,
    //         },
    //     ],
    //     time: 5.312e-5,
    // }

    Ok(())
}
