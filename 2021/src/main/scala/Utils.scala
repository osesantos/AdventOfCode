import scala.io.Source

extension (str: String) {
  def getInput: List[String] = Source.fromFile(s"src/main/input/$str.txt").getLines().toList
}

