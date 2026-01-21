#include "Question.h"
#include <algorithm>

Question::Question(int id, const std::string& text, QuestionType type, int points)
    : id(id), questionText(text), type(type), points(points) {}

void Question::addOption(const std::string& option) {
    options.push_back(option);
}

void Question::setCorrectAnswer(const std::string& answer) {
    correctAnswer = answer;
}

bool Question::checkAnswer(const std::string& answer) const {
    // Case-insensitive comparison for short answers using std::equal
    if (type == QuestionType::SHORT_ANSWER || type == QuestionType::ESSAY) {
        if (answer.length() != correctAnswer.length()) {
            return false;
        }
        return std::equal(answer.begin(), answer.end(), correctAnswer.begin(),
            [](char a, char b) { return std::tolower(a) == std::tolower(b); });
    }
    return answer == correctAnswer;
}
