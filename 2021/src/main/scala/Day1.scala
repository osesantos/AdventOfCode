def day1() =
  val input = "day1".getInput.map(s => s.toInt)
  day1_1(input)
  day1_2(input)

extension (lst: List[Int]) {
  def countIncrease: Int = lst.zipWithIndex.count((e, i) => i > 0 && e > lst(i - 1))
}

def day1_1(input: List[Int]): Unit =
  println(s"Day 1 - PART 1: ${input.countIncrease}")

def day1_2(input: List[Int]): Unit =
  val newList = input.zipWithIndex.map((e, i) =>
    if i < input.length - 2 then
      e + input(i + 1) + input(i + 2)
    else
      0
  )
  println(s"Day 1 - PART 2: ${newList.countIncrease}")





