//! Document search API types for `/v1/documents/search` endpoint.

use serde::{Deserialize, Serialize};

use crate::prelude::*;

/// `SearchRequest` defines the request to search for documents.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchRequest {
    /// The query to search for which will be embedded using the
    /// same embedding model as the one used for the source to query.
    pub query: String,

    /// The source to query.
    pub source: DocumentsSource,

    /// User-defined instructions to be included in the search query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// The number of chunks to return. Defaults to 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// Deprecated: Metric now comes from collection creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_metric: Option<RankingMetric>,

    /// How to perform the document search. Defaults to hybrid retrieval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_mode: Option<RetrievalMode>,
}

/// `SearchResponse` defines the response to a search request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchResponse {
    /// The search matches.
    pub matches: Vec<SearchMatch>,
}

/// `SearchMatch` defines a single match from a search request.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchMatch {
    /// The document ID.
    pub file_id: String,

    /// The chunk ID.
    pub chunk_id: String,

    /// The chunk content.
    pub chunk_content: String,

    /// The relevance score.
    pub score: f32,

    /// The collection ID(s).
    pub collection_ids: Vec<String>,

    /// Metadata fields belonging to the document of this chunk.
    #[serde(default)]
    pub fields: HashMap<String, String>,
}

/// `DocumentsSource` defines the source of documents to search over.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DocumentsSource {
    /// The collection IDs to search in.
    pub collection_ids: Vec<String>,
}

/// Parameters to control realtime data.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SearchParameters {
    /// Date from which to consider the results in ISO-8601 YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<String>,

    /// Date up to which to consider the results in ISO-8601 YYYY-MM-DD format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_date: Option<String>,

    /// Maximum number of search results to use. Defaults to 15.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_search_results: Option<i32>,

    /// Choose the mode to query realtime data: `off`, `on` (default), or `auto`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,

    /// Whether to return citations in the response or not. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_citations: Option<bool>,

    /// List of sources to search in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<SearchSource>>,
}

/// Search source for realtime data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SearchSource {
    /// Search X (Twitter).
    X {
        /// List of X handles to exclude from the search results.
        #[serde(skip_serializing_if = "Option::is_none")]
        excluded_x_handles: Option<Vec<String>>,

        /// X Handles of the users from whom to consider the posts.
        #[serde(skip_serializing_if = "Option::is_none")]
        included_x_handles: Option<Vec<String>>,

        /// DEPRECATED: Use `included_x_handles` instead.
        #[serde(skip_serializing_if = "Option::is_none")]
        x_handles: Option<Vec<String>>,

        /// The minimum favorite count of the X posts to consider.
        #[serde(skip_serializing_if = "Option::is_none")]
        post_favorite_count: Option<i32>,

        /// The minimum view count of the X posts to consider.
        #[serde(skip_serializing_if = "Option::is_none")]
        post_view_count: Option<i32>,
    },
    /// Search the web.
    Web {
        /// List of websites to allow in the search results (whitelist).
        #[serde(skip_serializing_if = "Option::is_none")]
        allowed_websites: Option<Vec<String>>,

        /// List of websites to exclude from the search results.
        #[serde(skip_serializing_if = "Option::is_none")]
        excluded_websites: Option<Vec<String>>,

        /// ISO alpha-2 code of the country for filtering results.
        #[serde(skip_serializing_if = "Option::is_none")]
        country: Option<String>,

        /// If set to true, mature content won't be considered. Defaults to true.
        #[serde(skip_serializing_if = "Option::is_none")]
        safe_search: Option<bool>,
    },
    /// Search news sources.
    News {
        /// ISO alpha-2 code of the country for filtering results.
        #[serde(skip_serializing_if = "Option::is_none")]
        country: Option<String>,

        /// List of websites to exclude from the search results.
        #[serde(skip_serializing_if = "Option::is_none")]
        excluded_websites: Option<Vec<String>>,
    },
    /// Search knowledge bases.
    Rss {
        /// List of RSS feed URLs to search.
        #[serde(skip_serializing_if = "Option::is_none")]
        urls: Option<Vec<String>>,
    },
}

impl Default for SearchSource {
    fn default() -> Self {
        SearchSource::Web {
            allowed_websites: None,
            excluded_websites: None,
            country: None,
            safe_search: None,
        }
    }
}

/// Deprecated: Metric now comes from collection creation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RankingMetric {
    /// Unknown ranking metric.
    #[default]
    RankingMetricUnknown,
    /// L2 distance metric.
    RankingMetricL2Distance,
    /// Cosine similarity metric.
    RankingMetricCosineSimilarity,
}

/// Retrieval mode configuration for document search.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum RetrievalMode {
    /// Hybrid search combining keyword and semantic search.
    Hybrid {
        /// Which reranker to use to limit results to the desired value.
        #[serde(skip_serializing_if = "Option::is_none")]
        reranker: Option<HybridReranker>,

        /// Additional multiplier to requested search limit. Valid range is [1, 100].
        #[serde(skip_serializing_if = "Option::is_none")]
        search_multiplier: Option<i32>,
    },
    /// Semantic search using dense embeddings.
    Semantic {
        /// Optional reranker, always used when doing search across multiple collections.
        #[serde(skip_serializing_if = "Option::is_none")]
        reranker: Option<RerankerModel>,
    },
    /// Keyword search using sparse embeddings.
    Keyword {
        /// Optional reranker, always used when doing search across multiple collections.
        #[serde(skip_serializing_if = "Option::is_none")]
        reranker: Option<RerankerModel>,
    },
}

impl Default for RetrievalMode {
    fn default() -> Self {
        RetrievalMode::Hybrid {
            reranker: None,
            search_multiplier: None,
        }
    }
}

/// Reranker configuration for hybrid retrieval.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum HybridReranker {
    /// Use a reranker model to perform the reranking.
    RerankerModel {
        /// Instructions for the reranking model.
        #[serde(skip_serializing_if = "Option::is_none")]
        instructions: Option<String>,

        /// The model to use for reranking.
        #[serde(skip_serializing_if = "Option::is_none")]
        model: Option<String>,
    },
    /// Use RRF (Reciprocal Rank Fusion) to perform the reranking.
    Rrf {
        /// Weight for embedding (dense) search results. Between 0 and 1, defaults to 0.5.
        #[serde(skip_serializing_if = "Option::is_none")]
        embedding_weight: Option<f32>,

        /// Weight for keyword (sparse) search results. Between 0 and 1, defaults to 0.5.
        #[serde(skip_serializing_if = "Option::is_none")]
        text_weight: Option<f32>,

        /// The RRF constant k used in the formula. Defaults to 60.
        #[serde(skip_serializing_if = "Option::is_none")]
        k: Option<i32>,
    },
}

impl Default for HybridReranker {
    fn default() -> Self {
        HybridReranker::Rrf {
            embedding_weight: None,
            text_weight: None,
            k: None,
        }
    }
}

/// Configuration for model-based reranking.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct RerankerModel {
    /// Instructions for the reranking model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,

    /// The model to use for reranking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Configuration for reciprocal rank fusion (RRF) reranking.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ReciprocalRankFusion {
    /// Weight for embedding (dense) search results. Between 0 and 1, defaults to 0.5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_weight: Option<f32>,

    /// Weight for keyword (sparse) search results. Between 0 and 1, defaults to 0.5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_weight: Option<f32>,

    /// The RRF constant k used in the reciprocal rank fusion formula. Defaults to 60.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<i32>,
}
