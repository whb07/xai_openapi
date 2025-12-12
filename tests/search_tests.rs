//! Tests for the search module types.

mod common;

use pretty_assertions::assert_eq;
use serde_json::json;
use xai_openapi::search::{
    DocumentsSource, HybridReranker, RankingMetric, ReciprocalRankFusion, RerankerModel,
    RetrievalMode, SearchMatch, SearchParameters, SearchRequest, SearchResponse, SearchSource,
};

#[test]
fn test_search_request() {
    let json = json!({
        "query": "machine learning",
        "source": {
            "collection_ids": ["col_123", "col_456"]
        },
        "instructions": "Focus on recent papers",
        "limit": 20
    });

    let request: SearchRequest = common::test_roundtrip(json);
    assert_eq!(request.query, "machine learning");
    assert_eq!(request.source.collection_ids.len(), 2);
    assert_eq!(
        request.instructions,
        Some("Focus on recent papers".to_string())
    );
    assert_eq!(request.limit, Some(20));
}

#[test]
fn test_search_request_default_roundtrip() {
    common::test_default_roundtrip::<SearchRequest>();
}

#[test]
fn test_search_response() {
    let json = json!({
        "matches": [
            {
                "file_id": "file_123",
                "chunk_id": "chunk_1",
                "chunk_content": "This is the content...",
                "score": 0.95,
                "collection_ids": ["col_123"],
                "fields": {"title": "ML Paper"}
            }
        ]
    });

    let response: SearchResponse = common::test_roundtrip(json);
    assert_eq!(response.matches.len(), 1);
    assert_eq!(response.matches[0].file_id, "file_123");
    assert_eq!(response.matches[0].score, 0.95);
}

#[test]
fn test_search_response_default_roundtrip() {
    common::test_default_roundtrip::<SearchResponse>();
}

#[test]
fn test_search_match() {
    let json = json!({
        "file_id": "file_abc",
        "chunk_id": "chunk_xyz",
        "chunk_content": "Content of the chunk",
        "score": 0.87,
        "collection_ids": ["col_1", "col_2"],
        "fields": {
            "author": "John Doe",
            "date": "2024-01-15"
        }
    });

    let match_result: SearchMatch = common::test_roundtrip(json);
    assert_eq!(match_result.file_id, "file_abc");
    assert_eq!(match_result.chunk_id, "chunk_xyz");
    assert_eq!(match_result.score, 0.87);
    assert_eq!(
        match_result.fields.get("author"),
        Some(&"John Doe".to_string())
    );
}

#[test]
fn test_documents_source() {
    let json = json!({
        "collection_ids": ["col_1", "col_2", "col_3"]
    });

    let source: DocumentsSource = common::test_roundtrip(json);
    assert_eq!(source.collection_ids.len(), 3);
}

#[test]
fn test_documents_source_default_roundtrip() {
    common::test_default_roundtrip::<DocumentsSource>();
}

#[test]
fn test_search_parameters() {
    let json = json!({
        "from_date": "2024-01-01",
        "to_date": "2024-12-31",
        "max_search_results": 25,
        "mode": "auto",
        "return_citations": true
    });

    let params: SearchParameters = common::test_roundtrip(json);
    assert_eq!(params.from_date, Some("2024-01-01".to_string()));
    assert_eq!(params.to_date, Some("2024-12-31".to_string()));
    assert_eq!(params.max_search_results, Some(25));
    assert_eq!(params.mode, Some("auto".to_string()));
    assert_eq!(params.return_citations, Some(true));
}

#[test]
fn test_search_parameters_default_roundtrip() {
    common::test_default_roundtrip::<SearchParameters>();
}

#[test]
fn test_search_source_x() {
    let json = json!({
        "type": "x",
        "included_x_handles": ["@elonmusk", "@xai"],
        "excluded_x_handles": ["@spam"],
        "post_favorite_count": 100
    });

    let source: SearchSource = common::test_roundtrip(json);
    match source {
        SearchSource::X {
            included_x_handles,
            excluded_x_handles,
            post_favorite_count,
            ..
        } => {
            assert_eq!(
                included_x_handles,
                Some(vec!["@elonmusk".to_string(), "@xai".to_string()])
            );
            assert_eq!(excluded_x_handles, Some(vec!["@spam".to_string()]));
            assert_eq!(post_favorite_count, Some(100));
        }
        _ => panic!("Expected X source"),
    }
}

#[test]
fn test_search_source_web() {
    let json = json!({
        "type": "web",
        "allowed_websites": ["example.com", "test.org"],
        "country": "US",
        "safe_search": true
    });

    let source: SearchSource = common::test_roundtrip(json);
    match source {
        SearchSource::Web {
            allowed_websites,
            country,
            safe_search,
            ..
        } => {
            assert_eq!(
                allowed_websites,
                Some(vec!["example.com".to_string(), "test.org".to_string()])
            );
            assert_eq!(country, Some("US".to_string()));
            assert_eq!(safe_search, Some(true));
        }
        _ => panic!("Expected Web source"),
    }
}

