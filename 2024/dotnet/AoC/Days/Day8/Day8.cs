using System.Runtime.Intrinsics.X86;
using AoC.Utils;

namespace AoC.Days.Day8;

/// <summary>
/// Day8 used for Day class generation
/// When using replace Day8 with the day
/// </summary>
public static class Day8 {

    private static string[] Lines => "Day8".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day8");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        // returning 227 - incorrect result, Why is one missing??
        var frequencies = ParseInput(input);
        var nodes =
            frequencies
                .SelectMany(f => f.GetAntiNodes())
                .DistinctBy(f => new { f.X, f.Y })
                .ToList();
        return nodes.Count;
    }

    public static int Part1_2(string[] input) {
        var uniqueAntiNodes = new HashSet<Position>();
        var gridSize= ParseInput(input).First().GridSize;
        foreach (var potentialAntiNode in
                 from frequency in ParseInput(input)
                 from antenna in frequency.Antennas
                 from sameFreqAntenna in frequency.Antennas
                    where antenna.X != sameFreqAntenna.X || antenna.Y != sameFreqAntenna.Y
                 let delta = new Position { X = sameFreqAntenna.X - antenna.X, Y = sameFreqAntenna.Y - antenna.Y }
                 select new Position { X = sameFreqAntenna.X + delta.X, Y = sameFreqAntenna.Y + delta.Y }
                 into potentialAntiNode
                 where InGrid((potentialAntiNode.X, potentialAntiNode.Y), gridSize.Item2, gridSize.Item1)
                 select potentialAntiNode) {
            uniqueAntiNodes.Add(potentialAntiNode);
        }
        return uniqueAntiNodes.Count;
    }

    public static int Part2(string[] input) {
        return 0;
    }

    private class Position {
        public int X { get; init; }
        public int Y { get; init; }
    }

    private class Frequency {
        public char Char { get; init; }
        public List<Position> Antennas { get; init; } = [];

        public (int, int) GridSize { get; init; }

        // Calculate the location of the AntiNodes based on Antennas
        // The AntiNodes are the points that are equidistant from 2 Antennas
        public IEnumerable<Position> GetAntiNodes() {
            foreach (var t in Antennas) {
                foreach (var t1 in Antennas) {
                    //var distance = GetDistance(Antennas[i].X, Antennas[i].Y, Antennas[j].X, Antennas[j].Y);
                    //var (x1, y1, x2, y2) = AddEquidistantPoints(Antennas[i].X, Antennas[i].Y, Antennas[j].X, Antennas[j].Y, distance);

                    //if (InGrid((x1, y1), GridSize.Item2, GridSize.Item1))
                    //yield return new Position { X = x1, Y = y1 };
                    //if (InGrid((x2, y2), GridSize.Item2, GridSize.Item1))
                    //yield return new Position { X = x2, Y = y2 };
                    if (t.X == t1.X || t.Y == t1.Y)
                        continue;

                    var delta = new Position { X = t.X - t1.X, Y = t.Y - t1.Y };
                    var possibleAntiNode = new Position { X = t1.X + delta.X, Y = t1.Y + delta.Y };
                    if (InGrid((possibleAntiNode.X, possibleAntiNode.Y), GridSize.Item1, GridSize.Item2))
                        yield return new Position { X =  t.X + delta.X, Y =  t.Y + delta.Y };
                }
            }
        }

        // Print the Antennas and AntiNodes
        public void Print() {
            Console.WriteLine($"Frequency {Char}");
            Console.WriteLine("Antennas:");
            foreach (var antenna in Antennas) {
                Console.WriteLine($"({antenna.X}, {antenna.Y})");
            }
            Console.WriteLine("AntiNodes:");
            foreach (var antiNode in GetAntiNodes()) {
                Console.WriteLine($"({antiNode.X}, {antiNode.Y})");
            }
        }

    }

    private static bool InGrid((int row, int col) rowCol, int rowCount, int colCount)
        => rowCol.row >= 0 && rowCol.row < rowCount && rowCol.col >= 0 && rowCol.col < colCount;


    // Parse the input into a list of Frequencies
    private static List<Frequency> ParseInput(string[] lines) {
        var frequencies = new List<Frequency>();
        for (var i = 0; i < lines.Length; i++) {
            var chars = lines[i].ToCharArray();
            for (var j = 0; j < chars.Length; j++) {
                var c = chars[j];
                if (c == '.') continue;
                var frequency = frequencies.FirstOrDefault(f => f.Char == c);
                if (frequency == null) {
                    frequency = new Frequency { Char = c, GridSize = (chars.Length, lines.Length)};
                    frequencies.Add(frequency);
                }
                frequency.Antennas.Add(new Position { X = j, Y = i });
            }
        }
        return frequencies;
    }

    // Get the Euclidean distance between 2 points
    private static double GetDistance(int x1, int y1, int x2, int y2) {
        return Math.Sqrt(Math.Pow(x2 - x1, 2) + Math.Pow(y2 - y1, 2));
    }

    // Use vectors to add the anti nodes using the distance between the antennas
    static (int, int, int, int) AddEquidistantPoints(int x1, int y1, int x2, int y2, double distance) {
        // Calculate the direction vector from (x1, y1) to (x2, y2)
        double dx = x2 - x1;
        double dy = y2 - y1;

        // Normalize the direction vector
        var magnitude = Math.Sqrt(dx * dx + dy * dy);
        var unitDx = dx / magnitude;
        var unitDy = dy / magnitude;

        // Move the new points away from the midpoint in both directions
        var distanceToMove = distance * 2;  // Half the distance between the original points

        // First AntiNode: move away from midpoint in the direction of the vector
        var newX1 = (int)(x1 + unitDx * distanceToMove);
        var newY1 = (int)(y1 + unitDy * distanceToMove);

        // Second AntiNode: move away from midpoint in the opposite direction of the vector
        var newX2 = (int)(x2 - unitDx * distanceToMove);
        var newY2 = (int)(y2 - unitDy * distanceToMove);

        return (newX1, newY1, newX2, newY2);
    }


    // Convert List of frequencies with Antennas and AntiNodes to a grid that is a string array
    private static string[] FrequenciesToGrid(List<Frequency> frequencies) {
        var grid = new char[frequencies.First().GridSize.Item2, frequencies.First().GridSize.Item1];
        foreach (var frequency in frequencies) {
            foreach (var antenna in frequency.Antennas) {
                grid[antenna.Y, antenna.X] = frequency.Char;
            }
            foreach (var antiNode in frequency.GetAntiNodes()) {
                grid[antiNode.Y, antiNode.X] = '#';
            }
        }
        var gridLines = new string[frequencies.First().GridSize.Item2];
        for (var i = 0; i < grid.GetLength(0); i++) {
            var line = "";
            for (var j = 0; j < grid.GetLength(1); j++) {
                line += grid[i, j] == 0 ? '.' : grid[i, j];
            }
            gridLines[i] = line;
        }
        return gridLines;
    }
}
