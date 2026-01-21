#include "StudentSession.h"

StudentSession::StudentSession(int sessionId, int studentId, const std::string& studentName, int questionnaireId)
    : sessionId(sessionId), studentId(studentId), studentName(studentName), 
      questionnaireId(questionnaireId), score(0), isCompleted(false) {
    startTime = std::chrono::system_clock::now();
}

void StudentSession::submitAnswer(int questionId, const std::string& answer) {
    answers[questionId] = answer;
}

void StudentSession::completeSession() {
    endTime = std::chrono::system_clock::now();
    isCompleted = true;
}

void StudentSession::calculateScore(const Questionnaire& questionnaire) {
    score = 0;
    for (const auto& [questionId, answer] : answers) {
        auto question = questionnaire.getQuestionById(questionId);
        if (question && question->checkAnswer(answer)) {
            score += question->getPoints();
        }
    }
}

std::chrono::duration<double> StudentSession::getElapsedTime() const {
    auto end = isCompleted ? endTime : std::chrono::system_clock::now();
    return end - startTime;
}
