# ClassRoom - Self-Directed Learning Platform

A complete self-directed learning application where students can create and manage their own personal classroom and courses. Built with Rust backend and React frontend.

## Features

### Core Functionality
- **Personal Classrooms**: Create and manage multiple personal classrooms
- **Course Management**: Organize learning materials into structured courses
- **User Authentication**: Secure registration and login system

### Digital Library
- **Ebook Storage**: Upload and manage PDF, EPUB, and other document formats
- **File Management**: Organize learning materials with metadata

### Learning Tools
- **Flashcards**: Create, edit, and review flashcards for memorization
- **Questionnaires**: 
  - Multiple Choice Questions (MCQ)
  - True/False questions
  - Fill-in-the-blank questions
  - Automatic scoring and results

### Code Editor & Terminal
- **Integrated Code Editor**: Monaco Editor with syntax highlighting
- **Multi-Language Support**: 
  - Python
  - JavaScript/Node.js
  - C++
  - Rust
  - Java
- **Timed Coding Sessions**: Practice coding with time limits
- **Code Execution**: Run code securely in isolated environment
- **Session History**: Track all code submissions

### Notes System
- **Rich Text Notes**: Create and edit detailed notes for each course
- **Organization**: Notes are organized by course
- **Timestamps**: Track when notes were created and updated

### Progress Tracking
- **Activity Timeline**: View chronological history of all learning activities
- **Progress Statistics**: 
  - Total time spent on each course
  - Number of completed activities
  - Overall progress percentage
- **Discipline Metrics**:
  - Days active
  - Current learning streak
  - Average daily study time
  - Activity patterns

## Technology Stack

### Backend (Rust)
- **Framework**: Actix-web (high-performance web framework)
- **Database**: SQLite with sqlx (async SQL toolkit)
- **Authentication**: JWT tokens with bcrypt password hashing
- **API**: RESTful API with JSON responses

### Frontend (React)
- **Framework**: React 18 with Vite
- **Routing**: React Router v6
- **State Management**: React Context API
- **Code Editor**: Monaco Editor (VS Code's editor)
- **HTTP Client**: Axios
- **Styling**: CSS with inline styles

## Getting Started

### Prerequisites
- Rust (latest stable version)
- Node.js (v18 or higher)
- npm or yarn
- SQLite (included)
- Compilers for supported languages (optional, for code execution):
  - Python 3
  - Node.js
  - GCC (for C++)
  - rustc (for Rust)
  - Java JDK

### Backend Setup

1. Navigate to the backend directory:
```bash
cd backend
```

2. Create a `.env` file from the example:
```bash
cp .env.example .env
```

3. Update the `.env` file with secure values:
   - Generate a strong JWT_SECRET: `openssl rand -base64 32`
   - Set appropriate database path if needed

4. Build and run the backend:
```bash
cargo run --release
```

The backend will start on `http://localhost:8080`

**Security Note**: The code execution feature is for demonstration purposes only. In production, implement proper sandboxing using Docker, VMs, or specialized sandboxing solutions.

### Frontend Setup

1. Navigate to the frontend directory:
```bash
cd frontend
```

2. Install dependencies:
```bash
npm install
```

3. Start the development server:
```bash
npm run dev
```

The frontend will start on `http://localhost:5173`

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register new user
- `POST /api/auth/login` - Login user
- `GET /api/auth/profile` - Get user profile (authenticated)

### Classrooms
- `POST /api/classrooms` - Create classroom
- `GET /api/classrooms` - Get all classrooms
- `GET /api/classrooms/{id}` - Get classroom by ID
- `PUT /api/classrooms/{id}` - Update classroom
- `DELETE /api/classrooms/{id}` - Delete classroom

### Courses
- `POST /api/classrooms/{classroom_id}/courses` - Create course
- `GET /api/classrooms/{classroom_id}/courses` - Get all courses
- `GET /api/classrooms/{classroom_id}/courses/{course_id}` - Get course
- `PUT /api/classrooms/{classroom_id}/courses/{course_id}` - Update course
- `DELETE /api/classrooms/{classroom_id}/courses/{course_id}` - Delete course

### Flashcards
- `POST /api/courses/{course_id}/flashcards` - Create flashcard
- `GET /api/courses/{course_id}/flashcards` - Get all flashcards
- `PUT /api/courses/{course_id}/flashcards/{id}` - Update flashcard
- `DELETE /api/courses/{course_id}/flashcards/{id}` - Delete flashcard

### Questionnaires
- `POST /api/courses/{course_id}/questionnaires` - Create questionnaire
- `GET /api/courses/{course_id}/questionnaires` - Get all questionnaires
- `POST /api/courses/{course_id}/questionnaires/{id}/questions` - Add question
- `GET /api/courses/{course_id}/questionnaires/{id}/questions` - Get questions
- `POST /api/courses/{course_id}/questionnaires/{id}/submit` - Submit answers

### Notes
- `POST /api/courses/{course_id}/notes` - Create note
- `GET /api/courses/{course_id}/notes` - Get all notes
- `PUT /api/courses/{course_id}/notes/{id}` - Update note
- `DELETE /api/courses/{course_id}/notes/{id}` - Delete note

### Code Execution
- `POST /api/courses/{course_id}/code/execute` - Execute code
- `POST /api/courses/{course_id}/code/sessions` - Create timed session
- `GET /api/courses/{course_id}/code/sessions` - Get session history

### Progress Tracking
- `POST /api/courses/{course_id}/progress` - Track activity
- `GET /api/courses/{course_id}/progress/stats` - Get progress statistics
- `GET /api/courses/{course_id}/progress/timeline` - Get activity timeline
- `GET /api/courses/{course_id}/progress/metrics` - Get discipline metrics

## Database Schema

The application uses SQLite with the following main tables:
- `users` - User accounts
- `classrooms` - Personal classrooms
- `courses` - Courses within classrooms
- `ebooks` - Digital library items
- `flashcards` - Study flashcards
- `questionnaires` - Quiz/test containers
- `questions` - Individual quiz questions
- `notes` - Course notes
- `code_sessions` - Code execution history
- `progress` - Learning activity tracking

## Security Features

- Password hashing with bcrypt
- JWT-based authentication with environment-based secrets
- Protected API endpoints
- Input validation
- SQL injection prevention (using parameterized queries)
- CORS configuration

## Security Considerations

**IMPORTANT**: This application is provided as a demonstration and educational tool. Before deploying to production:

1. **Code Execution Sandboxing**: The code execution feature currently runs user code directly on the server. Implement proper sandboxing:
   - Use Docker containers with resource limits and network isolation
   - Consider specialized sandboxing solutions (gVisor, Firecracker, WebAssembly)
   - Implement time limits and memory constraints
   - Run code in isolated environments without network access

2. **JWT Secret**: Always use a strong, randomly generated JWT_SECRET in production (see `.env.example`)

3. **Database**: Consider using PostgreSQL or MySQL for production deployments

4. **File Uploads**: Implement proper file validation, size limits, and virus scanning for ebook uploads

5. **Rate Limiting**: Add rate limiting to prevent abuse of API endpoints

6. **HTTPS**: Always use HTTPS in production environments

## Future Enhancements

- Real-time collaboration features
- Video/audio learning materials
- Spaced repetition algorithm for flashcards
- Advanced analytics and insights
- Mobile application
- Integration with external learning platforms
- AI-powered study recommendations
- Peer review system
- Achievement badges and gamification

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

