# ClassRoom Questionnaire Backend

A high-performance C++ backend for managing questionnaires with support for hundreds of concurrent students.

## Features

- **Thread-safe session management**: Handles 100+ concurrent student sessions
- **Flexible question types**: Multiple choice, true/false, short answer, and essay questions
- **Real-time scoring**: Automatic answer validation and score calculation
- **Session tracking**: Monitor active and completed sessions with statistics
- **Time management**: Optional time limits for questionnaires
- **Concurrent access**: Mutex-protected data structures for multi-threaded environments

## Architecture

### Core Components

1. **Question**: Represents individual questions with various types and scoring
2. **Questionnaire**: Collection of questions with metadata
3. **StudentSession**: Tracks individual student attempts with answers and timing
4. **SessionManager**: Central manager for all sessions and questionnaires (thread-safe)

### Design Highlights

- **Smart pointers**: Uses `std::shared_ptr` for automatic memory management
- **Thread safety**: Mutex locks ensure safe concurrent access
- **Scalability**: Designed to handle hundreds of simultaneous sessions
- **Standard C++17**: No external dependencies required

## Building the Project

### Prerequisites

- CMake 3.10 or higher
- C++17 compatible compiler (GCC 7+, Clang 5+, MSVC 2017+)
- pthread library (usually included on Unix systems)

### Build Instructions

```bash
# Create build directory
mkdir build && cd build

# Configure with CMake
cmake ..

# Build
make

# Run tests
make test

# Or run test executable directly
./tests/test_questionnaire

# Run demo application
./questionnaire_backend
```

## Usage Example

```cpp
#include "SessionManager.h"

int main() {
    SessionManager manager;
    
    // Create a questionnaire
    int qId = manager.createQuestionnaire(
        "C++ Programming Basics",
        "A quiz on C++ fundamentals",
        30  // 30 minutes time limit
    );
    
    // Add questions
    auto q1 = std::make_shared<Question>(
        1, 
        "What is the size of int in C++?", 
        QuestionType::MULTIPLE_CHOICE, 
        1
    );
    q1->addOption("2 bytes");
    q1->addOption("4 bytes");
    q1->addOption("8 bytes");
    q1->setCorrectAnswer("4 bytes");
    manager.addQuestionToQuestionnaire(qId, q1);
    
    // Start a student session
    int sessionId = manager.startSession(101, "John Doe", qId);
    
    // Submit answers
    manager.submitAnswer(sessionId, 1, "4 bytes");
    
    // Complete and score the session
    manager.completeSession(sessionId);
    
    // Get session results
    auto session = manager.getSession(sessionId);
    std::cout << "Score: " << session->getScore() << std::endl;
    
    return 0;
}
```

## Performance

The system is designed to handle 100+ concurrent students:

- **Concurrent sessions**: Thread-safe operations with minimal locking
- **Memory efficient**: Smart pointer-based memory management
- **Fast lookups**: Hash map-based session and questionnaire storage
- **Scalable**: Tested with 100 concurrent threads

## API Overview

### SessionManager

- `createQuestionnaire()`: Create a new questionnaire
- `addQuestionToQuestionnaire()`: Add questions to a questionnaire
- `startSession()`: Start a new student session
- `submitAnswer()`: Submit an answer for a question
- `completeSession()`: Complete and score a session
- `getSession()`: Retrieve session details
- `getActiveSessionCount()`: Get count of active sessions
- `getAverageScore()`: Calculate average score for a questionnaire

### Question Types

- `MULTIPLE_CHOICE`: Questions with predefined options
- `TRUE_FALSE`: Boolean questions
- `SHORT_ANSWER`: Text-based answers with case-insensitive matching
- `ESSAY`: Long-form text answers

## Testing

The project includes comprehensive unit tests covering:

- Basic questionnaire creation
- Question addition and retrieval
- Student session lifecycle
- Concurrent access (50+ threads)
- Answer validation
- Statistical calculations

Run tests with:
```bash
cd build
make test
```

## Thread Safety

All public methods of `SessionManager` are thread-safe through mutex protection. The system can safely handle:

- Multiple questionnaires being created simultaneously
- Hundreds of students starting sessions concurrently
- Concurrent answer submissions
- Simultaneous session completions

## Future Enhancements

Possible extensions:
- REST API endpoint integration
- Database persistence
- Real-time notifications
- Advanced analytics
- Question randomization
- Partial credit scoring
- Session resume capability

## License

This project is part of the ClassRoom learning platform.
