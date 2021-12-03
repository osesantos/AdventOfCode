import java.util.function.IntBinaryOperator

def day3(): Unit =
  val input = "day3".getInput
  day3_1(input)
  day3_2(input)

extension(lst: List[String])
  def findTheMostCommon(): String =
    val zeros = lst.count(s => s == "0")
    val ones = lst.count(s => s == "1")
    if (zeros > ones) "0"
    else if (zeros < ones) "1"
    else "1"

  def findTheLeastCommon(): String =
    val zeros = lst.count(s => s == "0")
    val ones = lst.count(s => s == "1")
    if (zeros < ones) "0"
    else if (zeros > ones) "1"
    else "0"

  def getColumnPerIndex(index: Int): List[String] = lst.map(s => s(index).toString)

  def getGammaRate: String =
    val numberSize = lst.head.length() - 1
    var rate = ""
    for(i <- 0 to numberSize)
      rate = rate + lst.getColumnPerIndex(i).findTheMostCommon()
    rate

  def getEpsilonRate: String =
    val numberSize = lst.head.length() - 1
    var rate = ""
    for(i <- 0 to numberSize)
      rate = rate + lst.getColumnPerIndex(i).findTheLeastCommon()
    rate

  def filterPerValue(value: String, index: Int): List[String] = lst.filter(s => s(index).toString == value)

  def getO2Rate: String =
    val numberSize = lst.head.length() - 1
    var tempLst = lst
    for(i <- 0 to numberSize if tempLst.length > 1)
      val mostCommon = tempLst.getColumnPerIndex(i).findTheMostCommon()
      tempLst = tempLst.filterPerValue(mostCommon, i)
    tempLst.head

  def getCO2Rate: String =
    val numberSize = lst.head.length() - 1
    var tempLst = lst
    for(i <- 0 to numberSize if tempLst.length > 1)
      val leastCommon = tempLst.getColumnPerIndex(i).findTheLeastCommon()
      tempLst = tempLst.filterPerValue(leastCommon, i)
    tempLst.head


extension(str: String)
  def toDecimal: Int = Integer.parseInt(str, 2)


def multiply(a: Int, b: Int): Int = a * b

def day3_1(input: List[String]): Unit =
  val gammaRate = input.getGammaRate.toDecimal
  val epsilonRate = input.getEpsilonRate.toDecimal
  println(s"Day 3 - PART 1: ${multiply(gammaRate, epsilonRate)}")

def day3_2(input: List[String]): Unit =
  val O2Rate = input.getO2Rate.toDecimal
  val CO2Rate = input.getCO2Rate.toDecimal
  println(s"Day 3 - PART 2: ${multiply(O2Rate, CO2Rate)}")





