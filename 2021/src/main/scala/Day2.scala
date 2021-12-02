def day2(): Unit =
  val input = "day2".getInput
  day2_1(input)
  day2_2(input)

def parseDepthInstruction(str: String): Int =
  val strParsed = str.split(" ")
  if (strParsed(0) == "up")
    -strParsed(1).toInt
  else if (strParsed(0) == "down")
    strParsed(1).toInt
  else
    0

def parseMovementInstruction(str: String): Int =
  val strParsed = str.split(" ")
  if (strParsed(0) == "forward")
    strParsed(1).toInt
  else
    0

def multiply(a: Int, b: Int): Int = a * b

def parseMovementToDepthInstruction(str: String, currentAim: Int): Int =
  val strParsed = str.split(" ")
  if (strParsed(0) == "forward")
    multiply(currentAim, strParsed(1).toInt)
  else
    0

def day2_1(input: List[String]): Unit =
  // Day 2 Part 1
  var totalMovement = 0
  var totalDepth = 0
  input.foreach { elem =>
    totalDepth += parseDepthInstruction(elem)
    totalMovement += parseMovementInstruction(elem)
  }
  println(s"Day 2 - PART 1: ${multiply(totalDepth, totalMovement)}")

def day2_2(input: List[String]): Unit =
  // Day 2 Part 1
  var aim = 0
  var totalMovement = 0
  var totalDepth = 0
  input.zipWithIndex.foreach{ case(elem, i) =>
    aim += parseDepthInstruction(elem)
    totalMovement += parseMovementInstruction(elem)
    totalDepth += parseMovementToDepthInstruction(elem, aim)
  }
  println(s"Day 2 - PART 2: ${multiply(totalDepth, totalMovement)}")