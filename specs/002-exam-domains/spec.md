# Feature Specification: Exam Domains and Weightings

**Feature Branch**: `002-exam-domains`  
**Created**: 2026-03-15  
**Status**: Draft  
**Input**: User description: "Read exam guide for domains and weightings. The exam has the following content domains and weightings: Domain 1: Data Preparation for Machine Learning (ML) (28% of scored content), Domain 2: ML Model Development (26% of scored content), Domain 3: Deployment and Orchestration of ML Workflows (22% of scored content), Domain 4: ML Solution Monitoring, Maintenance, and Security (24% of scored content)"

## Clarifications

### Session 2026-03-15
- Q: How should the domain categorization be formally represented in the JSON question bank schema and the internal application models? → A: An integer (`1`, `2`, `3`, or `4`) corresponding to the official domains.
- Q: How should the application handle questions parsed from a JSON bank that are missing the new domain field? → A: Throw a clear validation error and refuse to start the application with that bank.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Officially Weighted Exam Generation (Priority: P1)

As a user taking a simulated exam, I want the generated exam to proportionally match the official AWS MLA-C01 domain weightings, so that the experience mirrors the real test.

**Why this priority**: Simulating the actual exam structure is the core value proposition of an exam practice tool.

**Independent Test**: Can be independently tested by starting an exam and verifying that the count of questions from each domain matches the percentage defined in the official guide (e.g., 28% from Domain 1).

**Acceptance Scenarios**:

1. **Given** a user starts a new Exam mode session, **When** the system generates the test, **Then** the proportion of questions selected from each domain matches the official weightings (Domain 1: 28%, Domain 2: 26%, Domain 3: 22%, Domain 4: 24%).
2. **Given** a question bank does not have enough questions in a specific domain to meet the required weighting, **When** the exam is generated, **Then** the system should gracefully fall back to selecting available questions to reach the total question count without crashing, and notify the user of the skew.

---

### User Story 2 - Domain-Specific Performance Tracking (Priority: P2)

As a user reviewing my exam results, I want to see a breakdown of my score by exam domain, so I can identify which specific areas require more study.

**Why this priority**: Targeted studying is highly effective. Knowing overall pass/fail is good (P1), but knowing *why* (weakness in Domain 3) provides actionable value to the user.

**Independent Test**: Can be tested by completing an exam and verifying that the final summary screen displays the percentage correct for each individual domain.

**Acceptance Scenarios**:

1. **Given** a user has just finished an exam, **When** the results summary is displayed, **Then** the screen shows the user's score percentage broken down by Domain 1, Domain 2, Domain 3, and Domain 4.
2. **Given** a user reviews their historical past sessions, **When** they view the details of a past exam, **Then** they can see their domain-by-domain performance for that historical exam.

---

### User Story 3 - Domain-Targeted Practice (Priority: P3)

As a user in practice mode, I want the ability to focus my practice session exclusively on a single chosen domain, so I can drill into my weakest topic.

**Why this priority**: Useful for targeted studying, but not strictly required to simulate the primary exam experience.

**Independent Test**: Can be tested by starting a practice session, selecting a specific domain, and verifying that all presented questions belong strictly to that chosen domain.

**Acceptance Scenarios**:

1. **Given** a user starts a new Practice mode session, **When** they are prompted for settings, **Then** they can choose to filter questions by a specific domain (e.g., "Domain 1: Data Preparation").
2. **Given** a user has chosen a specific domain filter, **When** the practice session begins, **Then** 100% of the questions presented belong to the selected domain.

### Edge Cases

- What happens when calculating the exact number of questions per domain results in fractions (e.g., 28% of 65 is 18.2)? (Assumption: System will use standard rounding while ensuring the final question count exactly matches the target total).
- What happens if the imported question bank is completely missing questions for one or more domains? (Assumption: System will fill the missing quota with questions from other domains to meet the required total).
- What happens if questions parsed from a JSON bank are missing the new domain field? The system MUST throw a strict validation error and refuse to initialize the application using that invalid bank.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST categorize every question into one of the four official exam domains.
- **FR-002**: System MUST generate Exam mode sessions such that questions strictly adhere to the following weightings: Domain 1 (28%), Domain 2 (26%), Domain 3 (22%), and Domain 4 (24%).
- **FR-003**: System MUST calculate an exact number of questions per domain based on the total exam question count, using rounding logic that ensures the sum always equals the exact requested total.
- **FR-004**: System MUST calculate and display the user's correct answer percentage for each individual domain at the end of an Exam session.
- **FR-005**: System MUST store the domain-by-domain performance breakdown in the session history records for targeted review.
- **FR-006**: System MUST allow users to optionally restrict a Practice mode session to a single selected domain.

### Non-Functional Requirements (Constitution Alignment)

- **UX Consistency**: Domain performance breakdowns should be clearly formatted on screen, conforming to existing styles without overwhelming the user.
- **Performance**: The selection and shuffling of questions based on domain allocations must remain practically instantaneous (under 500ms).

### Key Entities

- **Question**: Extended to inherently belong to a specific exam Domain (represented as an integer 1-4).
- **Session Results**: Extended to record the aggregated performance (correct vs total) explicitly grouped by Domain.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Generated exams contain exactly the required proportion of questions per domain (e.g., for a 65 question test: 18 Domain 1, 17 Domain 2, 14 Domain 3, 16 Domain 4).
- **SC-002**: Users can accurately identify their weakest domain performance purely by looking at post-exam analytics.
- **SC-003**: System calculates exam compositions with correct weightings in under 500 milliseconds.
