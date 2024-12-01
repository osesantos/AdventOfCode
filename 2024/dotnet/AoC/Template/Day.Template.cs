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
        Console.WriteLine($"Part1 - {Part1()}");
        Console.WriteLine($"Part2 - {Part2()}");
    }

    public static string Part1() {
        return "";
    }

    public static string Part2() {
        return "";
    }
}
