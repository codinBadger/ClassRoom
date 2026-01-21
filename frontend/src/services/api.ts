import axios from 'axios';
import { Questionnaire, StudentSession, SessionResult } from '../types';

const API_BASE_URL = '/api';

// Mock data service for demonstration (until REST API is implemented)
export class QuestionnaireService {
  // Load questionnaire from JSON
  static async loadQuestionnaireFromJson(file: File): Promise<Questionnaire> {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        try {
          const json = JSON.parse(e.target?.result as string);
          const questionnaire: Questionnaire = {
            id: Date.now(),
            title: json.title,
            description: json.description,
            timeLimit: json.timeLimit,
            totalPoints: 0,
            questionCount: json.questions.length,
            questions: json.questions.map((q: any, index: number) => {
              const points = q.points || 1;
              return {
                id: index + 1,
                questionText: q.questionText || q.text,
                type: q.questionType || q.type,
                options: q.options || [],
                points: points
              };
            })
          };
          questionnaire.totalPoints = questionnaire.questions.reduce((sum, q) => sum + q.points, 0);
          resolve(questionnaire);
        } catch (error) {
          reject(error);
        }
      };
      reader.onerror = reject;
      reader.readAsText(file);
    });
  }

  // Get questionnaire by ID (mock)
  static async getQuestionnaire(id: number): Promise<Questionnaire | null> {
    // In a real implementation, this would call: GET ${API_BASE_URL}/questionnaires/${id}
    return null;
  }

  // Start a new session
  static async startSession(
    studentId: number,
    studentName: string,
    questionnaireId: number
  ): Promise<StudentSession> {
    // In a real implementation, this would call: POST ${API_BASE_URL}/sessions
    return {
      sessionId: Date.now(),
      studentId,
      studentName,
      questionnaireId,
      answers: {},
      score: 0,
      isCompleted: false
    };
  }

  // Submit an answer
  static async submitAnswer(
    sessionId: number,
    questionId: number,
    answer: string
  ): Promise<boolean> {
    // In a real implementation, this would call: POST ${API_BASE_URL}/sessions/${sessionId}/answers
    return true;
  }

  // Complete session
  static async completeSession(sessionId: number): Promise<boolean> {
    // In a real implementation, this would call: POST ${API_BASE_URL}/sessions/${sessionId}/complete
    return true;
  }

  // Get session results
  static async getSessionResults(
    sessionId: number
  ): Promise<SessionResult | null> {
    // In a real implementation, this would call: GET ${API_BASE_URL}/sessions/${sessionId}/results
    return null;
  }

  // Calculate results locally (for demo purposes)
  static calculateResults(
    questionnaire: Questionnaire,
    session: StudentSession,
    correctAnswers: Record<number, string>
  ): SessionResult {
    const results = questionnaire.questions.map(question => {
      const studentAnswer = session.answers[question.id];
      const correctAnswer = correctAnswers[question.id];
      const isCorrect = studentAnswer?.toLowerCase() === correctAnswer?.toLowerCase();
      
      return {
        question,
        studentAnswer,
        correctAnswer,
        isCorrect,
        pointsEarned: isCorrect ? question.points : 0
      };
    });

    const score = results.reduce((sum, r) => sum + r.pointsEarned, 0);
    const percentage = (score / questionnaire.totalPoints) * 100;

    return {
      session: { ...session, score, isCompleted: true },
      questionnaire,
      results,
      percentage
    };
  }
}
