#include "../include/logger.hpp"

Logger::Logger() { message = ""; }
Logger::~Logger() {}

void Logger::log(std::string message) { std::cout << message; }
void Logger::showInput() { std::cout << input << std::endl; }
std::string Logger::getInput() const { return input; }

void Logger::getInput(std::string message) {
  std::cin >> message;
  if (message == "q") {
    exitProgram("Exiting program...");
  }

  if (message == "4" || message == "5") {
    input = message;
  } else {
    std::cout << "Invalid input. Please try again." << std::endl;
    getInput(message);
  }
}

void Logger::exitProgram(std::string message) {
  std::cout << message << std::endl;
  exit(EXIT_SUCCESS);
}

void Logger::clearScreen() {
  std::cout << "\033[2J\033[1;1H";
  std::cout.flush();
}
