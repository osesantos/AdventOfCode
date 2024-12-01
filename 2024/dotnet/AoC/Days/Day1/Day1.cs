using AoC.Utils;

namespace AoC.Days.Day1;

/// <summary>
/// Day1 used for Day class generation
/// When using replace Day1 with the day
/// </summary>
public static class Day1 {
    
    private static string[] Lines => "Day1".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day1");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static string Part1(string[] input) {
        List<int> list1 = [];
        List<int> list2 = [];
        
        foreach (var line in input) {
            var split = line.Split("   ");
            var first = int.Parse(split.First());
            var last = int.Parse(split.Last());
            list1.Add(first);
            list2.Add(last);
        }

        var list1Ordered = list1.Order().ToArray();
        var list2Ordered = list2.Order().ToArray();

        if (list1Ordered.Length != list2Ordered.Length) {
            throw new Exception("list lengths are different!");
        }

        var sum = list1Ordered
            .Select((t, i) => t > list2Ordered[i] ? t - list2Ordered[i] : list2Ordered[i] - t)
            .Sum();

        return $"{sum}";
    }

    public static string Part2(string[] input) {
        return "";
    }
}
