import { useState, useEffect } from 'react';
import { SavedQuestion, FirestoreService } from '../services/firestore';
import { QuestionType } from '../types';

interface QuestionBankProps {
  userId: string;
  onSelectQuestions: (questions: SavedQuestion[]) => void;
  onClose: () => void;
}

export default function QuestionBank({ userId, onSelectQuestions, onClose }: QuestionBankProps) {
  const [questions, setQuestions] = useState<SavedQuestion[]>([]);
  const [selectedQuestions, setSelectedQuestions] = useState<Set<number>>(new Set());
  const [filters, setFilters] = useState({
    type: '' as QuestionType | '',
    topic: '',
    difficulty: ''
  });
  const [topics, setTopics] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadQuestions();
    loadTopics();
  }, [userId, filters]);

  const loadQuestions = async () => {
    try {
      setLoading(true);
      const filterObj: any = {};
      if (filters.type) filterObj.type = filters.type;
      if (filters.topic) filterObj.topic = filters.topic;
      if (filters.difficulty) filterObj.difficulty = filters.difficulty;
      
      const data = await FirestoreService.getQuestions(userId, filterObj);
      setQuestions(data);
    } catch (error) {
      console.error('Error loading questions:', error);
    } finally {
      setLoading(false);
    }
  };

  const loadTopics = async () => {
    try {
      const topicList = await FirestoreService.getTopics(userId);
      setTopics(topicList);
    } catch (error) {
      console.error('Error loading topics:', error);
    }
  };

  const toggleQuestion = (index: number) => {
    const newSelected = new Set(selectedQuestions);
    if (newSelected.has(index)) {
      newSelected.delete(index);
    } else {
      newSelected.add(index);
    }
    setSelectedQuestions(newSelected);
  };

  const handleCreateExam = () => {
    const selected = questions.filter((_, index) => selectedQuestions.has(index));
    onSelectQuestions(selected);
  };

  return (
    <div style={{ padding: '2rem' }}>
      <div className="card">
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '2rem' }}>
          <h1>Question Bank</h1>
          <button onClick={onClose}>Close</button>
        </div>

        {/* Filters */}
        <div style={{ 
          display: 'grid', 
          gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', 
          gap: '1rem',
          marginBottom: '2rem'
        }}>
          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '500' }}>
              Question Type
            </label>
            <select
              value={filters.type}
              onChange={(e) => setFilters({ ...filters, type: e.target.value as QuestionType | '' })}
            >
              <option value="">All Types</option>
              <option value="MULTIPLE_CHOICE">Multiple Choice</option>
              <option value="TRUE_FALSE">True/False</option>
              <option value="SHORT_ANSWER">Short Answer</option>
              <option value="ESSAY">Essay</option>
            </select>
          </div>

          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '500' }}>
              Topic
            </label>
            <select
              value={filters.topic}
              onChange={(e) => setFilters({ ...filters, topic: e.target.value })}
            >
              <option value="">All Topics</option>
              {topics.map(topic => (
                <option key={topic} value={topic}>{topic}</option>
              ))}
            </select>
          </div>

          <div>
            <label style={{ display: 'block', marginBottom: '0.5rem', fontWeight: '500' }}>
              Difficulty
            </label>
            <select
              value={filters.difficulty}
              onChange={(e) => setFilters({ ...filters, difficulty: e.target.value })}
            >
              <option value="">All Levels</option>
              <option value="easy">Easy</option>
              <option value="medium">Medium</option>
              <option value="hard">Hard</option>
            </select>
          </div>
        </div>

        {/* Question List */}
        {loading ? (
          <p>Loading questions...</p>
        ) : questions.length === 0 ? (
          <p>No questions found. Take some exams and save questions to build your question bank!</p>
        ) : (
          <>
            <div style={{ marginBottom: '1rem' }}>
              <strong>{questions.length}</strong> questions found | 
              <strong> {selectedQuestions.size}</strong> selected
            </div>

            <div style={{ maxHeight: '400px', overflowY: 'auto' }}>
              {questions.map((question, index) => (
                <div
                  key={index}
                  className="question-card"
                  style={{
                    cursor: 'pointer',
                    border: selectedQuestions.has(index) ? '3px solid #646cff' : '1px solid #e5e7eb',
                    background: selectedQuestions.has(index) ? '#f0f4ff' : 'white'
                  }}
                  onClick={() => toggleQuestion(index)}
                >
                  <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'start' }}>
                    <div style={{ flex: 1 }}>
                      <p style={{ fontWeight: '500', marginBottom: '0.5rem' }}>
                        {question.questionText}
                      </p>
                      <div style={{ display: 'flex', gap: '1rem', fontSize: '0.875rem', color: '#6b7280' }}>
                        <span>Type: {question.type.replace(/_/g, ' ')}</span>
                        {question.topic && <span>Topic: {question.topic}</span>}
                        {question.difficulty && <span>Difficulty: {question.difficulty}</span>}
                        <span>Points: {question.points}</span>
                      </div>
                    </div>
                    <input
                      type="checkbox"
                      checked={selectedQuestions.has(index)}
                      onChange={() => {}}
                      style={{ marginLeft: '1rem', transform: 'scale(1.5)' }}
                    />
                  </div>
                </div>
              ))}
            </div>

            <div style={{ marginTop: '2rem', textAlign: 'center' }}>
              <button
                onClick={handleCreateExam}
                disabled={selectedQuestions.size === 0}
                style={{ padding: '1rem 2rem', fontSize: '1.1rem' }}
              >
                Create Custom Exam ({selectedQuestions.size} questions)
              </button>
            </div>
          </>
        )}
      </div>
    </div>
  );
}
