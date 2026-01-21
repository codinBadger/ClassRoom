#ifndef QUESTION_H
#define QUESTION_H

#include <string>
#include <vector>

enum class QuestionType {
    MULTIPLE_CHOICE,
    TRUE_FALSE,
    SHORT_ANSWER,
    ESSAY
};

class Question {
private:
    int id;
    std::string questionText;
    QuestionType type;
    std::vector<std::string> options;
    std::string correctAnswer;
    int points;

public:
    Question(int id, const std::string& text, QuestionType type, int points = 1);
    
    void addOption(const std::string& option);
    void setCorrectAnswer(const std::string& answer);
    
    int getId() const { return id; }
    std::string getQuestionText() const { return questionText; }
    QuestionType getType() const { return type; }
    const std::vector<std::string>& getOptions() const { return options; }
    std::string getCorrectAnswer() const { return correctAnswer; }
    int getPoints() const { return points; }
    
    bool checkAnswer(const std::string& answer) const;
};

#endif
