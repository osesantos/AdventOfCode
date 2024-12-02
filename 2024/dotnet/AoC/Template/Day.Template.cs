using AoC.Utils;

namespace AoC.Template;

/// <summary>
/// Template used for Day class generation
/// When using replace Template with the day
/// </summary>
public static class Template {
    
    private static string[] Lines => "Template".GetInputLines();

    public static void Run() {
        Console.WriteLine("Template");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        return 0;
    }

    public static int Part2(string[] input) {
        return 0;
    }
}
