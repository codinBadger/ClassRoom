# ClassRoom
A complete application for customized learning. This includes:
- Document Storage
- Flash Card Tool
- **Multiple Choice Questionnaire Backend (C++)** ✅
- **Modern Web Frontend (React + TypeScript)** ✅
- Compiler support for multiple languages

## Questionnaire System

A full-stack questionnaire system with a high-performance C++ backend and modern React frontend.

### Backend (C++)
- Thread-safe session management
- Multiple question types (Multiple Choice, True/False, Short Answer, Essay)
- Real-time scoring and validation
- **Load exams from JSON files** 📄 ✅
- **Automatic results display after completion** ✅
- Handles 100+ concurrent students (tested with 100 threads in ~300ms)
- Zero external dependencies (C++17 standard library only)

### Frontend (React + TypeScript)
- Modern, responsive web interface
- Upload and take exams via JSON files
- Real-time progress tracking
- Interactive question navigation
- Detailed results with analytics
- Built with React 18, TypeScript, and Vite

### Quick Start

#### Backend (C++)
```bash
# Build the project
mkdir build && cd build
cmake ..
make

# Run tests
./tests/test_questionnaire

# Run demo with 100 students
./questionnaire_backend

# Load and run an exam from JSON
./json_exam_demo sample_exam.json
```

#### Frontend (React)
```bash
# Install dependencies
cd frontend
npm install

# Start development server
npm run dev
# Open http://localhost:3000

# Build for production
npm run build
```

For JSON exam format details, see [JSON_FORMAT.md](JSON_FORMAT.md)

For backend documentation, see [QUESTIONNAIRE_README.md](QUESTIONNAIRE_README.md)

For frontend documentation, see [frontend/README.md](frontend/README.md)

### Architecture
**Backend:**
- **Question**: Individual questions with various types
- **Questionnaire**: Collection of questions
- **StudentSession**: Tracks student attempts
- **SessionManager**: Thread-safe central manager
- **QuestionnaireLoader**: JSON file parser and results display

**Frontend:**
- **React Components**: QuestionCard, ResultsView, App
- **TypeScript Types**: Type-safe data models
- **API Service**: Backend communication layer
- **Vite**: Fast development and build tool

The backend uses smart pointers for memory management and mutex locks for thread safety, making it suitable for production use with hundreds of concurrent users.

The frontend provides a modern, intuitive interface for students to take exams and view results in real-time.

