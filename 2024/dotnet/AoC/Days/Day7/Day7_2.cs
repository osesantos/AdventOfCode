using AoC.Utils;

namespace AoC.Days.Day7;

// source: https://anothercasualcoder.blogspot.com/2024/12/advent-of-code-day-7-2024-backtracking.html

public static class Day7_2 {

    private static string[] Lines => "Day7".GetInputLines();

    public static void Run() {
        Console.WriteLine("Day7");
        Console.WriteLine($"Part1 - {Part1(Lines)}");
        Console.WriteLine($"Part2 - {Part2(Lines)}");
    }

    // TODO - try to do this via recursive
    public static long Part1(string[] input) {
        long count = 0;
        foreach (var line in input) {
            var nums = line.GetNums();
            var result = line.GetResult();
            if (TryAll1(nums.Select(n => n.ToString()).ToArray(), result, "", 0)) {
                count += result;
            }
        }

        return count;
    }

    static bool TryAll1(string[] numbers, long target, string str, int index) {
        if (index == numbers.Length - 1) {
            str += numbers[index];
            return Evalute1(str, target);
        }

        char[] ops = new char[] { '+', '*' };

        foreach (char op in ops)
            if (TryAll1(numbers, target, str + numbers[index] + op.ToString(), index + 1))
                return true;

        return false;
    }

    static bool Evalute1(string exp, long target) {
        long result = 0;
        long part = 0;
        bool isSum = true;
        for (int i = 0; i < exp.Length; i++) {
            if (exp[i] >= '0' && exp[i] <= '9') {
                part = 10 * part + (int)(exp[i] - '0');
            }
            else {
                if (isSum) result += part;
                else result *= part;
                part = 0;
                isSum = exp[i] == '+';
            }

            if (result > target) return false;
        }

        if (isSum) result += part;
        else result *= part;

        return result == target;
    }

    private static long GetResult(this string line) {
        return long.Parse(line.Split(":").First());
    }

    private static int[] GetNums(this string line) {
        return line.Split(":").Last().Split(" ", StringSplitOptions.RemoveEmptyEntries).Select(int.Parse).ToArray();
    }

    public static long Part2(string[] input) {
        long count = 0;
        foreach (var line in input) {
            var nums = line.GetNums();
            var result = line.GetResult();
            if (TryAll2(nums.Select(n => n.ToString()).ToArray(), result, "", 0)) {
                count += result;
            }

            ;
        }

        return count;
    }

    static bool TryAll2(string[] numbers, long target, string str, int index) {
        if (index == numbers.Length - 1) {
            str += numbers[index];
            return Evalute2(str, target);
        }

        string[] ops = new string[] { "+", "*", "||" };

        foreach (string op in ops)
            if (TryAll2(numbers, target, str + numbers[index] + op, index + 1))
                return true;

        return false;
    }

    static bool Evalute2(string exp, long target) {
        long result = 0;
        long part = 0;
        int op = 0; //0: +, 1: *, 2: ||
        for (int i = 0; i < exp.Length; i++) {
            if (exp[i] >= '0' && exp[i] <= '9') {
                part = 10 * part + (int)(exp[i] - '0');
            }
            else {
                if (op == 0) result += part;
                else if (op == 1) result *= part;
                else {
                    string s1 = result.ToString();
                    string s2 = part.ToString();
                    string val = s1 + s2;
                    result = Int64.Parse(val);
                }

                part = 0;
                if (exp[i] == '+') op = 0;
                else if (exp[i] == '*') op = 1;
                else {
                    op = 2;
                    i++;
                }
            }

            if (result > target) return false;
        }

        if (op == 0) result += part;
        else if (op == 1) result *= part;
        else {
            string s1 = result.ToString();
            string s2 = part.ToString();
            string val = s1 + s2;
            result = Int64.Parse(val);
        }

        return result == target;
    }

}
