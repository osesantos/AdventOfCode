using AoC.Template;

namespace Aoc.UnitTests.TemplateUnitTests;

[TestFixture]
public static class TemplateTests {
        
    [Test]
    public static void TestPart1() {
        Assert.That(Template.Part1([]), Is.EqualTo(""));
    }
    
    [Test]
    public static void TestPart0() {
        Assert.That(Template.Part2([]), Is.EqualTo(""));
    }
} 