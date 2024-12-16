using AoC.Days.Day10;
namespace Aoc.UnitTests.Day10Tests;

[TestFixture]
public static class Day10Tests {

    private static string[] Input => $"""
         89010123
         78121874
         87430965
         96549874
         45678903
         32019012
         01329801
         10456732
         """
        .Split(Environment.NewLine);

    [Test]
    public static void TestPart1() {
        Assert.That(Day10.Part1(Input), Is.EqualTo(0));
    }

    [Test]
    public static void TestPart2() {
        Assert.That(Day10.Part2(Input), Is.EqualTo(0));
    }
}