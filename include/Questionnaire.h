#ifndef QUESTIONNAIRE_H
#define QUESTIONNAIRE_H

#include "Question.h"
#include <vector>
#include <memory>
#include <string>

class Questionnaire {
private:
    int id;
    std::string title;
    std::string description;
    std::vector<std::shared_ptr<Question>> questions;
    int timeLimit; // in minutes, 0 for unlimited
    int totalPoints;

public:
    Questionnaire(int id, const std::string& title, const std::string& description, int timeLimit = 0);
    
    void addQuestion(std::shared_ptr<Question> question);
    const std::vector<std::shared_ptr<Question>>& getQuestions() const { return questions; }
    
    int getId() const { return id; }
    std::string getTitle() const { return title; }
    std::string getDescription() const { return description; }
    int getTimeLimit() const { return timeLimit; }
    int getTotalPoints() const { return totalPoints; }
    size_t getQuestionCount() const { return questions.size(); }
    
    std::shared_ptr<Question> getQuestionById(int questionId) const;
};

#endif
