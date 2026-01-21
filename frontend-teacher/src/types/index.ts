// Type definitions matching the C++ backend
export enum QuestionType {
  MULTIPLE_CHOICE = 'MULTIPLE_CHOICE',
  TRUE_FALSE = 'TRUE_FALSE',
  SHORT_ANSWER = 'SHORT_ANSWER',
  ESSAY = 'ESSAY'
}

export interface Question {
  id: number;
  questionText: string;
  type: QuestionType;
  options: string[];
  points: number;
}

export interface Questionnaire {
  id: number;
  title: string;
  description: string;
  timeLimit: number;
  totalPoints: number;
  questionCount: number;
  questions: Question[];
}

export interface Answer {
  questionId: number;
  answer: string;
}

export interface StudentSession {
  sessionId: number;
  studentId: number;
  studentName: string;
  questionnaireId: number;
  answers: Record<number, string>;
  score: number;
  isCompleted: boolean;
  elapsedTime?: number;
}

export interface SessionResult {
  session: StudentSession;
  questionnaire: Questionnaire;
  results: QuestionResult[];
  percentage: number;
}

export interface QuestionResult {
  question: Question;
  studentAnswer?: string;
  correctAnswer: string;
  isCorrect: boolean;
  pointsEarned: number;
}
