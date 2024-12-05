using System.Text.Json;
using AoC.Utils;

namespace AoC.Days.Day5;

/// <summary>
/// Day5 used for Day class generation
/// When using replace Day5 with the day
/// </summary>
public static class Day5 {

    private static string[] Lines => "Day5".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day5");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        return ExtractPagesToProduce(input).FilterInvalidLines(ExtractOrderRules(input).ConvertOrderRulesToTrees()).SumMiddleNum();
    }

    public static int Part2(string[] input) {
        return 0;
    }

    private static string[] ExtractOrderRules(this string[] lines) =>
        lines.TakeWhile(x => x != string.Empty).ToArray();

    private static string[] ExtractPagesToProduce(this string[] lines) =>
        lines.SkipWhile(x => x.Contains('|') || x == string.Empty).ToArray();

    private static IDictionary<int, List<int>> ConvertOrderRulesToTrees(this string[] lines) {
        var trees = new Dictionary<int, List<int>>();

        foreach (var line in lines) {
            var elems = line.Split("|").Select(int.Parse).ToArray();

            var children = new List<int>();
            if (trees.ContainsKey(elems.First())) {
                children = trees.GetValueOrDefault(elems.First()) ?? [];
                children.Add(elems.Last());
                trees[elems.First()] = children;
            } else {
                children.Add(elems.Last());
                trees.Add(elems.First(), children);
            }
        }

        Console.WriteLine("Rules:");
        Console.WriteLine($"{JsonSerializer.Serialize(trees)}");
        Console.WriteLine("");

        return trees;
    }

    private static string[] FilterInvalidLines(this string[] lines, IDictionary<int, List<int>> rules) {
        return lines.Where(line => {
            var elems = line.Split(",").Select(int.Parse).ToArray();
            var isLineValid = true;

            for (var i = 0; i < elems.Length; i++) {
                if (!rules.TryGetValue(elems[i], out var subsequentElems)) {
                    continue;
                }

                for (var j = i; j >= 0; j--) {
                    if (!subsequentElems.Contains(elems[j])) {
                        continue;
                    }

                    Console.WriteLine($"Invalid line: {JsonSerializer.Serialize(line)}");
                    Console.WriteLine($"Rules: {elems[i]}|{JsonSerializer.Serialize(subsequentElems)}, Elem: {elems[j]}");
                    isLineValid = false;
                    break;
                }

                if (!isLineValid) {
                    break;
                }
            }
            return isLineValid;
        }).ToArray();
    }

    private static int SumMiddleNum(this string[] lines) {
        return lines.Sum(line => {
            var lineSplit = line.Split(",").Select(int.Parse).ToArray();
            if (lineSplit.Length % 2 == 0) {
                throw new ("Line is pair! I cant get the middle elem.");
            }
            return lineSplit[(lineSplit.Length - 1) / 2];
        });
    }
}
