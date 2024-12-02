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
        var count = input.Select(line => line.Split(" ")).Count(line => {
            
        });
        
        return 0;
    }

    public static int Part2(string[] input) {
        return 0;
    }
}
