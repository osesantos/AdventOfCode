using AoC.Utils;

namespace AoC.Days.Day8;

/// <summary>
/// Day8 used for Day class generation
/// Credits: https://github.com/mikequinlan/AOC2024/blob/main/Day08.cs
/// </summary>
public static class Day8_2 {

    private static string[] Lines => "Day8".GetInputLines();

    private static int RowCount { get; set; }
    private static int ColCount { get; set; }

    public static void Run() {
        Console.WriteLine("Day8");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static long Part1(IReadOnlyList<string> input) {
        RowCount = input.Count;
        ColCount = input[0].Length;
        var uniqueAntiNodes = new HashSet<(int row, int col)>();
        var antennas = GetAntennas(input);
        var antennaToRowCol = GetAntennaToRowCol(antennas);
        foreach (var potentialAntiNode in
                 from antenna in antennas
                 let sameFrequencyAntennas = antennaToRowCol[antenna.Freq]
                     .Where(rowCol => rowCol.row != antenna.row || rowCol.col != antenna.col)
                     .ToList()
                 from rowCol in sameFrequencyAntennas
                 let delta = (row: antenna.row - rowCol.row, col: antenna.col - rowCol.col)
                 select (row: antenna.row + delta.row, col: antenna.col + delta.col)
                 into potentialAntiNode
                 where InGrid(potentialAntiNode)
                 select potentialAntiNode) {
            uniqueAntiNodes.Add(potentialAntiNode);
        }

        return uniqueAntiNodes.Count;
    }

    private static Dictionary<char, List<(int row, int col)>> GetAntennaToRowCol(
        List<(char Freq, int row, int col)> antennas) {
        var antennaToRowCol = antennas.GroupBy(t => t.Freq)
            .ToDictionary(g => g.Key,
                g => g.Select(t => (t.row, t.col)).ToList());
        return antennaToRowCol;
    }

    private static List<(char Freq, int row, int col)> GetAntennas(IReadOnlyList<string> input) {
        var antennas = input.SelectMany((line, row) => line.Select((ch, col) => (Freq: ch, row, col)))
            .Where(t => t.Freq != '.')
            .ToList();
        return antennas;
    }

    public static long Part2(IReadOnlyList<string> input) {
        RowCount = input.Count;
        ColCount = input[0].Length;
        var uniqueAntiNodes = new HashSet<(int row, int col)>();
        var antennas = GetAntennas(input);
        var antennaToRowCol = GetAntennaToRowCol(antennas);
        foreach (var antenna in antennas) {
            var sameFrequencyAntennas = antennaToRowCol[antenna.Freq]
                .Where(rowCol => rowCol.row != antenna.row || rowCol.col != antenna.col)
                .ToList();
            foreach (var rowCol in sameFrequencyAntennas) {
                var delta = (row: antenna.row - rowCol.row, col: antenna.col - rowCol.col);
                var potentialAntiNode = (row: antenna.row + delta.row, col: antenna.col + delta.col);
                while (InGrid(potentialAntiNode)) {
                    uniqueAntiNodes.Add(potentialAntiNode);
                    potentialAntiNode = (potentialAntiNode.row + delta.row, potentialAntiNode.col + delta.col);
                }

                potentialAntiNode = (row: antenna.row - delta.row, col: antenna.col - delta.col);
                while (InGrid(potentialAntiNode)) {
                    uniqueAntiNodes.Add(potentialAntiNode);
                    potentialAntiNode = (potentialAntiNode.row - delta.row, potentialAntiNode.col - delta.col);
                }
            }
        }

        return uniqueAntiNodes.Count;
    }

    private static bool InGrid((int row, int col) rowCol)
        => rowCol.row >= 0 && rowCol.row < RowCount && rowCol.col >= 0 && rowCol.col < ColCount;
}
