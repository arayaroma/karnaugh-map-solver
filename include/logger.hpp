#pragma once
#include <iostream>
#include <string>

class Logger {
private:
  std::string message, input;

public:
  Logger();
  ~Logger();

  void log(std::string message);
  void getInput(std::string message);
  std::string getInput();
  void showInput();
  void exitProgram(std::string message);
  void clearScreen();
};