import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { classroomService, courseService } from '../services/api';

function Classroom() {
  const { classroomId } = useParams();
  const [classroom, setClassroom] = useState(null);
  const [courses, setCourses] = useState([]);
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({ name: '', description: '' });
  const navigate = useNavigate();

  useEffect(() => {
    loadClassroom();
    loadCourses();
  }, [classroomId]);

  const loadClassroom = async () => {
    try {
      const response = await classroomService.get(classroomId);
      setClassroom(response.data);
    } catch (error) {
      console.error('Failed to load classroom:', error);
    }
  };

  const loadCourses = async () => {
    try {
      const response = await courseService.getAll(classroomId);
      setCourses(response.data);
    } catch (error) {
      console.error('Failed to load courses:', error);
    }
  };

  const handleCreate = async (e) => {
    e.preventDefault();
    try {
      await courseService.create(classroomId, formData);
      setFormData({ name: '', description: '' });
      setShowForm(false);
      loadCourses();
    } catch (error) {
      console.error('Failed to create course:', error);
    }
  };

  if (!classroom) return <div style={{ padding: '20px' }}>Loading...</div>;

  return (
    <div style={{ padding: '20px' }}>
      <button onClick={() => navigate('/classrooms')} style={{ marginBottom: '20px' }}>← Back to Classrooms</button>
      <h1>{classroom.name}</h1>
      <p>{classroom.description}</p>

      <h2 style={{ marginTop: '30px' }}>Courses</h2>
      <button onClick={() => setShowForm(!showForm)} style={{ marginBottom: '20px', padding: '10px 20px', cursor: 'pointer' }}>
        {showForm ? 'Cancel' : 'Create New Course'}
      </button>

      {showForm && (
        <form onSubmit={handleCreate} style={{ marginBottom: '30px', padding: '20px', border: '1px solid #ccc' }}>
          <div style={{ marginBottom: '15px' }}>
            <input
              type="text"
              placeholder="Course Name"
              value={formData.name}
              onChange={(e) => setFormData({ ...formData, name: e.target.value })}
              style={{ width: '100%', padding: '10px', fontSize: '16px' }}
              required
            />
          </div>
          <div style={{ marginBottom: '15px' }}>
            <textarea
              placeholder="Description"
              value={formData.description}
              onChange={(e) => setFormData({ ...formData, description: e.target.value })}
              style={{ width: '100%', padding: '10px', fontSize: '16px', minHeight: '80px' }}
            />
          </div>
          <button type="submit" style={{ padding: '10px 20px', cursor: 'pointer' }}>Create</button>
        </form>
      )}

      <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(300px, 1fr))', gap: '20px' }}>
        {courses.map((course) => (
          <div
            key={course.id}
            onClick={() => navigate(`/classrooms/${classroomId}/courses/${course.id}`)}
            style={{
              padding: '20px',
              border: '1px solid #ddd',
              borderRadius: '8px',
              cursor: 'pointer',
              transition: 'box-shadow 0.3s',
            }}
            onMouseEnter={(e) => e.currentTarget.style.boxShadow = '0 4px 8px rgba(0,0,0,0.1)'}
            onMouseLeave={(e) => e.currentTarget.style.boxShadow = 'none'}
          >
            <h3>{course.name}</h3>
            <p>{course.description}</p>
            <small>Created: {new Date(course.created_at).toLocaleDateString()}</small>
          </div>
        ))}
      </div>
    </div>
  );
}

export default Classroom;
