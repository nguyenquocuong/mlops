# Data Model: Exam Domains

## Entities

### `QuestionBank`
(Modified: Strict JSON validation requires every question to have a domain property).

*   `id`: String
*   `version`: String
*   `questions`: List<Question>

### `Question`
A single practice item.

*   `id`: String (UUID or predefined identifier)
*   **`domain`: u8 (Integer 1-4. MUST exist. Application crashes on load if missing)**
*   `prompt`: String (The text of the question)
*   `choices`: List<String>
*   `correct_answer_indices`: List<u8>
*   `explanation`: String

### `DomainStat`
In-memory structure for tallying performance per domain.

*   `correct`: u16
*   `total`: u16

### `TestSession`
Represents the in-memory state of an ongoing session.

*   `id`: UUID
*   `mode`: SessionType
*   `questions`: List<Question>
*   `user_answers`: Map<String, List<u8>>
*   `time_elapsed`: u64
*   `is_completed`: bool
*   **`domain_stats`: Map<u8, DomainStat> (Calculated at the end of the session, mapping Domain ID 1-4 to their corresponding performance.**

## SQLite Schema (Persistence)

```sql
-- Migration required for existing DBs!
ALTER TABLE session_history ADD COLUMN domain_stats TEXT DEFAULT '{}';
-- Stores the JSON serialized TestSession::domain_stats map.

-- `active_sessions` already stores all session state as JSON in `session_data` so it automatically inherits domain changes inside TestSession via Serialization.
```
