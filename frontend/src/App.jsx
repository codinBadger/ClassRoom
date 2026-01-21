import React from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './utils/AuthContext';
import Login from './pages/Login';
import Classrooms from './pages/Classrooms';
import Classroom from './pages/Classroom';
import Course from './pages/Course';
import './App.css';

function PrivateRoute({ children }) {
  const { user, loading } = useAuth();
  
  if (loading) {
    return <div style={{ padding: '20px' }}>Loading...</div>;
  }
  
  return user ? children : <Navigate to="/login" />;
}

function App() {
  return (
    <AuthProvider>
      <Router>
        <div style={{ minHeight: '100vh' }}>
          <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/classrooms" element={<PrivateRoute><Classrooms /></PrivateRoute>} />
            <Route path="/classrooms/:classroomId" element={<PrivateRoute><Classroom /></PrivateRoute>} />
            <Route path="/classrooms/:classroomId/courses/:courseId" element={<PrivateRoute><Course /></PrivateRoute>} />
            <Route path="/" element={<Navigate to="/classrooms" />} />
          </Routes>
        </div>
      </Router>
    </AuthProvider>
  );
}

export default App;