#[test]
fn test_search_source_news() {
    let json = json!({
        "type": "news",
        "country": "UK",
        "excluded_websites": ["tabloid.com"]
    });

    let source: SearchSource = common::test_roundtrip(json);
    match source {
        SearchSource::News {
            country,
            excluded_websites,
        } => {
            assert_eq!(country, Some("UK".to_string()));
            assert_eq!(excluded_websites, Some(vec!["tabloid.com".to_string()]));
        }
        _ => panic!("Expected News source"),
    }
}

#[test]
fn test_search_source_rss() {
    let json = json!({
        "type": "rss",
        "urls": ["https://example.com/feed.xml"]
    });

    let source: SearchSource = common::test_roundtrip(json);
    match source {
        SearchSource::Rss { urls } => {
            assert_eq!(urls, Some(vec!["https://example.com/feed.xml".to_string()]));
        }
        _ => panic!("Expected Rss source"),
    }
}

#[test]
fn test_ranking_metric() {
    let unknown: RankingMetric = serde_json::from_value(json!("RANKING_METRIC_UNKNOWN")).unwrap();
    assert_eq!(unknown, RankingMetric::RankingMetricUnknown);

    let l2: RankingMetric = serde_json::from_value(json!("RANKING_METRIC_L2_DISTANCE")).unwrap();
    assert_eq!(l2, RankingMetric::RankingMetricL2Distance);

    let cosine: RankingMetric =
        serde_json::from_value(json!("RANKING_METRIC_COSINE_SIMILARITY")).unwrap();
    assert_eq!(cosine, RankingMetric::RankingMetricCosineSimilarity);
}

#[test]
fn test_retrieval_mode_hybrid() {
    let json = json!({
        "type": "hybrid",
        "search_multiplier": 5
    });

    let mode: RetrievalMode = common::test_roundtrip(json);
    match mode {
        RetrievalMode::Hybrid {
            search_multiplier, ..
        } => {
            assert_eq!(search_multiplier, Some(5));
        }
        _ => panic!("Expected Hybrid mode"),
    }
}

#[test]
fn test_retrieval_mode_semantic() {
    let json = json!({
        "type": "semantic",
        "reranker": {
            "model": "grok-3",
            "instructions": "Focus on relevance"
        }
    });

    let mode: RetrievalMode = common::test_roundtrip(json);
    match mode {
        RetrievalMode::Semantic { reranker } => {
            let r = reranker.unwrap();
            assert_eq!(r.model, Some("grok-3".to_string()));
        }
        _ => panic!("Expected Semantic mode"),
    }
}

#[test]
fn test_retrieval_mode_keyword() {
    let json = json!({
        "type": "keyword"
    });

    let mode: RetrievalMode = common::test_roundtrip(json);
    match mode {
        RetrievalMode::Keyword { reranker } => {
            assert!(reranker.is_none());
        }
        _ => panic!("Expected Keyword mode"),
    }
}

#[test]
fn test_hybrid_reranker_model() {
    let json = json!({
        "type": "reranker_model",
        "model": "grok-3",
        "instructions": "Rank by relevance"
    });

    let reranker: HybridReranker = common::test_roundtrip(json);
    match reranker {
        HybridReranker::RerankerModel {
            model,
            instructions,
        } => {
            assert_eq!(model, Some("grok-3".to_string()));
            assert_eq!(instructions, Some("Rank by relevance".to_string()));
        }
        _ => panic!("Expected RerankerModel"),
    }
}

#[test]
fn test_hybrid_reranker_rrf() {
    let json = json!({
        "type": "rrf",
        "embedding_weight": 0.6,
        "text_weight": 0.4,
        "k": 60
    });

    let reranker: HybridReranker = common::test_roundtrip(json);
    match reranker {
        HybridReranker::Rrf {
            embedding_weight,
            text_weight,
            k,
        } => {
            assert_eq!(embedding_weight, Some(0.6));
            assert_eq!(text_weight, Some(0.4));
            assert_eq!(k, Some(60));
        }
        _ => panic!("Expected Rrf"),
    }
}

#[test]
fn test_reranker_model() {
    let json = json!({
        "model": "grok-3",
        "instructions": "Rerank by topic"
    });

    let model: RerankerModel = common::test_roundtrip(json);
    assert_eq!(model.model, Some("grok-3".to_string()));
    assert_eq!(model.instructions, Some("Rerank by topic".to_string()));
}

#[test]
fn test_reranker_model_default_roundtrip() {
    common::test_default_roundtrip::<RerankerModel>();
}

#[test]
fn test_reciprocal_rank_fusion() {
    let json = json!({
        "embedding_weight": 0.7,
        "text_weight": 0.3,
        "k": 100
    });

    let rrf: ReciprocalRankFusion = common::test_roundtrip(json);
    assert_eq!(rrf.embedding_weight, Some(0.7));
    assert_eq!(rrf.text_weight, Some(0.3));
    assert_eq!(rrf.k, Some(100));
}

#[test]
fn test_reciprocal_rank_fusion_default_roundtrip() {
    common::test_default_roundtrip::<ReciprocalRankFusion>();
}
