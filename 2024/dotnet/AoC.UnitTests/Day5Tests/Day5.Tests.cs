using AoC.Days.Day5;
namespace Aoc.UnitTests.Day5Tests;

[TestFixture]
public static class Day5Tests {
    private static string[] Input => $"""
                                      47|53
                                      97|13
                                      97|61
                                      97|47
                                      75|29
                                      61|13
                                      75|53
                                      29|13
                                      97|29
                                      53|29
                                      61|53
                                      97|53
                                      61|29
                                      47|13
                                      75|47
                                      97|75
                                      47|61
                                      75|61
                                      47|29
                                      75|13
                                      53|13

                                      75,47,61,53,29
                                      97,61,53,29,13
                                      75,29,13
                                      75,97,47,61,53
                                      61,13,29
                                      97,13,75,29,47
                                      """
        .Split(Environment.NewLine);

    [Test]
    public static void TestPart1() {
        Assert.That(Day5.Part1(Input), Is.EqualTo(143));
    }

    [Test]
    public static void TestPart2() {
        Assert.That(Day5.Part2(Input), Is.EqualTo(123));
    }
}
