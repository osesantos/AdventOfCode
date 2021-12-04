import scala.collection.mutable.ListBuffer

def day4(): Unit =
  val input = "day4".getInput
  day4_1(input)
  day4_2(input)


extension(lst: List[Int])
  def score(numberCalled: Int) = lst.sum * numberCalled

  def getNextNumber: (List[Int], Int) = (lst.splitAt(1)(1), lst.head)


extension(str: String)
  def toRow: List[Int] =
    if (str.nonEmpty)
      str.split(" ").map(e => if(e.nonEmpty) e.toInt else -1).toList.filter(e => e >= 0)
    else
      List(-1)

  def toNumbers: List[Int] =
    str.split(",").map(e => e.toInt).toList


extension(lst: List[String])
  def parse: List[List[Int]] = lst.zipWithIndex.map((e: String, i) =>
    if (i == 0)
      e.toNumbers
    else
      e.toRow
  )


extension(lst: List[List[Int]])
  def toBoard: List[List[(Int, Boolean)]] = lst.map(l => l.map(e => (e, false)))


extension(lst : List[List[(Int, Boolean)]])
  def markInAllBoards(number: Int) = lst.map(l => l.map(e => (e, e._1 == number)))

/*  def getWinnerIndex: Int = lst.zipWithIndex.foreach(
    (l, i) =>

  )*/


def day4_1(input: List[String]): Unit =
  val parsedInput = input.parse
  val numbers = parsedInput.head
  val boards = parsedInput.tail.toBoard

  boards.foreach(l => println(s"${l.foreach(e => print(s" $e "))}"))

  println(s"Day 4 - PART 1: ")

def day4_2(input: List[String]): Unit =
  println(s"Day 4 - PART 2: ")





