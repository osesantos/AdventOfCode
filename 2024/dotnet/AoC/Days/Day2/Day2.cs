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
        return input.ParseInput().Count(line => {
            var lineNums = line.Select(int.Parse).ToArray();
            var safe = false;
            var operation = 0;
            for (var i = 1; i < lineNums.Length; i++) {
                var newOperation = lineNums[i] - lineNums[i - 1];

                if (operation > 0 && newOperation < 0 || operation < 0 && newOperation > 0 || newOperation == 0) {
                    safe = false;
                    break;
                }

                operation = newOperation;
                safe = operation is >= 1 and <= 3 || -operation is >= 1 and <= 3;

                if (safe) {
                    continue;
                }

                safe = false;
                break;
            }
            return safe;
        });
    }

    public static int Part2(string[] input) {
        return input.ParseInput().Count(line => {
            var lineNums = line.Select(int.Parse).ToArray();
            var safe = false;
            var operation = 0;
            var levelCount = 0;

            var isABreak = bool () => {
                levelCount++;
                if (levelCount > 1) {
                    safe = false;
                    return true;
                }
                return false;
            };

            for (var i = 1; i < lineNums.Length; i++) {
                var newOperation = lineNums[i] - lineNums[i - 1];

                if (operation > 0 && newOperation < 0 || operation < 0 && newOperation > 0 || newOperation == 0) {
                    if (isABreak.Invoke()) {
                        break;
                    }
                }

                operation = newOperation;
                safe = operation is >= 1 and <= 3 || -operation is >= 1 and <= 3;

                if (safe) {
                    continue;
                }

                if (isABreak.Invoke()) {
                    break;
                }
            }
            return safe;
        });
    }

    private static string[][] ParseInput(this string[] input) => input.Select(line => line.Split(" ")).ToArray();
}
