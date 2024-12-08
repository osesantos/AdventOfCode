using AoC.Days.Day7;

namespace Aoc.UnitTests.Day7Tests;

[TestFixture]
public static class Day7Tests {

    private static string[] Input => $"""
         190: 10 19
         3267: 81 40 27
         83: 17 5
         156: 15 6
         7290: 6 8 6 15
         161011: 16 10 13
         192: 17 8 14
         21037: 9 7 18 13
         292: 11 6 16 20
         """
        .Split(Environment.NewLine);

    [Test]
    public static void TestPart1() {
        Assert.That(Day7.Part1(Input), Is.EqualTo(3749));
    }

    [Test]
    public static void TestPart2() {
        Assert.That(Day7.Part2(Input), Is.EqualTo(0));
    }

    [Test]
    public static void TestPart1_2() {
        Assert.That(Day7_2.Part1(Input), Is.EqualTo(3749));
    }

    [Test]
    public static void TestPart2_2() {
        Assert.That(Day7_2.Part2(Input), Is.EqualTo(11387));
    }
}
