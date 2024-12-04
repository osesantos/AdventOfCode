using System.Text;
using System.Text.RegularExpressions;
using AoC.Utils;

namespace AoC.Day4;

/// <summary>
/// Day4 used for Day class generation
/// When using replace Day4 with the day
/// </summary>
public static class Day4 {

    private static string[] Lines => "Day4".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day4");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        var count = 0;
        // Get horizontal XMAS count
        count += input.Sum(x => x.CountMatches());

        // Get horizontal Reverse XMAS count
        count += input.Sum(x => x.CountMatchesReverse());

        // Get vertical XMAS count
        count += input.Rotate().Sum(x => x.CountMatches());

        // Get vertical Reverse XMAS count
        count += input.Rotate().Sum(x => x.CountMatchesReverse());

        // Get diagonal XMAS count
        count += input.Rotate45().Sum(x => x.CountMatches());

        // Get diagonal Reverse XMAS count
        count += input.Rotate45().Sum(x => x.CountMatchesReverse());

        return count;
    }

    public static int Part2(string[] input) {
        return 0;
    }

    private static int CountMatchesReverse(this string line) {
        return Regex.Matches(line, @"SAMX").Count;
    }

    private static int CountMatches(this string line) {
        return Regex.Matches(line, @"XMAS").Count;
    }

    /// <summary>
    /// Rotate the line matrix so that column 0 becomes row 0, column 1 becomes row 1, etc.
    /// </summary>
    /// <param name="lines"></param>
    /// <returns></returns>
    private static string[] Rotate(this string[] lines) {
        var rotated = new string[lines.Length];
        for (var i = 0; i < lines.Length; i++) {
            var sb = new StringBuilder();
            foreach (var t in lines) {
                sb.Append(t[i]);
            }
            rotated[i] = sb.ToString();
        }
        return rotated;
    }

    /// <summary>
    /// Rotate the line matrix 45 degrees clockwise so that diagonal 0 becomes row 0, diagonal 1 becomes row 1, etc.
    /// </summary>
    /// <param name="lines"></param>
    /// <returns></returns>
    private static string[] Rotate45(this string[] lines) {
        var rotated = new string[lines.Length];
        for (var i = 0; i < lines.Length; i++) {
            var sb = new StringBuilder();
            for (var j = 0; j < lines.Length; j++) {
                sb.Append(lines[j][(i + j) % lines.Length]);
            }
            rotated[i] = sb.ToString();
        }
        return rotated;
    }

}
