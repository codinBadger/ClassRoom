import { useState } from 'react';
import { Question, QuestionType } from '../types';

interface QuestionCardProps {
  question: Question;
  questionNumber: number;
  selectedAnswer?: string;
  onAnswerSelect: (answer: string) => void;
  showCorrectAnswer?: boolean;
  correctAnswer?: string;
  disabled?: boolean;
}

export default function QuestionCard({
  question,
  questionNumber,
  selectedAnswer,
  onAnswerSelect,
  showCorrectAnswer = false,
  correctAnswer,
  disabled = false
}: QuestionCardProps) {
  const [textAnswer, setTextAnswer] = useState(selectedAnswer || '');

  const handleTextSubmit = () => {
    if (textAnswer.trim()) {
      onAnswerSelect(textAnswer);
    }
  };

  const renderOptions = () => {
    if (question.type === QuestionType.SHORT_ANSWER || question.type === QuestionType.ESSAY) {
      const isTextArea = question.type === QuestionType.ESSAY;
      return (
        <div>
          {isTextArea ? (
            <textarea
              value={textAnswer}
              onChange={(e) => setTextAnswer(e.target.value)}
              placeholder="Type your answer here..."
              rows={6}
              disabled={disabled}
            />
          ) : (
            <input
              type="text"
              value={textAnswer}
              onChange={(e) => setTextAnswer(e.target.value)}
              placeholder="Type your answer here..."
              disabled={disabled}
            />
          )}
          {!disabled && (
            <button
              onClick={handleTextSubmit}
              style={{ marginTop: '1rem' }}
              disabled={!textAnswer.trim()}
            >
              Submit Answer
            </button>
          )}
          {showCorrectAnswer && correctAnswer && (
            <div style={{ marginTop: '1rem', padding: '1rem', background: '#f0f9ff', borderRadius: '8px' }}>
              <strong>Correct Answer:</strong> {correctAnswer}
            </div>
          )}
        </div>
      );
    }

    return (
      <div>
        {question.options.map((option, index) => {
          const isSelected = selectedAnswer === option;
          const isCorrect = showCorrectAnswer && correctAnswer === option;
          const isWrong = showCorrectAnswer && isSelected && !isCorrect;

          let className = 'option-button';
          if (isSelected && !showCorrectAnswer) className += ' selected';
          if (isCorrect) className += ' correct';
          if (isWrong) className += ' incorrect';

          return (
            <button
              key={index}
              className={className}
              onClick={() => !disabled && onAnswerSelect(option)}
              disabled={disabled}
            >
              {String.fromCharCode(65 + index)}. {option}
              {isCorrect && ' ✓'}
              {isWrong && ' ✗'}
            </button>
          );
        })}
      </div>
    );
  };

  return (
    <div className="question-card">
      <h3 style={{ color: '#646cff', marginBottom: '1rem' }}>
        Question {questionNumber}
        <span style={{ float: 'right', fontSize: '0.9rem', color: '#6b7280' }}>
          {question.points} {question.points === 1 ? 'point' : 'points'}
        </span>
      </h3>
      <p style={{ fontSize: '1.1rem', marginBottom: '1.5rem', color: '#374151' }}>
        {question.questionText}
      </p>
      <div style={{ fontSize: '0.875rem', color: '#6b7280', marginBottom: '1rem' }}>
        Type: {question.type.replace(/_/g, ' ')}
      </div>
      {renderOptions()}
    </div>
  );
}
