#include "SessionManager.h"
#include "QuestionnaireLoader.h"
#include <iostream>
#include <string>

int main(int argc, char* argv[]) {
    std::cout << "===== ClassRoom JSON Exam Loader Demo =====" << std::endl;
    
    SessionManager manager;
    
    // Determine JSON file to load
    std::string jsonFile = "sample_exam.json";
    if (argc > 1) {
        jsonFile = argv[1];
    }
    
    std::cout << "Loading exam from: " << jsonFile << std::endl;
    
    // Load questionnaire from JSON file
    int questionnaireId = QuestionnaireLoader::loadFromJsonFile(manager, jsonFile);
    
    if (questionnaireId == -1) {
        std::cerr << "Failed to load questionnaire from JSON file!" << std::endl;
        return 1;
    }
    
    std::cout << "Successfully loaded questionnaire (ID: " << questionnaireId << ")" << std::endl;
    
    // Display questionnaire details
    auto questionnaire = manager.getQuestionnaire(questionnaireId);
    if (questionnaire) {
        std::cout << "\n===== Questionnaire Details =====" << std::endl;
        std::cout << "Title: " << questionnaire->getTitle() << std::endl;
        std::cout << "Description: " << questionnaire->getDescription() << std::endl;
        std::cout << "Time Limit: " << questionnaire->getTimeLimit() << " minutes" << std::endl;
        std::cout << "Number of Questions: " << questionnaire->getQuestionCount() << std::endl;
        std::cout << "Total Points: " << questionnaire->getTotalPoints() << std::endl;
        
        std::cout << "\n===== Questions =====" << std::endl;
        for (const auto& question : questionnaire->getQuestions()) {
            std::cout << "\nQ" << question->getId() << ": " << question->getQuestionText() << std::endl;
            std::cout << "Type: ";
            switch (question->getType()) {
                case QuestionType::MULTIPLE_CHOICE:
                    std::cout << "Multiple Choice";
                    break;
                case QuestionType::TRUE_FALSE:
                    std::cout << "True/False";
                    break;
                case QuestionType::SHORT_ANSWER:
                    std::cout << "Short Answer";
                    break;
                case QuestionType::ESSAY:
                    std::cout << "Essay";
                    break;
            }
            std::cout << std::endl;
            
            const auto& options = question->getOptions();
            if (!options.empty()) {
                std::cout << "Options: ";
                for (size_t i = 0; i < options.size(); i++) {
                    std::cout << options[i];
                    if (i < options.size() - 1) std::cout << ", ";
                }
                std::cout << std::endl;
            }
            std::cout << "Points: " << question->getPoints() << std::endl;
        }
    }
    
    // Simulate a student taking the exam
    std::cout << "\n===== Simulating Student Exam =====" << std::endl;
    int sessionId = manager.startSession(1001, "John Doe", questionnaireId);
    std::cout << "Student 'John Doe' started exam (Session ID: " << sessionId << ")" << std::endl;
    
    // Submit some answers
    std::cout << "\nSubmitting answers..." << std::endl;
    manager.submitAnswer(sessionId, 1, "4 bytes");  // Correct
    manager.submitAnswer(sessionId, 2, "True");     // Correct
    manager.submitAnswer(sessionId, 3, "Standard Template Library");  // Correct
    manager.submitAnswer(sessionId, 4, "malloc");   // Incorrect
    manager.submitAnswer(sessionId, 5, "True");     // Correct
    
    // Complete the session
    std::cout << "Completing exam..." << std::endl;
    manager.completeSession(sessionId);
    
    // Display results
    QuestionnaireLoader::displayResults(manager, sessionId);
    
    std::cout << "\n===== Demo completed successfully! =====" << std::endl;
    
    return 0;
}
