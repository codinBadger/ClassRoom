#ifndef STUDENT_SESSION_H
#define STUDENT_SESSION_H

#include "Questionnaire.h"
#include <string>
#include <map>
#include <chrono>

class StudentSession {
private:
    int sessionId;
    int studentId;
    std::string studentName;
    int questionnaireId;
    std::map<int, std::string> answers; // questionId -> answer
    std::chrono::system_clock::time_point startTime;
    std::chrono::system_clock::time_point endTime;
    int score;
    bool isCompleted;

public:
    StudentSession(int sessionId, int studentId, const std::string& studentName, int questionnaireId);
    
    void submitAnswer(int questionId, const std::string& answer);
    void completeSession();
    void calculateScore(const Questionnaire& questionnaire);
    
    int getSessionId() const { return sessionId; }
    int getStudentId() const { return studentId; }
    std::string getStudentName() const { return studentName; }
    int getQuestionnaireId() const { return questionnaireId; }
    const std::map<int, std::string>& getAnswers() const { return answers; }
    int getScore() const { return score; }
    bool getIsCompleted() const { return isCompleted; }
    
    std::chrono::duration<double> getElapsedTime() const;
};

#endif
