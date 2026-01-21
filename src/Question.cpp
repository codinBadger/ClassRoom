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
    // Case-insensitive comparison for short answers
    if (type == QuestionType::SHORT_ANSWER || type == QuestionType::ESSAY) {
        std::string lowerAnswer = answer;
        std::string lowerCorrect = correctAnswer;
        std::transform(lowerAnswer.begin(), lowerAnswer.end(), lowerAnswer.begin(), ::tolower);
        std::transform(lowerCorrect.begin(), lowerCorrect.end(), lowerCorrect.begin(), ::tolower);
        return lowerAnswer == lowerCorrect;
    }
    return answer == correctAnswer;
}
