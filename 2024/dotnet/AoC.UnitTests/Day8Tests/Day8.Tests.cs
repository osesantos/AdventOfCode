using AoC.Days.Day8;

namespace Aoc.UnitTests.Day8Tests;

[TestFixture]
public static class Day8Tests {

    private static string[] Input => $"""
         ............
         ........0...
         .....0......
         .......0....
         ....0.......
         ......A.....
         ............
         ............
         ........A...
         .........A..
         ............
         ............
         """
        .Split(Environment.NewLine);

    [Test]
    public static void TestPart1() {
       // Assert.That(Day8.Part1_2(Input), Is.EqualTo(14));
    }

    [Test]
    public static void TestPart2() {
        //Assert.That(Day8.Part2(Input), Is.EqualTo(0));
    }

    [Test]
    public static void TestPart1_2() {
        Assert.That(Day8_2.Part1(Input), Is.EqualTo(14));
    }

    [Test]
    public static void TestPart2_2() {
        Assert.That(Day8_2.Part2(Input), Is.EqualTo(34));
    }
}
