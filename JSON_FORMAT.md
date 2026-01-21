# JSON Exam Format Documentation

## Overview

The questionnaire backend supports loading exams from JSON files. This allows teachers to create and manage exams using a simple text format that can be version-controlled and easily shared.

## JSON File Format

### Basic Structure

```json
{
  "title": "Exam Title",
  "description": "Exam description",
  "timeLimit": 30,
  "numQuestions": 5,
  "questions": [
    // Array of question objects
  ]
}
```

### Root Level Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `title` | string | Yes | The title of the exam/questionnaire |
| `description` | string | Yes | A description of what the exam covers |
| `timeLimit` | integer | Yes | Time limit in minutes (0 for unlimited) |
| `numQuestions` | integer | No | Number of questions (informational) |
| `questions` | array | Yes | Array of question objects |

### Question Object Structure

```json
{
  "questionText": "What is the question?",
  "questionType": "MULTIPLE_CHOICE",
  "options": ["Option A", "Option B", "Option C", "Option D"],
  "correctAnswer": "Option B",
  "points": 1
}
```

### Question Fields

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `questionText` | string | Yes | The question text |
| `questionType` | string | Yes | Type of question (see below) |
| `options` | array | Conditional | Answer options (required for MULTIPLE_CHOICE and TRUE_FALSE) |
| `correctAnswer` | string | Yes | The correct answer |
| `points` | integer | No | Points for this question (default: 1) |

### Question Types

The following question types are supported:

1. **MULTIPLE_CHOICE** or **MCQ**
   - Requires `options` array
   - Student selects one answer from multiple options
   
2. **TRUE_FALSE** or **BOOL**
   - Requires `options` array with ["True", "False"]
   - Boolean questions
   
3. **SHORT_ANSWER** or **SHORT**
   - No options needed
   - Case-insensitive text matching
   
4. **ESSAY**
   - No options needed
   - For long-form answers

## Complete Example

```json
{
  "title": "C++ Programming Quiz",
  "description": "A comprehensive quiz on C++ fundamentals",
  "timeLimit": 30,
  "numQuestions": 5,
  "questions": [
    {
      "questionText": "What is the size of an int in most modern C++ compilers?",
      "questionType": "MULTIPLE_CHOICE",
      "options": ["2 bytes", "4 bytes", "8 bytes", "16 bytes"],
      "correctAnswer": "4 bytes",
      "points": 1
    },
    {
      "questionText": "Is C++ an object-oriented programming language?",
      "questionType": "TRUE_FALSE",
      "options": ["True", "False"],
      "correctAnswer": "True",
      "points": 1
    },
    {
      "questionText": "What does STL stand for?",
      "questionType": "SHORT_ANSWER",
      "correctAnswer": "Standard Template Library",
      "points": 2
    },
    {
      "questionText": "Which operator is used for dynamic memory allocation in C++?",
      "questionType": "MULTIPLE_CHOICE",
      "options": ["malloc", "new", "alloc", "create"],
      "correctAnswer": "new",
      "points": 1
    },
    {
      "questionText": "Does C++ support multiple inheritance?",
      "questionType": "TRUE_FALSE",
      "options": ["True", "False"],
      "correctAnswer": "True",
      "points": 1
    }
  ]
}
```

## Usage

### Loading an Exam from JSON

```cpp
#include "SessionManager.h"
#include "QuestionnaireLoader.h"

SessionManager manager;

// Load questionnaire from JSON file
int questionnaireId = QuestionnaireLoader::loadFromJsonFile(manager, "exam.json");

if (questionnaireId == -1) {
    std::cerr << "Failed to load exam!" << std::endl;
    return 1;
}

std::cout << "Exam loaded successfully!" << std::endl;
```

### Taking an Exam

```cpp
// Start a student session
int sessionId = manager.startSession(studentId, "Student Name", questionnaireId);

// Submit answers
manager.submitAnswer(sessionId, questionId, "answer");

// Complete the exam
manager.completeSession(sessionId);
```

### Displaying Results

```cpp
// Display detailed results after exam completion
QuestionnaireLoader::displayResults(manager, sessionId);
```

This will show:
- Student information
- Each question with the student's answer
- Correct/incorrect status
- Points earned per question
- Final score and percentage
- Time taken

## Running the Demo

```bash
# Build the project
mkdir build && cd build
cmake ..
make

# Run with sample exam
./json_exam_demo sample_exam.json

# Or with your own JSON file
./json_exam_demo path/to/your/exam.json
```

## Features

- ✅ Load complete exams from JSON files
- ✅ Support for multiple question types
- ✅ Flexible scoring system
- ✅ Time limits per exam
- ✅ Detailed results display
- ✅ Thread-safe operations
- ✅ No external dependencies (C++17 only)

## Notes

- JSON parsing is done with a simple custom parser (no external dependencies)
- Field names are case-sensitive
- The parser is tolerant of extra whitespace and trailing commas
- For SHORT_ANSWER questions, matching is case-insensitive
- Questions are identified by their position (auto-assigned IDs starting from 1)
