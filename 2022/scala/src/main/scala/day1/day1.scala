package day1

import scala.:+
import scala.collection.mutable.ListBuffer
import scala.util.control.Breaks.break

object day1 {

  def part1 (fileName: String): String = {
    findTheMostCal(getListWithSums(utils.ioutils.readFileLines(fileName))).toString
  }

  def part2(fileName: String): String = {
    findThe3MostCalSum(getListWithSums(utils.ioutils.readFileLines(fileName))).toString
  }

  private def getListWithSums(lines: List[String]): List[Int] = {
    val sumList = ListBuffer[Int]()
    var sum = 0
    lines.foreach(l => {
      if l == "" then {
        sumList.addOne(sum)
        sum = 0
      } else {
        sum += Integer.parseInt(l)
      }
    })
    sumList.toList
  }

  private def findTheMostCal(cals: List[Int]): Int = {
    var mostcal = 0
    cals.foreach(c => {
      if c > mostcal then {
        mostcal = c
      }
    })
    mostcal
  }

  private def findThe3MostCalSum(cals: List[Int]): Int = cals.sortWith((a, b) => a > b).slice(0, 3).sum

}
