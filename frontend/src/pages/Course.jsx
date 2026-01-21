import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { courseService, flashcardService, questionnaireService, noteService, codeService, progressService } from '../services/api';
import Editor from '@monaco-editor/react';

function Course() {
  const { classroomId, courseId } = useParams();
  const [course, setCourse] = useState(null);
  const [activeTab, setActiveTab] = useState('overview');
  const [flashcards, setFlashcards] = useState([]);
  const [questionnaires, setQuestionnaires] = useState([]);
  const [notes, setNotes] = useState([]);
  const [code, setCode] = useState('print("Hello, World!")');
  const [language, setLanguage] = useState('python');
  const [output, setOutput] = useState('');
  const [stats, setStats] = useState(null);
  const navigate = useNavigate();

  useEffect(() => {
    loadCourse();
    loadFlashcards();
    loadQuestionnaires();
    loadNotes();
    loadStats();
  }, [courseId]);

  const loadCourse = async () => {
    try {
      const response = await courseService.get(classroomId, courseId);
      setCourse(response.data);
    } catch (error) {
      console.error('Failed to load course:', error);
    }
  };

  const loadFlashcards = async () => {
    try {
      const response = await flashcardService.getAll(courseId);
      setFlashcards(response.data);
    } catch (error) {
      console.error('Failed to load flashcards:', error);
    }
  };

  const loadQuestionnaires = async () => {
    try {
      const response = await questionnaireService.getAll(courseId);
      setQuestionnaires(response.data);
    } catch (error) {
      console.error('Failed to load questionnaires:', error);
    }
  };

  const loadNotes = async () => {
    try {
      const response = await noteService.getAll(courseId);
      setNotes(response.data);
    } catch (error) {
      console.error('Failed to load notes:', error);
    }
  };

  const loadStats = async () => {
    try {
      const response = await progressService.getStats(courseId);
      setStats(response.data);
    } catch (error) {
      console.error('Failed to load stats:', error);
    }
  };

  const handleRunCode = async () => {
    try {
      setOutput('Running...');
      const response = await codeService.execute(courseId, { language, code });
      if (response.data.success) {
        setOutput(response.data.output || 'Program completed successfully with no output');
      } else {
        setOutput(`Error: ${response.data.error}`);
      }
    } catch (error) {
      setOutput('Failed to execute code: ' + error.message);
    }
  };

  const createFlashcard = async () => {
    const front = prompt('Enter front text:');
    const back = prompt('Enter back text:');
    if (front && back) {
      try {
        await flashcardService.create(courseId, { front, back });
        loadFlashcards();
      } catch (error) {
        alert('Failed to create flashcard');
      }
    }
  };

  const createNote = async () => {
    const title = prompt('Enter note title:');
    const content = prompt('Enter note content:');
    if (title && content) {
      try {
        await noteService.create(courseId, { title, content });
        loadNotes();
      } catch (error) {
        alert('Failed to create note');
      }
    }
  };

  if (!course) return <div style={{ padding: '20px' }}>Loading...</div>;

  return (
    <div style={{ padding: '20px' }}>
      <button onClick={() => navigate(`/classrooms/${classroomId}`)} style={{ marginBottom: '20px' }}>← Back to Classroom</button>
      <h1>{course.name}</h1>
      <p>{course.description}</p>

      <div style={{ borderBottom: '2px solid #ddd', marginBottom: '20px' }}>
        {['overview', 'flashcards', 'questionnaires', 'notes', 'code', 'progress'].map(tab => (
          <button
            key={tab}
            onClick={() => setActiveTab(tab)}
            style={{
              padding: '10px 20px',
              marginRight: '10px',
              border: 'none',
              borderBottom: activeTab === tab ? '2px solid blue' : 'none',
              background: 'none',
              cursor: 'pointer',
              fontWeight: activeTab === tab ? 'bold' : 'normal',
            }}
          >
            {tab.charAt(0).toUpperCase() + tab.slice(1)}
          </button>
        ))}
      </div>

      {activeTab === 'overview' && (
        <div>
          <h2>Course Overview</h2>
          <p>Welcome to {course.name}! Use the tabs above to access different learning tools.</p>
          {stats && (
            <div style={{ marginTop: '20px', padding: '20px', background: '#f5f5f5', borderRadius: '8px' }}>
              <h3>Your Progress</h3>
              <p>Time Spent: {Math.floor(stats.total_time_spent / 60)} minutes</p>
              <p>Completed Activities: {stats.completed_activities} / {stats.total_activities}</p>
              <p>Progress: {stats.progress_percentage.toFixed(1)}%</p>
            </div>
          )}
        </div>
      )}

      {activeTab === 'flashcards' && (
        <div>
          <h2>Flashcards</h2>
          <button onClick={createFlashcard} style={{ marginBottom: '20px', padding: '10px 20px' }}>Create Flashcard</button>
          <div style={{ display: 'grid', gap: '20px' }}>
            {flashcards.map(card => (
              <div key={card.id} style={{ padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
                <div><strong>Front:</strong> {card.front}</div>
                <div style={{ marginTop: '10px' }}><strong>Back:</strong> {card.back}</div>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'questionnaires' && (
        <div>
          <h2>Questionnaires</h2>
          <div style={{ display: 'grid', gap: '20px' }}>
            {questionnaires.map(q => (
              <div key={q.id} style={{ padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
                <h3>{q.title}</h3>
                <p>{q.description}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'notes' && (
        <div>
          <h2>Notes</h2>
          <button onClick={createNote} style={{ marginBottom: '20px', padding: '10px 20px' }}>Create Note</button>
          <div style={{ display: 'grid', gap: '20px' }}>
            {notes.map(note => (
              <div key={note.id} style={{ padding: '20px', border: '1px solid #ddd', borderRadius: '8px' }}>
                <h3>{note.title}</h3>
                <p>{note.content}</p>
                <small>Updated: {new Date(note.updated_at).toLocaleString()}</small>
              </div>
            ))}
          </div>
        </div>
      )}

      {activeTab === 'code' && (
        <div>
          <h2>Code Editor</h2>
          <div style={{ marginBottom: '10px' }}>
            <label>Language: </label>
            <select value={language} onChange={(e) => setLanguage(e.target.value)} style={{ padding: '5px' }}>
              <option value="python">Python</option>
              <option value="javascript">JavaScript</option>
              <option value="rust">Rust</option>
              <option value="cpp">C++</option>
              <option value="java">Java</option>
            </select>
            <button onClick={handleRunCode} style={{ marginLeft: '10px', padding: '5px 20px' }}>Run Code</button>
          </div>
          <Editor
            height="400px"
            language={language}
            value={code}
            onChange={(value) => setCode(value)}
            theme="vs-dark"
            options={{ minimap: { enabled: false } }}
          />
          <div style={{ marginTop: '20px' }}>
            <h3>Output:</h3>
            <pre style={{ padding: '10px', background: '#000', color: '#0f0', minHeight: '100px', whiteSpace: 'pre-wrap' }}>
              {output}
            </pre>
          </div>
        </div>
      )}

      {activeTab === 'progress' && (
        <div>
          <h2>Progress Tracking</h2>
          {stats && (
            <div style={{ padding: '20px', background: '#f5f5f5', borderRadius: '8px' }}>
              <h3>Statistics</h3>
              <p><strong>Total Time:</strong> {Math.floor(stats.total_time_spent / 60)} minutes</p>
              <p><strong>Completed:</strong> {stats.completed_activities} activities</p>
              <p><strong>Total:</strong> {stats.total_activities} activities</p>
              <p><strong>Progress:</strong> {stats.progress_percentage.toFixed(1)}%</p>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default Course;
