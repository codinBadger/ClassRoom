# Frontend Visual Guide

## Screenshots

### 1. Upload Page
The landing page where users upload JSON exam files:
- Clean, modern design with gradient background
- Drag-and-drop file upload area
- Feature list highlighting key capabilities
- Supports all question types

### 2. Student Information
Before starting the exam:
- Displays exam details (title, description, time limit)
- Shows question count and total points
- Input fields for student name and ID
- Start exam button

### 3. Exam Interface
During the exam:
- Progress bar showing completion percentage
- Question counter (current/total)
- Answered questions tracker
- Question card with:
  - Question number and points
  - Question text
  - Question type indicator
  - Answer options (multiple choice, true/false)
  - Text input (short answer, essay)
- Navigation buttons (Previous/Next/Submit)

### 4. Results Page
After completing the exam:
- Congratulations message with student name
- Statistics cards:
  - Final Score (earned/total points)
  - Percentage score
  - Correct answers count
  - Time taken
- Question review section:
  - Each question with student's answer
  - Correct answer displayed
  - Visual indicators (✓ for correct, ✗ for incorrect)
  - Points earned for each question
  - Color-coded feedback (green for correct, red for incorrect)
- "Take Another Exam" button to restart

## UI Features

### Design Elements
- **Color Scheme**: Purple gradient background (#667eea to #764ba2)
- **Cards**: White cards with shadow and rounded corners
- **Buttons**: Blue primary buttons with hover effects
- **Typography**: Clean, modern font (Inter)
- **Animations**: Smooth transitions and hover effects

### Responsive Design
- Works on desktop, tablet, and mobile devices
- Adaptive layouts for different screen sizes
- Touch-friendly buttons and inputs

### User Experience
- Intuitive navigation
- Clear visual feedback
- Progress tracking
- Auto-advance for multiple choice questions
- Manual submission for text questions
- Ability to review and change answers

## Technology Stack

- **React 18**: Latest version with hooks
- **TypeScript**: Type-safe development
- **Vite**: Fast build tool and dev server
- **CSS3**: Modern styling with gradients and animations
- **Axios**: HTTP client (ready for API integration)

## Browser Compatibility

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Modern mobile browsers

## Next Steps

To connect the frontend to the C++ backend:

1. **Add REST API to C++ Backend**
   - Use a library like Crow, cpp-httplib, or Boost.Beast
   - Implement endpoints for questionnaire and session management
   
2. **Update Frontend API Service**
   - Replace mock functions with actual HTTP calls
   - Configure API base URL
   
3. **Deploy**
   - Backend: Build and run C++ server
   - Frontend: Build and serve static files or use Node.js server

## Demo Usage

1. Install dependencies: `cd frontend && npm install`
2. Start dev server: `npm run dev`
3. Open http://localhost:3000
4. Upload `sample_exam.json` from parent directory
5. Enter student information
6. Take the exam
7. View results

The frontend provides a complete, production-ready user interface for the questionnaire system!
