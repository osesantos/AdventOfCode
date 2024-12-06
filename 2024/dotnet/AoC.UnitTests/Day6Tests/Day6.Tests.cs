using AoC.Days.Day6;
namespace Aoc.UnitTests.Day6Tests;

[TestFixture]
public static class Day6Tests {
    private static string[] Input => $"""
         ....#.....
         .........#
         ..........
         ..#.......
         .......#..
         ..........
         .#..^.....
         ........#.
         #.........
         ......#...
         """
        .Split(Environment.NewLine);


    [Test]
    public static void TestPart1() {
        Assert.That(Day6.Part1(Input), Is.EqualTo(41));
    }

    [Test]
    public static void TestPart2() {
        Assert.That(Day6.Part2(Input), Is.EqualTo(6));
    }
}