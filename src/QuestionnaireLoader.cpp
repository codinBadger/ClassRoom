#include "QuestionnaireLoader.h"
#include <fstream>
#include <sstream>
#include <iostream>
#include <algorithm>

// Helper function to trim whitespace
static std::string trim(const std::string& str) {
    size_t first = str.find_first_not_of(" \t\n\r");
    if (first == std::string::npos) return "";
    size_t last = str.find_last_not_of(" \t\n\r");
    return str.substr(first, (last - first + 1));
}

// Helper function to remove quotes from string
static std::string removeQuotes(const std::string& str) {
    std::string result = trim(str);
    if (result.length() >= 2 && result.front() == '"' && result.back() == '"') {
        return result.substr(1, result.length() - 2);
    }
    return result;
}

// Helper function to parse question type
static QuestionType parseQuestionType(const std::string& typeStr) {
    std::string type = trim(typeStr);
    std::transform(type.begin(), type.end(), type.begin(), ::toupper);
    
    if (type == "MULTIPLE_CHOICE" || type == "MULTIPLECHOICE" || type == "MCQ") {
        return QuestionType::MULTIPLE_CHOICE;
    } else if (type == "TRUE_FALSE" || type == "TRUEFALSE" || type == "BOOL") {
        return QuestionType::TRUE_FALSE;
    } else if (type == "SHORT_ANSWER" || type == "SHORTANSWER" || type == "SHORT") {
        return QuestionType::SHORT_ANSWER;
    } else if (type == "ESSAY") {
        return QuestionType::ESSAY;
    }
    return QuestionType::MULTIPLE_CHOICE; // Default
}

// Simple JSON parser for our specific format
int QuestionnaireLoader::loadFromJsonFile(SessionManager& manager, const std::string& filename) {
    std::ifstream file(filename);
    if (!file.is_open()) {
        std::cerr << "Error: Cannot open file " << filename << std::endl;
        return -1;
    }
    
    std::string line;
    std::string title, description;
    int timeLimit = 0;
    int numQuestions = 0;
    int questionId = 1;
    int questionnaireId = -1;
    
    // Parse JSON manually (simple approach for our format)
    bool inQuestionnaire = false;
    bool inQuestions = false;
    bool inQuestion = false;
    
    std::string currentQuestionText;
    std::string currentQuestionType;
    std::vector<std::string> currentOptions;
    std::string currentCorrectAnswer;
    int currentPoints = 1;
    
    while (std::getline(file, line)) {
        line = trim(line);
        
        if (line.empty() || line == "{" || line == "}" || line == "[" || line == "]") {
            continue;
        }
        
        // Remove trailing comma
        if (!line.empty() && line.back() == ',') {
            line.pop_back();
            line = trim(line);
        }
        
        // Parse key-value pairs
        size_t colonPos = line.find(':');
        if (colonPos != std::string::npos) {
            std::string key = trim(line.substr(0, colonPos));
            std::string value = trim(line.substr(colonPos + 1));
            
            key = removeQuotes(key);
            
            if (key == "title") {
                title = removeQuotes(value);
            } else if (key == "description") {
                description = removeQuotes(value);
            } else if (key == "timeLimit") {
                try {
                    timeLimit = std::stoi(value);
                } catch (const std::exception& e) {
                    std::cerr << "Warning: Invalid timeLimit value, using 0" << std::endl;
                    timeLimit = 0;
                }
            } else if (key == "numQuestions" || key == "numberOfQuestions") {
                try {
                    numQuestions = std::stoi(value);
                } catch (const std::exception& e) {
                    std::cerr << "Warning: Invalid numQuestions value" << std::endl;
                }
            } else if (key == "questions") {
                inQuestions = true;
                // Create questionnaire now that we have the metadata
                if (questionnaireId == -1) {
                    questionnaireId = manager.createQuestionnaire(title, description, timeLimit);
                }
            } else if (key == "questionText" || key == "text") {
                currentQuestionText = removeQuotes(value);
            } else if (key == "questionType" || key == "type") {
                currentQuestionType = removeQuotes(value);
            } else if (key == "correctAnswer" || key == "answer") {
                currentCorrectAnswer = removeQuotes(value);
            } else if (key == "points") {
                try {
                    currentPoints = std::stoi(value);
                } catch (const std::exception& e) {
                    std::cerr << "Warning: Invalid points value, using 1" << std::endl;
                    currentPoints = 1;
                }
            } else if (key == "options") {
                // Start collecting options
                currentOptions.clear();
                
                // Check if options are on the same line
                if (value.find('[') != std::string::npos) {
                    // Parse inline array
                    size_t start = value.find('[');
                    size_t end = value.find(']');
                    if (end != std::string::npos) {
                        std::string optionsStr = value.substr(start + 1, end - start - 1);
                        std::stringstream ss(optionsStr);
                        std::string option;
                        while (std::getline(ss, option, ',')) {
                            option = removeQuotes(trim(option));
                            if (!option.empty()) {
                                currentOptions.push_back(option);
                            }
                        }
                    }
                }
            }
        } else if (!line.empty() && line.front() == '"') {
            // This is an option in an array
            std::string option = removeQuotes(line);
            if (!option.empty()) {
                currentOptions.push_back(option);
            }
        }
        
        // Check if we should create a question
        if (!currentQuestionText.empty() && !currentQuestionType.empty() && !currentCorrectAnswer.empty()) {
            auto question = std::make_shared<Question>(
                questionId++,
                currentQuestionText,
                parseQuestionType(currentQuestionType),
                currentPoints
            );
            
            for (const auto& option : currentOptions) {
                question->addOption(option);
            }
            question->setCorrectAnswer(currentCorrectAnswer);
            
            manager.addQuestionToQuestionnaire(questionnaireId, question);
            
            // Reset for next question
            currentQuestionText.clear();
            currentQuestionType.clear();
            currentOptions.clear();
            currentCorrectAnswer.clear();
            currentPoints = 1;
        }
    }
    
    file.close();
    
    if (questionnaireId == -1) {
        std::cerr << "Error: Failed to create questionnaire from JSON" << std::endl;
    }
    
    return questionnaireId;
}

