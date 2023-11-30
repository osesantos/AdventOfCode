package utils

import scala.io.Source

object ioutils {

  def readFileLines(fileName: String): List[String] = Source.fromFile("src/main/scala/inputs/" + fileName).getLines().toList

}
