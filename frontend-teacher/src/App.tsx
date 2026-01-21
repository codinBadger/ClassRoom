import { useState } from 'react';
import './App.css';
import { Questionnaire, StudentSession, SessionResult } from './types';
import { QuestionnaireService } from './services/api';
import QuestionCard from './components/QuestionCard';
import ResultsView from './components/ResultsView';

type AppState = 'upload' | 'student-info' | 'exam' | 'results';

function App() {
  const [state, setState] = useState<AppState>('upload');
  const [questionnaire, setQuestionnaire] = useState<Questionnaire | null>(null);
  const [session, setSession] = useState<StudentSession | null>(null);
  const [studentName, setStudentName] = useState('');
  const [studentId, setStudentId] = useState('');
  const [currentQuestionIndex, setCurrentQuestionIndex] = useState(0);
  const [answers, setAnswers] = useState<Record<number, string>>({});
  const [correctAnswers, setCorrectAnswers] = useState<Record<number, string>>({});
  const [results, setResults] = useState<SessionResult | null>(null);
  const [startTime, setStartTime] = useState<number>(0);

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) return;

    try {
      const json = await QuestionnaireService.loadQuestionnaireFromJson(file);
      
      // Extract correct answers from the JSON
      const reader = new FileReader();
      reader.onload = (e) => {
        const jsonData = JSON.parse(e.target?.result as string);
        const correctAns: Record<number, string> = {};
        jsonData.questions.forEach((q: any, index: number) => {
          correctAns[index + 1] = q.correctAnswer || q.answer;
        });
        setCorrectAnswers(correctAns);
      };
      reader.readAsText(file);
      
      setQuestionnaire(json);
      setState('student-info');
    } catch (error) {
      alert('Error loading exam file. Please check the JSON format.');
      console.error(error);
    }
  };

  const handleStartExam = async () => {
    if (!studentName.trim() || !studentId.trim()) {
      alert('Please enter your name and ID');
      return;
    }

    if (questionnaire) {
      const newSession = await QuestionnaireService.startSession(
        parseInt(studentId),
        studentName,
        questionnaire.id
      );
      setSession(newSession);
      setStartTime(Date.now());
      setState('exam');
    }
  };

  const handleAnswerSelect = (answer: string) => {
    if (!session || !questionnaire) return;

    const currentQuestion = questionnaire.questions[currentQuestionIndex];
    const newAnswers = { ...answers, [currentQuestion.id]: answer };
    setAnswers(newAnswers);

    // Auto-advance for non-text questions
    if (currentQuestion.type !== 'SHORT_ANSWER' && currentQuestion.type !== 'ESSAY') {
      setTimeout(() => {
        if (currentQuestionIndex < questionnaire.questions.length - 1) {
          setCurrentQuestionIndex(currentQuestionIndex + 1);
        }
      }, 300);
    }
  };

  const handleNextQuestion = () => {
    if (questionnaire && currentQuestionIndex < questionnaire.questions.length - 1) {
      setCurrentQuestionIndex(currentQuestionIndex + 1);
    }
  };

  const handlePreviousQuestion = () => {
    if (currentQuestionIndex > 0) {
      setCurrentQuestionIndex(currentQuestionIndex - 1);
    }
  };

  const handleSubmitExam = () => {
    if (!session || !questionnaire) return;

    const elapsedTime = (Date.now() - startTime) / 1000;
    const updatedSession = { ...session, answers, elapsedTime };
    const examResults = QuestionnaireService.calculateResults(
      questionnaire,
      updatedSession,
      correctAnswers
    );
    
    setResults(examResults);
    setState('results');
  };

  const handleRestart = () => {
    setQuestionnaire(null);
    setSession(null);
    setStudentName('');
    setStudentId('');
    setCurrentQuestionIndex(0);
    setAnswers({});
    setCorrectAnswers({});
    setResults(null);
    setState('upload');
  };

  // Upload view
  if (state === 'upload') {
    return (
      <div>
        <div className="card" style={{ textAlign: 'center', maxWidth: '600px', margin: '0 auto' }}>
          <h1 style={{ fontSize: '2.5rem', marginBottom: '1rem' }}>
            📚 ClassRoom Questionnaire
          </h1>
          <p style={{ fontSize: '1.2rem', color: '#6b7280', marginBottom: '2rem' }}>
            Modern Web-Based Exam System
          </p>
          
          <div style={{ 
            border: '2px dashed #646cff', 
            borderRadius: '12px', 
            padding: '3rem', 
            marginBottom: '1rem',
            background: '#f8f9fa'
          }}>
            <h2 style={{ marginBottom: '1rem' }}>Upload Exam JSON File</h2>
            <input
              type="file"
              accept=".json"
              onChange={handleFileUpload}
              style={{ 
                display: 'block', 
                margin: '1rem auto',
                padding: '1rem',
                border: '1px solid #ccc',
                borderRadius: '8px'
              }}
            />
            <p style={{ color: '#6b7280', marginTop: '1rem' }}>
              Supports all question types: Multiple Choice, True/False, Short Answer, Essay
            </p>
          </div>
          
          <div style={{ textAlign: 'left', marginTop: '2rem', padding: '1rem', background: '#f8f9fa', borderRadius: '8px' }}>
            <h3 style={{ fontSize: '1.1rem', marginBottom: '0.5rem' }}>Features:</h3>
            <ul style={{ paddingLeft: '1.5rem', color: '#374151' }}>
              <li>Load exams from JSON files</li>
              <li>Real-time answer tracking</li>
              <li>Automatic scoring</li>
              <li>Detailed results with question review</li>
              <li>Modern, responsive design</li>
            </ul>
          </div>
        </div>
      </div>
    );
  }

  // Student info view
  if (state === 'student-info' && questionnaire) {
    return (
      <div>
        <div className="card" style={{ maxWidth: '600px', margin: '0 auto' }}>
          <h1 style={{ textAlign: 'center', marginBottom: '2rem' }}>Student Information</h1>
          
          <div className="card" style={{ background: '#f8f9fa', marginBottom: '2rem' }}>
            <h2 style={{ color: '#646cff' }}>{questionnaire.title}</h2>
            <p style={{ color: '#6b7280', marginBottom: '1rem' }}>{questionnaire.description}</p>
            <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '1rem' }}>
              <div>
                <strong>Questions:</strong> {questionnaire.questionCount}
              </div>
              <div>
                <strong>Total Points:</strong> {questionnaire.totalPoints}
              </div>
              <div>
                <strong>Time Limit:</strong> {questionnaire.timeLimit} minutes
              </div>
              <div>
                <strong>Status:</strong> Ready
              </div>
            </div>
          </div>

          <div style={{ marginBottom: '1.5rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '500' }}>
              Student Name
            </label>
            <input
              type="text"
              value={studentName}
              onChange={(e) => setStudentName(e.target.value)}
              placeholder="Enter your full name"
            />
          </div>

          <div style={{ marginBottom: '2rem' }}>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '500' }}>
              Student ID
            </label>
            <input
              type="text"
              value={studentId}
              onChange={(e) => setStudentId(e.target.value)}
              placeholder="Enter your student ID"
            />
          </div>

          <button
            onClick={handleStartExam}
            style={{ width: '100%', padding: '1rem', fontSize: '1.1rem' }}
            disabled={!studentName.trim() || !studentId.trim()}
          >
            Start Exam
          </button>
        </div>
      </div>
    );
  }

  // Exam view
  if (state === 'exam' && questionnaire && session) {
    const currentQuestion = questionnaire.questions[currentQuestionIndex];
    const progress = ((currentQuestionIndex + 1) / questionnaire.questions.length) * 100;
    const answeredCount = Object.keys(answers).length;

    return (
      <div>
        <div className="card">
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '1rem' }}>
            <h2>{questionnaire.title}</h2>
            <div style={{ color: '#6b7280' }}>
              {session.studentName} (ID: {session.studentId})
            </div>
          </div>
          
          <div style={{ 
            background: '#e5e7eb', 
            height: '8px', 
            borderRadius: '4px', 
            overflow: 'hidden',
            marginBottom: '1rem'
          }}>
            <div style={{ 
              background: '#646cff', 
              height: '100%', 
              width: `${progress}%`,
              transition: 'width 0.3s'
            }} />
          </div>
          
          <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '0.9rem', color: '#6b7280' }}>
            <span>Question {currentQuestionIndex + 1} of {questionnaire.questionCount}</span>
            <span>Answered: {answeredCount}/{questionnaire.questionCount}</span>
          </div>
        </div>

        <QuestionCard
          question={currentQuestion}
          questionNumber={currentQuestionIndex + 1}
          selectedAnswer={answers[currentQuestion.id]}
          onAnswerSelect={handleAnswerSelect}
        />

        <div className="card" style={{ display: 'flex', justifyContent: 'space-between', gap: '1rem' }}>
          <button
            onClick={handlePreviousQuestion}
            disabled={currentQuestionIndex === 0}
            style={{ flex: 1 }}
          >
            ← Previous
          </button>
          
          {currentQuestionIndex < questionnaire.questions.length - 1 ? (
            <button
              onClick={handleNextQuestion}
              style={{ flex: 1 }}
            >
              Next →
            </button>
          ) : (
            <button
              onClick={handleSubmitExam}
              style={{ flex: 1, background: '#10b981' }}
            >
              Submit Exam
            </button>
          )}
        </div>
      </div>
    );
  }

  // Results view
  if (state === 'results' && results) {
    return <ResultsView results={results} onRestart={handleRestart} />;
  }

  return null;
}

export default App;
