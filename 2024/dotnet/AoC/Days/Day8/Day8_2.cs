using AoC.Utils;

namespace AoC.Days.Day8;

/// <summary>
/// Day8 used for Day class generation
/// When using replace Day8 with the day
/// </summary>
public static class Day8_2 {

    private static string[] Lines => "Day8".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day8");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        // returning 227 - incorrect result, Why is one missing??
        // var frequencies = ParseInput(input);
        // var grid = FrequenciesToGrid(frequencies);
        // foreach (var line in grid) {
        //     Console.WriteLine(line);
        // }
        // var antennas = frequencies.SelectMany(f => f.Antennas).ToList();
        // var nodes =
        //     frequencies
        //         .SelectMany(f => f.GetAntiNodes())
        //         .DistinctBy(f => new { f.X, f.Y })
        //         .ToList();
        // Console.WriteLine($"Number of AntiNodes: {nodes.Count}");
        // Console.WriteLine($"Number of Antennas: {antennas.Count}");
        // foreach (var pos in antennas) {
        //     foreach (var node in nodes) {
        //         if (pos.X == node.X && pos.Y == node.Y) {
        //             Console.WriteLine($"Antenna at {pos.X}, {pos.Y} is an AntiNode");
        //         }
        //     }
        // }
        // return nodes.Count();

        // returning 228 - correct result
        return Solver(input, d =>
            new List<Point> { d.a + d.d, d.here - d.d }
                .Where(p => 0 <= p.X && p.X < d.width && 0 <= p.Y && p.Y < d.height));
    }

    private static int Solver(string[] lines, Func<((int j, int i) a, Point d, Point here, int width, int height), IEnumerable<Point>> selector)
        => lines.SelectMany((line, i) => line.Select((c, j) => (c, j, i))
            .Where(d => d.c != '.'))
            .GroupBy(d => d.c)
            .ToDictionary(g => g.Key, g => g.Select(d => (d.j, d.i)).ToArray())
            .SelectMany(kvp =>
                kvp.Value.Select((p, i) => (here: (Point)p, i))
                    .SelectMany(d => kvp.Value[d.i..].Where(x => x != (d.here.X, d.here.Y))
                        .Select(p => (a: p, d: p - d.here, d.here, width: lines[0].Length, height:lines.Length)) // a,d are position and direction vector in r = a + i*d
                        .SelectMany(selector)))
            .ToHashSet().Count;

    public struct Point(long x, long y)
    {
        public long X { get; set; } = x;
        public long Y { get; set; } = y;

        public readonly long MDistanceTo(Point b) => Math.Abs(b.X - X) + Math.Abs(b.Y - Y);
        public readonly long this[int index] => index == 0 ? X : Y;

        public static Point operator *(long scalar, Point point) => new(scalar * point.X, scalar * point.Y);
        public static Point operator +(Point a, Point b) => new(a.X + b.X, a.Y + b.Y);
        public static Point operator -(Point a, Point b) => new(a.X - b.X, a.Y - b.Y);
        public static Point operator -(Point a) => new(-a.X, a.Y);
        public static bool operator ==(Point? a, Point? b) => a.Equals(b);
        public static bool operator !=(Point? a, Point? b) => !(a == b);

        public override readonly bool Equals(object? obj) => obj is Point p && p.X.Equals(X) && p.Y.Equals(Y);
        public override readonly int GetHashCode() => HashCode.Combine(X, Y);


        public static implicit operator Point((long x, long y) coords) => new(coords.x, coords.y);
        //public static implicit operator (long X, long Y)(Point p) => (p.X, p.Y);
        public override readonly string ToString() => $"({X}, {Y})";
        public readonly long[] ToArray() => [X, Y];
        public readonly void Deconstruct(out long x, out long y)
        {
            x = X;
            y = Y;
        }
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
        public List<Position> Antennas { get; } = [];
        public (int, int) GridSize { get; init; }

        // Calculate the location of the AntiNodes based on Antennas
        // The AntiNodes are the points that are equidistant from 2 Antennas
        public IEnumerable<Position> GetAntiNodes() {
            if (Antennas.Count < 2) yield break;
            for (var i = 0; i < Antennas.Count; i++) {
                for (var j = i + 1; j < Antennas.Count; j++) {
                    var distance = GetDistance(Antennas[i].X, Antennas[i].Y, Antennas[j].X, Antennas[j].Y);
                    var (x1, y1, x2, y2) =
                        AddEquidistantPoints(Antennas[i].X, Antennas[i].Y, Antennas[j].X, Antennas[j].Y, distance);

                    if (x1 >= 0 && x1 < GridSize.Item1 && y1 >= 0 && y1 < GridSize.Item2)
                        yield return new Position { X = x1, Y = y1 };
                    if (x2 >= 0 && x2 < GridSize.Item1 && y2 >= 0 && y2 < GridSize.Item2)
                        yield return new Position { X = x2, Y = y2 };
                }
            }
        }
    }

    // Get the Euclidean distance between 2 points
    private static double GetDistance(int x1, int y1, int x2, int y2) {
        return Math.Sqrt(Math.Pow(x2 - x1, 2) + Math.Pow(y2 - y1, 2));
    }

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

    private static List<Frequency> ParseInput(string[] lines) {
        var frequencies = new List<Frequency>();
        for (var i = 0; i < lines.Length; i++) {
            var chars = lines[i].ToCharArray();
            for (var j = 0; j < chars.Length; j++) {
                var c = chars[j];
                if (c == '.') continue;
                var frequency = frequencies.FirstOrDefault(f => f.Char == c);
                if (frequency == null) {
                    frequency = new Frequency { Char = c, GridSize = (chars.Length, lines.Length) };
                    frequencies.Add(frequency);
                }

                frequency.Antennas.Add(new Position { X = j, Y = i });
            }
        }

        return frequencies;
    }

    private static string[] FrequenciesToGrid(List<Frequency> frequencies) {
        var grid = new char[frequencies.First().GridSize.Item2, frequencies.First().GridSize.Item1];
        foreach (var frequency in frequencies) {
            foreach (var antiNode in frequency.GetAntiNodes()) {
                grid[antiNode.Y, antiNode.X] = '#';
            }
            foreach (var antenna in frequency.Antennas) {
                grid[antenna.Y, antenna.X] = frequency.Char;
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
