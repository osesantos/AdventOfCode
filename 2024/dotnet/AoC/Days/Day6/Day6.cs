using AoC.Utils;
namespace AoC.Days.Day6;

/// <summary>
/// Day6 used for Day class generation
/// When using replace Day6 with the day
/// </summary>
public static class Day6 {


    private static string[] Lines => "Day6".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day6");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        var map = input.CreateMap();
        var stop = false;
        var direction = Direction.Up;
        while (!stop) {
            try {
                map = map.Move(map.GetCurrentPosition(), direction);
            } catch (Exception e) when("Out of bounds".Equals(e.Message)) {
                stop = true;
            } catch (Exception e) when("Hit a wall".Equals(e.Message)) {
                direction = direction.GetNextDirection();
            }
        }

        map.PrintMap();

        return map.Count('X');
    }

    public static int Part2(string[] input) {
        return 0;
    }

    private enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    private static char[][] CreateMap(this string[] lines) {
        var map = new char[lines.Length][];
        for (var i = 0; i < lines.Length; i++) {
            map[i] = lines[i].ToCharArray();
        }
        return map;
    }

    private static char[][] Move(this char[][] map, (int, int) currentPosition, Direction direction) {
        var newPosition = direction switch {
            Direction.Up => (currentPosition.Item1 - 1, currentPosition.Item2),
            Direction.Down => (currentPosition.Item1 + 1, currentPosition.Item2),
            Direction.Left => (currentPosition.Item1, currentPosition.Item2 - 1),
            Direction.Right => (currentPosition.Item1, currentPosition.Item2 + 1),
            _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null)
        };

        if (newPosition.Item1 < 0 || newPosition.Item1 >= map[0].Length || newPosition.Item2 < 0 || newPosition.Item2 >= map.Length) {
            // set old position to X
            map[currentPosition.Item1][currentPosition.Item2] = 'X';
            throw new Exception("Out of bounds");
        }

        if (map[newPosition.Item1][newPosition.Item2] == '#') {
            throw new Exception("Hit a wall");
        }

        // set old position to X
        map[currentPosition.Item1][currentPosition.Item2] = 'X';

        // set new position
        map[newPosition.Item1][newPosition.Item2] = '^';

        return map;
    }

    private static (int, int) GetCurrentPosition(this char[][] map) {
        for (var i = 0; i < map.Length; i++) {
            for (var j = 0; j < map[i].Length; j++) {
                if (map[i][j] == '^') {
                    return (i, j);
                }
            }
        }
        throw new Exception("No starting position found");
    }

    private static Direction GetNextDirection(this Direction currentDirection) {
        return currentDirection switch {
            Direction.Up => Direction.Right,
            Direction.Right => Direction.Down,
            Direction.Down => Direction.Left,
            Direction.Left => Direction.Up,
            _ => throw new ArgumentOutOfRangeException(nameof(currentDirection), currentDirection, null)
        };
    }

    private static int Count(this char[][] map, char c) {
        return map.Sum(line => line.Count(x => x == c));
    }

    private static void PrintMap(this char[][] map) {
        foreach (var line in map) {
            Console.WriteLine(line);
        }
    }
}
