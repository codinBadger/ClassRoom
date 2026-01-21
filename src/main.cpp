#include "SessionManager.h"
#include <iostream>
#include <thread>
#include <vector>
#include <chrono>

// Demo function to simulate multiple students taking a questionnaire
void simulateStudent(SessionManager& manager, int questionnaireId, int studentId) {
    std::string studentName = "Student_" + std::to_string(studentId);
    
    try {
        // Start session
        int sessionId = manager.startSession(studentId, studentName, questionnaireId);
        std::cout << studentName << " started session " << sessionId << std::endl;
        
        // Simulate answering questions
        auto questionnaire = manager.getQuestionnaire(questionnaireId);
        if (questionnaire) {
            for (const auto& question : questionnaire->getQuestions()) {
                // Simulate thinking time
                std::this_thread::sleep_for(std::chrono::milliseconds(100));
                
                // Submit an answer (for demo, using correct answer for half the students)
                std::string answer = (studentId % 2 == 0) ? 
                    question->getCorrectAnswer() : "Wrong Answer";
                manager.submitAnswer(sessionId, question->getId(), answer);
            }
        }
        
        // Complete session
        manager.completeSession(sessionId);
        std::cout << studentName << " completed session " << sessionId << std::endl;
        
    } catch (const std::exception& e) {
        std::cerr << "Error for " << studentName << ": " << e.what() << std::endl;
    }
}

int main() {
    std::cout << "===== ClassRoom Questionnaire Backend =====" << std::endl;
    std::cout << "Initializing session manager..." << std::endl;
    
    SessionManager manager;
    
    // Create a sample questionnaire
    std::cout << "\nCreating questionnaire..." << std::endl;
    int qId = manager.createQuestionnaire(
        "C++ Programming Basics",
        "A short quiz on C++ fundamentals",
        30  // 30 minutes time limit
    );
    
    // Add questions
    auto q1 = std::make_shared<Question>(1, "What is the size of int in C++?", QuestionType::MULTIPLE_CHOICE, 1);
    q1->addOption("2 bytes");
    q1->addOption("4 bytes");
    q1->addOption("8 bytes");
    q1->setCorrectAnswer("4 bytes");
    manager.addQuestionToQuestionnaire(qId, q1);
    
    auto q2 = std::make_shared<Question>(2, "Is C++ object-oriented?", QuestionType::TRUE_FALSE, 1);
    q2->addOption("True");
    q2->addOption("False");
    q2->setCorrectAnswer("True");
    manager.addQuestionToQuestionnaire(qId, q2);
    
    auto q3 = std::make_shared<Question>(3, "What does STL stand for?", QuestionType::SHORT_ANSWER, 2);
    q3->setCorrectAnswer("Standard Template Library");
    manager.addQuestionToQuestionnaire(qId, q3);
    
    std::cout << "Questionnaire created with " << manager.getQuestionnaire(qId)->getQuestionCount() 
              << " questions" << std::endl;
    
    // Simulate multiple students (100 students)
    std::cout << "\nSimulating 100 students taking the questionnaire..." << std::endl;
    const int NUM_STUDENTS = 100;
    std::vector<std::thread> threads;
    
    auto startTime = std::chrono::high_resolution_clock::now();
    
    for (int i = 1; i <= NUM_STUDENTS; i++) {
        threads.emplace_back(simulateStudent, std::ref(manager), qId, i);
    }
    
    // Wait for all students to complete
    for (auto& t : threads) {
        t.join();
    }
    
    auto endTime = std::chrono::high_resolution_clock::now();
    auto duration = std::chrono::duration_cast<std::chrono::milliseconds>(endTime - startTime);
    
    // Print statistics
    std::cout << "\n===== Session Statistics =====" << std::endl;
    std::cout << "Total students: " << NUM_STUDENTS << std::endl;
    std::cout << "Active sessions: " << manager.getActiveSessionCount() << std::endl;
    std::cout << "Completed sessions: " << manager.getCompletedSessionCount() << std::endl;
    std::cout << "Average score: " << manager.getAverageScore(qId) << " / " 
              << manager.getQuestionnaire(qId)->getTotalPoints() << std::endl;
    std::cout << "Time taken: " << duration.count() << " ms" << std::endl;
    
    std::cout << "\n===== Demo completed successfully! =====" << std::endl;
    
    return 0;
}
