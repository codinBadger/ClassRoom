# ClassRoom Questionnaire Frontend

Modern React + TypeScript frontend for the ClassRoom Questionnaire Backend.

## Tech Stack

- **React 18** - Latest React with hooks and concurrent features
- **TypeScript** - Type-safe development
- **Vite** - Lightning-fast build tool and dev server
- **Modern CSS** - Custom styling with CSS3 features
- **Axios** - HTTP client for API communication

## Features

✨ **Modern UI/UX**
- Responsive design that works on all devices
- Smooth animations and transitions
- Intuitive navigation
- Beautiful gradient backgrounds

📝 **Exam Management**
- Upload JSON exam files
- Real-time progress tracking
- Support for all question types:
  - Multiple Choice
  - True/False
  - Short Answer
  - Essay

🎯 **Student Experience**
- Easy student registration
- Question navigation (next/previous)
- Answer selection and modification
- Progress indicator

📊 **Results & Analytics**
- Detailed score breakdown
- Question-by-question review
- Correct/incorrect indicators
- Performance statistics
- Time tracking

## Getting Started

### Prerequisites

- Node.js 18+ and npm (or yarn/pnpm)

### Installation

```bash
cd frontend
npm install
```

### Development

```bash
npm run dev
```

Opens at `http://localhost:3000`

### Build for Production

```bash
npm run build
```

Outputs to `dist/` directory

### Preview Production Build

```bash
npm run preview
```

## Project Structure

```
frontend/
├── src/
│   ├── components/     # React components
│   │   ├── QuestionCard.tsx
│   │   └── ResultsView.tsx
│   ├── services/       # API services
│   │   └── api.ts
│   ├── types/          # TypeScript types
│   │   └── index.ts
│   ├── App.tsx         # Main app component
│   ├── App.css         # Styles
│   └── main.tsx        # Entry point
├── public/             # Static assets
├── index.html          # HTML template
├── package.json        # Dependencies
├── tsconfig.json       # TypeScript config
└── vite.config.ts      # Vite config
```

## Usage

### 1. Upload Exam

- Click "Upload Exam JSON File"
- Select a JSON file (use `sample_exam.json` from parent directory)
- Review exam details

### 2. Enter Student Information

- Enter your name and student ID
- Click "Start Exam"

### 3. Take the Exam

- Read each question carefully
- Select or type your answer
- Use Previous/Next buttons to navigate
- Submit when complete

### 4. View Results

- See your score and percentage
- Review each question with correct answers
- See which answers were correct/incorrect

## API Integration

Currently, the frontend works with JSON files loaded directly in the browser. To connect to the C++ backend:

### Required REST API Endpoints

The C++ backend should expose these endpoints:

```
POST   /api/questionnaires/load      - Load questionnaire from JSON
GET    /api/questionnaires/:id       - Get questionnaire by ID
POST   /api/sessions                 - Start new session
POST   /api/sessions/:id/answers     - Submit an answer
POST   /api/sessions/:id/complete    - Complete session
GET    /api/sessions/:id/results     - Get results
```

### Integration Steps

1. Implement REST API server in C++ (using libraries like Crow or cpp-httplib)
2. Update `src/services/api.ts` to use actual API endpoints
3. Enable CORS on the C++ backend
4. Update Vite proxy configuration if needed

## Customization

### Styling

Edit `src/App.css` to customize:
- Colors and gradients
- Button styles
- Card layouts
- Animations

### Question Types

Add new question types in:
1. `src/types/index.ts` - Add enum value
2. `src/components/QuestionCard.tsx` - Add rendering logic

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- Modern mobile browsers

## Performance

- Fast initial load with Vite
- Code splitting for optimal bundle size
- Lazy loading of components
- Optimized re-renders with React hooks

## Future Enhancements

- [ ] Real-time exam timer with warnings
- [ ] Save progress to localStorage
- [ ] Multi-language support
- [ ] Accessibility improvements (ARIA labels)
- [ ] Keyboard navigation
- [ ] Dark/light mode toggle
- [ ] Export results as PDF
- [ ] Admin dashboard for teachers
- [ ] Live exam monitoring
- [ ] Statistics and analytics

## Contributing

1. Follow TypeScript best practices
2. Maintain component modularity
3. Write descriptive commit messages
4. Test on multiple browsers

## License

Part of the ClassRoom project.
