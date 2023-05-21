#include "../include/karnaugh.hpp"
#include "../include/logger.hpp"
#include <iostream>

int main() {

  Logger logger;
  logger.log("+------------------------+\n"
             "|  Karnaugh Map Solver   |\n"
             "+------------------------+\n");
  logger.log("Enter the number of variables (4 or 5): ");
  logger.log("\nyou can exit the program by typing 'q'\n");
  logger.getInput(">> ");
  logger.log("You entered: " + logger.getInput() + "\n");

  Karnaugh karnaugh;
  if (karnaugh.isFourVariable(logger.getInput())) {
    logger.log(karnaugh.createFourVariableMap());
  } else {
    logger.log(karnaugh.createFiveVariableMap());
  }

  return EXIT_SUCCESS;
}