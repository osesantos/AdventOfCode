using AoC.Days.Day4;

namespace Aoc.UnitTests.Day4Tests;

[TestFixture]
public static class Day4Tests {

    [Test]
    public static void TestPart1() {
        var input = $"""
                     MMMSXXMASM
                     MSAMXMSMSA
                     AMXSXMAAMM
                     MSAMASMSMX
                     XMASAMXAMM
                     XXAMMXXAMA
                     SMSMSASXSS
                     SAXAMASAAA
                     MAMMMXMMMM
                     MXMXAXMASX
                     """.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);

        Assert.That(Day4.Part1(input), Is.EqualTo(18));
    }

    [Test]
    public static void TestPart2() {
        var input = $"""
                     MMMSXXMASM
                     MSAMXMSMSA
                     AMXSXMAAMM
                     MSAMASMSMX
                     XMASAMXAMM
                     XXAMMXXAMA
                     SMSMSASXSS
                     SAXAMASAAA
                     MAMMMXMMMM
                     MXMXAXMASX
                     """.Split(Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);

        Assert.That(Day4.Part2(input), Is.EqualTo(9));
    }
}