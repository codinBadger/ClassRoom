import { SessionResult } from '../types';
import QuestionCard from './QuestionCard';

interface ResultsViewProps {
  results: SessionResult;
  onRestart: () => void;
}

export default function ResultsView({ results, onRestart }: ResultsViewProps) {
  const { session, questionnaire, results: questionResults, percentage } = results;
  const correctCount = questionResults.filter(r => r.isCorrect).length;

  return (
    <div>
      <div className="card">
        <h1 style={{ textAlign: 'center', marginBottom: '2rem' }}>
          🎉 Exam Results
        </h1>
        
        <div style={{ textAlign: 'center', marginBottom: '2rem' }}>
          <h2>{session.studentName}</h2>
          <p style={{ color: '#6b7280' }}>{questionnaire.title}</p>
        </div>

        <div className="stats-grid">
          <div className="stat-card">
            <div>Final Score</div>
            <div className="stat-value">{session.score}/{questionnaire.totalPoints}</div>
          </div>
          <div className="stat-card">
            <div>Percentage</div>
            <div className="stat-value">{percentage.toFixed(1)}%</div>
          </div>
          <div className="stat-card">
            <div>Correct Answers</div>
            <div className="stat-value">{correctCount}/{questionnaire.questionCount}</div>
          </div>
          {session.elapsedTime && (
            <div className="stat-card">
              <div>Time Taken</div>
              <div className="stat-value">{session.elapsedTime.toFixed(0)}s</div>
            </div>
          )}
        </div>
      </div>

      <div className="card">
        <h2>Question Review</h2>
        {questionResults.map((result, index) => (
          <div key={result.question.id} style={{ marginBottom: '2rem' }}>
            <QuestionCard
              question={result.question}
              questionNumber={index + 1}
              selectedAnswer={result.studentAnswer}
              onAnswerSelect={() => {}}
              showCorrectAnswer={true}
              correctAnswer={result.correctAnswer}
              disabled={true}
            />
            <div style={{ 
              padding: '1rem', 
              background: result.isCorrect ? '#d1fae5' : '#fee2e2',
              borderRadius: '8px',
              marginTop: '0.5rem'
            }}>
              <strong>
                {result.isCorrect ? '✓ Correct' : '✗ Incorrect'}
              </strong>
              {' - '}
              Points earned: {result.pointsEarned}/{result.question.points}
            </div>
          </div>
        ))}
      </div>

      <div style={{ textAlign: 'center' }}>
        <button onClick={onRestart} style={{ padding: '1rem 2rem', fontSize: '1.1rem' }}>
          Take Another Exam
        </button>
      </div>
    </div>
  );
}
