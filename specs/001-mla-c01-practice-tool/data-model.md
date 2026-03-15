# Data Model: MLA C01 Practice Tool

## Entities

### `QuestionBank`
Collection of questions parsed from external or internal JSON.

*   `id`: String
*   `version`: String
*   `questions`: List<Question>

### `Question`
A single practice item.

*   `id`: String (UUID or predefined identifier)
*   `prompt`: String (The text of the question)
*   `choices`: List<String>
*   `correct_answer_indices`: List<u8> (Indices referencing correct items in choices)
*   `explanation`: String (Educational context explaining the correct answer)

### `SessionType` (Enum)
*   `Practice`
*   `Exam`

### `TestSession`
Represents the in-memory state of an ongoing session.

*   `id`: UUID
*   `mode`: SessionType
*   `questions`: List<Question>
*   `user_answers`: Map<String, List<u8>> (Question ID -> User selections)
*   `time_elapsed`: u64 (seconds)
*   `is_completed`: bool

## SQLite Schema (Persistence)
Used to satisfy Progress Tracking (User Story 4) and Suspend/Resume (FR-010).

```sql
CREATE TABLE session_history (
    id TEXT PRIMARY KEY,
    session_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    mode TEXT NOT NULL,           -- 'Practice' or 'Exam'
    score_percentage REAL,        -- e.g., 85.5
    passed BOOLEAN,               -- e.g., TRUE if >= 72.0 in Exam mode
    total_questions INTEGER,
    correct_answers INTEGER
);

CREATE TABLE active_sessions (
    id TEXT PRIMARY KEY,          -- Fixed ID like 'CURRENT_SESSION' or a UUID
    last_saved DATETIME DEFAULT CURRENT_TIMESTAMP,
    mode TEXT NOT NULL,
    session_data TEXT NOT NULL    -- JSON serialized TestSession state
);
```
