# Research & Technical Decisions: Exam Domains

## Topic 1: Calculating Domain Question Counts without Fractional Skeletons

- **Decision**: Use the Largest Remainder Method (Hare-Niemeyer method) algorithm to allocate integer counts of questions to domains based on exact percentages, ensuring the final question count perfectly matches the requested exam total.
- **Rationale**: The official percentage allocations (28%, 26%, 22%, 24%) when applied to an arbitrary number of questions (e.g., 65 questions) yield fractional amounts (18.2, 16.9, 14.3, 15.6). Simple `round()` functions might cause the total sum to be 64 or 66 questions. The Largest Remainder Method is the mathematical standard for proportionally allocating integer seats and guarantees exactly N questions will be picked without crashing or skewing allocations.
- **Alternatives considered**: 
  - Floor rounding with arbitrary padding (creates non-deterministic gaps).
  - Ceil rounding with truncating (could accidentally trim questions from the wrong group).

## Topic 2: Schema Migration for Session History

- **Decision**: Store a single new JSON string column in the `session_history` table named `domain_stats` that maps the domain IDs to the correct/total questions (e.g., `{"1": {"correct": 12, "total": 18}}`). 
- **Rationale**: This approach avoids having to alter the SQLite table to append 8 separate columns (4 domains * correct/total). It maintains lightweight storage in the DB and uses `serde_json` serialization directly out of a `HashMap<u8, DomainStat>` struct already defined for the `TestSession` in `models.rs`.
- **Alternatives considered**: 
  - Adding 8 new explicitly typed columns: `domain_1_correct`, `domain_1_total`, etc. Rejected because it's less concise and harder to maintain if AWS changes the number of domains in future MLA-C02 exams.

## Topic 3: Shuffling Missing Domain Constraints

- **Decision**: In the Edge Case where a Question Bank has insufficient items in a specific domain to meet the mathematical quota, the app will greedily fill the remaining deficit using unallocated questions from other domains (resampling randomly from available remaining pool).
- **Rationale**: A user with a custom Bank of 100 domain-1-only questions who requests a 65 question exam should just get 65 domain-1 questions without crashing. Availability trumps strict proportion if the bank is fundamentally skewed.
- **Alternatives considered**: Panic/Crash (Rejected: bad UX). Show fewer questions (Rejected: standard tests expect N questions).
