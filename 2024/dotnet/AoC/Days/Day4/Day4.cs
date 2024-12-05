using System.Text;
using System.Text.Json;
using System.Text.RegularExpressions;
using AoC.Utils;

namespace AoC.Days.Day4;

/// <summary>
/// Day4 used for Day class generation
/// When using replace Day4 with the day
/// </summary>
public static class Day4 {

    private static string[] Lines => "Day4".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day4");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    public static int Part1(string[] input) {
        var rows = input.Length;
        var cols = input[0].Length;

        const string word = "XMAS";
        var wordLength = word.Length;

        var count = 0;

        // Check all possible starting positions
        for (var i = 0; i < rows; i++) {
            for (var j = 0; j < cols; j++) {
                // Check all directions: horizontal, vertical, diagonal
                for (var dx = -1; dx <= 1; dx++) {
                    for (var dy = -1; dy <= 1; dy++) {
                        if (dx == 0 && dy == 0) continue; // Skip the current cell
                        int x = i, y = j;
                        int k;
                        for (k = 0; k < wordLength; k++) {
                            if (x < 0 || x >= rows || y < 0 || y >= cols || input[x][y] != word[k])
                            {
                                break;
                            }
                            x += dx;
                            y += dy;
                        }
                        if (k == wordLength) { // Found the word
                            Console.WriteLine($"Found {word} starting at position ({i}, {j}) in direction ({dx}, {dy})");
                            count++;
                        }
                    }
                }
            }
        }
        return count;
    }

    public static int Part2(string[] input) {
        var rows = input.Length;
        var cols = input[0].Length;

        var count = 0;

        // Check all possible starting positions
        for (var i = 1; i < rows - 1; i++) {
            for (var j = 1; j < cols - 1; j++) {
                if (input[i][j] == 'A') {
                    // Check for the X-MAS pattern
                    if (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S' &&
                        input[i + 1][j - 1] == 'M' && input[i - 1][j + 1] == 'S') {
                        count++;
                        Console.WriteLine($"Found X-MAS pattern at ({i},{j})");
                    }
                    if (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S' &&
                        input[i + 1][j - 1] == 'S' && input[i - 1][j + 1] == 'M') {
                        count++;
                        Console.WriteLine($"Found X-MAS pattern at ({i},{j})");
                    }
                    if (input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M' &&
                        input[i + 1][j - 1] == 'S' && input[i - 1][j + 1] == 'M') {
                        count++;
                        Console.WriteLine($"Found X-MAS pattern at ({i},{j})");
                    }
                    if (input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M' &&
                        input[i + 1][j - 1] == 'M' && input[i - 1][j + 1] == 'S') {
                        count++;
                        Console.WriteLine($"Found X-MAS pattern at ({i},{j})");
                    }
                }
            }
        }
        return count;
    }
}
