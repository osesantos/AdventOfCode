using AoC.Utils;
using NUnit.Framework;

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

    internal static string Part1() {
        return "";
    }

    internal static string Part2() {
        return "";
    }
}

[TestFixture]
public static class TemplateTests {
        
    [Test]
    public static void TestPart1() {
        Assert.That(Template.Part1(), Is.EqualTo(""));
    }
    
    [Test]
    public static void TestPart2() {
        Assert.That(Template.Part2(), Is.EqualTo(""));
    }
} 