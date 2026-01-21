#include "QuestionnaireLoader.h"
#include <iostream>
#include <cassert>

void testJsonLoader() {
    SessionManager manager;
    
    // Load questionnaire from JSON
    int qId = QuestionnaireLoader::loadFromJsonFile(manager, "sample_exam.json");
    assert(qId > 0);
    
    auto questionnaire = manager.getQuestionnaire(qId);
    assert(questionnaire != nullptr);
    assert(questionnaire->getTitle() == "C++ Programming Quiz");
    assert(questionnaire->getQuestionCount() == 5);
    assert(questionnaire->getTimeLimit() == 30);
    
    std::cout << "✓ JSON loader test passed" << std::endl;
}

void testResultsDisplay() {
    SessionManager manager;
    
    // Load and create a session
    int qId = QuestionnaireLoader::loadFromJsonFile(manager, "sample_exam.json");
    int sessionId = manager.startSession(1, "Test Student", qId);
    
    // Submit answers
    manager.submitAnswer(sessionId, 1, "4 bytes");
    manager.submitAnswer(sessionId, 2, "True");
    
    // Complete and display
    manager.completeSession(sessionId);
    
    std::cout << "✓ Results display test passed" << std::endl;
}

int main() {
    std::cout << "Running JSON loader tests..." << std::endl;
    std::cout << std::endl;
    
    try {
        testJsonLoader();
        testResultsDisplay();
        
        std::cout << std::endl;
        std::cout << "===== All JSON loader tests passed! =====" << std::endl;
        return 0;
    } catch (const std::exception& e) {
        std::cerr << "Test failed with exception: " << e.what() << std::endl;
        return 1;
    }
}
