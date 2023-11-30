package day1

object day2 {
  def part1(fileName: String): String = {
    getTotalScore(utils.ioutils.readFileLines(fileName)).toString
  }

  private def getTotalScore(lines: List[String]): Int = lines.parseToScores.parseToFinalScore.sum

  extension (lines: List[String])
    private def parseToScores: List[(Int, Int)] = lines.map(l => (l.substring(0, 1).parseToScore, l.substring(2, 3).parseToScore))

  extension (input: String) {
    private def parseToScore: Int = input match
      case "A" => 1 // Rock
      case "B" => 2 // Paper
      case "C" => 3 // Scissors
      case "X" => 1 // Rock
      case "Y" => 2 // Paper
      case "Z" => 3 // Scissors
  }

  extension (scores: List[(Int, Int)]) {
    private def parseToFinalScore: List[Int] = scores.map((a, b) => getMatchScore(a, b))
  }

  private def getMatchScore(a: Int, b: Int): Int = {
    // Lost
    if (a == 1 && b == 3) || (a == 2 && b == 1) || (a == 3 && b == 2) then {
      return b
    }
    // Draw
    if (a == 1 && b == 1) || (a == 2 && b == 2) || (a == 3 && b == 3) then {
      return b + 3
    }
    // Win
    else {
      return b + 6
    }
  }


  /**
   * Y -> Draw = 3
   * X -> Lose = 0
   * Z -> Win  = 6
   **/
  def part2(fileName: String): String = {
    getTotalScore(utils.ioutils.readFileLines(fileName)).toString
  }
}
