using AoC.Days.Day1;

namespace Aoc.UnitTests.Day1Tests;

[TestFixture]
public static class Day1Tests {
        
    [Test]
    public static void TestPart1() {
        var input = $"""
                     3   4
                     4   3
                     2   5
                     1   3
                     3   9
                     3   3
                     """.Split("\r\n", StringSplitOptions.RemoveEmptyEntries);
        
        Assert.That(Day1.Part1(input), Is.EqualTo(11));
    }
    
    [Test]
    public static void TestPart2() {
        var input = $"""
                     3   4
                     4   3
                     2   5
                     1   3
                     3   9
                     3   3
                     """.Split("\r\n", StringSplitOptions.RemoveEmptyEntries);
        
        Assert.That(Day1.Part2(input), Is.EqualTo(31));
    }
} 