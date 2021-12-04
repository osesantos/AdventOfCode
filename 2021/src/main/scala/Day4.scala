import scala.collection.mutable.ListBuffer
import scala.language.postfixOps

def day4(): Unit =
  val input = "day4".getInput
  day4_1(input)
  day4_2(input)


extension(lst: List[Int])
  def score(numberCalled: Int) = lst.sum * numberCalled

  def getNextNumber: (List[Int], Int) = (lst.tail, lst.head)


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
    var tempList = ListBuffer[Board]()
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
    var list = ListBuffer[ListBuffer[(Int, Boolean)]]()
    lst.foreach { (r: List[(Int, Boolean)]) =>
      var rowToAdd = ListBuffer[(Int, Boolean)]()
      r.foreach(e => rowToAdd += e)
      list += rowToAdd
    }
    list

  def checkRowsForBingo: Boolean =
    var rowBingo = false
    for(r <- lst if !rowBingo)
      var rowsMarked: List[(Int, Boolean)] = r.filter((i, b) => b)
      if(rowsMarked.length == 5)
        rowBingo = true
    rowBingo

  def pivot: List[List[(Int, Boolean)]] =
    var pivotedTable = lst.cloneToListBuffer
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
}

def day4_1(input: List[String]): Unit =
  val parsedInput = input.parse
  val numbers = parsedInput.head
  var boards = parsedInput.tail.addBool.toBoard

  for(n <- numbers if !boards.isBingo)
    //println(n)
    boards = boards.markInAllBoards(n)


  //boards.foreach(b => b.printBoard())

  println(s"Day 4 - PART 1: ")

def day4_2(input: List[String]): Unit =
  println(s"Day 4 - PART 2: ")