void QuestionnaireLoader::displayResults(SessionManager& manager, int sessionId) {
    auto session = manager.getSession(sessionId);
    if (!session) {
        std::cout << "Session not found!" << std::endl;
        return;
    }
    
    auto questionnaire = manager.getQuestionnaire(session->getQuestionnaireId());
    if (!questionnaire) {
        std::cout << "Questionnaire not found!" << std::endl;
        return;
    }
    
    std::cout << "\n========== EXAM RESULTS ==========" << std::endl;
    std::cout << "Student: " << session->getStudentName() << " (ID: " << session->getStudentId() << ")" << std::endl;
    std::cout << "Questionnaire: " << questionnaire->getTitle() << std::endl;
    std::cout << "=================================" << std::endl;
    
    const auto& answers = session->getAnswers();
    int correctCount = 0;
    
    for (const auto& question : questionnaire->getQuestions()) {
        std::cout << "\nQ" << question->getId() << ": " << question->getQuestionText() << std::endl;
        
        auto answerIt = answers.find(question->getId());
        if (answerIt != answers.end()) {
            std::cout << "Your answer: " << answerIt->second << std::endl;
            bool correct = question->checkAnswer(answerIt->second);
            std::cout << "Correct answer: " << question->getCorrectAnswer() << std::endl;
            std::cout << "Status: " << (correct ? "✓ CORRECT" : "✗ INCORRECT") << std::endl;
            std::cout << "Points: " << (correct ? question->getPoints() : 0) << "/" << question->getPoints() << std::endl;
            if (correct) correctCount++;
        } else {
            std::cout << "Your answer: (not answered)" << std::endl;
            std::cout << "Status: ✗ INCORRECT" << std::endl;
            std::cout << "Points: 0/" << question->getPoints() << std::endl;
        }
    }
    
    std::cout << "\n=================================" << std::endl;
    std::cout << "FINAL SCORE: " << session->getScore() << "/" << questionnaire->getTotalPoints() << std::endl;
    std::cout << "Questions Correct: " << correctCount << "/" << questionnaire->getQuestionCount() << std::endl;
    
    double percentage = (questionnaire->getTotalPoints() > 0) 
        ? (static_cast<double>(session->getScore()) / questionnaire->getTotalPoints() * 100.0) 
        : 0.0;
    std::cout << "Percentage: " << percentage << "%" << std::endl;
    
    if (session->getIsCompleted()) {
        std::cout << "Time taken: " << session->getElapsedTime().count() << " seconds" << std::endl;
    }
    
    std::cout << "=================================" << std::endl;
}
