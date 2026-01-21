import {
  collection,
  addDoc,
  getDocs,
  query,
  where,
  orderBy,
  Timestamp,
  deleteDoc,
  doc,
  updateDoc
} from 'firebase/firestore';
import { db } from './firebase';
import { Question, Questionnaire, QuestionType } from '../types';

export interface SavedQuestion extends Question {
  topic?: string;
  difficulty?: 'easy' | 'medium' | 'hard';
  tags?: string[];
  createdAt: Date;
  userId: string;
}

export interface SavedExam {
  id?: string;
  title: string;
  description: string;
  questions: SavedQuestion[];
  timeLimit: number;
  createdAt: Date;
  userId: string;
  isTemplate: boolean;
}

export class FirestoreService {
  // Save individual question to question bank
  static async saveQuestion(question: Omit<SavedQuestion, 'createdAt'>): Promise<string> {
    try {
      const docRef = await addDoc(collection(db, 'questions'), {
        ...question,
        createdAt: Timestamp.now()
      });
      return docRef.id;
    } catch (error) {
      console.error('Error saving question:', error);
      throw error;
    }
  }

  // Save entire exam
  static async saveExam(exam: Omit<SavedExam, 'createdAt' | 'id'>): Promise<string> {
    try {
      const docRef = await addDoc(collection(db, 'exams'), {
        ...exam,
        createdAt: Timestamp.now()
      });
      return docRef.id;
    } catch (error) {
      console.error('Error saving exam:', error);
      throw error;
    }
  }

  // Get all questions for a user with optional filters
  static async getQuestions(
    userId: string,
    filters?: {
      type?: QuestionType;
      topic?: string;
      difficulty?: string;
    }
  ): Promise<SavedQuestion[]> {
    try {
      let q = query(
        collection(db, 'questions'),
        where('userId', '==', userId),
        orderBy('createdAt', 'desc')
      );

      if (filters?.type) {
        q = query(q, where('type', '==', filters.type));
      }
      if (filters?.topic) {
        q = query(q, where('topic', '==', filters.topic));
      }
      if (filters?.difficulty) {
        q = query(q, where('difficulty', '==', filters.difficulty));
      }

      const querySnapshot = await getDocs(q);
      return querySnapshot.docs.map(doc => ({
        ...(doc.data() as any),
        id: doc.id,
        createdAt: doc.data().createdAt?.toDate() || new Date()
      }));
    } catch (error) {
      console.error('Error getting questions:', error);
      throw error;
    }
  }

  // Get all exams for a user
  static async getExams(userId: string): Promise<SavedExam[]> {
    try {
      const q = query(
        collection(db, 'exams'),
        where('userId', '==', userId),
        orderBy('createdAt', 'desc')
      );

      const querySnapshot = await getDocs(q);
      return querySnapshot.docs.map(doc => ({
        ...(doc.data() as any),
        id: doc.id,
        createdAt: doc.data().createdAt?.toDate() || new Date()
      }));
    } catch (error) {
      console.error('Error getting exams:', error);
      throw error;
    }
  }

  // Delete a question
  static async deleteQuestion(questionId: string): Promise<void> {
    try {
      await deleteDoc(doc(db, 'questions', questionId));
    } catch (error) {
      console.error('Error deleting question:', error);
      throw error;
    }
  }

  // Delete an exam
  static async deleteExam(examId: string): Promise<void> {
    try {
      await deleteDoc(doc(db, 'exams', examId));
    } catch (error) {
      console.error('Error deleting exam:', error);
      throw error;
    }
  }

  // Update a question
  static async updateQuestion(questionId: string, updates: Partial<SavedQuestion>): Promise<void> {
    try {
      await updateDoc(doc(db, 'questions', questionId), updates as any);
    } catch (error) {
      console.error('Error updating question:', error);
      throw error;
    }
  }

  // Get unique topics for filtering
  static async getTopics(userId: string): Promise<string[]> {
    try {
      const questions = await this.getQuestions(userId);
      const topics = new Set(questions.map(q => q.topic).filter(Boolean));
      return Array.from(topics) as string[];
    } catch (error) {
      console.error('Error getting topics:', error);
      return [];
    }
  }
}
