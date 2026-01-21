#include "Questionnaire.h"

Questionnaire::Questionnaire(int id, const std::string& title, const std::string& description, int timeLimit)
    : id(id), title(title), description(description), timeLimit(timeLimit), totalPoints(0) {}

void Questionnaire::addQuestion(std::shared_ptr<Question> question) {
    questions.push_back(question);
    totalPoints += question->getPoints();
}

std::shared_ptr<Question> Questionnaire::getQuestionById(int questionId) const {
    for (const auto& question : questions) {
        if (question->getId() == questionId) {
            return question;
        }
    }
    return nullptr;
}
