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
        var count = 0;

        // Get horizontal XMAS count
        count += input.Sum(x => x.CountMatches());

        // Get horizontal Reverse XMAS count
        count += input.Sum(x => x.CountMatchesReverse());

        // Get vertical XMAS count
        count += input.ConvertToMatrix().RotateClockWise().ConvertToLines().Sum(x => x.CountMatches());

        // Get vertical Reverse XMAS count
        count += input.ConvertToMatrix().RotateClockWise().ConvertToLines().Sum(x => x.CountMatchesReverse());

        // Get diagonal XMAS count
        count += input.ConvertToMatrix().RotateClockwise45().ConvertToLines().Sum(x => x.CountMatches());

        // Get diagonal Reverse XMAS count
        count += input.ConvertToMatrix().RotateClockwise45().ConvertToLines().Sum(x => x.CountMatchesReverse());

        // Get diagonal XMAS count
        count += input.ConvertToMatrix().RotateClockwise45(3).ConvertToLines().Sum(x => x.CountMatches());

        // Get diagonal Reverse XMAS count
        count += input.ConvertToMatrix().RotateClockwise45(3).ConvertToLines().Sum(x => x.CountMatchesReverse());

        return count;
    }

    public static int Part2(string[] input) {
        return 0;
    }

    private static int CountMatchesReverse(this string line) {
        return Regex.Matches(line, @"SAMX").Count;
    }

    private static int CountMatches(this string line) {
        return Regex.Matches(line, @"XMAS").Count;
    }

    private static char[,] RotateClockwise45(this char[,] matrix, int times = 1) {
        var newMatrix = matrix;
        
        for (var i = 0; i < times; i++) {
            newMatrix = Rotate();
        }

        return newMatrix;
        
        char[,] Rotate() {
            var n = matrix.GetLength(0); // Assuming a square matrix
            var newLength = 2 * n - 1;
            var rotated = new string[newLength];

            // Initialize the rotated array
            for (var i = 0; i < newLength; i++) {
                rotated[i] = new string(' ', newLength);
            }

            for (var i = 0; i < n; i++) {
                for (var j = 0; j < n; j++) {
                    var newRow = i + j;
                    var newCol = n - 1 + (j - i);

                    var rowChars = rotated[newRow].ToCharArray();
                    rowChars[newCol] = matrix[i, j];
                    rotated[newRow] = new string(rowChars);
                }
            }

            return rotated.ConvertToMatrix();
        }
    }

    private static char[,] ConvertToMatrix(this string[] lines) {
        var matrix = new char[lines.Length, lines.Length];
        for (var i = 0; i < lines.Length; i++) {
            for (var j = 0; j < lines.Length; j++) {
                matrix[i, j] = lines[i][j];
            }
        }
        return matrix;
    }
    
    private static string[] ConvertToLines(this char[,] matrix) {
        var lines = new string[matrix.GetLength(0)];
        for (var i = 0; i < matrix.GetLength(0); i++) {
            var sb = new StringBuilder();
            for (var j = 0; j < matrix.GetLength(1); j++) {
                sb.Append(matrix[i, j]);
            }
            lines[i] = sb.ToString();
        }
        return lines.Select(line => line.Replace(" ", "")).ToArray();
    }

    private static char[,] RotateClockWise(this char[,] matrix, int times = 1) {
        var newMatrix = matrix;
        
        for (var i = 0; i < times; i++) {
            newMatrix = Rotate();
        }

        return newMatrix;

        char[,] Rotate() {
            var n = matrix.GetLength(0);
            var rotated = new char[n, n];
            for (var i = 0; i < n; i++) {
                for (var j = 0; j < n; j++) {
                    rotated[i, j] = matrix[n - j - 1, i];
                }
            }

            return rotated;
        }
    }
}
