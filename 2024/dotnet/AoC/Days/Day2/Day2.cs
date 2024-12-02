using System.Text.Json;
using System.Text.Json.Nodes;
using AoC.Utils;

namespace AoC.Days.Day2;

/// <summary>
/// Day2 used for Day class generation
/// When using replace Day2 with the day
/// </summary>
public static class Day2 {

    private static string[] Lines => "Day2".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day2");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        return input.ParseInput().Count(line => line.Select(int.Parse).ToArray().IsSafe());
    }

    public static int Part2(string[] input) {
        return input.ParseInput().Count(line => line.Select(int.Parse).ToArray().IsSafeReport());
    }

    private static bool IsSafe(this int[] lineNums, bool useBadLevels = false) {
        var badLevelCounter = 0;

        for (var i = 1; i < lineNums.Length; i++) {
            var newOperation = lineNums[i] - lineNums[i - 1];

            if (lineNums.AreAllElemsIncreasing() != -1 && lineNums.AreAllElemsDecreasing() != -1) {
                if (!useBadLevels) {
                    Console.WriteLine($"{JsonSerializer.Serialize(lineNums)} - not safe!");
                    return false;
                }
                badLevelCounter++;
            }

            if (!newOperation.IsOperationSafe()) {
                if (!useBadLevels) {
                    Console.WriteLine($"{JsonSerializer.Serialize(lineNums)} - not safe!");
                    return false;
                }
                badLevelCounter++;
            }
        }

        if (useBadLevels && badLevelCounter > 1) {
            Console.WriteLine($"{JsonSerializer.Serialize(lineNums)} - not safe!");
            return false;
        }

        Console.WriteLine($"{JsonSerializer.Serialize(lineNums)} - safe!");
        return true;
    }

    private static bool IsSafeReport(this int[] line) {
        if (IsSafe2(line)) {
            Console.WriteLine($"{JsonSerializer.Serialize(line)} - safe!");
            return true;
        }

        var canBeMadeSafe = false;
        for (var i = 0; i < line.Length; i++) {
            var modifiedLine = line.RemoveLevel(i);
            if (IsSafe2(modifiedLine.ToArray())) {
                canBeMadeSafe = true;
                break;
            }
        }
        if (canBeMadeSafe) {
            Console.WriteLine($"{JsonSerializer.Serialize(line)} - safe!");
            return true;
        }

        Console.WriteLine($"{JsonSerializer.Serialize(line)} - not safe!");
        return false;
    }

    private static bool IsSafe2(int[] report) {
        if (report.Length < 2) {
            return true;
        }

        var increasing = true;
        var decreasing = true;

        for (var i = 1; i < report.Length; i++) {
            if (Math.Abs(report[i] - report[i - 1]) is < 1 or > 3) {
                return false;
            }

            if (report[i] <= report[i - 1]) {
                increasing = false;
            }
            if (report[i] >= report[i - 1]) {
                decreasing = false;
            }
            if (!increasing && !decreasing) {
                return false;
            }
        }

        return true;
    }

    private static int[] RemoveLevel(this int[] line, int index) {
        var newLine = line.ToList();
        newLine.RemoveAt(index);
        return newLine.ToArray();
    }

    /// <summary>
    /// Check if all elements are increasing and return the i that is not increasing or is equal to the previous
    /// </summary>
    /// <param name="line"></param>
    /// <returns></returns>
    private static int AreAllElemsIncreasing(this int[] line) {
        for (var i = 1; i < line.Length; i++) {
            if (line[i] <= line[i - 1]) {
                return line[i - 1];
            }
        }

        return -1;
    }

    /// <summary>
    /// Check if all elements are decreasing and return the i that is not decreasing or is equal to the previous
    /// </summary>
    /// <param name="line"></param>
    /// <returns></returns>
    private static int AreAllElemsDecreasing(this int[] line) {
        for (var i = 1; i < line.Length; i++) {
            if (line[i] >= line[i - 1]) {
                return line[i - 1];
            }
        }

        return -1;
    }

    private static bool IsOperationSafe(this int operation) => operation is >= 1 and <= 3 || -operation is >= 1 and <= 3;

    private static string[][] ParseInput(this string[] input) => input.Select(line => line.Split(" ")).ToArray();
}
