# HoosHungry
### Low-Latency Dining Intelligence Platform

HoosHungry is a systems-focused dining intelligence platform that aggregates, normalizes, and ranks dining options using heterogeneous open-source datasets. The project emphasizes **performance, correctness, and scalability**, leveraging a **Rust-based backend**, **GraphQL APIs**, **PostgreSQL**, **Redis**, and a **C++ client** to simulate production-grade, low-latency service design.

---

## Project Goals
- Build a **high-performance backend service** with predictable latency and strong memory safety guarantees  
- Design a **flexible data access layer** that supports complex filtering without over-fetching  
- Normalize **messy, real-world open-source data** into a structured, queryable system  
- Implement **deterministic ranking algorithms** with tunable parameters and correctness guarantees  
- Measure and document **performance tradeoffs** under concurrent load  

---

## System Architecture

External Data Sources
(OpenStreetMap, Open Food Facts)
↓
Rust Ingestion Workers
(Data cleaning & normalization)
↓
PostgreSQL (source of truth)
↓
Redis (hot-path caching)
↓
Rust GraphQL API (async)
↓
C++ Client Application


---

## Data Sources
- **OpenStreetMap (Overpass API):** dining locations, opening hours, cuisine tags  
- **Open Food Facts:** nutritional information, allergens, dietary classifications  

External data is ingested asynchronously, validated, and transformed into normalized relational schemas before being persisted.

---

## Backend Implementation
- Written in **Rust** using async I/O to support high concurrency with strong memory safety  
- **GraphQL API** enables client-driven queries, reducing over-fetching compared to REST  
- **PostgreSQL** schemas designed with indexing strategies to support multi-dimensional filtering  
- **Redis** used for caching frequently accessed queries and mitigating database load  
- Structured logging and explicit error handling ensure observability and debuggability  

---

## Ranking & Algorithms
- Deterministic scoring functions combining:
  - dietary compatibility  
  - availability and opening hours  
  - popularity proxies  
- Feature normalization and bounded scoring ensure stability under small input perturbations  
- Tunable weights allow sensitivity analysis and controlled experimentation  

---

## Client Application
- Lightweight **C++ client** consuming GraphQL endpoints  
- Emphasizes explicit memory ownership, minimal allocations, and predictable performance  
- Displays ranked dining results based on user-defined preferences  

---

## Correctness & Testing
- Unit tests for ranking logic and query correctness  
- Property-based tests validating ordering invariants and score monotonicity  
- Integration tests across ingestion, storage, caching, and API layers  

---

## Performance & Benchmarking
- Load tested under simulated concurrent traffic  
- Measured p50 / p95 latency and cache hit ratios  
- Documented performance bottlenecks and optimization tradeoffs  

---

## Deployment & Tooling
- Services containerized using Docker  
- CI/CD pipelines implemented via GitHub Actions for automated builds and testing  
- Deployed to a cloud environment with reproducible infrastructure  

---

## Key Engineering Takeaways
- Designing around **imperfect real-world data** requires robust ingestion and normalization layers  
- Performance improvements often come from **data access patterns and caching**, not just faster code  
- Deterministic algorithms and explicit tradeoffs are critical in low-latency systems  

---

## Tech Stack
- **Languages:** Rust, C++  
- **API Layer:** GraphQL  
- **Database:** PostgreSQL  
- **Caching:** Redis  
- **Data Sources:** OpenStreetMap, Open Food Facts  
- **Tooling:** Docker, GitHub Actions, k6  

---

## Future Work
- GraphQL subscriptions for real-time updates  
- More advanced ranking models with learned weights  
- Expanded ingestion support for additional open datasets  

---

## License
MIT License
