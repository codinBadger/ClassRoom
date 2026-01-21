import React, { useState, useEffect } from 'react';
import { useNavigate } from 'react-router-dom';
import { classroomService } from '../services/api';

function Classrooms() {
  const [classrooms, setClassrooms] = useState([]);
  const [showForm, setShowForm] = useState(false);
  const [formData, setFormData] = useState({ name: '', description: '' });
  const navigate = useNavigate();

  useEffect(() => {
    loadClassrooms();
  }, []);

  const loadClassrooms = async () => {
    try {
      const response = await classroomService.getAll();
      setClassrooms(response.data);
    } catch (error) {
      console.error('Failed to load classrooms:', error);
    }
  };

  const handleCreate = async (e) => {
    e.preventDefault();
    try {
      await classroomService.create(formData);
      setFormData({ name: '', description: '' });
      setShowForm(false);
      loadClassrooms();
    } catch (error) {
      console.error('Failed to create classroom:', error);
    }
  };

  return (
    <div style={{ padding: '20px' }}>
      <h1>My Classrooms</h1>
      <button onClick={() => setShowForm(!showForm)} style={{ marginBottom: '20px', padding: '10px 20px', cursor: 'pointer' }}>
        {showForm ? 'Cancel' : 'Create New Classroom'}
      </button>

      {showForm && (
        <form onSubmit={handleCreate} style={{ marginBottom: '30px', padding: '20px', border: '1px solid #ccc' }}>
          <div style={{ marginBottom: '15px' }}>
            <input
              type="text"
              placeholder="Classroom Name"
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
        {classrooms.map((classroom) => (
          <div
            key={classroom.id}
            onClick={() => navigate(`/classrooms/${classroom.id}`)}
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
            <h3>{classroom.name}</h3>
            <p>{classroom.description}</p>
            <small>Created: {new Date(classroom.created_at).toLocaleDateString()}</small>
          </div>
        ))}
      </div>
    </div>
  );
}

export default Classrooms;
