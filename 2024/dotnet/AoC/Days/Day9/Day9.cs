using System.Text;
using AoC.Utils;

namespace AoC.Days.Day9;

/// <summary>
/// Day9 used for Day class generation
/// When using replace Day9 with the day
/// </summary>
public static class Day9 {

    private static string[] Lines => "Day9".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day9");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static long Part1(string[] input) {
        var filesystem = ParseInput(input.First());
        var nDots = CountDots(filesystem);
        var leftMostDotIndex = GetLeftMostDotIndex(filesystem);
        var spaceAvailable = filesystem.Length - nDots;
        var i = filesystem.Length - 1;
        while (leftMostDotIndex < spaceAvailable) {
            leftMostDotIndex = GetLeftMostDotIndex(filesystem);
            var c = filesystem[i];
            // replace at the leftmost space
            filesystem = ReplaceAtIndex(filesystem, leftMostDotIndex, c);
            // replace at the previous position
            filesystem = ReplaceAtIndex(filesystem, i, ".");
            //Console.WriteLine(string.Join("", filesystem));

            leftMostDotIndex = GetLeftMostDotIndex(filesystem);
            i--;
        }
        return CalculateChecksum(filesystem);
    }

    public static long Part2(string[] input) {
        var filesystem = ParseInput(input.First());
        filesystem = DoAggregations(filesystem);
        Console.WriteLine(string.Join("", filesystem));
        var i = filesystem.Length - 1;
        while (i > 0) {
            var s = filesystem[i];
            if (s.Contains(".")) {
                i--;
                continue;
            }

            var leftMostSpaceIndex = GetLeftMostSpaceIndex(filesystem, s, i);
            if (leftMostSpaceIndex == -1) {
                i--;
                continue;
            }

            var spaceSize = filesystem[i].ToCharArray().Length;
            var sb = new StringBuilder();
            for (var j = 0; j < spaceSize; j++) {
                sb.Append(".");
            }

            var extraSpaceToAdd = filesystem[leftMostSpaceIndex].ToCharArray().Length - s.ToCharArray().Length;
            // replace at the leftmost space
            filesystem = ReplaceAtIndex(filesystem, leftMostSpaceIndex, s, extraSpaceToAdd);
            // replace at the previous position
            filesystem = ReplaceAtIndex(filesystem, i, sb.ToString());

            Console.WriteLine(string.Join("", filesystem));

            i--;
        }
        return CalculateChecksum(filesystem);
    }

    private static string[] ParseInput(string input) {
        return string.Join(" ", input
            .ToCharArray()
            .Select((c, i) => {
                var times = int.Parse(c.ToString());
                var sb = new StringBuilder();
                for (var j = 0; j < times; j++) {
                    sb.Append(i % 2 == 0 ? i / 2 : ".");
                    sb.Append(' ');
                }

                return sb.ToString();
            }).ToArray()).Split(" ", StringSplitOptions.RemoveEmptyEntries);
    }

    private static int CountDots(string[] filesystem) {
        return filesystem.Count(c => c == ".");
    }

    private static string[] ReplaceAtIndex(string[] filesystem, int index, string value, int extraSpaceToAdd = -1) {
        var newFileSystem = filesystem.ToList();
        newFileSystem[index] = value;

        if (extraSpaceToAdd == -1)
            return newFileSystem.ToArray();

        var sb = new StringBuilder();
        for (var i = 0; i < extraSpaceToAdd; i++) {
            sb.Append(".");
        }

        newFileSystem.Insert(index + 1, sb.ToString());

        return newFileSystem.ToArray();
    }

    private static int GetLeftMostDotIndex(string[] filesystem) {
        foreach (var se in filesystem.Index()) {
            if (se.Item == ".")
                return se.Index;
        }

        return -1;
    }

    private static int GetLeftMostSpaceIndex(string[] filesystem, string elem, int index) {
        var size = elem.Length;
        foreach (var se in filesystem.Index()) {
            if (se.Item.Contains(".") && se.Item.Length >= size && se.Index < index)
                return se.Index;
        }

        return -1;
    }

    private static int GetHighestId(string[] filesystem) {
        var highestId = 0;
        foreach (var se in filesystem) {
            if (int.TryParse(se, out var id) && id > highestId)
                highestId = id;
        }

        return highestId;
    }

    private static long CalculateChecksum(string[] filesystem) {
        return filesystem
            .Where(c => !c.Contains("."))
            .Select((c, i) => (c, i))
            .Sum(ci => (long)(int.Parse(ci.c.ToString()) * ci.i));
    }

    private static string[] DoAggregations(string[] filesystem) {
        var newList = new List<string>();
        var sb = new StringBuilder();
        foreach (var elem in filesystem) {
            if (sb.ToString().Contains(elem) || sb.ToString() == string.Empty) {
                sb.Append(elem);
                continue;
            }

            newList.Add(sb.ToString());
            sb = new StringBuilder();
            sb.Append(elem);
        }

        newList.Add(sb.ToString());
        return newList.ToArray();
    }

}
