using AoC.Days.Day2;

namespace Aoc.UnitTests.Day2Tests;

[TestFixture]
public static class Day2Tests {
        
    [Test]
    public static void TestPart1() {
        var input = $"""
                     7 6 4 2 1
                     1 2 7 8 9
                     9 7 6 2 1
                     1 3 2 4 5
                     8 6 4 4 1
                     1 3 6 7 9
                     """.Split("\r\n", StringSplitOptions.RemoveEmptyEntries);

        Assert.That(Day2.Part1(input), Is.EqualTo(2));
    }
    
    [Test]
    public static void TestPart2() {
        Assert.That(Day2.Part2([]), Is.EqualTo(0));
    }
} 