using System.Text.Json;
using AoC.Utils;

namespace AoC.Days.Day7;

/// <summary>
/// Day7 used for Day class generation
/// When using replace Day7 with the day
/// </summary>
public static class Day7 {

    private static string[] Lines => "Day7".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day7");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    // highest number of iterations in example
    // 11 + 6 + 16 + 20 | 11 * 6 + 16 + 20 | 11 + 6 * 16 + 20 | 11 + 6 + 16 * 20 | 11 + 6 * 16 * 20 | 11 * 6 * 16 + 20 | 11 * 6 + 16 * 20 | 11 * 6 * 16 * 20
    // 2^n => 2³ = 8 iterations
    // to brute force it we need to
    // 1. expand the nums list
    // 2. create all possible iterations with binary where 0 is + and 1 is *
    // 3. calculate all operations and check if any result match the total
    // example of binary for 2³ (8) -> 0 is 000 (+++), 1 is 001 (++*), ..., 6 is 110 (**+), 7 is 111 (***)
    public static long Part1(string[] input) {
        return input.GetOperations().ExpandOperations().SumValidOperations();
    }

    public static int Part2(string[] input) {
        return 0;
    }

    private class Operation {
        public int[] Nums { get; init; } = [];
        public int Op { get; init; }
        public long Result { get; init; }

        // 2^(n-1) where n is the number of numbers
        public int TotalCombinations => (int)Math.Pow(2, Nums.Length - 1);

        public bool IsValid() {
            var currentResult = Nums.First();
            var expression = $"{Nums.First()}";
            for (var i = 1; i < Nums.Length; i++) {
                // Op is a binary number where 0 is + and 1 is *
                // we can use the binary number to check if we should use multiplication or addition
                var useMultiplication = (Op & (1 << (i - 1))) != 0;
                if (useMultiplication) {
                    currentResult *= Nums[i];
                    expression += $" * {Nums[i]}";
                }
                else {
                    currentResult += Nums[i];
                    expression += $" + {Nums[i]}";
                }
            }

            if (currentResult == Result)
                Console.WriteLine($"Valid - {Result} = {expression}");

            return currentResult == Result;
        }
    }

    private static long SumValidOperations(this Operation[][] expandedOperations) {
        return expandedOperations.Sum(operations => operations.FirstOrDefault(o => o.IsValid())?.Result ?? 0);
    }

    private static Operation[] GetOperations(this string[] lines) {
        return lines.Select(line => new Operation() {
            Nums = line.GetNums(),
            Result = line.GetResult(),
        }).ToArray();
    }

    private static Operation[][] ExpandOperations(this Operation[] operations) {
        return operations.Select(op => {
            List<Operation> ops = [];
            for (var i = 0; i < op.TotalCombinations; i++) {
                ops.Add(new Operation() {
                    Nums = op.Nums,
                    Result = op.Result,
                    Op = i
                });
            }
            return ops.ToArray();
        }).ToArray();
    }

    private static long GetResult(this string line) {
        return long.Parse(line.Split(":").First());
    }

    private static int[] GetNums(this string line) {
        return line.Split(":").Last().Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray();
    }
}
