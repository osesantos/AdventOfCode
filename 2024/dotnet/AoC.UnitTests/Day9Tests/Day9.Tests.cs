using AoC.Days.Day9;

namespace Aoc.UnitTests.Day9Tests;

[TestFixture]
public static class Day9Tests {

    private static string[] Input => $"2333133121414131402".Split(Environment.NewLine);

    [Test]
    public static void TestPart1() {
        Assert.That(Day9.Part1(Input), Is.EqualTo(1928));
    }

    [Test]
    public static void TestPart2() {
        Assert.That(Day9.Part2(Input), Is.EqualTo(2858));
    }

}
