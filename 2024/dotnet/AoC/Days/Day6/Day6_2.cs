using AoC.Utils;

namespace AoC.Days.Day6;

public class Day6_2 {

    private static string[] Lines => "Day6".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day6");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] map)
    {
        int rows = map.Length;
        int cols = map[0].Length;
        char[,] grid = new char[rows, cols];

        // Initialize grid and find guard position and direction
        (int x, int y) guardPos = (-1, -1);
        int dir = 0; // 0=up, 1=right, 2=down, 3=left
        for (int i = 0; i < rows; i++)
        {
            for (int j = 0; j < cols; j++)
            {
                grid[i, j] = map[i][j];
                if (map[i][j] == '^') { guardPos = (i, j); dir = 0; }
                else if (map[i][j] == '>') { guardPos = (i, j); dir = 1; }
                else if (map[i][j] == 'v') { guardPos = (i, j); dir = 2; }
                else if (map[i][j] == '<') { guardPos = (i, j); dir = 3; }
            }
        }

        HashSet<(int, int)> visited = new();
        HashSet<(int, int, int)> states = new(); // Tracks (position + direction) to detect cycles
        visited.Add(guardPos);
        states.Add((guardPos.x, guardPos.y, dir));

        while (true)
        {
            // Calculate next position
            int nextX = guardPos.x + (dir == 0 ? -1 : dir == 2 ? 1 : 0);
            int nextY = guardPos.y + (dir == 1 ? 1 : dir == 3 ? -1 : 0);

            // Check if the guard will leave the grid
            if (nextX < 0 || nextY < 0 || nextX >= rows || nextY >= cols)
            {
                break;
            }

            if (grid[nextX, nextY] == '#')
            {
                // Obstacle detected, turn right
                dir = (dir + 1) % 4;
            }
            else
            {
                // Move forward
                guardPos = (nextX, nextY);
                visited.Add(guardPos);
            }

            // Detect infinite loops by checking if the state repeats
            if (states.Contains((guardPos.x, guardPos.y, dir)))
            {
                Console.WriteLine("Infinite loop detected. Terminating simulation.");
                break;
            }

            states.Add((guardPos.x, guardPos.y, dir));
        }

        return visited.Count;
    }

    public static int Part2(string[] map)
    {
        int rows = map.Length;
        int cols = map[0].Length;
        char[,] grid = new char[rows, cols];

        // Initialize grid
        for (int i = 0; i < rows; i++)
        {
            for (int j = 0; j < cols; j++)
            {
                grid[i, j] = map[i][j];
            }
        }

        HashSet<(int x, int y)> validObstructionPoints = new();

        // Iterate over every empty position in the grid
        for (int x = 0; x < rows; x++)
        {
            for (int y = 0; y < cols; y++)
            {
                if (grid[x, y] == '.')
                {
                    // Temporarily place an obstruction
                    grid[x, y] = '#';

                    // Simulate the guard's path
                    if (DoesObstructionCauseLoop(grid))
                    {
                        validObstructionPoints.Add((x, y));
                    }

                    // Remove the obstruction
                    grid[x, y] = '.';
                }
            }
        }

        Console.WriteLine($"Part 2: Number of valid obstruction points = {validObstructionPoints.Count}");
        foreach (var point in validObstructionPoints)
        {
            Console.WriteLine($"Valid obstruction at ({point.x}, {point.y})");
        }

        return validObstructionPoints.Count;
    }

    static bool DoesObstructionCauseLoop(char[,] grid)
    {
        int rows = grid.GetLength(0);
        int cols = grid.GetLength(1);

        // Find guard's starting position and direction
        (int x, int y) guardPos = (-1, -1);
        int dir = 0; // 0=up, 1=right, 2=down, 3=left
        for (int i = 0; i < rows; i++)
        {
            for (int j = 0; j < cols; j++)
            {
                if (grid[i, j] == '^') { guardPos = (i, j); dir = 0; }
                else if (grid[i, j] == '>') { guardPos = (i, j); dir = 1; }
                else if (grid[i, j] == 'v') { guardPos = (i, j); dir = 2; }
                else if (grid[i, j] == '<') { guardPos = (i, j); dir = 3; }
            }
        }

        HashSet<(int, int, int)> states = new(); // Tracks (position + direction)
        states.Add((guardPos.x, guardPos.y, dir));

        while (true)
        {
            // Calculate next position
            int nextX = guardPos.x + (dir == 0 ? -1 : dir == 2 ? 1 : 0);
            int nextY = guardPos.y + (dir == 1 ? 1 : dir == 3 ? -1 : 0);

            // Check if the guard leaves the grid
            if (nextX < 0 || nextY < 0 || nextX >= rows || nextY >= cols)
            {
                return false; // Guard leaves the grid, no loop
            }

            if (grid[nextX, nextY] == '#')
            {
                // Obstacle detected, turn right
                dir = (dir + 1) % 4;
            }
            else
            {
                // Move forward
                guardPos = (nextX, nextY);
            }

            // Detect infinite loops
            if (states.Contains((guardPos.x, guardPos.y, dir)))
            {
                return true; // Loop detected
            }

            states.Add((guardPos.x, guardPos.y, dir));
        }
    }
}
