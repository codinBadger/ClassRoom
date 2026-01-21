# Dual Frontend System with Firebase Firestore

## Overview

The ClassRoom Questionnaire System now includes two separate frontends with Firebase Firestore integration:

1. **Student Frontend** (Port 3000) - For taking exams and managing question banks
2. **Teacher/Admin Frontend** (Port 3001) - For creating and managing exams

## Features

### Student Frontend

**Main Features:**
- User login with ID
- Take new exams from JSON files
- Save questions to personal question bank
- Save entire exams for later retaking
- Create custom exams from saved questions
- Filter questions by type, topic, difficulty
- Retake saved exams

**Question Bank:**
- Save individual questions from any exam
- Organize by topic and difficulty
- Filter and search capabilities
- Build custom practice exams

**Saved Exams:**
- Save complete exam templates
- Retake exams as originally designed
- Track previous attempts

### Teacher/Admin Frontend

**Main Features:**
- Create new questionnaires from scratch
- Manage question libraries
- Organize questions by subject/topic
- Set difficulty levels
- Export exams as JSON
- View student statistics (when integrated with backend)

## Technology Stack

**Frontend:**
- React 18
- TypeScript
- Vite
- Firebase SDK 10.7.1
- Modern CSS3

**Database:**
- Firebase Firestore (NoSQL)
- Real-time synchronization
- Scalable cloud storage

## Setup Instructions

### 1. Firebase Project Setup

1. Go to [Firebase Console](https://console.firebase.google.com/)
2. Create a new project
3. Enable Firestore Database
4. Get your Firebase configuration:
   - Go to Project Settings
   - Under "Your apps", add a web app
   - Copy the configuration object

### 2. Configure Firebase Credentials

Update the Firebase configuration in both frontends:

**Student Frontend:**
Edit `frontend-student/src/services/firebase.ts`:
```typescript
const firebaseConfig = {
  apiKey: "YOUR_API_KEY",
  authDomain: "YOUR_AUTH_DOMAIN",
  projectId: "YOUR_PROJECT_ID",
  storageBucket: "YOUR_STORAGE_BUCKET",
  messagingSenderId: "YOUR_MESSAGING_SENDER_ID",
  appId: "YOUR_APP_ID"
};
```

**Teacher Frontend:**
Edit `frontend-teacher/src/services/firebase.ts` with the same configuration.

### 3. Firestore Security Rules

Set up security rules in Firebase Console:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    // Allow users to read/write their own questions
    match /questions/{questionId} {
      allow read, write: if request.auth != null && 
                          request.resource.data.userId == request.auth.uid;
    }
    
    // Allow users to read/write their own exams
    match /exams/{examId} {
      allow read, write: if request.auth != null && 
                          request.resource.data.userId == request.auth.uid;
    }
  }
}
```

### 4. Install Dependencies

**Student Frontend:**
```bash
cd frontend-student
npm install
```

**Teacher Frontend:**
```bash
cd frontend-teacher
npm install
```

### 5. Run the Applications

**Student Frontend (Port 3000):**
```bash
cd frontend-student
npm run dev
```

**Teacher Frontend (Port 3001):**
```bash
cd frontend-teacher
npm run dev
```

## Database Schema

### Questions Collection

```typescript
{
  id: string,              // Auto-generated
  questionText: string,
  type: QuestionType,      // MULTIPLE_CHOICE, TRUE_FALSE, SHORT_ANSWER, ESSAY
  options: string[],
  points: number,
  correctAnswer: string,
  topic?: string,
  difficulty?: 'easy' | 'medium' | 'hard',
  tags?: string[],
  userId: string,
  createdAt: Timestamp
}
```

### Exams Collection

```typescript
{
  id: string,              // Auto-generated
  title: string,
  description: string,
  questions: SavedQuestion[],
  timeLimit: number,
  userId: string,
  isTemplate: boolean,
  createdAt: Timestamp
}
```

## Usage Workflows

### Student Workflow

1. **Login** - Enter user ID
2. **Take Exam** - Upload JSON or select from saved exams
3. **During Exam:**
   - Answer questions
   - Navigate between questions
   - Submit when complete
4. **Save Options:**
   - Save individual questions to question bank
   - Save entire exam for retaking
5. **Custom Exam Creation:**
   - Browse question bank
   - Filter by type, topic, difficulty
   - Select questions
   - Create custom practice exam
6. **Retake Exams:**
   - Access saved exams
   - Retake as originally designed

### Teacher Workflow

1. **Login** - Enter admin credentials
2. **Create Questionnaire:**
   - Set title, description, time limit
   - Add questions one by one
   - Set question types and points
3. **Manage Question Library:**
   - View all created questions
   - Edit existing questions
   - Organize by topic/difficulty
4. **Export:**
   - Export questionnaire as JSON
   - Share with students
5. **Analytics** (when backend integrated):
   - View student performance
   - Track completion rates
   - Analyze question difficulty

## API Integration

Currently, both frontends work standalone with Firebase Firestore. To integrate with the C++ backend:

### Required REST Endpoints

```cpp
// Backend should expose:
POST   /api/questionnaires          - Create questionnaire
GET    /api/questionnaires/:id      - Get questionnaire
POST   /api/sessions                - Start session
POST   /api/sessions/:id/answers    - Submit answer
POST   /api/sessions/:id/complete   - Complete session
GET    /api/sessions/:id/results    - Get results
GET    /api/stats/:questionnaireId  - Get statistics
```

### Integration Steps

1. Implement REST API in C++ backend (using Crow, cpp-httplib, or Boost.Beast)
2. Update `services/api.ts` in both frontends
3. Replace Firestore calls with HTTP requests where applicable
4. Use Firestore for user-specific data (question banks, saved exams)
5. Use C++ backend for exam execution and scoring

## Security Considerations

1. **Authentication:**
   - Implement proper user authentication
   - Use Firebase Authentication for production
   - Secure API endpoints

2. **Data Privacy:**
   - Users can only access their own questions/exams
   - Implement role-based access control
   - Validate all inputs

3. **Firebase Rules:**
   - Restrict read/write access
   - Validate data structure
   - Rate limiting

## Future Enhancements

- [ ] Firebase Authentication integration
- [ ] Real-time collaborative features
- [ ] Advanced analytics dashboard
- [ ] Question sharing between users
- [ ] Exam scheduling
- [ ] Automatic grading for essays (AI integration)
- [ ] Mobile app versions
- [ ] Offline mode with sync
- [ ] Export results to PDF
- [ ] Integration with LMS platforms

## Troubleshooting

**Firebase Connection Issues:**
- Verify Firebase configuration
- Check internet connectivity
- Ensure Firestore is enabled in Firebase Console

**Build Errors:**
- Run `npm install` again
- Clear node_modules and reinstall
- Check Node.js version (18+)

**Port Conflicts:**
- Change port in `vite.config.ts`
- Kill existing processes on ports 3000/3001

## Support

For issues or questions:
1. Check Firebase Console for errors
2. Review browser console for client-side errors
3. Verify Firestore security rules
4. Check network tab for API calls

## License

Part of the ClassRoom project.
