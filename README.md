# ClassRoom
A complete application for customized learning. This includes:
- Document Storage
- Flash Card Tool
- **Multiple Choice Questionnaire Backend (C++)** ✅
- Compiler support for multiple languages

## Questionnaire Backend

A high-performance C++ backend for managing questionnaires with support for 100+ concurrent students per session.

### Features
- Thread-safe session management
- Multiple question types (Multiple Choice, True/False, Short Answer, Essay)
- Real-time scoring and validation
- Handles 100+ concurrent students (tested with 100 threads in ~300ms)
- Zero external dependencies (C++17 standard library only)

### Quick Start

```bash
# Build the project
mkdir build && cd build
cmake ..
make

# Run tests
./tests/test_questionnaire

# Run demo with 100 students
./questionnaire_backend
```

For detailed documentation, see [QUESTIONNAIRE_README.md](QUESTIONNAIRE_README.md)

### Architecture
- **Question**: Individual questions with various types
- **Questionnaire**: Collection of questions
- **StudentSession**: Tracks student attempts
- **SessionManager**: Thread-safe central manager

The backend uses smart pointers for memory management and mutex locks for thread safety, making it suitable for production use with hundreds of concurrent users.

