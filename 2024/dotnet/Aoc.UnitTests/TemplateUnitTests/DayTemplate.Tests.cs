using AoC.Template;

namespace Aoc.UnitTests.TemplateUnitTests;

[TestFixture]
public static class TemplateTests {

    [Test]
    public static void TestPart1() {
        Assert.That(Template.Part1([]), Is.EqualTo(0));
    }

    [Test]
    public static void TestPart2() {
        Assert.That(Template.Part2([]), Is.EqualTo(0));
    }
}