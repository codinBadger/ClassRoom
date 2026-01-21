import axios from 'axios';

const API_URL = 'http://localhost:8080/api';

const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add token to requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

export const authService = {
  register: (userData) => api.post('/auth/register', userData),
  login: (credentials) => api.post('/auth/login', credentials),
  getProfile: () => api.get('/auth/profile'),
};

export const classroomService = {
  create: (data) => api.post('/classrooms', data),
  getAll: () => api.get('/classrooms'),
  get: (id) => api.get(`/classrooms/${id}`),
  update: (id, data) => api.put(`/classrooms/${id}`, data),
  delete: (id) => api.delete(`/classrooms/${id}`),
};

export const courseService = {
  create: (classroomId, data) => api.post(`/classrooms/${classroomId}/courses`, data),
  getAll: (classroomId) => api.get(`/classrooms/${classroomId}/courses`),
  get: (classroomId, courseId) => api.get(`/classrooms/${classroomId}/courses/${courseId}`),
  update: (classroomId, courseId, data) => api.put(`/classrooms/${classroomId}/courses/${courseId}`, data),
  delete: (classroomId, courseId) => api.delete(`/classrooms/${classroomId}/courses/${courseId}`),
};

export const ebookService = {
  upload: (courseId, formData) => api.post(`/courses/${courseId}/ebooks`, formData, {
    headers: { 'Content-Type': 'multipart/form-data' },
  }),
  getAll: (courseId) => api.get(`/courses/${courseId}/ebooks`),
  delete: (courseId, ebookId) => api.delete(`/courses/${courseId}/ebooks/${ebookId}`),
};

export const flashcardService = {
  create: (courseId, data) => api.post(`/courses/${courseId}/flashcards`, data),
  getAll: (courseId) => api.get(`/courses/${courseId}/flashcards`),
  update: (courseId, flashcardId, data) => api.put(`/courses/${courseId}/flashcards/${flashcardId}`, data),
  delete: (courseId, flashcardId) => api.delete(`/courses/${courseId}/flashcards/${flashcardId}`),
};

export const questionnaireService = {
  create: (courseId, data) => api.post(`/courses/${courseId}/questionnaires`, data),
  getAll: (courseId) => api.get(`/courses/${courseId}/questionnaires`),
  addQuestion: (courseId, questionnaireId, data) => api.post(`/courses/${courseId}/questionnaires/${questionnaireId}/questions`, data),
  getQuestions: (courseId, questionnaireId) => api.get(`/courses/${courseId}/questionnaires/${questionnaireId}/questions`),
  submit: (courseId, questionnaireId, answers) => api.post(`/courses/${courseId}/questionnaires/${questionnaireId}/submit`, answers),
};

export const noteService = {
  create: (courseId, data) => api.post(`/courses/${courseId}/notes`, data),
  getAll: (courseId) => api.get(`/courses/${courseId}/notes`),
  update: (courseId, noteId, data) => api.put(`/courses/${courseId}/notes/${noteId}`, data),
  delete: (courseId, noteId) => api.delete(`/courses/${courseId}/notes/${noteId}`),
};

export const codeService = {
  execute: (courseId, data) => api.post(`/courses/${courseId}/code/execute`, data),
  createSession: (courseId, data) => api.post(`/courses/${courseId}/code/sessions`, data),
  getSessions: (courseId) => api.get(`/courses/${courseId}/code/sessions`),
};

export const progressService = {
  track: (courseId, data) => api.post(`/courses/${courseId}/progress`, data),
  getStats: (courseId) => api.get(`/courses/${courseId}/progress/stats`),
  getTimeline: (courseId) => api.get(`/courses/${courseId}/progress/timeline`),
  getMetrics: (courseId) => api.get(`/courses/${courseId}/progress/metrics`),
};

export default api;
