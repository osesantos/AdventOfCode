def day1(): Unit =
  val input = "day1".getInput.map(s => s.toInt)
  day1_1(input)
  day1_2(input)

def isSum2020(a: Int, b: Int): Boolean = a + b == 2020

def multiply(a: Int, b: Int): Int = a * b

def isSum2020(a: Int, b: Int, c: Int): Boolean = a + b + c == 2020

def multiply(a: Int, b: Int, c: Int): Int = a * b * c

def day1_1(input: List[Int]): Unit =
  var result: Int = 0
  input.foreach(a => input.foreach(b => if (isSum2020(a, b)) result = multiply(a, b) else 0))
  println(s"Day 1 - Part 1: $result")

def day1_2(input: List[Int]): Unit =
  var result: Int = 0
  input.foreach(a => input.foreach(b => input.foreach(c => if (isSum2020(a, b, c)) result = multiply(a, b, c) else 0)))
  println(s"Day 1 - Part 2: $result")
