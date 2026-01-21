#include "SessionManager.h"
#include <iostream>
#include <cassert>
#include <thread>
#include <vector>

void testBasicQuestionnaire() {
    SessionManager manager;
    
    // Create questionnaire
    int qId = manager.createQuestionnaire("Test Quiz", "A test questionnaire", 30);
    assert(qId > 0);
    
    auto questionnaire = manager.getQuestionnaire(qId);
    assert(questionnaire != nullptr);
    assert(questionnaire->getTitle() == "Test Quiz");
    
    std::cout << "✓ Basic questionnaire creation test passed" << std::endl;
}

void testQuestionAddition() {
    SessionManager manager;
    int qId = manager.createQuestionnaire("Test Quiz", "Description", 30);
    
    auto q1 = std::make_shared<Question>(1, "What is 2+2?", QuestionType::MULTIPLE_CHOICE, 1);
    q1->setCorrectAnswer("4");
    
    bool added = manager.addQuestionToQuestionnaire(qId, q1);
    assert(added);
    
    auto questionnaire = manager.getQuestionnaire(qId);
    assert(questionnaire->getQuestionCount() == 1);
    
    std::cout << "✓ Question addition test passed" << std::endl;
}

void testStudentSession() {
    SessionManager manager;
    int qId = manager.createQuestionnaire("Test Quiz", "Description", 30);
    
    auto q1 = std::make_shared<Question>(1, "What is 2+2?", QuestionType::MULTIPLE_CHOICE, 1);
    q1->setCorrectAnswer("4");
    manager.addQuestionToQuestionnaire(qId, q1);
    
    // Start session
    int sessionId = manager.startSession(101, "Test Student", qId);
    assert(sessionId > 0);
    
    // Submit answer
    bool submitted = manager.submitAnswer(sessionId, 1, "4");
    assert(submitted);
    
    // Complete session
    bool completed = manager.completeSession(sessionId);
    assert(completed);
    
    // Check score
    auto session = manager.getSession(sessionId);
    assert(session != nullptr);
    assert(session->getScore() == 1);
    
    std::cout << "✓ Student session test passed" << std::endl;
}

void testConcurrentSessions() {
    SessionManager manager;
    int qId = manager.createQuestionnaire("Test Quiz", "Description", 30);
    
    auto q1 = std::make_shared<Question>(1, "Question 1", QuestionType::MULTIPLE_CHOICE, 1);
    q1->setCorrectAnswer("A");
    manager.addQuestionToQuestionnaire(qId, q1);
    
    const int NUM_STUDENTS = 50;
    std::vector<std::thread> threads;
    
    auto studentTask = [&manager, qId](int studentId) {
        int sessionId = manager.startSession(studentId, "Student_" + std::to_string(studentId), qId);
        manager.submitAnswer(sessionId, 1, "A");
        manager.completeSession(sessionId);
    };
    
    for (int i = 1; i <= NUM_STUDENTS; i++) {
        threads.emplace_back(studentTask, i);
    }
    
    for (auto& t : threads) {
        t.join();
    }
    
    assert(manager.getCompletedSessionCount() == NUM_STUDENTS);
    assert(manager.getActiveSessionCount() == 0);
    
    std::cout << "✓ Concurrent sessions test passed (50 students)" << std::endl;
}

void testAnswerValidation() {
    SessionManager manager;
    int qId = manager.createQuestionnaire("Test Quiz", "Description", 30);
    
    auto q1 = std::make_shared<Question>(1, "Is this correct?", QuestionType::TRUE_FALSE, 1);
    q1->setCorrectAnswer("True");
    manager.addQuestionToQuestionnaire(qId, q1);
    
    // Correct answer
    int session1 = manager.startSession(1, "Student1", qId);
    manager.submitAnswer(session1, 1, "True");
    manager.completeSession(session1);
    assert(manager.getSession(session1)->getScore() == 1);
    
    // Wrong answer
    int session2 = manager.startSession(2, "Student2", qId);
    manager.submitAnswer(session2, 1, "False");
    manager.completeSession(session2);
    assert(manager.getSession(session2)->getScore() == 0);
    
    std::cout << "✓ Answer validation test passed" << std::endl;
}

void testStatistics() {
    SessionManager manager;
    int qId = manager.createQuestionnaire("Test Quiz", "Description", 30);
    
    auto q1 = std::make_shared<Question>(1, "Question", QuestionType::MULTIPLE_CHOICE, 10);
    q1->setCorrectAnswer("A");
    manager.addQuestionToQuestionnaire(qId, q1);
    
    // Student 1: correct answer
    int s1 = manager.startSession(1, "S1", qId);
    manager.submitAnswer(s1, 1, "A");
    manager.completeSession(s1);
    
    // Student 2: wrong answer
    int s2 = manager.startSession(2, "S2", qId);
    manager.submitAnswer(s2, 1, "B");
    manager.completeSession(s2);
    
    double avg = manager.getAverageScore(qId);
    assert(avg == 5.0); // (10 + 0) / 2 = 5
    
    std::cout << "✓ Statistics test passed" << std::endl;
}

int main() {
    std::cout << "Running questionnaire backend tests..." << std::endl;
    std::cout << std::endl;
    
    try {
        testBasicQuestionnaire();
        testQuestionAddition();
        testStudentSession();
        testConcurrentSessions();
        testAnswerValidation();
        testStatistics();
        
        std::cout << std::endl;
        std::cout << "===== All tests passed! =====" << std::endl;
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Test failed with exception: " << e.what() << std::endl;
        return 1;
    }
}
