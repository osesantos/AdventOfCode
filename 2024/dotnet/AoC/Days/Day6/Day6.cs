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
        var obstructionsMap = input.CreateMap().MapWithObstructions();
        var loopCount = 0;

        var map = input.CreateMap();
        DoALoopWithObstruction(map);
        PrintMap(map);

        for (var i = 0; i < obstructionsMap.Length; i++) {
            for (var j = 0; j < obstructionsMap[i].Length; j++) {
                try {
                    map = input.CreateMap();
                    map[i][j] = map[i][j] != '#' ? 'O' : map[i][j];
                    DoALoopWithObstruction(map);
                }
                catch (Exception e) when ("Hit an obstruction".Equals(e.Message)) {
                    loopCount++;
                }
            }
        }

        return loopCount;
    }

    private static void DoALoopWithObstruction(this char[][] map) {
        var stop = false;
        var direction = Direction.Up;
        while (!stop) {
            try {
                map = map.Move2(map.GetCurrentPosition(), direction);
            }
            catch (Exception e) when ("Out of bounds".Equals(e.Message)) {
                stop = true;
            }
            catch (Exception e) when ("No starting position found".Equals(e.Message)) {
                stop = true;
            }
            catch (Exception e) when ("Hit a wall".Equals(e.Message)) {
                direction = direction.GetNextDirection();
            }
            catch (Exception e) when ("Hit an obstruction".Equals(e.Message)) {
                throw;
            }
        }
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
        var newPosition = currentPosition.GetNextPosition(direction);

        if (IsPositionOutOfBounds(map, newPosition)) {
            // set old position to X
            map[currentPosition.Item1][currentPosition.Item2] = 'X';
            throw new Exception("Out of bounds");
        }

        switch (map[newPosition.Item1][newPosition.Item2]) {
            case '#':
                throw new Exception("Hit a wall");
            case 'O':
                throw new Exception("Hit an obstruction");
        }

        // set old position to X
        map[currentPosition.Item1][currentPosition.Item2] = 'X';

        // set new position
        map[newPosition.Item1][newPosition.Item2] = '^';

        return map;
    }

    private static char[][] Move2(this char[][] map, (int, int) currentPosition, Direction direction) {
        var newPosition = currentPosition.GetNextPosition(direction);

        if (IsPositionOutOfBounds(map, newPosition)) {
            map[currentPosition.Item1][currentPosition.Item2] = GetChar(direction, map[currentPosition.Item1][currentPosition.Item2]);
            throw new Exception("Out of bounds");
        }

        switch (map[newPosition.Item1][newPosition.Item2]) {
            case '#':
                var newDirection = direction.GetNextDirection();
                var nextPosition = currentPosition.GetNextPosition(newDirection);
                map[nextPosition.Item1][nextPosition.Item2] = '^';
                map[currentPosition.Item1][currentPosition.Item2] = '+';
                throw new Exception("Hit a wall");
            case 'O':
                throw new Exception("Hit an obstruction");
        }

        map[currentPosition.Item1][currentPosition.Item2] = GetChar(direction, map[currentPosition.Item1][currentPosition.Item2]);

        // set new position
        map[newPosition.Item1][newPosition.Item2] = '^';

        return map;
    }

    private static char GetChar(Direction direction, char currentChar) {
        if (currentChar is '+') {
            return '+';
        }

        return direction switch {
            Direction.Down => '|',
            Direction.Up => '|',
            Direction.Left => '-',
            Direction.Right => '-',
            _ => throw new ArgumentOutOfRangeException(nameof(direction), direction, null)
        };
    }

    private static bool IsPositionOutOfBounds(this char[][] map, (int, int) position) {
        return position.Item1 < 0 || position.Item1 >= map[0].Length || position.Item2 < 0 || position.Item2 >= map.Length;
    }

    private static (int, int) CanNextMoveBeLoopObstruction(this char[][] map, (int, int) currentPosition, Direction currentDirection) {
        var newPosition = currentPosition.GetNextPosition(currentDirection);
        var nextDirection = GetNextDirection(currentDirection);
        var nextPosition = newPosition.GetNextPosition(nextDirection);

        // We check if the next position is a path already taken,
        // if so we add an obstruction in the new position to add a loop
        if (!IsPositionOutOfBounds(map, nextPosition) && map[nextPosition.Item1][nextPosition.Item2] == 'X') {
            var futurePosition = newPosition.GetNextPosition(currentDirection);
            return (futurePosition.Item1, futurePosition.Item2);
        };

        return (-1, -1);
    }

    private static char[][] MapWithObstructions(this char[][] map) {
        for (var i = 0; i < map.Length; i++) {
            for (var j = 0; j < map[i].Length; j++) {
                if (map[i][j] != '#' && map[i][j] != '^') {
                    map[i][j] = 'O';
                }
            }
        }
        return map;
    }

    private static (int, int) GetNextPosition(this (int, int) currentPosition, Direction direction) {
        return direction switch {
                   Direction.Up    => (currentPosition.Item1 - 1, currentPosition.Item2),
                   Direction.Down  => (currentPosition.Item1 + 1, currentPosition.Item2),
                   Direction.Left  => (currentPosition.Item1, currentPosition.Item2 - 1),
                   Direction.Right => (currentPosition.Item1, currentPosition.Item2 + 1),
                   _               => throw new ArgumentOutOfRangeException(nameof(direction), direction, null)
               };
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

    private static void PrintMap(this char[][] map, List<(int, int)>? obstructionsToAdd = null) {
        var mapClone = map.Select(x => x.ToArray()).ToArray();
        if (obstructionsToAdd != null) {
            foreach (var (x, y) in obstructionsToAdd) {
                mapClone[x][y] = 'O';
            }
        }

        foreach (var line in mapClone) {
            Console.WriteLine(line);
        }
    }
}
