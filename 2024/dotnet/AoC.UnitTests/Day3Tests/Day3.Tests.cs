using AoC.Days.Day3;

namespace Aoc.UnitTests.Day3Tests;

[TestFixture]
public static class Day3Tests {

    [Test]
    public static void TestPart1() {
        var input = $"""
                     xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
                     """.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);

        Assert.That(Day3.Part1(input), Is.EqualTo(161));
    }

    [Test]
    public static void TestPart2() {
        var input = $"""
                     xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
                     """.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);

        Assert.That(Day3.Part2(input), Is.EqualTo(48));
    }
}