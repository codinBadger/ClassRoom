#ifndef SESSION_MANAGER_H
#define SESSION_MANAGER_H

#include "StudentSession.h"
#include "Questionnaire.h"
#include <map>
#include <memory>
#include <mutex>
#include <vector>

class SessionManager {
private:
    std::map<int, std::shared_ptr<Questionnaire>> questionnaires;
    std::map<int, std::shared_ptr<StudentSession>> activeSessions;
    std::map<int, std::shared_ptr<StudentSession>> completedSessions;
    mutable std::mutex sessionMutex;
    int nextSessionId;
    int nextQuestionnaireId;

public:
    SessionManager();
    
    // Questionnaire management
    int createQuestionnaire(const std::string& title, const std::string& description, int timeLimit = 0);
    bool addQuestionToQuestionnaire(int questionnaireId, std::shared_ptr<Question> question);
    std::shared_ptr<Questionnaire> getQuestionnaire(int questionnaireId);
    
    // Session management
    int startSession(int studentId, const std::string& studentName, int questionnaireId);
    bool submitAnswer(int sessionId, int questionId, const std::string& answer);
    bool completeSession(int sessionId);
    
    std::shared_ptr<StudentSession> getSession(int sessionId);
    std::vector<std::shared_ptr<StudentSession>> getActiveSessionsForQuestionnaire(int questionnaireId);
    std::vector<std::shared_ptr<StudentSession>> getCompletedSessionsForQuestionnaire(int questionnaireId);
    
    // Statistics
    int getActiveSessionCount() const;
    int getCompletedSessionCount() const;
    double getAverageScore(int questionnaireId) const;
};

#endif
