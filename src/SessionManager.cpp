#include "SessionManager.h"
#include <stdexcept>

SessionManager::SessionManager() : nextSessionId(1), nextQuestionnaireId(1) {}

int SessionManager::createQuestionnaire(const std::string& title, const std::string& description, int timeLimit) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    int id = nextQuestionnaireId++;
    auto questionnaire = std::make_shared<Questionnaire>(id, title, description, timeLimit);
    questionnaires[id] = questionnaire;
    return id;
}

bool SessionManager::addQuestionToQuestionnaire(int questionnaireId, std::shared_ptr<Question> question) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    auto it = questionnaires.find(questionnaireId);
    if (it == questionnaires.end()) {
        return false;
    }
    it->second->addQuestion(question);
    return true;
}

std::shared_ptr<Questionnaire> SessionManager::getQuestionnaire(int questionnaireId) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    auto it = questionnaires.find(questionnaireId);
    return (it != questionnaires.end()) ? it->second : nullptr;
}

int SessionManager::startSession(int studentId, const std::string& studentName, int questionnaireId) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    
    // Verify questionnaire exists
    if (questionnaires.find(questionnaireId) == questionnaires.end()) {
        throw std::runtime_error("Questionnaire not found");
    }
    
    int sessionId = nextSessionId++;
    auto session = std::make_shared<StudentSession>(sessionId, studentId, studentName, questionnaireId);
    activeSessions[sessionId] = session;
    return sessionId;
}

bool SessionManager::submitAnswer(int sessionId, int questionId, const std::string& answer) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    auto it = activeSessions.find(sessionId);
    if (it == activeSessions.end()) {
        return false;
    }
    it->second->submitAnswer(questionId, answer);
    return true;
}

bool SessionManager::completeSession(int sessionId) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    auto it = activeSessions.find(sessionId);
    if (it == activeSessions.end()) {
        return false;
    }
    
    auto session = it->second;
    session->completeSession();
    
    // Calculate score - check questionnaire exists
    auto questIt = questionnaires.find(session->getQuestionnaireId());
    if (questIt != questionnaires.end()) {
        session->calculateScore(*questIt->second);
    }
    
    // Move to completed sessions
    completedSessions[sessionId] = session;
    activeSessions.erase(it);
    return true;
}

std::shared_ptr<StudentSession> SessionManager::getSession(int sessionId) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    
    auto it = activeSessions.find(sessionId);
    if (it != activeSessions.end()) {
        return it->second;
    }
    
    auto it2 = completedSessions.find(sessionId);
    if (it2 != completedSessions.end()) {
        return it2->second;
    }
    
    return nullptr;
}

std::vector<std::shared_ptr<StudentSession>> SessionManager::getActiveSessionsForQuestionnaire(int questionnaireId) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    std::vector<std::shared_ptr<StudentSession>> result;
    
    for (const auto& [sessionId, session] : activeSessions) {
        if (session->getQuestionnaireId() == questionnaireId) {
            result.push_back(session);
        }
    }
    return result;
}

std::vector<std::shared_ptr<StudentSession>> SessionManager::getCompletedSessionsForQuestionnaire(int questionnaireId) {
    std::lock_guard<std::mutex> lock(sessionMutex);
    std::vector<std::shared_ptr<StudentSession>> result;
    
    for (const auto& [sessionId, session] : completedSessions) {
        if (session->getQuestionnaireId() == questionnaireId) {
            result.push_back(session);
        }
    }
    return result;
}

int SessionManager::getActiveSessionCount() const {
    std::lock_guard<std::mutex> lock(sessionMutex);
    return activeSessions.size();
}

int SessionManager::getCompletedSessionCount() const {
    std::lock_guard<std::mutex> lock(sessionMutex);
    return completedSessions.size();
}

double SessionManager::getAverageScore(int questionnaireId) const {
    std::lock_guard<std::mutex> lock(sessionMutex);
    int totalScore = 0;
    int count = 0;
    
    for (const auto& [sessionId, session] : completedSessions) {
        if (session->getQuestionnaireId() == questionnaireId) {
            totalScore += session->getScore();
            count++;
        }
    }
    
    return (count > 0) ? static_cast<double>(totalScore) / count : 0.0;
}
