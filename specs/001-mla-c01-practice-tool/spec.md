# Feature Specification: MLA C01 Practice Tool

**Feature Branch**: `001-mla-c01-practice-tool`  
**Created**: 2026-03-15  
**Status**: Draft  
**Input**: User description: "Develop terminal user-interface tool that help users do practice tests for AWS Certified Machine Learning Engineer - Associate Cert (MLA-C01). This is MLA-C01 exam guide. The tool will load the questions bank from a file and embedded into a single binary. Practice tests are learning tools that help users practice answering questions in preparation to take a standardized exam. Users can take the practice test as many times as they like in various modes. Practice test has two modes are Practice Mode and Exam Mode. Practice Mode - Practice answering questions in a low-stakes environment and review each answer before moving on to the next question. Exam Mode - Simulate a standardized test by completing your practice test within the time limit and achieving a passing score."

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Practice Mode Experience (Priority: P1)

Users need to practice answering questions and immediately learn from their mistakes to build their knowledge base iteratively without the stress of a time limit.

**Why this priority**: It is the core study feature needed by users when preparing for the exam.

**Independent Test**: Can be tested by selecting "Practice Mode", answering a question, and confirming that the correct answer and a detailed explanation are immediately displayed before the next question is shown.

**Acceptance Scenarios**:

1. **Given** the user is on the main menu, **When** they select Practice Mode, **Then** the first question is displayed without a countdown timer.
2. **Given** a question is displayed in Practice Mode, **When** the user submits their answer, **Then** the system immediately displays whether the answer was correct, along with the detailed explanation.
3. **Given** the user has reviewed the explanation, **When** they choose to proceed, **Then** the next question in the bank is presented.

---

### User Story 2 - Exam Mode Simulation (Priority: P1)

Users need to simulate the actual exam environment to test their readiness under realistic constraints, including time pressure and lack of immediate feedback.

**Why this priority**: Essential to validate whether the user is ready for the real standardized exam.

**Independent Test**: Can be tested by selecting "Exam Mode", completing the exam or letting the timer run out, and verifying that a final pass/fail score is calculated and displayed without showing per-question feedback during the exam.

**Acceptance Scenarios**:

1. **Given** the user is on the main menu, **When** they select Exam Mode, **Then** the first question is displayed along with a running countdown timer.
2. **Given** the user answers a question in Exam Mode, **When** they submit the answer, **Then** the system immediately moves to the next question without revealing if the answer was correct.
3. **Given** the user completes all questions or the timer expires, **When** the exam concludes, **Then** the system displays the final score and whether they achieved a passing score.

---

### User Story 3 - Standalone Binary Distribution (Priority: P2)

Users need an application that requires zero setup or external dependencies, allowing them to download a single file and immediately start practicing.

**Why this priority**: Removes friction and technical barriers for users who just want to study.

**Independent Test**: Can be tested by running the compiled executable without any other files present in the directory, and verifying that the default question bank loads successfully.

**Acceptance Scenarios**:

1. **Given** the user downloads the standalone binary, **When** they execute it in an empty directory, **Then** the TUI launches successfully with the embedded question bank.
2. **Given** the user has an updated custom question bank file, **When** they launch the binary with a command-line argument specifying the file path, **Then** the TUI overrides the embedded bank and loads the custom file.

### User Story 4 - Progress Tracking (Priority: P2)

Users need to review their past practice and exam sessions to track their improvement over time and identify weak areas.

**Why this priority**: Helps users measure their readiness and focus their studies.

**Independent Test**: Can be tested by completing a practice or exam session, navigating to the "History" menu, and verifying that the session's score and timestamp are correctly displayed.

**Acceptance Scenarios**:

1. **Given** the user has completed at least one session, **When** they navigate to the History view from the main menu, **Then** a list of past sessions (timestamp, mode, score) is displayed.
2. **Given** the user is viewing their history, **When** they reset their history, **Then** all recorded sessions are removed and the history view is empty.

### Edge Cases

- What happens when the user resizes the terminal window while a long question is displayed? (The text should wrap gracefully).
- How does the system handle an externally supplied question bank file that is malformed or missing required fields? (Should display a clear error message and exit or fallback to embedded bank).
- What happens if the embedded database file containing the saved session is tampered with between launches? (Should detect invalid state, discard the save, and fall back to the main menu).

## Clarifications

### Session 2026-03-15
- Q: Question Bank Randomization → A: Randomly pull a subset (e.g., 65 questions) from the bank and shuffle their order.
- Q: Suspend and Resume → A: Soft persistence; save the active session state on exit and prompt to resume it upon next launch.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide a Terminal User Interface (TUI) with a main menu to select between multiple modes.
- **FR-002**: System MUST embed a default question bank directly into the single compiled executable.
- **FR-003**: System MUST offer a Practice Mode where users receive immediate feedback (correct/incorrect and explanation) after each attempted question.
- **FR-004**: System MUST offer an Exam Mode that enforces an overall timer and prevents immediate feedback.
- **FR-005**: System MUST calculate and display a final score (Pass/Fail based on a predefined threshold, e.g., 72% for AWS) at the conclusion of Exam Mode.
- **FR-006**: System MUST allow users to override the embedded question bank by providing a path to an external file.
- **FR-007**: System MUST parse the question bank file format (JSON by default) handling multiple-choice and multiple-response questions.
- **FR-008**: System MUST persist session history (timestamp, mode, score) locally across sessions and provide a history viewer screen within the TUI to track progress.
- **FR-009**: System MUST randomly pull a subset (e.g., 65 questions) from the selected bank and shuffle their order to initiate a new session.
- **FR-010**: System MUST save the active session state upon exit and prompt the user to resume it upon the next launch.

### Non-Functional Requirements (Constitution Alignment)

- **Performance**: The TUI MUST launch in under 500ms and navigate between questions instantly without perceptible lag.
- **UX Consistency**: The terminal styling (colors, navigation keys) MUST be consistent and accessible, utilizing standard keybindings (arrows for navigation, Enter to select).
- **Code Quality**: The application architecture MUST separate the core test logic, the data models, and the UI representation to ensure high testability.

### Key Entities *(include if feature involves data)*

- **QuestionBank**: A collection of Questions along with test metadata (e.g., passing score percentage, time limit).
- **Question**: Represents a single exam question, including the prompt, available choices, the correct answer(s), and an educational explanation.
- **TestSession**: Tracks the ongoing state of a user's practice or exam session, including the current timer, provided answers, and navigation state.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can launch the application and start a practice test in under 3 seconds with zero configuration.
- **SC-002**: The application is distributed as exactly one executable file per platform (Windows/macOS/Linux).
- **SC-003**: The terminal interface accurately renders questions up to 1000 characters without clipping or crashing.
- **SC-004**: Users can complete a full 65-question simulated exam in Exam Mode without application degradation or memory leaks.
