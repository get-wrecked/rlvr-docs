---
domain: Recommender Systems
category: data-science
verification_type: exact_match
dataset_scale: 25M-233M ratings (MovieLens, Amazon)
difficulty_range: easy/medium/hard
modality: text
status: strong_hypothesis
---

# Recommender Systems

## Overview

Recommender system tasks predict user preferences: rating prediction (what score will user U give item I?), ranking prediction (order these items by how much user U will like them), and next-item prediction (what will user U interact with next?). The model reasons about user-item interaction matrices, user history, and item metadata to generate personalized predictions.

RLVR verification is well-defined: predicted ratings are compared to held-out ground truth using RMSE or MAE, and predicted rankings are evaluated using NDCG, MAP, or Hit Rate. The challenge is that recommendation quality is inherently noisy (users are inconsistent), and the gold standard is what users actually did, not necessarily what they would prefer.

## Verification Mechanism

```python
import numpy as np

def verify_rating_prediction(predicted_rating: float, gold_rating: float,
                               scale_min: float = 1.0, scale_max: float = 5.0) -> dict:
    """Verify a single rating prediction."""
    # Clamp prediction to valid range
    predicted_clamped = max(scale_min, min(scale_max, predicted_rating))
    error = abs(predicted_clamped - gold_rating)
    rmse = error  # single-sample RMSE = absolute error

    # Reward: inverse error, normalized by scale range
    scale_range = scale_max - scale_min
    reward = max(0, 1.0 - error / scale_range)

    return {
        "absolute_error": error,
        "reward": reward
    }

def verify_ranking(predicted_ranking: list, gold_relevance: list, k: int = 10) -> dict:
    """
    Verify a predicted item ranking using NDCG@k.
    predicted_ranking: list of item IDs in predicted order
    gold_relevance: dict {item_id: relevance_score}
    """
    # DCG
    dcg = 0.0
    for i, item in enumerate(predicted_ranking[:k]):
        rel = gold_relevance.get(item, 0.0)
        dcg += rel / np.log2(i + 2)

    # Ideal DCG
    ideal_rels = sorted(gold_relevance.values(), reverse=True)[:k]
    idcg = sum(rel / np.log2(i + 2) for i, rel in enumerate(ideal_rels))

    ndcg = dcg / idcg if idcg > 0 else 0.0

    # Hit Rate: did any relevant item appear in top-k?
    hit = any(gold_relevance.get(item, 0) > 0 for item in predicted_ranking[:k])

    return {
        "ndcg_at_k": ndcg,
        "hit_rate": 1.0 if hit else 0.0,
        "reward": ndcg
    }

def verify_batch_rmse(predictions: list, golds: list) -> dict:
    """Batch RMSE computation."""
    errors = [(p - g) ** 2 for p, g in zip(predictions, golds)]
    rmse = np.sqrt(np.mean(errors))
    return {"rmse": rmse, "reward": max(0, 1.0 - rmse / 2.0)}
```

## Dataset Sources

- **MovieLens 25M**: 25 million ratings (1-5 stars) from 162,000 users on 62,000 movies. Includes tags and timestamps. GroupLens Research. The standard recommendation benchmark.
- **MovieLens 100K/1M/10M**: Smaller versions for rapid iteration.
- **Amazon Product Reviews**: 233 million reviews across 29 product categories. Ratings, text reviews, and product metadata. McAuley lab, UCSD.
- **Netflix Prize**: 100M ratings from 480K users on 17K movies. Historical benchmark (2006-2009). Data no longer officially distributed but widely replicated.
- **Yelp Dataset**: 8M reviews of 160K businesses. Includes user social network.
- **Last.fm**: Music listening history for 1K users. Implicit feedback (play counts, not explicit ratings).
- **Steam**: 7.8M reviews of 15K games. Includes play time as implicit signal.
- **Book-Crossing**: 1.1M ratings of 270K books by 278K users. Sparse and noisy.
- **Gowalla / Foursquare**: Location check-in data for POI recommendation.

## Task Format

- **Input**: User history and candidate items.
```
User U has rated the following movies:
- The Shawshank Redemption: 5/5
- Pulp Fiction: 4/5
- The Matrix: 5/5
- Forrest Gump: 3/5
- Inception: 5/5

Predict the rating User U would give to:
1. The Dark Knight
2. The Notebook
3. Interstellar
```
- **Output**: Predicted ratings.
```
1. The Dark Knight: 4.5/5
2. The Notebook: 2.5/5
3. Interstellar: 5.0/5
```

Ranking task:
```
Given User U's history, rank these 20 movies from most to least likely to be enjoyed.
[list of 20 movies]
```

## Difficulty Curriculum

- Level 1: Rating prediction for active users (100+ ratings) on popular items
- Level 2: Rating prediction for moderate users (20-50 ratings)
- Level 3: Ranking top-10 items from a candidate set of 100
- Level 4: Cold-start users (< 5 ratings), relying on item metadata and demographics
- Level 5: Cold-start items (new items, no interaction history)
- Level 6: Cross-domain recommendation (train on movies, predict books)
- Level 7: Sequential/session-based recommendation (predict next item from session)
- Level 8: Context-aware recommendation (time of day, location, device)
- Level 9: Explanation generation (recommend AND explain why), multi-stakeholder fairness constraints

## Limitations & Risks

- **User inconsistency**: The same user might rate the same movie differently on different days. Gold ratings are noisy by nature, limiting achievable RMSE to ~0.8-0.9 on MovieLens.
- **Popularity bias**: Models tend to recommend popular items because they have more training signal. This reduces diversity and discovery.
- **Evaluation protocol sensitivity**: Results vary dramatically with train/test split strategy (random split vs. temporal split vs. leave-one-out). RLVR must use consistent evaluation.
- **Implicit vs. explicit feedback**: MovieLens has explicit ratings; most real systems have only implicit signals (clicks, views, purchases). Implicit feedback is noisier -- not clicking an item does not mean disliking it.
- **Memorization risk**: The model may memorize user-item pairs from pretraining data that overlap with evaluation sets. Strict data decontamination is essential.
- **Filter bubble risk**: Optimizing for predicted user preference reinforces existing preferences and narrows exposure. This is ethically concerning in production systems.
- **Scale mismatch**: Real recommender systems handle millions of items. Benchmark tasks with 100 candidate items do not capture the full difficulty.

## Connections

- [[semantic-textual-similarity]] — Content-based recommendation relies on computing item similarity
- [[emotion-recognition]] — Sentiment from reviews informs preference prediction
- [[reading-comprehension]] — Understanding review text requires comprehension
- [[information-retrieval]] — Recommendation is a form of personalized information retrieval
