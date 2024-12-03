using System.Text.RegularExpressions;
using AoC.Utils;
namespace AoC.Days.Day3;

/// <summary>
/// Day3 used for Day class generation
/// When using replace Day3 with the day
/// </summary>
public static class Day3 {

    private static string[] Lines => "Day3".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day3");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        return input.SelectMany(line => line.ScanMul()).ToArray().Select(ConvertMul).Sum(mul => Mul(mul.Item1, mul.Item2));
    }

    public static int Part2(string[] input) {
        return input.ScanAndFilterMul().Select(ConvertMul).Sum(mul => Mul(mul.Item1, mul.Item2));
    }

    private static string[] ScanAndFilterMul(this string[] input) {
        var mAndDos = input.SelectMany(line => line.ScanMulAndDos()).ToArray();
        var canMul = true;
        var newList = new List<string>();
        foreach (var elem in mAndDos) {
            if (elem.Contains("do()")) {
                canMul = true;
            } else if (elem.Contains("don't()")) {
                canMul = false;
            }

            if (canMul && elem.Contains("mul")) {
                newList.Add(elem);
            }
        }
        return newList.ToArray();
    }

    private static int Mul(int a, int b) => a * b;

    private static (int, int) ConvertMul(string mul) {
        var split = mul.Replace("mul(", "").Replace(")", "").Split(",");
        return (int.Parse(split.First()), int.Parse(split.Last()));
    }

    private static string[] ScanMul(this string line) =>
        Regex.Matches(line, @"mul\(\d+,\d+\)").Select(match => match.Value).ToArray();

    private static string[] ScanMulAndDos(this string line) =>
        Regex.Matches(line, @"mul\(\d+,\d+\)|do\(\)|don\'t\(\)").Select(match => match.Value).ToArray();
}
