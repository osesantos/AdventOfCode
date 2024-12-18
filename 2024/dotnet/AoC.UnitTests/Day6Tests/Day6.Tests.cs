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
    [Ignore("Failing, Refactor part 2")]
    public static void TestPart2() {
        Assert.That(Day6.Part2(Input), Is.EqualTo(6));
    }

    [Test]
    public static void TestPart1_2() {
        Assert.That(Day6_2.Part1(Input), Is.EqualTo(41));
    }

    [Test]
    public static void TestPart2_2() {
        Assert.That(Day6_2.Part2(Input), Is.EqualTo(6));
    }
}
