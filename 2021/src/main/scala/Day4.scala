import scala.collection.mutable.ListBuffer
import scala.language.postfixOps

def day4(): Unit =
  val input = "day4".getInput
  day4_1(input)
  day4_2(input)


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
  def addBool: List[List[(Int, Boolean)]] = lst.map(l => l.map(e => (e, false)))


extension(lst: List[List[(Int, Boolean)]])
  def toBoard: List[Board] =
    val tempList = ListBuffer[Board]()
    var tempRows = ListBuffer[List[(Int, Boolean)]]()
    lst.foreach(r =>
      if(r.length == 1)
        tempList += Board(tempRows.toList)
        tempRows = ListBuffer[List[(Int, Boolean)]]()
      else
        tempRows += r
    )
    tempList += Board(tempRows.toList)
    tempRows = ListBuffer[List[(Int, Boolean)]]()
    tempList.toList.tail

  def cloneToListBuffer: ListBuffer[ListBuffer[(Int, Boolean)]] =
    val list = ListBuffer[ListBuffer[(Int, Boolean)]]()
    lst.foreach { (r: List[(Int, Boolean)]) =>
      val rowToAdd = ListBuffer[(Int, Boolean)]()
      r.foreach(e => rowToAdd += e)
      list += rowToAdd
    }
    list

  def checkRowsForBingo: Boolean =
    var rowBingo = false
    for(r <- lst if !rowBingo)
      val rowsMarked: List[(Int, Boolean)] = r.filter((i, b) => b)
      if(rowsMarked.length == 5)
        rowBingo = true
    rowBingo

  def pivot: List[List[(Int, Boolean)]] =
    val pivotedTable = lst.cloneToListBuffer
    for(y <- 0 to 4; x <- 0 to 4)
      pivotedTable(y)(x) = lst(x)(y)
    pivotedTable.map(r => r.toList).toList


extension(lst : List[Board])
  def markInAllBoards(number: Int): List[Board] = lst.map(b =>b.markInAllRows(number))

  def isBingo: Boolean =
    var isBingo = false
    for (b <- lst if !isBingo)
      isBingo = b.isBingo()
    isBingo

  def isBingoIndex: Int =
    var index = -1
    for (i <- 0 to lst.length if index == -1)
      if(lst(i).isBingo())
        index = i
    index

class Board(var rows: List[List[(Int, Boolean)]] = List[List[(Int, Boolean)]]()){

  def markInAllRows(number: Int): Board =
    rows = rows.map(l => l.map { e =>
      (e._1, e._2 || e._1 == number)
    })
    this

  def isBingo(): Boolean =
    var isBingo = false
    if(rows.checkRowsForBingo) isBingo = true
    if(rows.pivot.checkRowsForBingo) isBingo = true
    isBingo

  def printBoard(): Unit =
    rows.foreach(l => println(s"${l.foreach(e => print(s" $e "))}"))

  def sumUnmarked(): Int =
    var sum = 0
    rows.foreach(r => r.foreach(e => if !e._2 then sum += e._1))
    sum
}

def day4_1(input: List[String]): Unit =
  val parsedInput = input.parse
  val numbers = parsedInput.head
  var boards = parsedInput.tail.addBool.toBoard

  var score = 0
  for(n <- numbers if !boards.isBingo)
    boards = boards.markInAllBoards(n)
    if(boards.isBingo)
      score = boards(boards.isBingoIndex).sumUnmarked() * n

  println(s"Day 4 - PART 1: $score")

def day4_2(input: List[String]): Unit =
  println(s"Day 4 - PART 2: ")





