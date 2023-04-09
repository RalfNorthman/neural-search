use anyhow::Result;

use async_openai::types::{CreateEmbeddingRequestArgs, CreateEmbeddingResponse, EmbeddingInput};
use async_openai::Client;
use qdrant_client::prelude::*;
use qdrant_client::qdrant::vectors_config::Config;
use qdrant_client::qdrant::{
    CreateCollection, Filter, HasIdCondition, ScrollPoints, SearchPoints, VectorParams,
    VectorsConfig,
};
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

async fn embed<T>(inputs: T) -> Result<CreateEmbeddingResponse>
where
    T: Into<EmbeddingInput>,
{
    let client = Client::new();

    // An embedding is a vector (list) of floating point numbers.
    // The distance between two vectors measures their relatedness.
    // Small distances suggest high relatedness and large distances suggest low relatedness.

    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input(inputs)
        .build()?;

    let response = client.embeddings().create(request).await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let client = client().await?;

    let collection_name = "test";

    // let collection_info = client.collection_info(collection_name).await?;
    // dbg!(collection_info);

    let scroll_input = ScrollPoints {
        collection_name: collection_name.into(),
        filter: Filter {
            must: vec![HasIdCondition {
                has_id: vec![0.into()],
            }
            .into()],
            ..Default::default()
        }
        .into(),
        ..Default::default()
    };
    let mouse = client.scroll(&scroll_input).await?;
    dbg!(mouse);

    // let collections_list = client.list_collections().await?;
    // dbg!(collections_list);
    // collections_list = ListCollectionsResponse {
    //     collections: [
    //         CollectionDescription {
    //             name: "test",
    //         },
    //     ],
    //     time: 1.78e-6,
    // }

    /*
    client.delete_collection(collection_name).await?;

    let new_collection = CreateCollection::with_name_dim(collection_name.into(), 1536);
    client.create_collection(&new_collection).await?;

    // let payload: Payload = vec![("foo", "Bar".into()), ("bar", 12.into())]
    // .into_iter()
    // .collect::<HashMap<_, Value>>()
    // .into();

    let texts = vec![
        "Cat chases mouse.",
        "Feline hunts rodent.",
        "Det var en katt- och råttalek.",
        "Katt jagar råtta.",
        "Ham sandwich.",
        "Swiss cheese from the alps.",
    ];
    let embed_response = embed(&texts).await?;

    let points = embed_response
        .data
        .into_iter()
        .zip(texts.iter())
        .map(|(emb, txt)| {
            let mut payload = Payload::new();
            payload.insert("text", *txt);
            PointStruct::new(emb.index as u64, emb.embedding, payload)
        })
        .collect();
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
    */
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
