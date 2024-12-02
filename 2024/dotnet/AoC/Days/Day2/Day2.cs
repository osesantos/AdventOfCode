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
        return input.ParseInput().Count(line => line.Select(int.Parse).ToArray().CleanSingleBadLevels().IsSafe());
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

    private static int[] CleanSingleBadLevels(this int[] line) {
        var newLine = line.ToList();
        var badLevel = -1;

        if (line.AreAllElementsUnsafe() != -1) {
            newLine.Remove(line.AreAllElementsUnsafe());
            return newLine.ToArray();
        }

        if (line.Direction() > 0) {
            badLevel = line.AreAllElemsIncreasing();

            if (badLevel == -1) {
                return newLine.ToArray();
            }

            newLine.Remove(badLevel);

            return newLine.ToArray();
        }

        badLevel = line.AreAllElemsDecreasing();

        if (badLevel == -1) {
            return newLine.ToArray();
        }

        newLine.Remove(badLevel);

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

    /// <summary>
    /// Check if all elements are safe and return the i that is not safe
    /// </summary>
    /// <param name="line"></param>
    /// <returns></returns>
    private static int AreAllElementsUnsafe(this int[] line) {
        for (var i = 1; i < line.Length; i++) {
            var newOperation = line[i] - line[i - 1];
            if (!newOperation.IsOperationSafe()) {
                return line[i];
            }
        }

        return -1;
    }

    /// <summary>
    /// Provides the direction of the line
    /// </summary>
    /// <param name="line"></param>
    /// <returns>returns above 0 if increasing below 0 if decreasing</returns>
    private static int Direction(this int[] line) {
        return line[1] - line[0];
    }

    private static bool IsOperationSafe(this int operation) => operation is >= 1 and <= 3 || -operation is >= 1 and <= 3;

    private static string[][] ParseInput(this string[] input) => input.Select(line => line.Split(" ")).ToArray();
}
