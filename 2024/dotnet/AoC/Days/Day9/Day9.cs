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
        Console.WriteLine($"Part2_2 - {Part2_2(Lines)}");
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
            filesystem = ReplaceAtIndex(filesystem, i + 1, sb.ToString());

            i--;
        }

        var filesystemString = string.Join("", filesystem);
        var filesystemArray = filesystemString.ToCharArray().Select(c => c.ToString()).ToArray();
        Console.WriteLine(string.Join("", filesystemArray));
        return CalculateChecksum(filesystemArray);
    }

    public static long Part2_2(string[] input) {
        // credits to https://github.com/javillo-ur/AoC-2024-OL-Day-9/blob/main/Day9Part2/Program.cs
        // WTF is this? I don't know, but it works
        return input.Select(line => line.Aggregate((new List<(int,int)>(), 0, true), (curr, it) => curr.Item3 ? ([..curr.Item1, (int.Parse(it+""), curr.Item2)], curr.Item2+1, false) : ([..curr.Item1, (int.Parse(it+""), -1)], curr.Item2, true))).Select(list => (System.Collections.Immutable.ImmutableList.ToImmutableList(list.Item1), list.Item2)).Select(list => Enumerable.Range(0, list.Item2).Reverse().Aggregate(list.Item1, (curr, it) => new List<(System.Collections.Immutable.ImmutableList<(int,int)>,int)>(){(curr, curr.IndexOf(curr.First(t => t.Item2 == it)))}.Select(curraux => curraux.Item1.Where((t,ind) => ind < curraux.Item2 && t.Item2 == -1 && t.Item1 >= curraux.Item1[curraux.Item2].Item1).Select(t => (t, curr.IndexOf(t))).Select(t => curr[t.Item2].Item1 == curr[curraux.Item2].Item1 ? curr.RemoveAt(curraux.Item2).RemoveAt(t.Item2).Insert(t.Item2, (t.t.Item1, it)).Insert(curraux.Item2, (t.t.Item1, -1)) : curr.RemoveAt(curraux.Item2).RemoveAt(t.Item2).Insert(t.Item2, (t.t.Item1 - curr[curraux.Item2].Item1, -1)).Insert(t.Item2, (curr[curraux.Item2].Item1, it)).Insert(curraux.Item2+1, (curr[curraux.Item2].Item1, -1))).FirstOrDefault(t => true, curr)).First())).Select(list => list.Aggregate<(int,int),List<long>>([], (curr, it) => [..curr, ..Enumerable.Range(0, it.Item1).Select(t => it.Item2)])).Select(list => list.Select((i,pos) => i != -1 ? i * pos : 0)).Sum(t => t.Sum());
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
            .Select((c, i) => (c, i))
            .Where(ci => ci.c != ".")
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
